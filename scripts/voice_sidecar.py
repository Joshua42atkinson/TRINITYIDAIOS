#!/usr/bin/env python3
"""
TRINITY ID AI OS — Voice Sidecar Service
=========================================

Bridge service for STT (faster-whisper) + TTS (piper) until PersonaPlex NPU is ready.

Endpoints:
    POST /stt          — Audio bytes (WAV) → transcribed text
    POST /tts          — JSON {"text": "..."} → WAV audio bytes
    POST /conversation — Full loop: audio in → STT → Trinity chat → TTS → audio out
    GET  /health       — Health check

Architecture:
    Mic → this sidecar /stt → Trinity /api/chat → this sidecar /tts → Speaker
    OR
    Mic → this sidecar /conversation (does the full round-trip) → Speaker

Runs on port 8200 by default.
When PersonaPlex ONNX is wired via FastFlowLM, this sidecar becomes optional.
"""

import io
import os
import sys
import json
import time
import wave
import struct
import logging
import subprocess
import tempfile
from pathlib import Path

import requests
import soundfile as sf
from flask import Flask, request, jsonify, Response

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

VOICE_PORT = int(os.environ.get("VOICE_PORT", "8200"))
TRINITY_URL = os.environ.get("TRINITY_URL", "http://127.0.0.1:3000")
WHISPER_MODEL = os.environ.get("WHISPER_MODEL", "small")
PIPER_MODEL = os.environ.get(
    "PIPER_MODEL",
    str(Path.home() / "trinity-models/tts/piper/en_US-lessac-medium.onnx"),
)

# ---------------------------------------------------------------------------
# Globals (loaded once at startup)
# ---------------------------------------------------------------------------

app = Flask(__name__)
logging.basicConfig(level=logging.INFO, format="%(asctime)s [%(levelname)s] %(message)s")
log = logging.getLogger("voice-sidecar")

whisper_model = None


def load_whisper():
    """Load Whisper model (downloads on first use, ~500MB for 'small')."""
    global whisper_model
    from faster_whisper import WhisperModel

    log.info(f"Loading Whisper model '{WHISPER_MODEL}' (this may download on first run)...")
    t0 = time.time()
    whisper_model = WhisperModel(WHISPER_MODEL, device="cpu", compute_type="int8")
    log.info(f"Whisper loaded in {time.time() - t0:.1f}s")


# ---------------------------------------------------------------------------
# STT endpoint
# ---------------------------------------------------------------------------

@app.route("/stt", methods=["POST"])
def stt():
    """Transcribe audio to text.

    Accepts: WAV audio as request body (Content-Type: audio/wav)
    Returns: JSON {"text": "transcribed text", "latency_ms": 123}
    """
    t0 = time.time()

    if whisper_model is None:
        return jsonify({"error": "Whisper model not loaded"}), 503

    audio_bytes = request.get_data()
    if not audio_bytes:
        return jsonify({"error": "No audio data provided"}), 400

    # Write to temp file for faster-whisper
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

    Accepts: JSON {"text": "text to speak"}
    Returns: WAV audio bytes (Content-Type: audio/wav)
    """
    t0 = time.time()

    data = request.get_json(silent=True) or {}
    text = data.get("text", "")
    if not text:
        return jsonify({"error": "No text provided"}), 400

    # Use piper CLI for synthesis (full path to venv binary)
    piper_bin = os.path.join(os.path.dirname(sys.executable), "piper")
    with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as f:
        tmp_path = f.name

    try:
        proc = subprocess.run(
            [piper_bin, "-m", PIPER_MODEL, "-f", tmp_path],
            input=text.encode(),
            capture_output=True,
            timeout=30,
        )
        if proc.returncode != 0:
            log.error(f"Piper error: {proc.stderr.decode()}")
            return jsonify({"error": "TTS synthesis failed"}), 500

        with open(tmp_path, "rb") as f:
            wav_bytes = f.read()
    finally:
        if os.path.exists(tmp_path):
            os.unlink(tmp_path)

    latency_ms = int((time.time() - t0) * 1000)
    log.info(f"TTS: '{text[:60]}...' → {len(wav_bytes)} bytes ({latency_ms}ms)")

    resp = Response(wav_bytes, mimetype="audio/wav")
    resp.headers["X-Latency-Ms"] = str(latency_ms)
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

    # Step 2: Call Trinity server with mode
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
    with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as f:
        tts_path = f.name

    try:
        proc = subprocess.run(
            ["piper", "-m", PIPER_MODEL, "-f", tts_path],
            input=response_text.encode(),
            capture_output=True,
            timeout=30,
        )
        if proc.returncode != 0:
            return jsonify({"error": "TTS failed"}), 500

        with open(tts_path, "rb") as f:
            wav_bytes = f.read()
    finally:
        if os.path.exists(tts_path):
            os.unlink(tts_path)

    latency_ms = int((time.time() - t0) * 1000)
    log.info(f"[{mode}] Full loop: {latency_ms}ms")

    resp = Response(wav_bytes, mimetype="audio/wav")
    resp.headers["X-Transcript"] = transcript[:200].replace("\n", " ").replace("\r", "")
    resp.headers["X-Response"] = response_text[:200].replace("\n", " ").replace("\r", "")
    resp.headers["X-Latency-Ms"] = str(latency_ms)
    resp.headers["X-Mode"] = mode
    return resp


# ---------------------------------------------------------------------------
# Health
# ---------------------------------------------------------------------------

@app.route("/health", methods=["GET"])
def health():
    whisper_ok = whisper_model is not None
    piper_ok = Path(PIPER_MODEL).exists()
    return jsonify({
        "status": "ok" if (whisper_ok and piper_ok) else "degraded",
        "whisper": "loaded" if whisper_ok else "not loaded",
        "piper_model": str(PIPER_MODEL) if piper_ok else "missing",
        "trinity_url": TRINITY_URL,
    })


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

if __name__ == "__main__":
    load_whisper()
    log.info(f"Voice sidecar starting on port {VOICE_PORT}")
    log.info(f"  STT: faster-whisper ({WHISPER_MODEL})")
    log.info(f"  TTS: piper ({PIPER_MODEL})")
    log.info(f"  Trinity: {TRINITY_URL}")
    app.run(host="0.0.0.0", port=VOICE_PORT, debug=False)
