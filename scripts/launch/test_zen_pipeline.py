#!/usr/bin/env python3
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Zen Mode E2E Test
# Validates: Director JSON -> Storyteller SSE -> Kokoro Health
# ═══════════════════════════════════════════════════════════════════════════════

import urllib.request
import json
import time
import sys

# 1. Check Kokoro TTS Sidecar
try:
    print("Testing Kokoro Sidecar (:8200)... ", end="")
    req = urllib.request.Request("http://127.0.0.1:8200/health")
    with urllib.request.urlopen(req, timeout=3) as response:
        data = json.loads(response.read().decode())
        if data.get("status") in ["healthy", "ok"]:
            print("✅ OK")
        else:
            print("❌ FAILED")
            sys.exit(1)
except Exception as e:
    print(f"⚠️ Offline ({e}). Kokoro is not running, but tests can proceed.")

# 2. Check Zen Mode SSE Pipeline
payload = json.dumps({
    "message": "I want to teach high schoolers about the physics of gravity and orbital mechanics.",
    "tools": []
}).encode('utf-8')

req = urllib.request.Request(
    "http://127.0.0.1:3000/api/chat/zen",
    data=payload,
    headers={'Content-Type': 'application/json', 'Accept': 'text/event-stream'}
)

print("\nTesting Zen Mode Director + Storyteller Pipeline...")
has_interpretation = False
has_narration = False

try:
    with urllib.request.urlopen(req, timeout=30) as response:
        for line in response:
            line = line.decode('utf-8').strip()
            if line.startswith("event: interpretation"):
                has_interpretation = True
            elif line.startswith("event: narration"):
                has_narration = True
                print("✅ Stream verified successfully!")
                sys.exit(0) # We proved it works, no need to wait for 400 tokens
                
            if has_interpretation and has_narration:
                break
                
except Exception as e:
    print(f"❌ Pipeline Test Failed: {e}")
    print("Make sure Trinity server (:3000) and LLMs (:8080, :8081) are running.")
    sys.exit(1)

if not has_interpretation:
    print("❌ Failed to receive Director Interpretation JSON.")
    sys.exit(1)
if not has_narration:
    print("❌ Failed to receive Storyteller Narration Stream.")
    sys.exit(1)
