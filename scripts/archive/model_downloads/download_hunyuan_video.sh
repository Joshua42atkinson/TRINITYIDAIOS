#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════
# HunyuanVideo Model Downloader for ComfyUI
#
# Downloads the official fp8 scaled weights for HunyuanVideo
# These weights are ~14GB and work with 8GB VRAM via temporal tiling
#
# Usage:
#   ./scripts/model_downloads/download_hunyuan_video.sh
# ═══════════════════════════════════════════════════════════════════════
set -euo pipefail

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     HunyuanVideo Model Downloader for ComfyUI              ║"
echo "╚══════════════════════════════════════════════════════════════╝"

COMFYUI_MODELS="$HOME/ComfyUI/models"
DIFFUSION_DIR="$COMFYUI_MODELS/DiffusionModels"
LLM_DIR="$COMFYUI_MODELS/LLM"
TEXT_ENCODER_DIR="$COMFYUI_MODELS/text_encoders"
VAE_DIR="$COMFYUI_MODELS/vae"

# Create directories
mkdir -p "$DIFFUSION_DIR" "$LLM_DIR" "$TEXT_ENCODER_DIR" "$VAE_DIR"

echo ""
echo "Downloading HunyuanVideo fp8 scaled weights (~14GB)..."
echo "This version works with 8GB VRAM via temporal tiling."
echo ""

# Check if huggingface-cli is installed
if ! command -v huggingface-cli &> /dev/null; then
    echo "Installing huggingface_hub..."
    pip install huggingface_hub --quiet
fi

# Download the fp8 scaled model (recommended for quality/memory balance)
echo "Downloading fp8 scaled model..."
huggingface-cli download tencent/HunyuanVideo \
    hunyuan-video-t2v-720p/transformers/mp_rank_00_model_states_fp8.pt \
    --local-dir "$DIFFUSION_DIR/hunyuan_video" \
    --local-dir-use-symlinks False

echo ""
echo "Downloading text encoder (LLaMA 3.1 8B for HunyuanVideo)..."
echo "This is required for prompt encoding."
echo ""

# Download text encoder
huggingface-cli download Kijai/HunyuanVideo-text_encoder \
    --local-dir "$TEXT_ENCODER_DIR/hunyuan_video" \
    --local-dir-use-symlinks False 2>/dev/null || {
    echo "Text encoder download failed - you may need to download manually:"
    echo "  huggingface-cli download Kijai/HunyuanVideo-text_encoder \\"
    echo "    --local-dir ~/ComfyUI/models/text_encoders/hunyuan_video/"
}

echo ""
echo "Downloading VAE..."
huggingface-cli download tencent/HunyuanVideo \
    hunyuan-video-t2v-720p/vae/3d-vae.pt \
    --local-dir "$VAE_DIR/hunyuan_video" \
    --local-dir-use-symlinks False 2>/dev/null || {
    echo "VAE download failed - you may need to download manually"
}

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "HunyuanVideo setup complete!"
echo ""
echo "Model locations:"
echo "  Diffusion: $DIFFUSION_DIR/hunyuan_video/"
echo "  Text Enc:  $TEXT_ENCODER_DIR/hunyuan_video/"
echo "  VAE:       $VAE_DIR/hunyuan_video/"
echo ""
echo "To use in ComfyUI:"
echo "  1. Start ComfyUI"
echo "  2. Load example workflow from:"
echo "     ~/ComfyUI/custom_nodes/ComfyUI-HunyuanVideoWrapper/example_workflows/"
echo "  3. Select fp8_scaled quantization in the model loader"
echo ""
echo "Memory requirements:"
echo "  fp8 model: ~14GB VRAM (works with temporal tiling on 8GB)"
echo "════════════════════════════════════════════════════════════════"
