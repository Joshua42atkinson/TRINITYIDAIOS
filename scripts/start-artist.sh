#!/bin/bash
# ════════════════════════════════════════════════════════════════════
# ART — THE ARTIST STUDIO (Triad Swarm) — *** FINAL ***
# ════════════════════════════════════════════════════════════════════
# Visual design, ComfyUI orchestration, UI/UX workflows
# Ports: 8081, 8082, 8083 | Total: ~26GB (5.3 + 5.4 + 15GB)
#
# *** DO NOT CHANGE MODELS WITHOUT USER APPROVAL ***
# Brain:    Crow-9B-Opus-4.6-Distill (5.3GB)    — docs/model_specs/Crow.md
# Evaluator: OmniCoder-9B (5.4GB)              — docs/model_specs/OmniCoder.md
# Builder:   Qwen3-Coder-REAP-25B-Rust (15GB)  — docs/model_specs/Qwen3-REAP-Rust.md
# ════════════════════════════════════════════════════════════════════

set -e

MODEL_DIR="$HOME/trinity-models/gguf"
LLAMA_BIN="$HOME/Workflow/desktop_trinity/trinity-genesis/llama.cpp/build-vulkan/bin/llama-server"

# *** FINAL MODELS ***
BRAIN="Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf"
EVALUATOR="OmniCoder-9B-Q4_K_M.gguf"
BUILDER="Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf"

# Kill any existing llama-servers
pkill -f "llama-server.*--port 808" 2>/dev/null || true
sleep 2

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  🎨 ARTIST STUDIO — Triad Swarm                               ║"
echo "╠══════════════════════════════════════════════════════════════╣"
echo "║  Brain: Crow 9B Opus (5.3GB) on :8081                        ║"
echo "║  Evaluator: OmniCoder 9B (5.4GB) on :8082                    ║"
echo "║  Builder: Qwen 25B Rust (15GB) on :8083                      ║"
echo "║  Role: Visual design, ComfyUI, UI/UX                         ║"
echo "╚══════════════════════════════════════════════════════════════╝"

# Verify models exist
for model in "$BRAIN" "$EVALUATOR" "$BUILDER"; do
    if [ ! -f "$MODEL_DIR/$model" ]; then
        echo "❌ Model not found: $MODEL_DIR/$model"
        exit 1
    fi
done

if [ ! -f "$LLAMA_BIN" ]; then
    echo "❌ llama-server not found: $LLAMA_BIN"
    exit 1
fi

# Start Brain (Primary)
echo "🧠 Starting Artist Brain on :8081..."
nohup "$LLAMA_BIN" \
    -m "$MODEL_DIR/$BRAIN" \
    -c 32768 \
    --port 8081 \
    -ngl 99 \
    --threads 4 \
    > /tmp/artist-brain.log 2>&1 &
BRAIN_PID=$!

sleep 3

# Start Evaluator (Secondary)
echo "📊 Starting Artist Evaluator on :8082..."
nohup "$LLAMA_BIN" \
    -m "$MODEL_DIR/$EVALUATOR" \
    -c 16384 \
    --port 8082 \
    -ngl 99 \
    --threads 4 \
    > /tmp/artist-evaluator.log 2>&1 &
EVAL_PID=$!

sleep 3

# Start Builder (Tertiary)
echo "🔧 Starting Artist Builder on :8083..."
nohup "$LLAMA_BIN" \
    -m "$MODEL_DIR/$BUILDER" \
    -c 16384 \
    --port 8083 \
    -ngl 99 \
    --threads 6 \
    > /tmp/artist-builder.log 2>&1 &
BUILD_PID=$!

echo ""
echo "PIDs: Brain=$BRAIN_PID, Evaluator=$EVAL_PID, Builder=$BUILD_PID"
sleep 5

# Verify all started
echo ""
echo "Checking endpoints..."
for port in 8081 8082 8083; do
    if curl -s "http://localhost:$port/v1/models" | grep -q "object"; then
        echo "✅ Port $port: OK"
    else
        echo "⚠️  Port $port: Loading..."
    fi
done

echo ""
echo "📋 Logs: /tmp/artist-*.log"
