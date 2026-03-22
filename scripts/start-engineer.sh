#!/bin/bash
# ════════════════════════════════════════════════════════════════════
# Y — THE ENGINEER (Step-Flash-121B) — *** FINAL ***
# ════════════════════════════════════════════════════════════════════
# Code generation, bugfix, refactoring, local evaluation
# Port: 8081 | Context: 16K | Memory: ~98GB (83GB model + 15GB KV cache)
#
# *** DO NOT CHANGE MODEL WITHOUT USER APPROVAL ***
# Model: Step-3.5-Flash-REAP-121B-A11B.Q4_K_S
# Doc: docs/model_specs/Step-Flash.md
# ════════════════════════════════════════════════════════════════════

set -e

MODEL_DIR="$HOME/trinity-models/gguf"
MODEL="Step-3.5-Flash-REAP-121B-A11B.Q4_K_S.gguf"
LLAMA_BIN="$HOME/Workflow/desktop_trinity/trinity-genesis/llama.cpp/build-vulkan/bin/llama-server"
PORT=8081
CTX=16384

# Kill any existing llama-server on this port
pkill -f "llama-server.*--port $PORT" 2>/dev/null || true
sleep 2

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  ⚙️  THE ENGINEER — Code Forge                                ║"
echo "╠══════════════════════════════════════════════════════════════╣"
echo "║  Model: Step-Flash-121B (83GB Q4_K_S)                        ║"
echo "║  Port: $PORT | Context: 16K tokens                                ║"
echo "║  Role: Code generation, bugfix, refactoring                  ║"
echo "╚══════════════════════════════════════════════════════════════╝"

if [ ! -f "$MODEL_DIR/$MODEL" ]; then
    echo "❌ Model not found: $MODEL_DIR/$MODEL"
    exit 1
fi

if [ ! -f "$LLAMA_BIN" ]; then
    echo "❌ llama-server not found: $LLAMA_BIN"
    exit 1
fi

echo "🚀 Starting Engineer on port $PORT..."
nohup "$LLAMA_BIN" \
    -m "$MODEL_DIR/$MODEL" \
    -c $CTX \
    --port $PORT \
    -ngl 99 \
    --threads 8 \
    > /tmp/engineer.log 2>&1 &

echo "PID: $!"
echo "⏳ Loading 83GB model... this may take 60-90 seconds"
sleep 10

# Verify it started
if curl -s "http://localhost:$PORT/v1/models" | grep -q "Step"; then
    echo "✅ Engineer is running on http://localhost:$PORT"
    echo "📋 Log: /tmp/engineer.log"
else
    echo "⏳ Engineer still loading..."
    echo "📋 Check log: tail -f /tmp/engineer.log"
fi
