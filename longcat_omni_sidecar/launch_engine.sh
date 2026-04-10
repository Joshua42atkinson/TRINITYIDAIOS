#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — launch_engine.sh (Tauri Sidecar Proxy)
# ═══════════════════════════════════════════════════════════════════════════════
#
# PURPOSE:  Launch the deployed LongCat-Next Inference Matrix locally.
# HARDWARE: AMD Strix Halo (APU) — gfx1151, 128GB unified LPDDR5x
#
# ARCHITECTURE:
#   - Executes pure C++ implementation via Llama.cpp backend.
#   - NO DOCKER. NO PYTHON DEPENDENCIES.
#   - Fully integrated to be called via Tauri Rust Sidecar API.
#   - Directly maps to port 8010 utilizing OpenAI-compatible API format.
#
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

echo "🐱 LongCat-Next Native Backend — Trinity AI Tauri Application"
echo "════════════════════════════════════════════════════════"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TARGET_MODEL="$HOME/trinity-models/gguf/LongCat-Next-Q4_K_M.gguf"

if [ ! -f "$TARGET_MODEL" ]; then
    echo "❌ Error: Quantized .gguf model not found at $TARGET_MODEL!"
    echo "   You must convert LongCat-Next to GGUF format using llama.cpp locally."
    exit 1
fi

echo ""
echo "🚀 Booting Universal Llama-Server Matrix natively on APU..."
echo "   Engine Core: Port 8010 (Native API Router)"
echo "   Access:      http://127.0.0.1:8010/v1/chat/completions"
echo ""

# The explicit binary call that Tauri will execute silently under the hood
exec "./llama-server" \
  --model "$TARGET_MODEL" \
  --port 8010 \
  --host 0.0.0.0 \
  --ctx-size 8192 \
  --n-gpu-layers 99 \
  --flash-attn
