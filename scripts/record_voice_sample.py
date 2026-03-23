#!/usr/bin/env python3
"""
TRINITY — Voice Sample Recorder
================================
Records your voice for TTS cloning. Run this script, read the displayed text,
and it will save WAV files ready for voice model training.

Usage:
    python3 scripts/record_voice_sample.py

The recordings are saved to ~/trinity-models/voice/samples/
"""

import os
import subprocess
import time
import sys

SAMPLE_DIR = os.path.expanduser("~/trinity-models/voice/samples")
os.makedirs(SAMPLE_DIR, exist_ok=True)

# Sample texts for the narrator to read (Great Recycler voice)
SAMPLES = [
    {
        "id": "welcome",
        "text": "Welcome to Zen Mode, traveler. This is the quiet car. No buttons, no dashboards. Just you and the page.",
        "duration": 10,
    },
    {
        "id": "journey",
        "text": "Every great journey begins not with a step, but with a question. What is it that you truly want to learn? Not what you think you should know, but what keeps you awake at three in the morning.",
        "duration": 15,
    },
    {
        "id": "recycler",
        "text": "I am the Great Recycler. I take what you discard, the failed attempts, the abandoned ideas, the half-formed thoughts, and I weave them into something new. Nothing is wasted on the Iron Road.",
        "duration": 15,
    },
    {
        "id": "reflection",
        "text": "Consider this. A train does not choose its tracks, but it chooses its speed. You cannot control what life teaches you, but you can choose how deeply you listen.",
        "duration": 12,
    },
    {
        "id": "discovery",
        "text": "The most profound discoveries are often hiding in plain sight, disguised as ordinary moments. Tell me, what did you notice today that surprised you?",
        "duration": 12,
    },
]


def list_devices():
    """List available audio input devices."""
    print("\n🎤 Available recording devices:")
    result = subprocess.run(["arecord", "-l"], capture_output=True, text=True)
    for line in result.stdout.split("\n"):
        if "card" in line.lower():
            print(f"  {line.strip()}")
    print()


def record_sample(sample, device="default"):
    """Record a single voice sample."""
    filepath = os.path.join(SAMPLE_DIR, f"{sample['id']}.wav")
    duration = sample["duration"]

    print(f"\n{'='*60}")
    print(f"📖 READ THIS ALOUD (narrator voice, warm and steady):")
    print(f"{'='*60}")
    print(f"\n  \"{sample['text']}\"\n")
    print(f"{'='*60}")
    print(f"⏱  Recording for {duration} seconds...")
    print(f"🎤 Press ENTER when ready, then start reading.\n")

    input("  → Press ENTER to start recording...")

    print(f"  🔴 RECORDING... ({duration}s)")

    # Try mono first, then stereo with conversion
    for channels in [1, 2]:
        tmp_path = filepath if channels == 1 else filepath + ".tmp.wav"
        try:
            subprocess.run(
                [
                    "arecord",
                    "-D", device,
                    "-f", "S16_LE",
                    "-r", "22050",
                    "-c", str(channels),
                    "-d", str(duration),
                    tmp_path,
                ],
                check=True,
                capture_output=True,
            )
            # If we recorded stereo, convert to mono
            if channels == 2:
                try:
                    subprocess.run(
                        ["ffmpeg", "-y", "-i", tmp_path, "-ac", "1", filepath],
                        check=True, capture_output=True,
                    )
                    os.unlink(tmp_path)
                except FileNotFoundError:
                    # No ffmpeg, just keep the stereo file
                    os.rename(tmp_path, filepath)
                print(f"  (recorded stereo, converted to mono)")

            size_kb = os.path.getsize(filepath) // 1024
            print(f"  ✅ Saved: {filepath} ({size_kb} KB)")
            return True
        except subprocess.CalledProcessError:
            if channels == 1:
                continue  # Try stereo
            print(f"  ❌ Recording failed on both mono and stereo")
            return False

    return False


def main():
    print("\n" + "="*60)
    print("  TRINITY — Voice Sample Recorder")
    print("  Record your voice for the Great Recycler narrator")
    print("="*60)

    list_devices()

    # Let user choose device
    print("Which device? (enter card number, or press Enter for default)")
    print("  Example: for 'card 1: Air', enter: hw:1,0")
    device = input("  Device [default]: ").strip() or "default"

    print(f"\nUsing device: {device}")
    print(f"Saving to: {SAMPLE_DIR}")
    print(f"Total samples: {len(SAMPLES)}")
    print("\nTips for best results:")
    print("  • Speak in your narrator voice — warm, steady, contemplative")
    print("  • Keep a consistent distance from the mic (~6 inches)")
    print("  • Minimize background noise")
    print("  • Read naturally, not robotically")
    print()

    recorded = 0
    for i, sample in enumerate(SAMPLES):
        print(f"\n--- Sample {i+1}/{len(SAMPLES)} ---")
        if record_sample(sample, device):
            recorded += 1

        if i < len(SAMPLES) - 1:
            cont = input("\nContinue to next sample? (y/n) [y]: ").strip().lower()
            if cont == 'n':
                break

    print(f"\n{'='*60}")
    print(f"  ✅ Recorded {recorded}/{len(SAMPLES)} samples")
    print(f"  📂 Saved to: {SAMPLE_DIR}")
    print(f"  Next: Use these for voice cloning with CosyVoice3 or XTTS")
    print(f"{'='*60}\n")


if __name__ == "__main__":
    main()
