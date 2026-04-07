#!/bin/bash
# ═══════════════════════════════════════════════════════════════
# Trinity — Great Recycler (Port 8001)
# Gemma-4-31B Dense AWQ — Socratic OS Brain
# ═══════════════════════════════════════════════════════════════
# Run with: distrobox enter vllm -- bash /path/to/start_recycler.sh

set -e

export VLLM_ROCM_SHUFFLE_KV_CACHE_LAYOUT=0
export PYTORCH_TUNABLEOP_ENABLED=1
export HSA_OVERRIDE_GFX_VERSION=11.5.1

MODEL_DIR="$HOME/trinity-models/vllm"
MAIN_MODEL="$MODEL_DIR/gemma-4-31B-it-AWQ-4bit"
DRAFT_MODEL="$MODEL_DIR/gemma-4-E2B-it"

echo "🚀 Starting Great Recycler on port 8001..."
echo "   Main model:  $MAIN_MODEL"
echo "   Draft model: $DRAFT_MODEL"

# Check models exist
if [ ! -d "$MAIN_MODEL" ]; then
    echo "❌ Main model not found: $MAIN_MODEL"
    exit 1
fi

if [ ! -d "$DRAFT_MODEL" ]; then
    echo "⚠️  Draft model not found, launching WITHOUT speculative decoding"
    exec /opt/venv/bin/vllm serve "$MAIN_MODEL" \
        --port 8001 \
        --gpu-memory-utilization 0.45 \
        --max-model-len 32768 \
        --enable-prefix-caching \
        --enable-chunked-prefill \
        --max-num-batched-tokens 4096 \
        --dtype half \
        --trust-remote-code \
        --served-model-name Great_Recycler
fi

echo "🔮 Launching with speculative decoding (E2B draft)..."
exec /opt/venv/bin/vllm serve "$MAIN_MODEL" \
    --port 8001 \
    --gpu-memory-utilization 0.45 \
    --max-model-len 32768 \
    --enable-prefix-caching \
    --enable-chunked-prefill \
    --max-num-batched-tokens 4096 \
    --speculative-config '{"method": "draft", "model": "'"$DRAFT_MODEL"'", "num_speculative_tokens": 5}' \
    --dtype half \
    --trust-remote-code \
    --served-model-name Great_Recycler
