import requests
import json
import base64
import os

API_URL = "http://127.0.0.1:8010/v1/chat/completions"
MODEL_ID = "/model"

def print_separator(title):
    print(f"\n{'-'*20} {title} {'-'*20}\n")

def test_language_reasoning():
    print_separator("TEST 1: Socratic Reasoning (Text Only)")
    payload = {
        "model": MODEL_ID,
        "messages": [
            {"role": "system", "content": "You are Pete, the blunt and direct Yardmaster of the Iron Road."},
            {"role": "user", "content": "Pete, explain the difference between unified memory architecture and dedicated VRAM in exactly two sentences."}
        ],
        "max_tokens": 150,
        "temperature": 0.3
    }
    
    try:
        response = requests.post(API_URL, json=payload)
        response.raise_for_status()
        data = response.json()
        print(f"✅ Pete's Response:\n{data['choices'][0]['message']['content']}")
        print(f"⚡ Token Usage: {data['usage']}")
    except Exception as e:
        print(f"❌ Failed to reach LongCat-Next Inference Engine: {e}")

def test_multimodal_vision_routing():
    print_separator("TEST 2: Multimodal Architecture Hook (DiNA Router Check)")
    print("WARNING: This tests the unquantized DiNA vision matrices. Ensure RAM buffer is intact.\n")
    
    # We send a request designed to trigger the visual generation token stream.
    payload = {
        "model": MODEL_ID,
        "messages": [
            {"role": "user", "content": "Pete, generate an ambient image representing the cold steel of the Iron Road at dawn."}
        ],
        "max_tokens": 200,
        "temperature": 0.7
    }
    
    try:
        response = requests.post(API_URL, json=payload)
        response.raise_for_status()
        data = response.json()
        content = data['choices'][0]['message']['content']
        print(f"✅ Route Success! Multimodal block output length: {len(content)} characters.")
        if "<image>" in content or "iVBORw0KGgo" in content:
            print("🚀 DiNA Tokenizer confirmed active! Visual binary payload detected.")
        else:
            print("👁️ Response captured, but no direct image tokens embedded in this completion. Examine output:")
            print(content)
    except Exception as e:
        print(f"❌ Vision Router Crash Detected: {e}")

if __name__ == "__main__":
    print(f"Testing local vLLM API mapped to AWQ {MODEL_ID} at {API_URL}...\n")
    test_language_reasoning()
    test_multimodal_vision_routing()
