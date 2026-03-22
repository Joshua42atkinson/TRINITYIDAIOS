#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — vLLM Serve Script
# Universal launcher for any safetensors model
# ═══════════════════════════════════════════════════════════════════════════════
set -e

VENV="${HOME}/trinity-vllm-env"
PORT="${VLLM_PORT:-8000}"
GPU_MEM="${VLLM_GPU_MEM:-0.5}"
MAX_LEN="${VLLM_MAX_LEN:-8192}"

# AMD Strix Halo optimizations
export HSA_OVERRIDE_GFX_VERSION=11.0.0
export ROCBLAS_USE_HIPBLASLT=1

usage() {
    echo "Usage: $0 <model_path_or_hf_id> [options]"
    echo ""
    echo "Examples:"
    echo "  $0 Qwen/Qwen2.5-0.5B-Instruct                    # HuggingFace ID"
    echo "  $0 ~/trinity-models/safetensors/Mistral-Small-4-119B-2603-eagle"
    echo ""
    echo "Environment variables:"
    echo "  VLLM_PORT      - Port for vLLM server (default: 8000)"
    echo "  VLLM_GPU_MEM   - GPU memory utilization 0.0-1.0 (default: 0.5)"
    echo "  VLLM_MAX_LEN   - Max context length (default: 8192)"
    exit 1
}

if [ -z "$1" ]; then
    usage
fi

MODEL="$1"
shift

# Check if venv exists
if [ ! -d "$VENV" ]; then
    echo "ERROR: vLLM venv not found at $VENV"
    echo "Install with: uv venv $VENV && source $VENV/bin/activate && uv pip install vllm --extra-index-url https://wheels.vllm.ai/rocm/"
    exit 1
fi

# Check if it's a local path that doesn't exist
if [[ "$MODEL" == /* ]] && [ ! -d "$MODEL" ]; then
    echo "ERROR: Model directory not found: $MODEL"
    exit 1
fi

echo "Starting vLLM server..."
echo "  Model: $MODEL"
echo "  Port: $PORT"
echo "  GPU Memory: $GPU_MEM"
echo "  Max Length: $MAX_LEN"
echo ""

exec "${VENV}/bin/vllm" serve "$MODEL" \
    --port "$PORT" \
    --gpu-memory-utilization "$GPU_MEM" \
    --max-model-len "$MAX_LEN" \
    --trust-remote-code \
    "$@"
