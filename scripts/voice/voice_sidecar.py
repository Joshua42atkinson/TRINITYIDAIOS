#!/usr/bin/env python3
"""
TRINITY ID AI OS — Voice Sidecar Service
=========================================

Bridge service for STT (faster-whisper) + TTS (Supertonic-2 ONNX).

TTS Backend: Supertonic-2 — 66M params, 167x realtime, pure ONNX.
             Zero PyTorch dependency. Just onnxruntime + numpy.

Endpoints:
    POST /stt          — Audio bytes (WAV) → transcribed text
    POST /tts          — JSON {"text": "...", "voice": "M1"} → WAV audio bytes
    POST /conversation — Full loop: audio in → STT → Trinity chat → TTS → audio out
    GET  /health       — Health check

Runs on port 8200 by default.
"""

import io
import os
import sys
import time
import logging
import tempfile
from pathlib import Path

import numpy as np
import requests
import soundfile as sf
from flask import Flask, request, jsonify, Response

# Add scripts dir to path for supertonic_helper
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from supertonic_helper import load_text_to_speech, load_voice_style

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

VOICE_PORT = int(os.environ.get("VOICE_PORT", "8200"))
TRINITY_URL = os.environ.get("TRINITY_URL", "http://127.0.0.1:3000")
WHISPER_MODEL = os.environ.get("WHISPER_MODEL", "small")

# Supertonic model path
SUPERTONIC_DIR = os.environ.get(
    "SUPERTONIC_DIR",
    str(Path.home() / "trinity-models/tts/supertonic-2"),
)
# Voice preset: M1-M5 (male), F1-F5 (female)
DEFAULT_VOICE = os.environ.get("VOICE", "M1")
# Denoising steps (2=fastest, 5=balanced, 10=highest quality)
DENOISE_STEPS = int(os.environ.get("DENOISE_STEPS", "5"))
# Speech speed (1.0=normal, 1.05=slightly faster, 0.9=slower)
SPEECH_SPEED = float(os.environ.get("SPEECH_SPEED", "1.05"))

# ---------------------------------------------------------------------------
# Globals (loaded once at startup)
# ---------------------------------------------------------------------------

app = Flask(__name__)
logging.basicConfig(level=logging.INFO, format="%(asctime)s [%(levelname)s] %(message)s")
log = logging.getLogger("voice-sidecar")

whisper_model = None
tts_engine = None
voice_styles = {}  # Cache loaded voice styles


def load_whisper():
    """Load Whisper model (downloads on first use, ~500MB for 'small')."""
    global whisper_model
    from faster_whisper import WhisperModel

    log.info(f"Loading Whisper model '{WHISPER_MODEL}' (this may download on first run)...")
    t0 = time.time()
    whisper_model = WhisperModel(WHISPER_MODEL, device="cpu", compute_type="int8")
    log.info(f"Whisper loaded in {time.time() - t0:.1f}s")


def load_supertonic():
    """Load Supertonic-2 ONNX TTS engine."""
    global tts_engine, voice_styles
    onnx_dir = os.path.join(SUPERTONIC_DIR, "onnx")

    log.info(f"Loading Supertonic-2 from {onnx_dir}...")
    t0 = time.time()
    tts_engine = load_text_to_speech(onnx_dir, use_gpu=False)
    log.info(f"Supertonic-2 loaded in {time.time() - t0:.1f}s")

    # Pre-load all voice styles
    styles_dir = os.path.join(SUPERTONIC_DIR, "voice_styles")
    for vs_file in Path(styles_dir).glob("*.json"):
        voice_name = vs_file.stem  # e.g., "M1", "F3"
        voice_styles[voice_name] = load_voice_style([str(vs_file)])
        log.info(f"  Loaded voice style: {voice_name}")
    log.info(f"  {len(voice_styles)} voice styles ready")


def synthesize_supertonic(text: str, voice: str = None) -> bytes:
    """Generate speech with Supertonic-2. Returns WAV bytes."""
    voice = voice or DEFAULT_VOICE
    if voice not in voice_styles:
        log.warning(f"Voice '{voice}' not found, using '{DEFAULT_VOICE}'")
        voice = DEFAULT_VOICE

    style = voice_styles[voice]
    wav, duration = tts_engine(text, "en", style, DENOISE_STEPS, SPEECH_SPEED)

    # Trim to actual duration
    samples = int(tts_engine.sample_rate * duration[0].item())
    wav_trimmed = wav[0, :samples]

    # Write to WAV bytes
    buf = io.BytesIO()
    sf.write(buf, wav_trimmed, tts_engine.sample_rate, format="WAV")
    buf.seek(0)
    return buf.read()


# ---------------------------------------------------------------------------
# STT endpoint
# ---------------------------------------------------------------------------

@app.route("/stt", methods=["POST"])
def stt():
    """Transcribe audio to text."""
    t0 = time.time()

    if whisper_model is None:
        return jsonify({"error": "Whisper model not loaded"}), 503

    audio_bytes = request.get_data()
    if not audio_bytes:
        return jsonify({"error": "No audio data provided"}), 400

    with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as f:
        f.write(audio_bytes)
        tmp_path = f.name

    try:
        segments, info = whisper_model.transcribe(tmp_path, beam_size=5)
        text = " ".join(seg.text.strip() for seg in segments)
    finally:
        os.unlink(tmp_path)

    latency_ms = int((time.time() - t0) * 1000)
    log.info(f"STT: '{text[:80]}...' ({latency_ms}ms)")
    return jsonify({"text": text, "language": info.language, "latency_ms": latency_ms})


# ---------------------------------------------------------------------------
# TTS endpoint
# ---------------------------------------------------------------------------

@app.route("/tts", methods=["POST"])
def tts():
    """Synthesize text to audio.

    Accepts: JSON {"text": "text to speak", "voice": "M1"}
    Returns: WAV audio bytes (Content-Type: audio/wav)
    """
    t0 = time.time()

    data = request.get_json(silent=True) or {}
    text = data.get("text", "")
    voice = data.get("voice", DEFAULT_VOICE)
    if not text:
        return jsonify({"error": "No text provided"}), 400

    try:
        wav_bytes = synthesize_supertonic(text, voice)
    except Exception as e:
        log.error(f"TTS synthesis failed: {e}")
        return jsonify({"error": f"TTS synthesis failed: {str(e)}"}), 500

    latency_ms = int((time.time() - t0) * 1000)
    log.info(f"TTS [supertonic-{voice}]: '{text[:60]}...' → {len(wav_bytes)} bytes ({latency_ms}ms)")

    resp = Response(wav_bytes, mimetype="audio/wav")
    resp.headers["X-Latency-Ms"] = str(latency_ms)
    resp.headers["X-TTS-Backend"] = f"supertonic-{voice}"
    return resp


# ---------------------------------------------------------------------------
# Full conversation endpoint
# ---------------------------------------------------------------------------

@app.route("/conversation", methods=["POST"])
def conversation():
    """Full voice conversation loop.

    Accepts: WAV audio as request body
    Optional header X-Mode: "pete" (default) or "trinity"
    Returns: WAV audio response (Content-Type: audio/wav)
    Headers: X-Transcript, X-Response, X-Latency-Ms
    """
    t0 = time.time()
    mode = request.headers.get("X-Mode", "pete")

    # Step 1: STT
    audio_bytes = request.get_data()
    if not audio_bytes:
        return jsonify({"error": "No audio data"}), 400

    with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as f:
        f.write(audio_bytes)
        tmp_path = f.name

    try:
        segments, info = whisper_model.transcribe(tmp_path, beam_size=5)
        transcript = " ".join(seg.text.strip() for seg in segments)
    finally:
        os.unlink(tmp_path)

    if not transcript.strip():
        return jsonify({"error": "No speech detected"}), 400

    log.info(f"[{mode}] User said: '{transcript}'")

    # Step 2: Call Trinity server
    try:
        trinity_resp = requests.post(
            f"{TRINITY_URL}/api/chat",
            json={"message": transcript, "max_tokens": 256, "mode": mode},
            timeout=60,
        )
        trinity_resp.raise_for_status()
        response_text = trinity_resp.json().get("response", "I'm sorry, I didn't understand.")
    except Exception as e:
        log.error(f"Trinity call failed: {e}")
        response_text = "I'm having trouble connecting to the inference engine."

    log.info(f"[{mode}] Trinity replied: '{response_text[:80]}...'")

    # Step 3: TTS
    try:
        wav_bytes = synthesize_supertonic(response_text)
    except Exception as e:
        log.error(f"TTS failed: {e}")
        return jsonify({"error": "TTS failed"}), 500

    latency_ms = int((time.time() - t0) * 1000)
    log.info(f"[{mode}] Full loop (supertonic): {latency_ms}ms")

    resp = Response(wav_bytes, mimetype="audio/wav")
    resp.headers["X-Transcript"] = transcript[:200].replace("\n", " ").replace("\r", "")
    resp.headers["X-Response"] = response_text[:200].replace("\n", " ").replace("\r", "")
    resp.headers["X-Latency-Ms"] = str(latency_ms)
    resp.headers["X-Mode"] = mode
    resp.headers["X-TTS-Backend"] = "supertonic"
    return resp


# ---------------------------------------------------------------------------
# Health
# ---------------------------------------------------------------------------

@app.route("/health", methods=["GET"])
def health():
    whisper_ok = whisper_model is not None
    tts_ok = tts_engine is not None and len(voice_styles) > 0
    return jsonify({
        "status": "ok" if (whisper_ok and tts_ok) else "degraded",
        "whisper": "loaded" if whisper_ok else "not loaded",
        "tts_backend": "supertonic-2",
        "tts_ready": tts_ok,
        "voices": list(voice_styles.keys()),
        "default_voice": DEFAULT_VOICE,
        "denoise_steps": DENOISE_STEPS,
        "trinity_url": TRINITY_URL,
    })


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

if __name__ == "__main__":
    # Load TTS first (fast — ~1 second)
    load_supertonic()

    # Load STT
    load_whisper()

    log.info(f"Voice sidecar starting on port {VOICE_PORT}")
    log.info(f"  STT: faster-whisper ({WHISPER_MODEL})")
    log.info(f"  TTS: Supertonic-2 (voice={DEFAULT_VOICE}, steps={DENOISE_STEPS}, speed={SPEECH_SPEED})")
    log.info(f"  Trinity: {TRINITY_URL}")
    app.run(host="0.0.0.0", port=VOICE_PORT, debug=False)
