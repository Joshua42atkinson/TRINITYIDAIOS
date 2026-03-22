#!/usr/bin/env python3
"""
Trinity Voice Server — Web UI + Hands-Free Voice Pipeline
==========================================================

Single process: FastAPI web server + background audio loop.
Web UI on :7777 for voice selection, status, conversation log.
Audio runs in background thread: Wake Word → ASR → Brain → TTS → Speaker.

Usage:
  source ~/trinity-vllm-env/bin/activate
  CUDA_VISIBLE_DEVICES="" python3 trinity_voice_server.py
  Open http://localhost:7777
"""

import os
import sys
import time
import json
import threading
import traceback
import numpy as np
import pyaudio
import webrtcvad
import requests
import soundfile as sf
from pathlib import Path
from contextlib import asynccontextmanager

from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from fastapi.responses import HTMLResponse, JSONResponse
import uvicorn

# ─── Configuration ────────────────────────────────────────────────────────────

TRINITY_LLM_URL = os.environ.get("TRINITY_LLM_URL", "http://localhost:8080")
VOICE_SERVER_PORT = int(os.environ.get("TRINITY_VOICE_PORT", "7777"))
WHISPER_MODEL_SIZE = os.environ.get("TRINITY_WHISPER_MODEL", "base.en")
SAMPLE_RATE = 16000
CHANNELS = 1
CHUNK_SIZE = 480  # 30ms at 16kHz
FORMAT = pyaudio.paInt16
VAD_AGGRESSIVENESS = 1
SPEECH_PADDING_MS = 2500
MIN_SPEECH_MS = 500
MAX_SPEECH_S = 30
WAKE_WORD_THRESHOLD = 0.5
WAKE_BUFFER_SECONDS = 2  # seconds of audio to keep for ASR wake word check

WORKSPACE_ROOT = os.environ.get("TRINITY_WORKSPACE", os.path.expanduser(
    "~/Workflow/desktop_trinity/trinity-genesis"))

SYSTEM_PROMPT = (
    "You are Pete — the voice of Trinity, a hands-free AI coding IDE. "
    "You run on a local GMKtec EVO-X2 with 128GB RAM, fully offline. "
    "You speak like a gruff, warm Master Operator. Be direct, 1-3 sentences for voice. "
    "You use railroad metaphors: Coal (energy), Steam (focus), Drive Wheels (discipline). "
    "\n\n"
    "You have TOOLS to interact with the codebase. When the user asks you to read, edit, "
    "search, or run code, use the appropriate tool. Output ONE tool call per message.\n\n"
    "TOOL FORMAT — when you need a tool, output EXACTLY this and nothing else:\n"
    "<tool>name|argument</tool>\n\n"
    "TOOLS (use | to separate name from argument, NO parameter names):\n"
    "  <tool>read_file|scripts/launch/main.py</tool>\n"
    "  <tool>list_dir|scripts/launch</tool>\n"
    "  <tool>shell|cargo build 2>&1</tool>\n"
    "  <tool>search|fn main</tool>\n"
    "  <tool>git|status</tool>\n"
    "  <tool>git|diff --stat</tool>\n"
    "  <tool>edit_file|path|old text|new text</tool>\n"
    "  <tool>write_file|path|content</tool>\n\n"
    "Paths are relative to workspace root. After a tool result, summarize it concisely for voice.\n"
    "WORKSPACE: " + WORKSPACE_ROOT + "\n"
    "Keep spoken responses under 3 sentences — the user is LISTENING, not reading."
)

AVAILABLE_VOICES = {
    "am_adam": "Adam — Clear, professional",
    "am_echo": "Echo — Warm, resonant",
    "am_eric": "Eric — Steady, authoritative",
    "am_fenrir": "Fenrir — Deep, commanding",
    "am_liam": "Liam — Friendly, approachable",
    "am_michael": "Michael — Mature, grounded",
    "am_onyx": "Onyx — Deep, rich bass",
    "am_puck": "Puck — Energetic, expressive",
    "af_heart": "Heart — Warm female",
    "af_bella": "Bella — Clear female",
    "bm_daniel": "Daniel — British male",
    "bm_george": "George — British authoritative",
}

# ─── Shared State ─────────────────────────────────────────────────────────────

class PipelineState:
    def __init__(self):
        self.voice = os.environ.get("TRINITY_VOICE", "am_adam")
        self.running = False
        self.status = "stopped"
        self.last_user = ""
        self.last_reply = ""
        self.conversation = []
        self.ws_clients = set()
        self.lock = threading.Lock()
        # Pipeline components
        self.audio_interface = None
        self.tts_pipeline = None
        self.whisper_model = None
        self.wake_model = None
        self.is_speaking = False
        self.pipeline_thread = None
        self.active_prompt = None  # Set by wake word classifier (DEV_PROMPT or IRON_ROAD_PROMPT)

state = PipelineState()

# ─── WebSocket Broadcast ─────────────────────────────────────────────────────

_broadcast_queue = []
_broadcast_lock = threading.Lock()

def broadcast(event_type, data=None):
    """Thread-safe broadcast to WebSocket clients via polling queue."""
    msg = json.dumps({"type": event_type, "data": data or {}})
    with _broadcast_lock:
        _broadcast_queue.append(msg)

def log(level, msg):
    """Log + broadcast status."""
    ts = time.strftime("%H:%M:%S")
    print(f"[{ts}] [{level}] {msg}")
    broadcast("log", {"level": level, "msg": msg, "ts": ts})

def set_status(s):
    state.status = s
    broadcast("status", {"status": s, "voice": state.voice})

# ─── Model Init ──────────────────────────────────────────────────────────────

def init_models():
    """Load all pipeline models. Each component checked individually."""
    log("INIT", "Loading pipeline models...")

    # Audio
    if state.audio_interface is None:
        log("INIT", "PyAudio...")
        state.audio_interface = pyaudio.PyAudio()

    # Wake word — load ALL models as generic "hey" detector
    if state.wake_model is None:
        log("INIT", "Wake word (all models)...")
        import openwakeword
        from openwakeword.model import Model
        state.wake_model = Model(wakeword_model_paths=[])
        log("INIT", f"Wake word loaded: {list(state.wake_model.models.keys())}")

    # ASR
    if state.whisper_model is None:
        log("INIT", f"ASR (faster-whisper {WHISPER_MODEL_SIZE})...")
        from faster_whisper import WhisperModel
        state.whisper_model = WhisperModel(WHISPER_MODEL_SIZE, device="cpu", compute_type="int8")

    # TTS
    if state.tts_pipeline is None:
        log("INIT", "TTS (Kokoro 82M)...")
        from kokoro import KPipeline
        state.tts_pipeline = KPipeline(lang_code='a')

    log("INIT", "All models loaded.")

# ─── Audio Helpers ────────────────────────────────────────────────────────────

def play_beep(freq=880, duration=0.15):
    """Play a short confirmation beep so user knows to speak."""
    try:
        t = np.linspace(0, duration, int(24000 * duration), dtype=np.float32)
        tone = (np.sin(2 * np.pi * freq * t) * 0.3 * 32767).astype(np.int16)
        # Fade out last 20%
        fade_len = int(len(tone) * 0.2)
        tone[-fade_len:] = (tone[-fade_len:] * np.linspace(1, 0, fade_len)).astype(np.int16)
        stream = state.audio_interface.open(format=pyaudio.paInt16, channels=1, rate=24000, output=True)
        stream.write(tone.tobytes())
        stream.stop_stream()
        stream.close()
    except Exception as e:
        log("AUDIO", f"Beep error: {e}")

def play_audio_file(filepath):
    state.is_speaking = True
    try:
        data, sr = sf.read(filepath)
        audio_int16 = (data * 32767).astype(np.int16)
        stream = state.audio_interface.open(format=pyaudio.paInt16, channels=1, rate=sr, output=True)
        stream.write(audio_int16.tobytes())
        stream.stop_stream()
        stream.close()
    except Exception as e:
        log("AUDIO", f"Playback error: {e}")
    finally:
        state.is_speaking = False

def to_numpy(audio):
    """Convert audio chunk to numpy, handling both numpy arrays and torch tensors."""
    if hasattr(audio, 'numpy'):
        return audio.cpu().numpy() if hasattr(audio, 'cpu') else audio.numpy()
    return np.asarray(audio)

import re

def split_sentences(text):
    """Split text into speakable sentence chunks."""
    # Split on sentence boundaries but keep punctuation
    parts = re.split(r'(?<=[.!?])\s+', text.strip())
    # Merge very short fragments with the previous sentence
    merged = []
    for p in parts:
        p = p.strip()
        if not p:
            continue
        if merged and len(merged[-1]) < 20:
            merged[-1] += " " + p
        else:
            merged.append(p)
    return merged if merged else [text]

def speak(text):
    """Generate + play TTS audio sentence by sentence for low latency."""
    state.is_speaking = True
    t0 = time.time()
    sentences = split_sentences(text)
    log("TTS", f'Streaming {len(sentences)} sentence(s): "{text[:60]}..."')
    total_dur = 0.0
    stream_obj = None

    try:
        for i, sentence in enumerate(sentences):
            for gs, ps, audio in state.tts_pipeline(sentence, voice=state.voice):
                audio_np = to_numpy(audio)
                audio_int16 = (audio_np * 32767).astype(np.int16)
                if stream_obj is None:
                    first_t = time.time() - t0
                    log("TTS", f"First audio in {first_t:.2f}s")
                    stream_obj = state.audio_interface.open(
                        format=pyaudio.paInt16, channels=1, rate=24000, output=True
                    )
                stream_obj.write(audio_int16.tobytes())
                total_dur += len(audio_np) / 24000

        if stream_obj:
            stream_obj.stop_stream()
            stream_obj.close()
    except Exception as e:
        log("TTS", f"Error: {e}")
        traceback.print_exc()
    finally:
        state.is_speaking = False

    log("TTS", f"Done: {total_dur:.1f}s audio in {time.time()-t0:.1f}s")

def transcribe(audio_np):
    log("ASR", "Transcribing...")
    t0 = time.time()
    segments, _ = state.whisper_model.transcribe(audio_np, beam_size=5, language="en", vad_filter=True)
    text = " ".join(seg.text.strip() for seg in segments).strip()
    log("ASR", f'"{text}" ({time.time()-t0:.2f}s)')
    return text

# ─── Agentic Tools ────────────────────────────────────────────────────────────

import subprocess
import shlex

def resolve_path(p):
    """Resolve a path relative to workspace root."""
    p = p.strip().strip("'\"")
    if not os.path.isabs(p):
        p = os.path.join(WORKSPACE_ROOT, p)
    return os.path.normpath(p)

def tool_read_file(path):
    path = resolve_path(path)
    try:
        with open(path, "r") as f:
            content = f.read()
        lines = content.split("\n")
        if len(lines) > 100:
            return f"[{len(lines)} lines, showing first 100]\n" + "\n".join(f"{i+1}: {l}" for i, l in enumerate(lines[:100]))
        return "\n".join(f"{i+1}: {l}" for i, l in enumerate(lines))
    except Exception as e:
        return f"Error reading {path}: {e}"

def tool_write_file(path, content):
    path = resolve_path(path)
    try:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w") as f:
            f.write(content)
        return f"Written {len(content)} bytes to {path}"
    except Exception as e:
        return f"Error writing {path}: {e}"

def tool_edit_file(path, old, new):
    path = resolve_path(path)
    try:
        with open(path, "r") as f:
            content = f.read()
        if old not in content:
            return f"Error: old text not found in {path}"
        content = content.replace(old, new, 1)
        with open(path, "w") as f:
            f.write(content)
        return f"Edited {path}: replaced text successfully"
    except Exception as e:
        return f"Error editing {path}: {e}"

def tool_shell(command):
    try:
        result = subprocess.run(
            command, shell=True, capture_output=True, text=True,
            timeout=30, cwd=WORKSPACE_ROOT
        )
        output = result.stdout + result.stderr
        if len(output) > 3000:
            output = output[:3000] + "\n... [truncated]"
        return output or "(no output)"
    except subprocess.TimeoutExpired:
        return "Command timed out after 30 seconds"
    except Exception as e:
        return f"Error: {e}"

def tool_search(pattern):
    try:
        result = subprocess.run(
            ["grep", "-rn", "--include=*.rs", "--include=*.py", "--include=*.toml",
             "--include=*.md", "--include=*.sh", "--include=*.html", "--include=*.js",
             "-m", "20", pattern, WORKSPACE_ROOT],
            capture_output=True, text=True, timeout=15
        )
        output = result.stdout
        if len(output) > 3000:
            output = output[:3000] + "\n... [truncated]"
        return output or "No matches found"
    except Exception as e:
        return f"Error searching: {e}"

def tool_list_dir(path):
    path = resolve_path(path)
    try:
        entries = sorted(os.listdir(path))
        lines = []
        for e in entries[:50]:
            full = os.path.join(path, e)
            if os.path.isdir(full):
                lines.append(f"  {e}/")
            else:
                size = os.path.getsize(full)
                lines.append(f"  {e}  ({size:,} bytes)")
        if len(entries) > 50:
            lines.append(f"  ... and {len(entries)-50} more")
        return "\n".join(lines)
    except Exception as e:
        return f"Error listing {path}: {e}"

def tool_git(args):
    return tool_shell(f"git {args}")

def parse_tool_call(text):
    """Extract tool call from <tool>...</tool> tags. Supports both pipe and paren formats."""
    # Try pipe format first: <tool>name|args</tool>
    match = re.search(r'<tool>\s*(\w+)\|(.+?)\s*</tool>', text, re.DOTALL)
    if match:
        name = match.group(1)
        args_str = match.group(2).strip()
        return name, args_str, text.replace(match.group(0), "").strip()
    # Fallback: paren format <tool>name(args)</tool>
    match = re.search(r'<tool>\s*(\w+)\((.*?)\)\s*</tool>', text, re.DOTALL)
    if match:
        name = match.group(1)
        args_str = match.group(2).strip()
        return name, args_str, text.replace(match.group(0), "").strip()
    return None, None, None

def clean_arg(s):
    """Strip quotes, whitespace, and common LLM noise from a tool argument."""
    s = s.strip().strip("'\"")
    # Strip parameter name prefixes like 'path=' or 'command='
    if '=' in s and s.split('=')[0].isidentifier():
        s = s.split('=', 1)[1].strip().strip("'\"")
    return s

def execute_tool(name, args_str):
    """Execute a tool call and return the result."""
    log("TOOL", f"{name}({args_str[:80]})")
    try:
        # Split on pipe for multi-arg tools
        parts = [clean_arg(p) for p in args_str.split('|')]
        if name == "read_file":
            return tool_read_file(parts[0])
        elif name == "write_file":
            return tool_write_file(parts[0], parts[1] if len(parts) > 1 else "")
        elif name == "edit_file":
            if len(parts) < 3:
                return "Error: edit_file needs path|old_text|new_text"
            return tool_edit_file(parts[0], parts[1], parts[2])
        elif name == "shell":
            return tool_shell(parts[0])
        elif name == "search":
            return tool_search(parts[0])
        elif name == "list_dir":
            return tool_list_dir(parts[0])
        elif name == "git":
            return tool_git(parts[0])
        else:
            return f"Unknown tool: {name}"
    except Exception as e:
        return f"Tool error: {e}"

# ─── Brain ────────────────────────────────────────────────────────────────────

conversation_history = []

def llm_call(messages):
    """Single LLM call, returns content string."""
    resp = requests.post(
        f"{TRINITY_LLM_URL}/v1/chat/completions",
        json={
            "model": "default",
            "messages": messages,
            "max_tokens": 1024,
            "temperature": 0.7,
        },
        timeout=120,
    )
    resp.raise_for_status()
    msg = resp.json()["choices"][0]["message"]
    reply = (msg.get("content") or "").strip()
    if not reply:
        reasoning = (msg.get("reasoning_content") or "").strip()
        if reasoning:
            lines = [l.strip() for l in reasoning.split("\n") if l.strip() and not l.strip().startswith("-")]
            reply = lines[-1] if lines else "Give me a moment, Operator. That one needs more steam."
        else:
            reply = "Ran out of track on that one. Ask again and I'll route it better."
    return reply

def think(user_text):
    """Multi-turn agentic brain: LLM can call tools, get results, then summarize."""
    global conversation_history
    conversation_history.append({"role": "user", "content": user_text})
    if len(conversation_history) > 20:
        conversation_history = conversation_history[-20:]

    prompt = state.active_prompt or SYSTEM_PROMPT
    messages = [{"role": "system", "content": prompt}] + conversation_history
    t0 = time.time()
    max_tool_rounds = 5

    try:
        for round_i in range(max_tool_rounds):
            log("BRAIN", f"Thinking (round {round_i+1})...")
            reply = llm_call(messages)

            # Check if LLM wants to use a tool
            tool_name, tool_args, spoken_part = parse_tool_call(reply)
            if tool_name:
                set_status("tool:" + tool_name)
                result = execute_tool(tool_name, tool_args)
                log("TOOL", f"Result: {result[:100]}...")
                # Feed tool result back to LLM
                messages.append({"role": "assistant", "content": reply})
                messages.append({"role": "user", "content":
                    f"[TOOL RESULT from {tool_name}]:\n{result}\n\n"
                    "Now summarize the result concisely for the user (they're listening, not reading). "
                    "If you need another tool, use <tool> again. Otherwise just speak."
                })
                continue
            else:
                # No tool call — this is the final spoken response
                break

    except requests.exceptions.ConnectionError:
        reply = "The brain engine isn't running right now. Start it up and I'll be ready to think."
    except Exception as e:
        reply = f"Hit a snag in the engine room: {str(e)[:100]}"

    log("BRAIN", f'"{reply[:60]}..." ({time.time()-t0:.2f}s)')
    conversation_history.append({"role": "assistant", "content": reply})
    return reply

# ─── Wake Word (Dual Mode: "Hey Trinity" / "Hey Pete") ───────────────────────

DEV_PROMPT = SYSTEM_PROMPT  # Default: dev/coding mode

IRON_ROAD_PROMPT = (
    "You are Pete — the Master Operator of the Iron Network at Trinity ID AI OS. "
    "You speak like a gruff, warm Master Operator running a steam locomotive dispatch. "
    "You use railroad and steam engine metaphors naturally: Coal (energy/attention), "
    "Steam (cognitive focus/willpower), Drive Wheels (discipline), Governor (inner critic), "
    "Firebox (metabolic core), Boiler (emotional reserves). "
    "Guide the user through ADDIECRAPEYE stations disguised as waypoints on the track. "
    "Track their Coal, Steam, and XP. Flag Scope Creep as enemy encounters. "
    "Be direct. No platitudes. Keep voice responses under 3 sentences. "
    "When they stall, apply the Shunting Protocol: drop the heavy load, build momentum on light cargo first."
)

def classify_wake_word(audio_buffer):
    """Quick ASR on trigger audio to detect 'trinity' vs 'pete'."""
    if not audio_buffer or state.whisper_model is None:
        return "trinity"  # default to dev mode
    try:
        audio_np = np.concatenate(audio_buffer).astype(np.float32) / 32768.0
        segments, _ = state.whisper_model.transcribe(
            audio_np, beam_size=1, language="en", vad_filter=False
        )
        text = " ".join(seg.text.lower().strip() for seg in segments).strip()
        log("WAKE", f'Heard: "{text}"')
        if "pete" in text:
            return "pete"
        return "trinity"
    except Exception as e:
        log("WAKE", f"ASR classify error: {e}")
        return "trinity"

def wait_for_wake_word():
    """Listen for any wake word, then ASR-classify as 'trinity' or 'pete'."""
    set_status("listening")
    log("WAKE", "Listening for 'Hey Trinity' or 'Hey Pete'...")

    stream = state.audio_interface.open(
        format=FORMAT, channels=CHANNELS, rate=SAMPLE_RATE,
        input=True, frames_per_buffer=CHUNK_SIZE,
    )
    # Rolling buffer for ASR classification
    buf_max = int(WAKE_BUFFER_SECONDS * SAMPLE_RATE / CHUNK_SIZE)
    audio_buffer = []

    try:
        while state.running:
            if state.is_speaking:
                time.sleep(0.1)
                continue
            data = stream.read(CHUNK_SIZE, exception_on_overflow=False)
            audio_np = np.frombuffer(data, dtype=np.int16)
            audio_buffer.append(audio_np.copy())
            if len(audio_buffer) > buf_max:
                audio_buffer.pop(0)

            prediction = state.wake_model.predict(audio_np)
            for name, score in prediction.items():
                if score > WAKE_WORD_THRESHOLD:
                    log("WAKE", f"Trigger '{name}' ({score:.2f})")
                    state.wake_model.reset()
                    # Classify which wake word was actually said
                    mode = classify_wake_word(audio_buffer)
                    if mode == "pete":
                        log("WAKE", "Mode: Iron Road (Pete)")
                        state.active_prompt = IRON_ROAD_PROMPT
                    else:
                        log("WAKE", "Mode: Dev (Trinity)")
                        state.active_prompt = DEV_PROMPT
                    return True
    finally:
        stream.stop_stream()
        stream.close()
    return False

# ─── Record Speech ────────────────────────────────────────────────────────────

def record_speech():
    set_status("recording")
    log("REC", "Listening... speak now")

    vad = webrtcvad.Vad(VAD_AGGRESSIVENESS)
    stream = state.audio_interface.open(
        format=FORMAT, channels=CHANNELS, rate=SAMPLE_RATE,
        input=True, frames_per_buffer=CHUNK_SIZE,
    )
    frames = []
    speech_started = False
    silence_frames = 0
    speech_frames = 0
    max_frames = int(MAX_SPEECH_S * SAMPLE_RATE / CHUNK_SIZE)
    padding_frames = int(SPEECH_PADDING_MS * SAMPLE_RATE / (CHUNK_SIZE * 1000))
    min_speech_frames = int(MIN_SPEECH_MS * SAMPLE_RATE / (CHUNK_SIZE * 1000))

    try:
        for _ in range(max_frames):
            if not state.running:
                break
            data = stream.read(CHUNK_SIZE, exception_on_overflow=False)
            frames.append(data)
            is_speech = vad.is_speech(data, SAMPLE_RATE)
            if is_speech:
                speech_started = True
                speech_frames += 1
                silence_frames = 0
            elif speech_started:
                silence_frames += 1
                if silence_frames >= padding_frames:
                    if speech_frames >= min_speech_frames:
                        log("REC", f"Captured {speech_frames * CHUNK_SIZE / SAMPLE_RATE:.1f}s")
                        break
                    else:
                        speech_started = False
                        speech_frames = 0
                        silence_frames = 0
                        frames.clear()
    finally:
        stream.stop_stream()
        stream.close()

    if not frames:
        return None
    raw = b"".join(frames)
    return np.frombuffer(raw, dtype=np.int16).astype(np.float32) / 32768.0

# ─── Pipeline Loop ────────────────────────────────────────────────────────────

def pipeline_loop():
    """Main voice pipeline running in background thread."""
    try:
        init_models()
    except Exception as e:
        log("ERROR", f"Model init failed: {e}")
        traceback.print_exc()
        state.running = False
        set_status("error")
        return

    set_status("listening")
    log("READY", "Trinity is listening. Say 'Hey Trinity' or 'Hey Pete' to activate.")

    while state.running:
        try:
            # Phase 1: Wake word
            if not wait_for_wake_word():
                break

            # Phase 2: Acknowledge with beep
            set_status("acknowledged")
            play_beep()

            # Phase 3: Record
            audio_np = record_speech()
            if audio_np is None or len(audio_np) < SAMPLE_RATE * 0.3:
                log("REC", "No speech detected")
                set_status("listening")
                continue

            # Phase 4: Transcribe
            set_status("transcribing")
            user_text = transcribe(audio_np)
            if not user_text or len(user_text.strip()) < 2:
                log("ASR", "Empty transcription")
                set_status("listening")
                continue

            state.last_user = user_text
            broadcast("user_msg", {"text": user_text})
            state.conversation.append({"role": "user", "text": user_text, "ts": time.time()})

            # Phase 5: Think
            set_status("thinking")
            reply = think(user_text)
            state.last_reply = reply
            broadcast("assistant_msg", {"text": reply})
            state.conversation.append({"role": "assistant", "text": reply, "ts": time.time()})

            # Phase 6: Speak
            set_status("speaking")
            speak(reply)
            set_status("listening")

        except Exception as e:
            log("ERROR", str(e))
            traceback.print_exc()
            time.sleep(1)
            if state.running:
                set_status("listening")

    set_status("stopped")
    log("STOP", "Pipeline stopped.")

# ─── FastAPI App ──────────────────────────────────────────────────────────────

@asynccontextmanager
async def lifespan(app: FastAPI):
    import asyncio
    async def _ws_flush():
        """Flush broadcast queue to all WebSocket clients every 50ms."""
        while True:
            await asyncio.sleep(0.05)
            with _broadcast_lock:
                msgs = list(_broadcast_queue)
                _broadcast_queue.clear()
            if msgs and state.ws_clients:
                dead = set()
                for ws in list(state.ws_clients):
                    for m in msgs:
                        try:
                            await ws.send_text(m)
                        except Exception:
                            dead.add(ws)
                            break
                state.ws_clients -= dead
    task = asyncio.create_task(_ws_flush())
    yield
    task.cancel()
    state.running = False

app = FastAPI(title="Trinity Voice", lifespan=lifespan)

# CORS — allow Trinity web UI on :3000 to call voice server on :7777
from fastapi.middleware.cors import CORSMiddleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.get("/api/status")
def api_status():
    return {
        "status": state.status,
        "voice": state.voice,
        "running": state.running,
        "llm_url": TRINITY_LLM_URL,
        "conversation_count": len(state.conversation),
    }

@app.get("/api/voices")
def api_voices():
    return {"voices": AVAILABLE_VOICES, "current": state.voice}

@app.post("/api/voice/{voice_id}")
def api_set_voice(voice_id: str):
    if voice_id not in AVAILABLE_VOICES:
        return JSONResponse({"error": f"Unknown voice: {voice_id}"}, status_code=400)
    state.voice = voice_id
    log("VOICE", f"Switched to {voice_id} ({AVAILABLE_VOICES[voice_id]})")
    broadcast("status", {"status": state.status, "voice": state.voice})
    return {"voice": voice_id, "name": AVAILABLE_VOICES[voice_id]}

@app.post("/api/start")
def api_start():
    if state.running:
        return {"msg": "Already running"}
    state.running = True
    state.pipeline_thread = threading.Thread(target=pipeline_loop, daemon=True)
    state.pipeline_thread.start()
    return {"msg": "Pipeline started"}

@app.post("/api/stop")
def api_stop():
    state.running = False
    set_status("stopping")
    return {"msg": "Pipeline stopping"}

@app.post("/api/preview/{voice_id}")
def api_preview(voice_id: str):
    """Generate a short preview of a voice and play it."""
    if state.tts_pipeline is None:
        # Quick-load TTS only
        from kokoro import KPipeline
        state.tts_pipeline = KPipeline(lang_code='a')
        if state.audio_interface is None:
            state.audio_interface = pyaudio.PyAudio()

    def _preview():
        old_voice = state.voice
        state.voice = voice_id
        speak("Alright Operator, boiler's hot and the track is clear. Let's roll.")
        state.voice = old_voice

    t = threading.Thread(target=_preview, daemon=True)
    t.start()
    return {"msg": f"Previewing {voice_id}"}

@app.post("/api/say")
async def api_say(body: dict):
    """Type a message instead of speaking — for testing without mic."""
    text = body.get("text", "").strip()
    if not text:
        return JSONResponse({"error": "No text"}, status_code=400)

    if state.tts_pipeline is None:
        from kokoro import KPipeline
        state.tts_pipeline = KPipeline(lang_code='a')
        if state.audio_interface is None:
            state.audio_interface = pyaudio.PyAudio()

    state.last_user = text
    broadcast("user_msg", {"text": text})
    state.conversation.append({"role": "user", "text": text, "ts": time.time()})

    reply = think(text)
    state.last_reply = reply
    broadcast("assistant_msg", {"text": reply})
    state.conversation.append({"role": "assistant", "text": reply, "ts": time.time()})

    def _speak():
        speak(reply)
    threading.Thread(target=_speak, daemon=True).start()

    return {"reply": reply}

@app.post("/api/tts/generate")
async def api_tts_generate(body: dict):
    """Generate TTS audio and return as WAV bytes (for browser playback).
    POST {"text": "Hello world", "voice": "am_adam"}
    Returns: audio/wav binary
    """
    text = body.get("text", "").strip()
    if not text:
        return JSONResponse({"error": "No text"}, status_code=400)

    voice = body.get("voice", state.voice)

    if state.tts_pipeline is None:
        from kokoro import KPipeline
        state.tts_pipeline = KPipeline(lang_code='a')

    import io
    import struct

    t0 = time.time()
    audio_chunks = []
    for gs, ps, audio in state.tts_pipeline(text, voice=voice):
        audio_np = to_numpy(audio)
        audio_chunks.append(audio_np)

    if not audio_chunks:
        return JSONResponse({"error": "TTS produced no audio"}, status_code=500)

    full_audio = np.concatenate(audio_chunks)
    audio_int16 = (full_audio * 32767).astype(np.int16)

    # Build WAV in memory
    buf = io.BytesIO()
    sample_rate = 24000
    num_samples = len(audio_int16)
    data_size = num_samples * 2  # 16-bit = 2 bytes per sample
    buf.write(b'RIFF')
    buf.write(struct.pack('<I', 36 + data_size))
    buf.write(b'WAVE')
    buf.write(b'fmt ')
    buf.write(struct.pack('<IHHIIHH', 16, 1, 1, sample_rate, sample_rate * 2, 2, 16))
    buf.write(b'data')
    buf.write(struct.pack('<I', data_size))
    buf.write(audio_int16.tobytes())

    log("TTS", f"Generated {num_samples/sample_rate:.1f}s audio in {time.time()-t0:.2f}s for browser")

    from fastapi.responses import Response as FastResponse
    return FastResponse(
        content=buf.getvalue(),
        media_type="audio/wav",
        headers={
            "X-Audio-Duration": f"{num_samples/sample_rate:.2f}",
            "X-Generation-Time": f"{time.time()-t0:.2f}",
            "Access-Control-Allow-Origin": "*",
        }
    )

@app.get("/api/conversation")
def api_conversation():
    return {"messages": state.conversation[-50:]}

@app.post("/api/clear")
def api_clear():
    global conversation_history
    state.conversation.clear()
    conversation_history.clear()
    state.last_user = ""
    state.last_reply = ""
    return {"msg": "Conversation cleared"}

@app.websocket("/ws")
async def websocket_endpoint(ws: WebSocket):
    await ws.accept()
    state.ws_clients.add(ws)
    await ws.send_text(json.dumps({
        "type": "status",
        "data": {"status": state.status, "voice": state.voice}
    }))
    try:
        while True:
            await ws.receive_text()
    except WebSocketDisconnect:
        state.ws_clients.discard(ws)

# ─── Web UI ───────────────────────────────────────────────────────────────────

@app.get("/", response_class=HTMLResponse)
def index():
    return HTML_PAGE

HTML_PAGE = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Trinity Voice</title>
<style>
  :root {
    --bg: #0a0a0f; --surface: #14141f; --border: #2a2a3a;
    --text: #e0e0e8; --dim: #888899; --accent: #6c5ce7;
    --green: #00b894; --amber: #fdcb6e; --red: #e17055;
    --blue: #74b9ff;
  }
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body {
    font-family: 'SF Mono', 'Cascadia Code', 'Fira Code', monospace;
    background: var(--bg); color: var(--text);
    min-height: 100vh;
  }
  .container { max-width: 900px; margin: 0 auto; padding: 1rem; }

  /* Header */
  .header {
    display: flex; align-items: center; justify-content: space-between;
    padding: 1rem 0; border-bottom: 1px solid var(--border); margin-bottom: 1rem;
  }
  .header h1 { font-size: 1.4rem; color: var(--accent); }
  .header h1 span { color: var(--dim); font-weight: 300; }

  /* Status bar */
  .status-bar {
    display: flex; align-items: center; gap: 0.75rem;
    padding: 0.75rem 1rem; background: var(--surface);
    border: 1px solid var(--border); border-radius: 8px;
    margin-bottom: 1rem;
  }
  .status-dot {
    width: 10px; height: 10px; border-radius: 50%;
    background: var(--dim); flex-shrink: 0;
  }
  .status-dot.listening { background: var(--green); animation: pulse 2s infinite; }
  .status-dot.recording { background: var(--red); animation: pulse 0.5s infinite; }
  .status-dot.transcribing, .status-dot.thinking { background: var(--amber); animation: pulse 1s infinite; }
  .status-dot.speaking { background: var(--blue); animation: pulse 0.8s infinite; }
  .status-dot.stopped { background: var(--dim); }
  @keyframes pulse { 0%,100% { opacity:1; } 50% { opacity:0.4; } }
  .status-text { font-size: 0.85rem; color: var(--dim); flex: 1; }
  .status-text strong { color: var(--text); }

  /* Controls row */
  .controls {
    display: flex; gap: 0.5rem; flex-wrap: wrap;
    margin-bottom: 1rem;
  }
  button {
    padding: 0.5rem 1rem; border: 1px solid var(--border);
    background: var(--surface); color: var(--text); border-radius: 6px;
    cursor: pointer; font-family: inherit; font-size: 0.8rem;
    transition: all 0.15s;
  }
  button:hover { border-color: var(--accent); background: #1a1a2e; }
  button.primary { background: var(--accent); border-color: var(--accent); color: #fff; }
  button.primary:hover { background: #5a4bd6; }
  button.danger { border-color: var(--red); color: var(--red); }
  button.danger:hover { background: #2a1515; }

  /* Voice selector */
  .voice-section {
    background: var(--surface); border: 1px solid var(--border);
    border-radius: 8px; padding: 1rem; margin-bottom: 1rem;
  }
  .voice-section h2 { font-size: 0.9rem; color: var(--dim); margin-bottom: 0.75rem; }
  .voice-grid {
    display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 0.5rem;
  }
  .voice-card {
    padding: 0.6rem 0.8rem; border: 1px solid var(--border);
    border-radius: 6px; cursor: pointer; transition: all 0.15s;
    display: flex; justify-content: space-between; align-items: center;
  }
  .voice-card:hover { border-color: var(--accent); background: #1a1a2e; }
  .voice-card.active { border-color: var(--green); background: #0a2a1a; }
  .voice-card .name { font-size: 0.85rem; font-weight: 600; }
  .voice-card .desc { font-size: 0.7rem; color: var(--dim); }
  .voice-card .preview-btn {
    font-size: 0.7rem; padding: 0.2rem 0.5rem; border-radius: 4px;
    background: none; border: 1px solid var(--border); color: var(--dim);
    cursor: pointer;
  }
  .voice-card .preview-btn:hover { color: var(--accent); border-color: var(--accent); }

  /* Chat */
  .chat {
    background: var(--surface); border: 1px solid var(--border);
    border-radius: 8px; min-height: 300px; max-height: 500px;
    overflow-y: auto; padding: 1rem; margin-bottom: 1rem;
  }
  .msg { margin-bottom: 0.75rem; padding: 0.6rem 0.8rem; border-radius: 6px; }
  .msg.user { background: #1a1a2e; border-left: 3px solid var(--accent); }
  .msg.assistant { background: #0a2a1a; border-left: 3px solid var(--green); }
  .msg .role { font-size: 0.7rem; color: var(--dim); margin-bottom: 0.25rem; }
  .msg .text { font-size: 0.85rem; line-height: 1.5; }

  /* Text input */
  .input-row { display: flex; gap: 0.5rem; margin-bottom: 1rem; }
  .input-row input {
    flex: 1; padding: 0.6rem 0.8rem; background: var(--surface);
    border: 1px solid var(--border); border-radius: 6px;
    color: var(--text); font-family: inherit; font-size: 0.85rem;
  }
  .input-row input:focus { outline: none; border-color: var(--accent); }

  /* Log */
  .log {
    background: #080810; border: 1px solid var(--border);
    border-radius: 8px; padding: 0.75rem; max-height: 200px;
    overflow-y: auto; font-size: 0.72rem; color: var(--dim);
  }
  .log-line { margin-bottom: 0.15rem; }
  .log-line .ts { color: #555; }
  .log-line .lvl { font-weight: 600; }
  .log-line .lvl.WAKE { color: var(--amber); }
  .log-line .lvl.ASR { color: var(--blue); }
  .log-line .lvl.TTS { color: var(--green); }
  .log-line .lvl.BRAIN { color: var(--accent); }
  .log-line .lvl.ERROR { color: var(--red); }
  .log-line .lvl.INIT { color: var(--dim); }
</style>
</head>
<body>
<div class="container">

<div class="header">
  <h1>TRINITY <span>VOICE</span></h1>
  <div style="font-size:0.75rem; color:var(--dim)">
    LLM: <span id="llm-url">""" + TRINITY_LLM_URL + """</span>
  </div>
</div>

<div class="status-bar">
  <div class="status-dot" id="status-dot"></div>
  <div class="status-text">
    <strong id="status-label">Stopped</strong> &mdash;
    Voice: <span id="current-voice">am_adam</span>
  </div>
</div>

<div class="controls">
  <button class="primary" id="btn-start" onclick="startPipeline()">Start Listening</button>
  <button class="danger" id="btn-stop" onclick="stopPipeline()" style="display:none">Stop</button>
  <button onclick="clearChat()">Clear Chat</button>
</div>

<div class="voice-section">
  <h2>VOICE SELECT <span style="font-weight:300">&mdash; click to switch, speaker icon to preview</span></h2>
  <div class="voice-grid" id="voice-grid"></div>
</div>

<div class="chat" id="chat"></div>

<div class="input-row">
  <input type="text" id="text-input" placeholder="Type a message (or use voice)..." />
  <button onclick="sendText()">Send</button>
</div>

<div class="log" id="log"></div>

</div>
<script>
const API = '';
let ws = null;
let currentVoice = 'am_adam';

// ─── API Calls ────────────────────────────────────────────────────────
async function startPipeline() {
  await fetch(API + '/api/start', {method:'POST'});
  document.getElementById('btn-start').style.display = 'none';
  document.getElementById('btn-stop').style.display = '';
}
async function stopPipeline() {
  await fetch(API + '/api/stop', {method:'POST'});
  document.getElementById('btn-start').style.display = '';
  document.getElementById('btn-stop').style.display = 'none';
}
async function setVoice(v) {
  await fetch(API + '/api/voice/' + v, {method:'POST'});
  currentVoice = v;
  renderVoices();
}
async function previewVoice(v, e) {
  e.stopPropagation();
  await fetch(API + '/api/preview/' + v, {method:'POST'});
}
async function sendText() {
  const input = document.getElementById('text-input');
  const text = input.value.trim();
  if (!text) return;
  input.value = '';
  addMsg('user', text);
  const resp = await fetch(API + '/api/say', {
    method:'POST', headers:{'Content-Type':'application/json'},
    body: JSON.stringify({text})
  });
  const data = await resp.json();
  if (data.reply) addMsg('assistant', data.reply);
}
async function clearChat() {
  await fetch(API + '/api/clear', {method:'POST'});
  document.getElementById('chat').innerHTML = '';
}

// ─── WebSocket ────────────────────────────────────────────────────────
function connectWS() {
  const proto = location.protocol === 'https:' ? 'wss' : 'ws';
  ws = new WebSocket(proto + '://' + location.host + '/ws');
  ws.onmessage = (e) => {
    const msg = JSON.parse(e.data);
    if (msg.type === 'status') updateStatus(msg.data);
    if (msg.type === 'log') addLog(msg.data);
    if (msg.type === 'user_msg') addMsg('user', msg.data.text);
    if (msg.type === 'assistant_msg') addMsg('assistant', msg.data.text);
  };
  ws.onclose = () => setTimeout(connectWS, 2000);
}

function updateStatus(d) {
  const dot = document.getElementById('status-dot');
  dot.className = 'status-dot ' + (d.status || 'stopped');
  const labels = {
    stopped:'Stopped', listening:'Listening for wake word...',
    recording:'Recording speech...', transcribing:'Transcribing...',
    thinking:'Thinking...', speaking:'Speaking...', acknowledged:'Acknowledged',
    stopping:'Stopping...', error:'Error'
  };
  document.getElementById('status-label').textContent = labels[d.status] || d.status;
  if (d.voice) {
    currentVoice = d.voice;
    document.getElementById('current-voice').textContent = d.voice;
    renderVoices();
  }
}

function addMsg(role, text) {
  const chat = document.getElementById('chat');
  const div = document.createElement('div');
  div.className = 'msg ' + role;
  div.innerHTML = '<div class="role">' + (role==='user'?'You':'Pete') + '</div><div class="text">' + escHtml(text) + '</div>';
  chat.appendChild(div);
  chat.scrollTop = chat.scrollHeight;
}

function addLog(d) {
  const log = document.getElementById('log');
  const div = document.createElement('div');
  div.className = 'log-line';
  div.innerHTML = '<span class="ts">' + d.ts + '</span> <span class="lvl ' + d.level + '">[' + d.level + ']</span> ' + escHtml(d.msg);
  log.appendChild(div);
  log.scrollTop = log.scrollHeight;
}

function escHtml(s) { const d=document.createElement('div'); d.textContent=s; return d.innerHTML; }

// ─── Voice Grid ───────────────────────────────────────────────────────
const VOICES = """ + json.dumps(AVAILABLE_VOICES) + """;

function renderVoices() {
  const grid = document.getElementById('voice-grid');
  grid.innerHTML = '';
  for (const [id, desc] of Object.entries(VOICES)) {
    const card = document.createElement('div');
    card.className = 'voice-card' + (id === currentVoice ? ' active' : '');
    card.onclick = () => setVoice(id);
    const parts = desc.split(' — ');
    card.innerHTML = `
      <div>
        <div class="name">${parts[0]}</div>
        <div class="desc">${parts[1] || ''}</div>
      </div>
      <button class="preview-btn" onclick="previewVoice('${id}', event)">&#9654;</button>
    `;
    grid.appendChild(card);
  }
}

// ─── Init ─────────────────────────────────────────────────────────────
document.getElementById('text-input').addEventListener('keydown', (e) => {
  if (e.key === 'Enter') sendText();
});

(async function init() {
  const resp = await fetch(API + '/api/status');
  const data = await resp.json();
  updateStatus(data);
  if (data.running) {
    document.getElementById('btn-start').style.display = 'none';
    document.getElementById('btn-stop').style.display = '';
  }
  // Load conversation
  const conv = await fetch(API + '/api/conversation');
  const cd = await conv.json();
  for (const m of cd.messages) addMsg(m.role, m.text);

  renderVoices();
  connectWS();
})();
</script>
</body>
</html>"""

# ─── Entry Point ──────────────────────────────────────────────────────────────

if __name__ == "__main__":
    os.environ["CUDA_VISIBLE_DEVICES"] = ""
    print(f"\n  Trinity Voice Server → http://localhost:{VOICE_SERVER_PORT}")
    print(f"  LLM Brain → {TRINITY_LLM_URL}\n")
    uvicorn.run(app, host="0.0.0.0", port=VOICE_SERVER_PORT, log_level="warning")
