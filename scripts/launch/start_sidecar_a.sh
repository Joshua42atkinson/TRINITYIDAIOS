#!/bin/bash
# ==============================================================================
# TRINITY AI OS - SIDECAR A : IRON ROAD (Socratic Reasoning Engine)
# Model: Gemma-4-31B-it-AWQ-4bit
# Port: 8001
# Context Limit: 327,680 Tokens (TurboQuant 4-bit)
# VRAM Utilization: 0.35 (~45GB)
# ==============================================================================

MODEL_DIR="$HOME/trinity-models/vllm"
TARGET_MODEL="$MODEL_DIR/gemma-4-31B-it-AWQ-4bit"
CONTAINER_NAME="vllm_sidecar_a"

echo "⚙️  Initializing Sidecar A: Iron Road (31B Great Recycler)..."

# Ensure container exists
if ! distrobox list | grep -q "$CONTAINER_NAME"; then
    echo "📦 Creating isolated container: $CONTAINER_NAME..."
    distrobox create --name "$CONTAINER_NAME" --image kyuz0/vllm-therock-gfx1151 --yes
fi

# Verify Weights
if [ ! -d "$TARGET_MODEL" ]; then
    echo "❌ CRITICAL: Weights missing at $TARGET_MODEL"
    exit 1
fi

echo "🚀 Starting vLLM Socratic Engine in $CONTAINER_NAME..."

# TurboQuant Configuration
export TQ4_K_BITS=4
export TQ4_V_BITS=4

distrobox enter "$CONTAINER_NAME" -- vllm serve "$TARGET_MODEL" \
    --port 8001 \
    --gpu-memory-utilization 0.35 \
    --max-model-len 327680 \
    --dtype half \
    --attention-backend CUSTOM \
    --served-model-name "Great_Recycler"
