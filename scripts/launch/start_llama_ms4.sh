#!/bin/bash
# ═══════════════════════════════════════════════════════════════
# Trinity Llama-Server — Mistral Small 4 (119B) with Vision
# ═══════════════════════════════════════════════════════════════

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE="$(cd "$SCRIPT_DIR/../.." && pwd)"

LLAMA_SERVER="$WORKSPACE/llama.cpp/build-rocm/bin/llama-server"
MODEL="$HOME/trinity-models/llm/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf"
MMPROJ="$HOME/trinity-models/vision_and_ocr/mmproj-BF16.gguf"

PORT="${TRINITY_BRAIN_PORT:-8080}"

# 512K context for dual-persona massive stacks.
# Q8 KV cache keeps memory tight within 128GB UMA.
CONTEXT_SIZE=524288

# Check if llama.cpp gives us hipblas support
export HSA_OVERRIDE_GFX_VERSION=11.0.0
export LD_LIBRARY_PATH="$WORKSPACE/llama.cpp/build-rocm/bin:$LD_LIBRARY_PATH"

echo "╔══════════════════════════════════════════════════════════╗"
echo "║      TRINITY LLM — Mistral Small 4 119B + VISION        ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo "  Backend : llama.cpp (ROCm)"
echo "  Model   : Q4_K_M (~67GB)"
echo "  MMProj  : BF16 (~867MB)"
echo "  Context : $CONTEXT_SIZE tokens (2x 256K stacks)"
echo "  KV Cache: q8_0 / q8_0 (8-bit compressed)"
echo "  Port    : $PORT"
echo "═══════════════════════════════════════════════════════════"

# Ensure clean slate for port
pkill -f "llama-server.*$PORT" 2>/dev/null || true
sleep 1

exec "$LLAMA_SERVER" \
    -m "$MODEL" \
    --mmproj "$MMPROJ" \
    --host 0.0.0.0 --port "$PORT" \
    -c $CONTEXT_SIZE \
    -np 2 \
    -ctk q8_0 -ctv q8_0 \
    -ngl 99 \
    -fa on
