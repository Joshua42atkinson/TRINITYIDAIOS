#!/bin/bash
set -e

MODELS_DIR="$HOME/trinity-models"
mkdir -p "$MODELS_DIR/gguf"
mkdir -p "$MODELS_DIR/safetensors"

echo "Downloading Q4 GGUF for FLUX (Images)..."
hf download city96/FLUX.1-schnell-gguf flux1-schnell-Q4_K_S.gguf --local-dir "$MODELS_DIR/gguf"

echo "Downloading TripoSR (3D Avatar - natively small 2GB)..."
hf download stabilityai/TripoSR --local-dir "$MODELS_DIR/safetensors/TripoSR"

echo "Downloading CogVideoX-2b (Video)..."
echo "(Note: CogVideo is streamed via NF4 4-bit transformers, so we pull the base safetensors.)"
hf download THUDM/CogVideoX-2b --local-dir "$MODELS_DIR/safetensors/CogVideoX-2b"

echo "Downloading ACE-Step v1 3.5B (Music Generation - Apache 2.0)..."
hf download ACE-Step/ACE-Step-v1-3.5B --local-dir "$MODELS_DIR/safetensors/ACE-Step-v1-3.5B"

echo "Downloads complete!"
