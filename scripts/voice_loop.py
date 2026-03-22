#!/usr/bin/env python3
"""
TRINITY ID AI OS — Continuous Voice Loop
==========================================

Glasses on. Mic hot. Talk to Trinity.

Listens to your microphone, detects speech, sends to voice sidecar,
plays back the response. Repeat forever.

Usage:
    python scripts/voice_loop.py              # DEV mode (default)
    python scripts/voice_loop.py iron-road    # Iron Road mode

Say "play iron road" or "switch to dev" to change modes mid-conversation.
Say "goodbye" or "exit" to stop.

Requires: pyaudio (or sounddevice), requests
"""

import os
import sys
import io
import time
import wave
import struct
import tempfile
import logging
import requests

logging.basicConfig(level=logging.INFO, format="%(asctime)s [%(levelname)s] %(message)s")
log = logging.getLogger("voice-loop")

VOICE_SIDECAR_URL = os.environ.get("VOICE_SIDECAR_URL", "http://127.0.0.1:8200")
SAMPLE_RATE = 16000
CHANNELS = 1
CHUNK_SIZE = 1024
SILENCE_THRESHOLD = 500  # RMS amplitude threshold
SILENCE_DURATION = 1.5   # Seconds of silence to end recording
MIN_SPEECH_DURATION = 0.5  # Minimum seconds of speech to process


def rms(data):
    """Calculate RMS amplitude of audio data."""
    if not data:
        return 0
    count = len(data) // 2
    if count == 0:
        return 0
    shorts = struct.unpack(f"<{count}h", data)
    sum_squares = sum(s * s for s in shorts)
    return int((sum_squares / count) ** 0.5)


def record_until_silence(stream, pa):
    """Record audio until silence is detected after speech."""
    frames = []
    is_speaking = False
    silence_start = None
    speech_start = None

    log.info("🎤 Listening...")

    while True:
        data = stream.read(CHUNK_SIZE, exception_on_overflow=False)
        amplitude = rms(data)

        if amplitude > SILENCE_THRESHOLD:
            if not is_speaking:
                is_speaking = True
                speech_start = time.time()
                log.info("🗣️  Speech detected...")
            silence_start = None
            frames.append(data)
        elif is_speaking:
            frames.append(data)
            if silence_start is None:
                silence_start = time.time()
            elif time.time() - silence_start > SILENCE_DURATION:
                duration = time.time() - speech_start
                if duration >= MIN_SPEECH_DURATION:
                    log.info(f"⏸️  Silence detected. Captured {duration:.1f}s of audio.")
                    return frames
                else:
                    log.info("Too short, ignoring.")
                    frames = []
                    is_speaking = False
                    silence_start = None


def frames_to_wav(frames):
    """Convert raw audio frames to WAV bytes."""
    buf = io.BytesIO()
    with wave.open(buf, "wb") as wf:
        wf.setnchannels(CHANNELS)
        wf.setsampwidth(2)  # 16-bit
        wf.setframerate(SAMPLE_RATE)
        wf.writeframes(b"".join(frames))
    return buf.getvalue()


def play_wav(wav_bytes):
    """Play WAV audio through speakers."""
    import pyaudio
    buf = io.BytesIO(wav_bytes)
    with wave.open(buf, "rb") as wf:
        pa = pyaudio.PyAudio()
        stream = pa.open(
            format=pa.get_format_from_width(wf.getsampwidth()),
            channels=wf.getnchannels(),
            rate=wf.getframerate(),
            output=True,
        )
        data = wf.readframes(1024)
        while data:
            stream.write(data)
            data = wf.readframes(1024)
        stream.stop_stream()
        stream.close()
        pa.terminate()


def send_to_sidecar(wav_bytes, mode):
    """Send audio to voice sidecar and get response audio."""
    try:
        resp = requests.post(
            f"{VOICE_SIDECAR_URL}/conversation",
            headers={
                "Content-Type": "audio/wav",
                "X-Mode": mode,
            },
            data=wav_bytes,
            timeout=120,
        )

        if resp.status_code != 200:
            log.error(f"Sidecar error {resp.status_code}: {resp.text[:200]}")
            return None, None, None

        transcript = resp.headers.get("X-Transcript", "")
        response_text = resp.headers.get("X-Response", "")
        latency = resp.headers.get("X-Latency-Ms", "?")

        log.info(f"📝 You said: \"{transcript}\"")
        log.info(f"🤖 Trinity: \"{response_text[:120]}...\"")
        log.info(f"⏱️  Round trip: {latency}ms")

        return resp.content, transcript, response_text

    except requests.exceptions.ConnectionError:
        log.error("Voice sidecar not running at " + VOICE_SIDECAR_URL)
        return None, None, None
    except Exception as e:
        log.error(f"Error: {e}")
        return None, None, None


def check_mode_switch(transcript, current_mode):
    """Check if user wants to switch modes."""
    t = transcript.lower().strip()
    if any(phrase in t for phrase in ["play iron road", "iron road mode", "switch to iron road", "start iron road"]):
        return "iron-road"
    if any(phrase in t for phrase in ["dev mode", "switch to dev", "production mode", "exit iron road"]):
        return "dev"
    return current_mode


def check_exit(transcript):
    """Check if user wants to quit."""
    t = transcript.lower().strip()
    return any(phrase in t for phrase in ["goodbye", "exit", "quit", "stop listening", "shut down"])


def main():
    import pyaudio

    mode = sys.argv[1] if len(sys.argv) > 1 else "dev"
    log.info(f"═══════════════════════════════════════")
    log.info(f"  TRINITY Voice Loop")
    log.info(f"  Mode: {mode.upper()}")
    log.info(f"  Say 'play iron road' or 'dev mode' to switch")
    log.info(f"  Say 'goodbye' to exit")
    log.info(f"═══════════════════════════════════════")

    # Check sidecar health
    try:
        health = requests.get(f"{VOICE_SIDECAR_URL}/health", timeout=3).json()
        if health.get("status") != "ok":
            log.error(f"Voice sidecar not healthy: {health}")
            return
        log.info(f"✅ Voice sidecar: {health.get('whisper', '?')} STT, piper TTS")
    except Exception:
        log.error(f"Cannot reach voice sidecar at {VOICE_SIDECAR_URL}")
        log.error("Start it: source ~/trinity-vllm-env/bin/activate && python scripts/voice_sidecar.py")
        return

    pa = pyaudio.PyAudio()

    # List available input devices
    log.info("Audio input devices:")
    for i in range(pa.get_device_count()):
        info = pa.get_device_info_by_index(i)
        if info["maxInputChannels"] > 0:
            log.info(f"  [{i}] {info['name']} (channels={info['maxInputChannels']})")

    stream = pa.open(
        format=pyaudio.paInt16,
        channels=CHANNELS,
        rate=SAMPLE_RATE,
        input=True,
        frames_per_buffer=CHUNK_SIZE,
    )

    try:
        while True:
            # Record speech
            frames = record_until_silence(stream, pa)
            if not frames:
                continue

            wav_bytes = frames_to_wav(frames)

            # Send to sidecar
            log.info("🧠 Thinking...")
            audio_response, transcript, response_text = send_to_sidecar(wav_bytes, mode)

            if transcript:
                # Check for mode switch
                new_mode = check_mode_switch(transcript, mode)
                if new_mode != mode:
                    mode = new_mode
                    log.info(f"🔄 Switched to {mode.upper()} mode")

                # Check for exit
                if check_exit(transcript):
                    log.info("👋 Goodbye!")
                    break

            # Play response
            if audio_response:
                log.info("🔊 Speaking...")
                play_wav(audio_response)

    except KeyboardInterrupt:
        log.info("\n👋 Voice loop stopped.")
    finally:
        stream.stop_stream()
        stream.close()
        pa.terminate()


if __name__ == "__main__":
    main()
