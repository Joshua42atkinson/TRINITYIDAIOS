#!/usr/bin/env python3
"""
TRINITY VOICE LOOP — Hands-Free Voice Conversation
═══════════════════════════════════════════════════════
Radio Protocol: STANDBY → [speech detected] → KEYED → [silence] → SEND → RECEIVE → STANDBY

No keyboard required after launch. Uses WebRTC Voice Activity Detection (VAD)
to auto-detect when you start and stop speaking.

Ctrl+C to exit.

Dependencies: pip install pyaudio webrtcvad numpy soundfile openai
"""

import argparse
import base64
import collections
import io
import os
import struct
import sys
import threading
import time
from queue import Queue

import numpy as np
import pyaudio
import soundfile as sf
import webrtcvad
from openai import OpenAI

# ═══════════════════════════════════════════════════════════════════════════
# Trinity/Pete system prompt — 8K Ask Pete Field Manual
# ═══════════════════════════════════════════════════════════════════════════
TRINITY_SYSTEM_PROMPT = (
    "You are Pete — the voice of Trinity ID AI OS, an instructional design "
    "assistant for K-12 teachers. You are running on a local GMKtec EVO-X2 "
    "with 128GB RAM, AMD Strix Halo APU. You are an AI speech assistant. "
    "The user is speaking to you through a microphone and you respond with voice. "
    "Keep responses concise — 1 to 3 sentences max. Be warm, direct, and helpful. "
    "You help teachers build better lessons using research-backed instructional design. "
    "If the user asks what you are: you are Trinity, a local AI teaching assistant "
    "that runs entirely on their own hardware with no cloud dependency. "
    "Respond with interleaved text and audio."
)

# ═══════════════════════════════════════════════════════════════════════════
# VAD Configuration
# ═══════════════════════════════════════════════════════════════════════════
SAMPLE_RATE = 16000          # WebRTC VAD requires 8000, 16000, 32000, or 48000
FRAME_DURATION_MS = 30       # 10, 20, or 30 ms per frame
FRAME_SIZE = int(SAMPLE_RATE * FRAME_DURATION_MS / 1000)  # samples per frame
VAD_AGGRESSIVENESS = 2       # 0-3 (0=least aggressive, 3=most — filters more noise)
SPEECH_PADDING_MS = 1500     # Silence after speech before triggering send (1.5s for thinkers)
MIN_SPEECH_MS = 500          # Minimum speech duration to avoid false triggers
MAX_SPEECH_S = 30            # Maximum recording length (seconds)


class AudioPlayer:
    """Non-blocking audio playback via PyAudio."""

    def __init__(self, sample_rate=None):
        self.sample_rate = sample_rate
        self.pyaudio = None
        self.stream = None
        self.queue = Queue()
        self.thread = None
        self.running = False
        self.started = False

    def _playback_thread(self):
        while self.running or not self.queue.empty():
            try:
                pcm_data = self.queue.get(timeout=0.1)
                if self.stream:
                    self.stream.write(pcm_data)
            except Exception:
                pass

    def start(self):
        self.running = True
        self.started = False

    def _start_stream(self):
        if self.started or self.sample_rate is None:
            return
        self.pyaudio = pyaudio.PyAudio()
        self.stream = self.pyaudio.open(
            format=pyaudio.paInt16,
            channels=1,
            rate=self.sample_rate,
            output=True,
        )
        self.thread = threading.Thread(target=self._playback_thread, daemon=True)
        self.thread.start()
        self.started = True

    def add_samples(self, samples, sample_rate=None):
        if sample_rate is not None and self.sample_rate is None:
            self.sample_rate = sample_rate
        self._start_stream()
        pcm_data = np.array(samples, dtype=np.int16).tobytes()
        self.queue.put(pcm_data)

    def stop(self):
        self.running = False
        if self.thread:
            self.thread.join(timeout=2)
            self.thread = None
        if self.stream:
            self.stream.stop_stream()
            self.stream.close()
            self.stream = None
        if self.pyaudio:
            self.pyaudio.terminate()
            self.pyaudio = None

    @property
    def is_playing(self):
        return not self.queue.empty()


def frames_to_wav(frames, sample_rate=SAMPLE_RATE):
    """Convert raw PCM int16 frames to WAV bytes."""
    raw_pcm = b"".join(frames)
    buf = io.BytesIO()
    sf.write(buf, np.frombuffer(raw_pcm, dtype=np.int16).astype(np.float32) / 32768.0,
             sample_rate, format="WAV")
    buf.seek(0)
    return buf.read()


def create_audio_message(wav_data):
    """Create OpenAI-format audio message."""
    encoded = base64.b64encode(wav_data).decode("utf-8")
    return {
        "role": "user",
        "content": [
            {
                "type": "input_audio",
                "input_audio": {"data": encoded, "format": "wav"},
            }
        ],
    }


def send_to_lfm(client, wav_data, is_first, max_tokens=512):
    """Send audio to LFM and return streaming response."""
    messages = []
    if is_first:
        messages.append({"role": "system", "content": TRINITY_SYSTEM_PROMPT})
    messages.append(create_audio_message(wav_data))

    return client.chat.completions.create(
        model="",
        modalities=["text", "audio"],
        messages=messages,
        stream=True,
        max_tokens=max_tokens,
        extra_body={"reset_context": is_first},
    )


def process_response(stream, audio_player):
    """Process LFM streaming response, playing audio and printing text."""
    t0 = time.time()
    text_parts = []
    total_samples = 0
    audio_sample_rate = None

    for chunk in stream:
        if chunk.choices[0].finish_reason == "stop":
            break

        delta = chunk.choices[0].delta

        # Text content
        if text := delta.content:
            text_parts.append(text)
            print(text, end="", flush=True)

        # Audio content
        if hasattr(delta, "audio") and delta.audio and "data" in delta.audio:
            if audio_sample_rate is None and "sample_rate" in delta.audio:
                audio_sample_rate = delta.audio["sample_rate"]
            pcm_bytes = base64.b64decode(delta.audio["data"])
            samples = np.frombuffer(pcm_bytes, dtype=np.int16)
            total_samples += len(samples)
            if audio_player:
                audio_player.add_samples(samples, sample_rate=audio_sample_rate)

    total_time = time.time() - t0
    full_text = "".join(text_parts)

    if full_text:
        print()  # newline after text

    # Stats line
    stats = [f"{total_time:.1f}s"]
    if audio_sample_rate and total_samples > 0:
        stats.append(f"audio {total_samples / audio_sample_rate:.1f}s")
    print(f"  [{' | '.join(stats)}]")

    return full_text


def listen_for_speech(vad, pa, audio_player):
    """
    Block until speech is detected and finished.
    Returns WAV bytes of the captured utterance.

    Radio Protocol:
      STANDBY → voice detected → KEYED → silence detected → returns audio
    """
    stream = pa.open(
        format=pyaudio.paInt16,
        channels=1,
        rate=SAMPLE_RATE,
        input=True,
        frames_per_buffer=FRAME_SIZE,
    )

    # Ring buffer to capture pre-speech audio (so we don't clip the start)
    num_padding_frames = int(SPEECH_PADDING_MS / FRAME_DURATION_MS)
    ring_buffer = collections.deque(maxlen=num_padding_frames)
    triggered = False
    voiced_frames = []
    speech_start = None

    try:
        while True:
            # Don't listen while Trinity is speaking (echo suppression)
            if audio_player and audio_player.is_playing:
                time.sleep(0.05)
                continue

            frame = stream.read(FRAME_SIZE, exception_on_overflow=False)
            is_speech = vad.is_speech(frame, SAMPLE_RATE)

            if not triggered:
                ring_buffer.append((frame, is_speech))
                # Count how many recent frames have speech
                num_voiced = len([f for f, speech in ring_buffer if speech])
                # Trigger when >90% of ring buffer is speech
                if num_voiced > 0.9 * ring_buffer.maxlen:
                    triggered = True
                    speech_start = time.time()
                    sys.stdout.write("\r  >> KEYED — listening...       ")
                    sys.stdout.flush()
                    # Add the ring buffer frames to capture speech start
                    voiced_frames.extend([f for f, s in ring_buffer])
                    ring_buffer.clear()
            else:
                voiced_frames.append(frame)
                ring_buffer.append((frame, is_speech))
                num_unvoiced = len([f for f, speech in ring_buffer if not speech])

                # Stop when >90% of ring buffer is silence
                if num_unvoiced > 0.9 * ring_buffer.maxlen:
                    speech_duration = time.time() - speech_start
                    # Check minimum speech duration
                    if speech_duration * 1000 < MIN_SPEECH_MS:
                        # Too short, reset
                        triggered = False
                        voiced_frames = []
                        ring_buffer.clear()
                        sys.stdout.write("\r  STANDBY — speak anytime...    ")
                        sys.stdout.flush()
                        continue

                    sys.stdout.write(f"\r  >> SEND — {speech_duration:.1f}s captured          \n")
                    sys.stdout.flush()
                    break

                # Safety: max recording length
                if time.time() - speech_start > MAX_SPEECH_S:
                    sys.stdout.write(f"\r  >> SEND — max length reached          \n")
                    sys.stdout.flush()
                    break

    finally:
        stream.stop_stream()
        stream.close()

    if not voiced_frames:
        return None

    return frames_to_wav(voiced_frames)


def main():
    parser = argparse.ArgumentParser(description="Trinity Hands-Free Voice Chat")
    parser.add_argument("--base-url", type=str, default="http://localhost:7861/v1",
                        help="LFM server URL")
    parser.add_argument("--max-tokens", type=int, default=512)
    parser.add_argument("--vad-level", type=int, default=VAD_AGGRESSIVENESS,
                        choices=[0, 1, 2, 3],
                        help="VAD aggressiveness (0=permissive, 3=strict)")
    parser.add_argument("--no-audio", action="store_true",
                        help="Disable audio playback (text only)")
    args = parser.parse_args()

    # Init
    vad = webrtcvad.Vad(args.vad_level)
    pa = pyaudio.PyAudio()
    client = OpenAI(base_url=args.base_url, api_key="dummy")
    is_first = True

    print()
    print("═" * 54)
    print("  TRINITY VOICE — HANDS-FREE")
    print("  Local AI • LFM2.5-Audio • No Cloud")
    print("═" * 54)
    print(f"  Server:  {args.base_url}")
    print(f"  VAD:     level {args.vad_level} (0=permissive, 3=strict)")
    print(f"  Audio:   {'speakers' if not args.no_audio else 'text only'}")
    print("═" * 54)
    print()
    print("  Just start talking. Ctrl+C to exit.")
    print()
    print("  STANDBY — speak anytime...    ", end="", flush=True)

    try:
        while True:
            # === STANDBY → KEYED → SEND ===
            audio_player = None
            if not args.no_audio:
                audio_player = AudioPlayer()

            wav_data = listen_for_speech(vad, pa, audio_player)
            if wav_data is None:
                continue

            # === RECEIVE ===
            try:
                if audio_player:
                    audio_player.start()

                print("  << RECEIVE", flush=True)
                stream = send_to_lfm(client, wav_data, is_first, args.max_tokens)
                is_first = False
                print("  ", end="")
                process_response(stream, audio_player)

            except Exception as e:
                print(f"\n  [Error: {e}]")
            finally:
                if audio_player:
                    # Wait for playback to finish before listening again
                    time.sleep(0.3)
                    while audio_player.is_playing:
                        time.sleep(0.1)
                    audio_player.stop()

            # === Back to STANDBY ===
            print()
            print("  STANDBY — speak anytime...    ", end="", flush=True)

    except KeyboardInterrupt:
        print("\n\n  Dispatch closed. Keep building, Operator.")

    finally:
        pa.terminate()


if __name__ == "__main__":
    main()
