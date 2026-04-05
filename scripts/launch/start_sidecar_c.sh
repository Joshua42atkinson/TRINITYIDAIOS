#!/bin/bash
# ==============================================================================
# TRINITY AI OS - SIDECAR C : ART STUDIO (Tempo Engine / Media)
# Model: Gemma-4-E4B-it-AWQ-4bit + Diffusers Dynamics
# Port: 8003
# Context Limit: 131,072 Tokens (TurboQuant 4-bit)
# VRAM Utilization: 0.07 (~9GB)
# ==============================================================================

MODEL_DIR="$HOME/trinity-models/vllm"
TARGET_MODEL="$MODEL_DIR/gemma-4-E4B-it-AWQ-4bit"
CONTAINER_NAME="vllm_sidecar_c"

echo "⚙️  Initializing Sidecar C: Art Studio (E4B Vibe Conductor)..."

if ! distrobox list | grep -q "$CONTAINER_NAME"; then
    echo "📦 Creating isolated container: $CONTAINER_NAME..."
    distrobox create --name "$CONTAINER_NAME" --image kyuz0/vllm-therock-gfx1151 --yes
fi

if [ ! -d "$TARGET_MODEL" ]; then
    echo "❌ CRITICAL: Weights missing at $TARGET_MODEL"
    exit 1
fi

echo "🚀 Starting vLLM Tempo Engine in $CONTAINER_NAME..."

export TQ4_K_BITS=4
export TQ4_V_BITS=4

distrobox enter "$CONTAINER_NAME" -- vllm serve "$TARGET_MODEL" \
    --port 8003 \
    --gpu-memory-utilization 0.07 \
    --max-model-len 131072 \
    --dtype half \
    --attention-backend CUSTOM \
    --served-model-name "Tempo_Engine"
