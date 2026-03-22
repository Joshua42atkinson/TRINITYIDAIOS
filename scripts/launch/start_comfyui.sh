#!/bin/bash
# Start ComfyUI for ART pipeline (SDXL Turbo + HunyuanVideo + ACE-Step + Trellis)
# Memory: ~7-32GB depending on loaded models
set -e

COMFYUI_DIR="$HOME/ComfyUI"
VENV="$HOME/trinity-vllm-env"
PORT="${COMFYUI_PORT:-8188}"

if [ ! -d "$COMFYUI_DIR" ]; then
    echo "ERROR: ComfyUI not found at $COMFYUI_DIR"
    exit 1
fi

echo "Starting ComfyUI on :${PORT}..."
echo "Models: SDXL Turbo, HunyuanVideo, ACE-Step, TextureAlchemy"

source "$VENV/bin/activate"
cd "$COMFYUI_DIR"

# Use ROCm GPU, listen on all interfaces for network access
python3 main.py \
    --port "$PORT" \
    --listen 0.0.0.0 \
    --preview-method auto \
    2>&1
