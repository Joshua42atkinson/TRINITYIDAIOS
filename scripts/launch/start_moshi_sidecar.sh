#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Moshi Voice Sidecar (Python/GPU)
# Full-duplex conversational AI on ROCm GPU
# Isolated Python process — crashes don't affect Trinity core (Rust)
# ═══════════════════════════════════════════════════════════════════════════════
set -e

VENV="${HOME}/trinity-vllm-env"
PORT="${MOSHI_PORT:-8998}"

export HSA_OVERRIDE_GFX_VERSION=11.0.0
export ROCBLAS_USE_HIPBLASLT=1

if [ ! -d "$VENV" ]; then
    echo "ERROR: Python venv not found at $VENV"
    exit 1
fi

source "$VENV/bin/activate"

echo "═══════════════════════════════════════"
echo "  Moshi Voice Sidecar (GPU)"
echo "  Port: $PORT"
echo "  Model: moshiko (PyTorch ROCm)"
echo "  Access: https://localhost:$PORT"
echo "═══════════════════════════════════════"

exec python -m moshi.server \
    --port "$PORT" \
    --hf-repo kyutai/moshiko-pytorch-bf16
