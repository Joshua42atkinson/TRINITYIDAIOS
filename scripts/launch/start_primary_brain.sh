#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════
# Trinity Primary Brain Sidecar — Nemotron 3 Super Launcher
#
# Starts the Primary Brain agent:
#   Nemotron 3 Super 120B: Complex reasoning, research, evaluation
#   Port: 8100 (OpenAI-compatible API)
#
# This is the PRIMARY BRAIN that receives broadcasts from the Great Recycler
# (NPU) and handles complex reasoning tasks.
#
# Usage:
#   ./scripts/launch/start_primary_brain.sh   # Normal start
# ═══════════════════════════════════════════════════════════════════════
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_ROOT"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     TRINITY PRIMARY BRAIN — Nemotron 3 Super (120B)         ║"
echo "╚══════════════════════════════════════════════════════════════╝"

# Kill any stale primary brain processes
echo "Cleaning up stale processes..."
pkill -f "llama-server.*8100" 2>/dev/null || true
pkill -f "Nemotron.*8100" 2>/dev/null || true
sleep 1

# Verify model exists (ggml-org Q4_K - single file)
NEMOTRON_MODEL="$HOME/trinity_models/Nemotron_ggml/Nemotron-3-Super-120B-Q4_K.gguf"

if [ ! -f "$NEMOTRON_MODEL" ]; then
    echo "ERROR: Nemotron 3 Super model not found: $NEMOTRON_MODEL"
    echo ""
    echo "Download from HuggingFace (ggml-org, compatible with llama.cpp):"
    echo "  pip install huggingface_hub"
    echo "  huggingface-cli download ggml-org/nemotron-3-super-120b-GGUF \\"
    echo "    Nemotron-3-Super-120B-Q4_K.gguf --local-dir ~/trinity_models/Nemotron_ggml/"
    exit 1
fi

echo "Model found: $(du -h "$NEMOTRON_MODEL" | cut -f1)"
echo "Model: ggml-org Q4_K (llama.cpp compatible)"

# Check if llama.cpp exists (CMake build)
LLAMA_CPP="$PROJECT_ROOT/llama.cpp/build/bin/llama-server"
if [ ! -f "$LLAMA_CPP" ]; then
    echo "Building llama.cpp with CMake..."
    mkdir -p "$PROJECT_ROOT/llama.cpp/build"
    cd "$PROJECT_ROOT/llama.cpp/build"
    cmake .. -DGGML_HIPBLAS=ON -DCMAKE_BUILD_TYPE=Release
    make -j$(nproc) llama-server
    cd "$PROJECT_ROOT"
fi

# Start llama-server with Nemotron 3 Super (ggml-org Q4_K)
echo ""
echo "Starting Primary Brain (Nemotron 3 Super)..."
echo "  Model: Nemotron-3-Super-120B-Q4_K (ggml-org)"
echo "  Port:  http://127.0.0.1:8100"
echo "  Memory: 70GB model + KV cache"
echo "  Bandwidth: 212 GB/s available (Strix Halo)"
echo ""

# Run llama-server with ggml-org GGUF
# -ngl 99: Offload all layers to GPU (unified memory)
# -c 32768: 32K context window (can go to 128K)
# -fa on: Flash attention (required in recent llama.cpp)
# --host 0.0.0.0: Allow external access for Purdue collaboration
"$LLAMA_CPP" \
    --model "$HOME/trinity_models/Nemotron_ggml/Nemotron-3-Super-120B-Q4_K.gguf" \
    --port 8100 \
    --host 0.0.0.0 \
    --ctx-size 32768 \
    --ngl 99 \
    -fa on \
    --log-disable \
    2>&1 &

LLAMA_PID=$!
echo "llama-server PID: $LLAMA_PID"

# Wait for server to be ready
echo "Waiting for Primary Brain to load (this takes 2-3 minutes for 70GB model)..."
for i in $(seq 1 300); do
    if curl -s http://127.0.0.1:8100/v1/models > /dev/null 2>&1; then
        echo "Primary Brain is READY!"
        break
    fi
    if ! kill -0 $LLAMA_PID 2>/dev/null; then
        echo "ERROR: llama-server crashed during startup"
        exit 1
    fi
    sleep 1
done

echo ""
echo "Primary Brain is running. Endpoints:"
echo "  GET  http://127.0.0.1:8100/v1/models"
echo "  POST http://127.0.0.1:8100/v1/chat/completions"
echo ""
echo "This is the PRIMARY BRAIN for complex reasoning."
echo "It receives broadcasts from Great Recycler (NPU) on port 52625."
echo ""
echo "Press Ctrl+C to stop."

wait $LLAMA_PID
