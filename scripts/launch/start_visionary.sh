#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════
# Trinity Visionary Sidecar — Vision Model Launcher
#
# Starts Qwen3.5-35B-A3B + mmproj for vision processing
#   Vision API: http://127.0.0.1:8081
#
# Usage:
#   ./scripts/launch/start_visionary.sh
# ═══════════════════════════════════════════════════════════════════════
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_ROOT"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     TRINITY VISIONARY SIDECAR — Qwen3.5-35B + mmproj        ║"
echo "╚══════════════════════════════════════════════════════════════╝"

# Kill any stale visionary processes
echo "Cleaning up stale processes..."
pkill -f "llama-server.*8081" 2>/dev/null || true
sleep 1

# Model paths
VISION_MODEL="$HOME/.lmstudio/models/lmstudio-community/Qwen3.5-35B-A3B-GGUF/Qwen3.5-35B-A3B-Q4_K_M.gguf"
MMPROJ="$HOME/ai_models/gguf/mmproj-Qwen3.5-35B-A3B-BF16.gguf"

# Verify models exist
if [ ! -f "$VISION_MODEL" ]; then
    echo "ERROR: Vision model not found: $VISION_MODEL"
    exit 1
fi
if [ ! -f "$MMPROJ" ]; then
    echo "ERROR: mmproj not found: $MMPROJ"
    exit 1
fi

echo "Vision model: $(du -h "$VISION_MODEL" | cut -f1)"
echo "mmproj: $(du -h "$MMPROJ" | cut -f1)"

# Start llama-server with vision model
echo ""
echo "Starting llama-server on port 8081..."
echo "  Model: Qwen3.5-35B-A3B-Q4_K_M"
echo "  mmproj: enabled for vision"
echo "  API: http://127.0.0.1:8081/v1/chat/completions"

llama-server \
    --model "$VISION_MODEL" \
    --mmproj "$MMPROJ" \
    --port 8081 \
    --host 127.0.0.1 \
    --ctx-size 32768 \
    --ngl 99 \
    --n-gpu-layers 99 \
    --threads 8 \
    --batch-size 512 \
    --ubatch-size 128 \
    --flash-attn \
    --log-disable

# Note: To run in background, use:
# nohup ./scripts/launch/start_visionary.sh > logs/visionary.log 2>&1 &
