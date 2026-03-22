#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Serve Pete (Conductor)
# Model: Mistral-Small-4-119B-2603-eagle (safetensors via vLLM)
# Port: 8000 (default VLLM_URL)
# ═══════════════════════════════════════════════════════════════════════════════
set -e

MODEL_PATH="${HOME}/trinity-models/safetensors/Mistral-Small-4-119B-2603-eagle"
PORT="${VLLM_PORT:-8000}"
VENV="${HOME}/trinity-vllm-env"

# AMD Strix Halo optimizations
export HSA_OVERRIDE_GFX_VERSION=11.0.0
export ROCBLAS_USE_HIPBLASLT=1

if [ ! -d "$MODEL_PATH" ]; then
    echo "ERROR: Model not found at $MODEL_PATH"
    echo "Download with: git clone https://huggingface.co/mistralai/Mistral-Small-4-119B-2603-eagle $MODEL_PATH"
    exit 1
fi

echo "Starting Pete (Mistral Small 4 119B Eagle) on port $PORT..."
exec "${VENV}/bin/vllm" serve "$MODEL_PATH" \
    --port "$PORT" \
    --gpu-memory-utilization 0.5 \
    --max-model-len 32768 \
    --max-num-seqs 4 \
    --trust-remote-code \
    --no-enable-prefix-caching
