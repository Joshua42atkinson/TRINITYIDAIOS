#!/bin/bash
# ═══════════════════════════════════════════════════════════════
# Trinity — Programmer Pete (Port 8002)
# Gemma-4-26B MoE (A4B) AWQ — IDE Vision Agent
# ═══════════════════════════════════════════════════════════════
# Run with: distrobox enter vllm -- bash /path/to/start_pete.sh

set -e

export VLLM_ROCM_SHUFFLE_KV_CACHE_LAYOUT=0
export PYTORCH_TUNABLEOP_ENABLED=1
export HSA_OVERRIDE_GFX_VERSION=11.5.1

MODEL_DIR="$HOME/trinity-models/vllm"
PETE_MODEL="$MODEL_DIR/gemma-4-26B-A4B-it-AWQ-4bit"

echo "🚀 Starting Programmer Pete on port 8002..."
echo "   Model: $PETE_MODEL"

if [ ! -d "$PETE_MODEL" ]; then
    echo "❌ Pete model not found: $PETE_MODEL"
    exit 1
fi

exec /opt/venv/bin/vllm serve "$PETE_MODEL" \
    --port 8002 \
    --gpu-memory-utilization 0.30 \
    --max-model-len 32768 \
    --enable-prefix-caching \
    --dtype half \
    --trust-remote-code \
    --served-model-name Programmer_Pete
