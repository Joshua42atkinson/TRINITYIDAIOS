#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Serve Pete LIGHT (Low-End Hardware Profile)
# Model: Phi-3-Mini-4k-Instruct (3.8B parameters)
# Port: 8000 (default)
# Platform: llama.cpp (CPU/Light GPU)
# ═══════════════════════════════════════════════════════════════════════════════
set -e

MODEL_DIR="${HOME}/trinity-models/gguf"
MODEL_FILE="Phi-3-mini-4k-instruct-q4.gguf"
MODEL_URL="https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf/resolve/main/Phi-3-mini-4k-instruct-q4.gguf?download=true"
MODEL_PATH="${MODEL_DIR}/${MODEL_FILE}"
PORT="${VLLM_PORT:-8000}"

mkdir -p "$MODEL_DIR"

if [ ! -f "$MODEL_PATH" ]; then
    echo "⬇️ Downloading lightweight Pete model ($MODEL_FILE) for your hardware..."
    curl -L "$MODEL_URL" -o "$MODEL_PATH"
fi

# Check for existing longcat-sglang in standard paths
LLAMA_SERVER=""
if [ -x "./llama.cpp/build/bin/longcat-sglang" ]; then
    LLAMA_SERVER="./llama.cpp/build/bin/longcat-sglang"
elif [ -x "./llama.cpp/build-vulkan/bin/longcat-sglang" ]; then
    LLAMA_SERVER="./llama.cpp/build-vulkan/bin/longcat-sglang"
elif [ -x "./llama.cpp/build-rocm/bin/longcat-sglang" ]; then
    LLAMA_SERVER="./llama.cpp/build-rocm/bin/longcat-sglang"
elif command -v longcat-sglang &> /dev/null; then
    LLAMA_SERVER="longcat-sglang"
else
    echo "❌ ERROR: longcat-sglang not found! Please ensure llama.cpp is built or installed."
    # If the user has LM Studio installed from AppImage script instruction:
    echo "💡 Tip: If you have LM Studio, you can start tracking it locally on port 1234!"
    exit 1
fi

echo "🚀 Starting Pete LIGHT (Phi-3-Mini) on port $PORT..."
exec "$LLAMA_SERVER" -m "$MODEL_PATH" --port "$PORT" -c 4096 -cb --host 0.0.0.0
