#!/usr/bin/env python3
"""
Trinity Voice Pipeline — Professional Hands-Free Voice System
=============================================================

Architecture (same as Alexa/Google Home/Domino's AI Phone):
  Wake Word → ASR → Brain (LLM) → TTS → Speaker → Loop

Components:
  - Wake Word: openwakeword (hey_jarvis placeholder, ~2MB ONNX)
  - ASR: faster-whisper (base.en model, CPU)
  - Brain: Any OpenAI-compatible API (llama.cpp, vLLM, etc.)
  - TTS: Kokoro 82M (CPU, real-time on Zen5 AVX-512)
  - Audio I/O: PyAudio + WebRTC VAD

Usage:
  source ~/trinity-vllm-env/bin/activate
  CUDA_VISIBLE_DEVICES="" python3 trinity_voice_pipeline.py

Requires LLM server running on TRINITY_LLM_URL (default http://localhost:8080)
"""

import os
import sys
import time
import json
import wave
import struct
import threading
import numpy as np
import pyaudio
import webrtcvad
import requests
import soundfile as sf
from pathlib import Path

# ─── Configuration ────────────────────────────────────────────────────────────

TRINITY_LLM_URL = os.environ.get("TRINITY_LLM_URL", "http://localhost:8080")
TRINITY_VOICE = os.environ.get("TRINITY_VOICE", "am_adam")
WAKE_WORD_MODEL = os.environ.get("TRINITY_WAKE_WORD", "hey_jarvis")
WHISPER_MODEL = os.environ.get("TRINITY_WHISPER_MODEL", "base.en")

# Audio parameters
SAMPLE_RATE = 16000
CHANNELS = 1
CHUNK_SIZE = 480  # 30ms at 16kHz (required by WebRTC VAD)
FORMAT = pyaudio.paInt16

# VAD parameters
VAD_AGGRESSIVENESS = 2       # 0-3 (higher = more aggressive noise filtering)
SPEECH_PADDING_MS = 1500     # Silence after speech before triggering send (1.5s)
MIN_SPEECH_MS = 300          # Minimum speech duration to process
MAX_SPEECH_S = 30            # Maximum recording duration

# Wake word parameters
WAKE_WORD_THRESHOLD = 0.5    # Confidence threshold for wake word detection

# Pre-cached acknowledgment audio
ACK_AUDIO_PATH = os.path.expanduser("~/trinity-models/tts/kokoro_ack_ready.wav")

# Pete's system prompt
SYSTEM_PROMPT = (
    "You are Pete — the Operating System of the Iron Network at Trinity ID AI OS. "
    "You speak like a gruff, warm Master Operator running a steam locomotive dispatch. "
    "You use railroad and steam engine metaphors naturally: Coal (energy/attention), "
    "Steam (cognitive focus/willpower), Drive Wheels (discipline), Governor (inner critic), "
    "Firebox (metabolic core), Boiler (emotional reserves). "
    "Keep voice responses under 3 sentences. Be direct. No platitudes. "
    "Treat the user as a capable Operator who needs calibration, not coddling."
)

# ─── Globals ──────────────────────────────────────────────────────────────────

audio_interface = None
tts_pipeline = None
whisper_model = None
wake_model = None
is_speaking = False  # Echo suppression flag


# ─── Initialization ──────────────────────────────────────────────────────────

def init_tts():
    """Load Kokoro TTS (82M) on CPU."""
    global tts_pipeline
    print("[TTS] Loading Kokoro 82M...")
    t0 = time.time()
    from kokoro import KPipeline
    tts_pipeline = KPipeline(lang_code='a')
    print(f"[TTS] Loaded in {time.time()-t0:.1f}s — voice: {TRINITY_VOICE}")


def init_asr():
    """Load faster-whisper ASR model on CPU."""
    global whisper_model
    print(f"[ASR] Loading faster-whisper {WHISPER_MODEL}...")
    t0 = time.time()
    from faster_whisper import WhisperModel
    whisper_model = WhisperModel(WHISPER_MODEL, device="cpu", compute_type="int8")
    print(f"[ASR] Loaded in {time.time()-t0:.1f}s")


def init_wake_word():
    """Load openwakeword detector."""
    global wake_model
    print(f"[WAKE] Loading openwakeword ({WAKE_WORD_MODEL})...")
    t0 = time.time()
    import openwakeword
    from openwakeword.model import Model
    # Resolve built-in model names to full paths
    all_paths = openwakeword.get_pretrained_model_paths()
    matching = [p for p in all_paths if WAKE_WORD_MODEL in p]
    if matching:
        wake_model = Model(wakeword_model_paths=matching)
    else:
        # Try as literal path or load all
        wake_model = Model(wakeword_model_paths=[WAKE_WORD_MODEL] if os.path.exists(WAKE_WORD_MODEL) else [])
    print(f"[WAKE] Loaded in {time.time()-t0:.1f}s")


def init_audio():
    """Initialize PyAudio."""
    global audio_interface
    audio_interface = pyaudio.PyAudio()


# ─── Audio Helpers ────────────────────────────────────────────────────────────

def play_audio_file(filepath):
    """Play a WAV file through the speakers. Sets is_speaking flag for echo suppression."""
    global is_speaking
    is_speaking = True
    try:
        data, sr = sf.read(filepath)
        # Convert to int16
        audio_int16 = (data * 32767).astype(np.int16)
        stream = audio_interface.open(
            format=pyaudio.paInt16,
            channels=1,
            rate=sr,
            output=True,
        )
        stream.write(audio_int16.tobytes())
        stream.stop_stream()
        stream.close()
    except Exception as e:
        print(f"[AUDIO] Playback error: {e}")
    finally:
        is_speaking = False


def play_audio_array(audio_np, sr=24000):
    """Play a numpy audio array through the speakers."""
    global is_speaking
    is_speaking = True
    try:
        audio_int16 = (audio_np * 32767).astype(np.int16)
        stream = audio_interface.open(
            format=pyaudio.paInt16,
            channels=1,
            rate=sr,
            output=True,
        )
        stream.write(audio_int16.tobytes())
        stream.stop_stream()
        stream.close()
    except Exception as e:
        print(f"[AUDIO] Playback error: {e}")
    finally:
        is_speaking = False


def speak(text):
    """Generate speech with Kokoro TTS and play it."""
    global is_speaking
    print(f"[TTS] Generating: \"{text[:80]}{'...' if len(text) > 80 else ''}\"")
    t0 = time.time()

    all_audio = []
    first_chunk_played = False

    for gs, ps, audio in tts_pipeline(text, voice=TRINITY_VOICE):
        all_audio.append(audio)

        # Stream first chunk immediately for low perceived latency
        if not first_chunk_played:
            first_chunk_time = time.time() - t0
            print(f"[TTS] First chunk in {first_chunk_time:.2f}s")
            is_speaking = True
            audio_int16 = (audio * 32767).astype(np.int16)
            stream = audio_interface.open(
                format=pyaudio.paInt16, channels=1, rate=24000, output=True
            )
            stream.write(audio_int16.tobytes())
            first_chunk_played = True
            continue

        # Play subsequent chunks
        audio_int16 = (audio * 32767).astype(np.int16)
        stream.write(audio_int16.tobytes())

    if first_chunk_played:
        stream.stop_stream()
        stream.close()

    is_speaking = False
    total_time = time.time() - t0
    if all_audio:
        total_audio = np.concatenate(all_audio)
        duration = len(total_audio) / 24000
        print(f"[TTS] Done: {duration:.1f}s audio in {total_time:.1f}s (RTF: {total_time/duration:.2f}x)")


# ─── Wake Word Detection ─────────────────────────────────────────────────────

def wait_for_wake_word():
    """Listen continuously for the wake word. Returns when detected."""
    print(f"\n[WAKE] Listening for '{WAKE_WORD_MODEL}'... (say 'Hey Jarvis')")

    stream = audio_interface.open(
        format=FORMAT,
        channels=CHANNELS,
        rate=SAMPLE_RATE,
        input=True,
        frames_per_buffer=CHUNK_SIZE,
    )

    try:
        while True:
            if is_speaking:
                time.sleep(0.1)
                continue

            audio_data = stream.read(CHUNK_SIZE, exception_on_overflow=False)
            audio_np = np.frombuffer(audio_data, dtype=np.int16)

            # openwakeword expects int16 numpy arrays
            prediction = wake_model.predict(audio_np)

            for model_name, score in prediction.items():
                if score > WAKE_WORD_THRESHOLD:
                    print(f"[WAKE] Detected '{model_name}' (confidence: {score:.2f})")
                    wake_model.reset()
                    return
    finally:
        stream.stop_stream()
        stream.close()


# ─── Speech Recording with VAD ───────────────────────────────────────────────

def record_speech():
    """Record speech using VAD until silence is detected. Returns raw audio bytes."""
    print("[REC] Listening... (speak now)")

    vad = webrtcvad.Vad(VAD_AGGRESSIVENESS)
    stream = audio_interface.open(
        format=FORMAT,
        channels=CHANNELS,
        rate=SAMPLE_RATE,
        input=True,
        frames_per_buffer=CHUNK_SIZE,
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
            audio_data = stream.read(CHUNK_SIZE, exception_on_overflow=False)
            frames.append(audio_data)

            is_speech = vad.is_speech(audio_data, SAMPLE_RATE)

            if is_speech:
                speech_started = True
                speech_frames += 1
                silence_frames = 0
            elif speech_started:
                silence_frames += 1
                if silence_frames >= padding_frames:
                    if speech_frames >= min_speech_frames:
                        print(f"[REC] Captured {speech_frames * CHUNK_SIZE / SAMPLE_RATE:.1f}s of speech")
                        break
                    else:
                        # Too short, reset
                        speech_started = False
                        speech_frames = 0
                        silence_frames = 0
                        frames.clear()
    finally:
        stream.stop_stream()
        stream.close()

    if not frames:
        return None

    # Convert to numpy
    raw_audio = b"".join(frames)
    audio_np = np.frombuffer(raw_audio, dtype=np.int16).astype(np.float32) / 32768.0
    return audio_np


# ─── ASR (Speech to Text) ────────────────────────────────────────────────────

def transcribe(audio_np):
    """Transcribe audio numpy array to text using faster-whisper."""
    print("[ASR] Transcribing...")
    t0 = time.time()

    segments, info = whisper_model.transcribe(
        audio_np,
        beam_size=5,
        language="en",
        vad_filter=True,
    )

    text = " ".join(seg.text.strip() for seg in segments).strip()
    elapsed = time.time() - t0
    print(f"[ASR] \"{text}\" ({elapsed:.2f}s)")
    return text


# ─── Brain (LLM) ─────────────────────────────────────────────────────────────

conversation_history = []

def think(user_text):
    """Send text to the LLM brain and get a response."""
    global conversation_history

    conversation_history.append({"role": "user", "content": user_text})

    # Keep conversation history manageable (last 10 turns)
    if len(conversation_history) > 20:
        conversation_history = conversation_history[-20:]

    messages = [{"role": "system", "content": SYSTEM_PROMPT}] + conversation_history

    print(f"[BRAIN] Thinking...")
    t0 = time.time()

    try:
        resp = requests.post(
            f"{TRINITY_LLM_URL}/v1/chat/completions",
            json={
                "model": "default",
                "messages": messages,
                "max_tokens": 256,
                "temperature": 0.7,
                "stream": False,
            },
            timeout=30,
        )
        resp.raise_for_status()
        data = resp.json()
        reply = data["choices"][0]["message"]["content"].strip()
    except requests.exceptions.ConnectionError:
        reply = "The brain engine isn't running right now. Start it up and I'll be ready to think."
    except Exception as e:
        reply = f"Hit a snag in the engine room: {str(e)[:100]}"

    elapsed = time.time() - t0
    print(f"[BRAIN] \"{reply[:80]}{'...' if len(reply) > 80 else ''}\" ({elapsed:.2f}s)")

    conversation_history.append({"role": "assistant", "content": reply})
    return reply


# ─── Main Pipeline ───────────────────────────────────────────────────────────

def run_pipeline():
    """Main voice pipeline loop."""
    print()
    print("=" * 60)
    print("  TRINITY VOICE PIPELINE")
    print("  Professional Hands-Free Voice System")
    print("=" * 60)
    print(f"  Wake Word:  {WAKE_WORD_MODEL}")
    print(f"  ASR:        faster-whisper {WHISPER_MODEL}")
    print(f"  Brain:      {TRINITY_LLM_URL}")
    print(f"  TTS:        Kokoro 82M ({TRINITY_VOICE})")
    print(f"  Echo Sup:   Mic muted during playback")
    print("=" * 60)

    # Pre-load all models
    init_audio()
    init_wake_word()
    init_asr()
    init_tts()

    # Pre-generate acknowledgment if not cached
    ack_path = ACK_AUDIO_PATH
    if not os.path.exists(ack_path):
        print("[INIT] Generating acknowledgment audio...")
        all_audio = []
        for gs, ps, audio in tts_pipeline("I'm here. Go ahead.", voice=TRINITY_VOICE):
            all_audio.append(audio)
        full = np.concatenate(all_audio)
        os.makedirs(os.path.dirname(ack_path), exist_ok=True)
        sf.write(ack_path, full, 24000)
        print(f"[INIT] Saved acknowledgment to {ack_path}")

    print("\n[READY] Trinity is listening. Say 'Hey Jarvis' to activate.\n")

    while True:
        try:
            # Phase 1: Wait for wake word
            wait_for_wake_word()

            # Phase 2: Instant acknowledgment (pre-cached, zero latency)
            play_thread = threading.Thread(target=play_audio_file, args=(ack_path,))
            play_thread.start()

            # Phase 3: Record user speech (overlaps with ack playback)
            # Wait for ack to finish before recording to avoid echo
            play_thread.join()

            audio_np = record_speech()
            if audio_np is None or len(audio_np) < SAMPLE_RATE * 0.3:
                print("[REC] No speech detected, returning to wake word.")
                continue

            # Phase 4: Transcribe
            user_text = transcribe(audio_np)
            if not user_text or len(user_text.strip()) < 2:
                print("[ASR] Empty transcription, returning to wake word.")
                continue

            # Phase 5: Think
            reply = think(user_text)

            # Phase 6: Speak
            speak(reply)

            # Phase 7: Quick follow-up mode (listen for more without wake word)
            print("[FOLLOW] Listening for follow-up (5s window)...")
            follow_up = record_follow_up(timeout_s=5)
            if follow_up is not None:
                user_text = transcribe(follow_up)
                if user_text and len(user_text.strip()) >= 2:
                    reply = think(user_text)
                    speak(reply)

        except KeyboardInterrupt:
            print("\n[EXIT] Trinity voice pipeline shutting down.")
            break
        except Exception as e:
            print(f"[ERROR] {e}")
            time.sleep(1)

    # Cleanup
    if audio_interface:
        audio_interface.terminate()


def record_follow_up(timeout_s=5):
    """Listen briefly for a follow-up utterance without requiring wake word."""
    vad = webrtcvad.Vad(VAD_AGGRESSIVENESS)
    stream = audio_interface.open(
        format=FORMAT,
        channels=CHANNELS,
        rate=SAMPLE_RATE,
        input=True,
        frames_per_buffer=CHUNK_SIZE,
    )

    frames = []
    speech_started = False
    silence_frames = 0
    speech_frames = 0
    total_frames = 0
    max_wait_frames = int(timeout_s * SAMPLE_RATE / CHUNK_SIZE)
    padding_frames = int(SPEECH_PADDING_MS * SAMPLE_RATE / (CHUNK_SIZE * 1000))
    min_speech_frames = int(MIN_SPEECH_MS * SAMPLE_RATE / (CHUNK_SIZE * 1000))

    try:
        for _ in range(max_wait_frames + int(MAX_SPEECH_S * SAMPLE_RATE / CHUNK_SIZE)):
            audio_data = stream.read(CHUNK_SIZE, exception_on_overflow=False)
            total_frames += 1

            is_speech = vad.is_speech(audio_data, SAMPLE_RATE)

            if is_speech:
                speech_started = True
                speech_frames += 1
                silence_frames = 0
                frames.append(audio_data)
            elif speech_started:
                silence_frames += 1
                frames.append(audio_data)
                if silence_frames >= padding_frames and speech_frames >= min_speech_frames:
                    raw = b"".join(frames)
                    return np.frombuffer(raw, dtype=np.int16).astype(np.float32) / 32768.0
            else:
                # No speech yet — check if we've exceeded the wait window
                if total_frames >= max_wait_frames:
                    return None
    finally:
        stream.stop_stream()
        stream.close()

    return None


# ─── Entry Point ──────────────────────────────────────────────────────────────

if __name__ == "__main__":
    # Force CPU for TTS (GPU is slower for 82M model)
    os.environ["CUDA_VISIBLE_DEVICES"] = ""

    run_pipeline()
