#!/bin/bash
# ════════════════════════════════════════════════════════════════════
# P — THE CONDUCTOR (Nemotron-3-Super-120B) — *** FINAL ***
# ════════════════════════════════════════════════════════════════════
# Party leader, orchestration, strategy, ADDIE Analysis/Design/Evaluation
# Port: 8081 | Context: 128K | Memory: ~90GB (74GB model + 16GB KV cache)
#
# *** DO NOT CHANGE MODEL WITHOUT USER APPROVAL ***
# Model: NVIDIA-Nemotron-3-Super-120B-A12B-BF16-Q4_K_M
# Doc: docs/model_specs/Nemotron.md
# ════════════════════════════════════════════════════════════════════

set -e

MODEL_DIR="$HOME/ai_models/gguf/conductor"
MODEL="NVIDIA-Nemotron-3-Super-120B-A12B-BF16-Q4_K_M-merged.gguf"
LLAMA_BIN="$HOME/Workflow/desktop_trinity/trinity-genesis/llama.cpp/build-vulkan/bin/llama-server"
PORT=8081
CTX=131072

# Kill any existing llama-server on this port
pkill -f "llama-server.*--port $PORT" 2>/dev/null || true
sleep 2

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  🎭 THE CONDUCTOR — Party Leader                              ║"
echo "╠══════════════════════════════════════════════════════════════╣"
echo "║  Model: Nemotron-3-Super-120B (74GB Q4_K_M)                  ║"
echo "║  Port: $PORT | Context: 128K tokens                                ║"
echo "║  Role: Strategic oversight, ADDIE orchestration              ║"
echo "╚══════════════════════════════════════════════════════════════╝"

if [ ! -f "$MODEL_DIR/$MODEL" ]; then
    echo "❌ Model not found: $MODEL_DIR/$MODEL"
    echo "   Expected 3-part split files in ~/ai_models/gguf/conductor/"
    exit 1
fi

if [ ! -f "$LLAMA_BIN" ]; then
    echo "❌ llama-server not found: $LLAMA_BIN"
    exit 1
fi

echo "🚀 Starting Conductor on port $PORT..."
nohup "$LLAMA_BIN" \
    -m "$MODEL_DIR/$MODEL" \
    -c $CTX \
    --port $PORT \
    -ngl 99 \
    --threads 8 \
    > /tmp/conductor.log 2>&1 &

echo "PID: $!"
echo "⏳ Loading 74GB model... this may take 60-90 seconds"
sleep 10

# Verify it started
if curl -s "http://localhost:$PORT/v1/models" | grep -q "Nemotron"; then
    echo "✅ Conductor is running on http://localhost:$PORT"
    echo "📋 Log: /tmp/conductor.log"
else
    echo "⏳ Conductor still loading..."
    echo "📋 Check log: tail -f /tmp/conductor.log"
fi
