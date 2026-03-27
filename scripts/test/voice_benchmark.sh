#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY Voice Pipeline Benchmark
# Measures STT latency, TTS latency, and Voxtral availability
# ═══════════════════════════════════════════════════════════════════════════════
set -e

TRINITY_URL="${TRINITY_URL:-http://localhost:3000}"

echo "═══════════════════════════════════════════════════"
echo "  TRINITY VOICE BENCHMARK"
echo "═══════════════════════════════════════════════════"
echo ""

# ── 1. Trinity Health Check ──
echo "── 1. Trinity Server Health ──"
HEALTH=$(curl -s "${TRINITY_URL}/api/health" 2>/dev/null)
if [ -z "$HEALTH" ]; then
    echo "  ❌ Trinity server not responding at ${TRINITY_URL}"
    echo "  Start with: cargo run -p trinity --release"
    exit 1
fi
echo "  ✅ Trinity healthy"
echo "  $(echo "$HEALTH" | python3 -c 'import sys,json; d=json.load(sys.stdin); print(f"  Engine: {d.get(\"engine\",\"?\")}")' 2>/dev/null || echo "  (details unavailable)")"
echo ""

# ── 2. Voice Status ──
echo "── 2. Voice Pipeline Status ──"
VOICE_STATUS=$(curl -s "${TRINITY_URL}/api/voice/status" 2>/dev/null)
if [ -z "$VOICE_STATUS" ]; then
    echo "  ⚠️  Voice status endpoint not available"
else
    echo "  $VOICE_STATUS" | python3 -c '
import sys, json
try:
    d = json.load(sys.stdin)
    print(f"  Pipeline: {d.get(\"pipeline\", \"??\")}")
    print(f"  Status:   {d.get(\"status\", \"??\")}")
    print(f"  Voxtral:  {\"✅ available\" if d.get(\"voxtral_available\") else \"⬚ not running\"}")
except:
    print("  (could not parse voice status)")
' 2>/dev/null
fi
echo ""

# ── 3. TTS Latency Benchmark (Supertonic-2 / ONNX) ──
echo "── 3. TTS Latency (Supertonic-2 via ONNX) ──"
TEST_TEXT="The Iron Road stretches before you. Choose your words wisely, Yardmaster."
START_TTS=$(date +%s%N)
TTS_RESULT=$(curl -s -w "%{http_code}" -o /tmp/trinity_tts_bench.wav \
    -X POST "${TRINITY_URL}/api/voice/speak" \
    -H "Content-Type: application/json" \
    -d "{\"text\": \"${TEST_TEXT}\", \"voice\": \"pete\"}" 2>/dev/null)
END_TTS=$(date +%s%N)

if [ "$TTS_RESULT" = "200" ] && [ -f /tmp/trinity_tts_bench.wav ]; then
    TTS_MS=$(( (END_TTS - START_TTS) / 1000000 ))
    TTS_SIZE=$(stat -c %s /tmp/trinity_tts_bench.wav 2>/dev/null || echo "?")
    echo "  ✅ TTS synthesis: ${TTS_MS}ms"
    echo "  Audio size: ${TTS_SIZE} bytes"
    rm -f /tmp/trinity_tts_bench.wav
else
    echo "  ⚠️  TTS endpoint returned: ${TTS_RESULT}"
    echo "  (Voice may need an active microphone session to initialize)"
fi
echo ""

# ── 4. Voxtral-4B Check ──
echo "── 4. Voxtral-4B (port 8100) ──"
VOXTRAL=$(curl -s "http://127.0.0.1:8100/v1/models" 2>/dev/null)
if [ -z "$VOXTRAL" ]; then
    echo "  ⬚ Voxtral not running on :8100"
    echo "  Start with: scripts/launch/start_voxtral.sh"
else
    echo "  ✅ Voxtral-4B available"
    echo "  $VOXTRAL" | python3 -c '
import sys, json
try:
    d = json.load(sys.stdin)
    models = d.get("data", [])
    for m in models:
        print(f"  Model: {m.get(\"id\", \"?\")}")
except:
    print("  (could not parse models)")
' 2>/dev/null
fi
echo ""

# ── 5. Inference Backend Check ──
echo "── 5. Inference Backend ──"
INFERENCE=$(curl -s "${TRINITY_URL}/api/inference/status" 2>/dev/null)
if [ -z "$INFERENCE" ]; then
    echo "  ⚠️  Inference status not available"
else
    echo "  $INFERENCE" | python3 -c '
import sys, json
try:
    d = json.load(sys.stdin)
    print(f"  Active:    {d.get(\"active_backend\", \"?\")} @ {d.get(\"active_url\", \"?\")}")
    print(f"  Context:   {d.get(\"ctx_size\", \"?\")} tokens")
    print(f"  Max Reply: {d.get(\"max_tokens\", \"?\")} tokens")
    backends = d.get("backends", [])
    healthy = sum(1 for b in backends if b.get("healthy"))
    print(f"  Backends:  {healthy}/{len(backends)} healthy")
    for b in backends:
        status = "✅" if b.get("healthy") else "⬚"
        model = b.get("model_name", "")
        name = b.get("name", "?")
        print(f"    {status} {name}: {b.get(\"base_url\", \"?\")} {f\"({model})\" if model else \"\"}")
except:
    print("  (could not parse inference status)")
' 2>/dev/null
fi
echo ""

echo "═══════════════════════════════════════════════════"
echo "  BENCHMARK COMPLETE"
echo "═══════════════════════════════════════════════════"
