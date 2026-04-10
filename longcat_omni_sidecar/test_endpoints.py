#!/usr/bin/env python3
"""
Test harness for LongCat-Next Omni-Brain on port 8010.
Runs text, image, TTS, and TEMPO audio generations to verify Dual-Process architecture.
"""

import os
import json
import base64
import requests

BASE_URL = "http://127.0.0.1:8010"

def test_health():
    print("🟢 [1/5] Testing Health Check...")
    try:
        r = requests.get(f"{BASE_URL}/health", timeout=5)
        r.raise_for_status()
        print(f"   => Health OK: {r.json()}")
    except requests.exceptions.ConnectionError:
        print("   ❌ Error: Omni-Brain server is down. Did you run launch_engine.sh?")
        exit(1)

def test_chat():
    print("\n🟢 [2/5] Testing Text Generation (Streaming) ...")
    payload = {
        "messages": [
            {"role": "system", "content": "You are Pete from Trinity AI OS."},
            {"role": "user", "content": "What is the purpose of the LongCat-Next Omni-Brain?"}
        ],
        "max_tokens": 100,
        "temperature": 0.7,
        "stream": True
    }
    r = requests.post(f"{BASE_URL}/v1/chat/completions", json=payload, stream=True)
    r.raise_for_status()
    
    print("   => Stream started: ", end="", flush=True)
    for line in r.iter_lines():
        if line:
            decoded = line.decode("utf-8")
            if decoded.startswith("data: ") and not decoded.endswith("[DONE]"):
                data = json.loads(decoded[6:])
                chunk = data["choices"][0].get("delta", {}).get("content", "")
                print(chunk, end="", flush=True)
    print("\n   => ✅ Text test completed!")

def test_image():
    print("\n🟢 [3/5] Testing Image Generation ...")
    payload = {
        "prompt": "A futuristic glowing cyber-cat sitting on a neon mainframe",
        "size": "256x256" # keeping it small to test speed
    }
    r = requests.post(f"{BASE_URL}/v1/images/generations", json=payload)
    r.raise_for_status()
    
    data = r.json()
    b64 = data["data"][0]["b64_json"]
    out_path = "/tmp/test_omni_image.png"
    
    with open(out_path, "wb") as f:
        f.write(base64.b64decode(b64))
    print(f"   => ✅ Image written to {out_path} ({len(b64)} bytes base64)")

def test_tts():
    print("\n🟢 [4/5] Testing Text-To-Speech ...")
    payload = {
        "text": "Hello user, I am Pete. System functionality is fully operational.",
        "voice": "am_adam"
    }
    r = requests.post(f"{BASE_URL}/tts", json=payload)
    r.raise_for_status()
    
    out_path = "/tmp/test_omni_tts.wav"
    with open(out_path, "wb") as f:
        f.write(r.content)
    print(f"   => ✅ Audio written to {out_path} ({len(r.content)} bytes)")

def test_tempo():
    print("\n🟢 [5/5] Testing TEMPO Music Generation ...")
    payload = {
        "prompt": "ambient, synthwave, slow arpeggios, atmospheric",
        "duration": 5
    }
    r = requests.post(f"{BASE_URL}/v1/audio/generations", json=payload)
    r.raise_for_status()
    
    data = r.json()
    b64 = data["data"][0]["b64_json"]
    out_path = "/tmp/test_omni_tempo.wav"
    
    with open(out_path, "wb") as f:
        f.write(base64.b64decode(b64))
    print(f"   => ✅ TEMPO audio written to {out_path} ({len(b64)} bytes base64)")


if __name__ == "__main__":
    print(f"🧪 TRINITY ID AI OS — Running Omni-Brain Endpoints Test Suite")
    test_health()
    test_chat()
    test_image()
    test_tts()
    test_tempo()
    print("\n🎉 All tests dispatched successfully!")
