#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════
# ACE-Step 1.5 Model Downloader for ComfyUI
#
# Downloads the ACE-Step 1.5 Turbo AIO model for local music generation
# This model generates full songs in under 10 seconds on RTX 3090+
#
# Usage:
#   ./scripts/model_downloads/download_ace_step.sh
# ═══════════════════════════════════════════════════════════════════════
set -euo pipefail

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     ACE-Step 1.5 Music Generation Model Downloader         ║"
echo "╚══════════════════════════════════════════════════════════════╝"

COMFYUI_MODELS="$HOME/ComfyUI/models"
CHECKPOINTS_DIR="$COMFYUI_MODELS/checkpoints"

# Create directories
mkdir -p "$CHECKPOINTS_DIR"

echo ""
echo "ACE-Step 1.5 offers two modes:"
echo "  1. Cloud mode (requires API key from acemusic.ai)"
echo "  2. Local mode (runs on your GPU)"
echo ""
echo "This script downloads the LOCAL model (~4GB)."
echo ""

# Check if huggingface-cli is installed
if ! command -v huggingface-cli &> /dev/null; then
    echo "Installing huggingface_hub..."
    pip install huggingface_hub --quiet
fi

# Download the AIO (All-In-One) checkpoint
echo "Downloading ACE-Step 1.5 Turbo AIO model (~4GB)..."
echo "This model packages everything into a single checkpoint."
echo ""

huggingface-cli download ACE-Step/ACE-Step-1.5-Turbo-AIO \
    ace_step_1.5_turbo_aio.safetensors \
    --local-dir "$CHECKPOINTS_DIR" \
    --local-dir-use-symlinks False

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "ACE-Step 1.5 setup complete!"
echo ""
echo "Model location:"
echo "  $CHECKPOINTS_DIR/ace_step_1.5_turbo_aio.safetensors"
echo ""
echo "To use in ComfyUI:"
echo "  1. Restart ComfyUI"
echo "  2. Load example workflow from:"
echo "     ~/ComfyUI/custom_nodes/ACE-Step-ComfyUI/workflows/"
echo "  3. Or use ACE-Step nodes directly"
echo ""
echo "Performance:"
echo "  - Under 10 seconds per full song on RTX 3090"
echo "  - Under 2 seconds on A100"
echo "  - Works on 8GB VRAM with optimizations"
echo ""
echo "Alternative: Cloud mode (no local GPU needed)"
echo "  Get API key at: https://acemusic.ai/api-key"
echo "════════════════════════════════════════════════════════════════"
