#!/bin/bash
# ==============================================================================
# TRINITY AI OS - SIDECAR B : THE YARD IDE (Yardmaster Pete)
# Model: Gemma-4-26B-A4B-it-AWQ-4bit
# Port: 8002
# Context Limit: 327,680 Tokens (TurboQuant 4-bit)
# VRAM Utilization: 0.18 (~23GB)
# ==============================================================================

MODEL_DIR="$HOME/trinity-models/vllm"
TARGET_MODEL="$MODEL_DIR/gemma-4-26B-A4B-it-AWQ-4bit"
CONTAINER_NAME="vllm_sidecar_b"

echo "⚙️  Initializing Sidecar B: The Yard (26B Programmer Pete)..."

if ! distrobox list | grep -q "$CONTAINER_NAME"; then
    echo "📦 Creating isolated container: $CONTAINER_NAME..."
    distrobox create --name "$CONTAINER_NAME" --image kyuz0/vllm-therock-gfx1151 --yes
fi

if [ ! -d "$TARGET_MODEL" ]; then
    echo "❌ CRITICAL: Weights missing at $TARGET_MODEL"
    exit 1
fi

echo "🚀 Starting vLLM Action Engine in $CONTAINER_NAME..."

export TQ4_K_BITS=4
export TQ4_V_BITS=4

distrobox enter "$CONTAINER_NAME" -- vllm serve "$TARGET_MODEL" \
    --port 8002 \
    --gpu-memory-utilization 0.18 \
    --max-model-len 327680 \
    --dtype half \
    --attention-backend CUSTOM \
    --served-model-name "Programmer_Pete"
