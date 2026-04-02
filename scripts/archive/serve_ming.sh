#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Serve Ming (Yardmaster)
# Model: Ming-flash-omni-2.0 (safetensors, custom talker via vLLM)
# Port: 8000 (swaps with Pete — Hotel pattern, one heavyweight at a time)
# ═══════════════════════════════════════════════════════════════════════════════
set -e

MODEL_PATH="${HOME}/trinity-models/safetensors/Ming-flash-omni-2.0"
TALKER_PATH="${MODEL_PATH}/talker"
PORT="${VLLM_PORT:-8000}"
VENV="${HOME}/trinity-vllm-env"

# AMD Strix Halo optimizations
export HSA_OVERRIDE_GFX_VERSION=11.0.0
export ROCBLAS_USE_HIPBLASLT=1

# Ming's talker needs to be on the Python path so vLLM can find it
export PYTHONPATH="${MODEL_PATH}:${TALKER_PATH}:${PYTHONPATH}"

if [ ! -f "$MODEL_PATH/config.json" ]; then
    echo "ERROR: Ming model not found at $MODEL_PATH"
    exit 1
fi

echo "Starting Ming (Yardmaster — flash-omni-2.0) on port $PORT..."
echo "  Model path: $MODEL_PATH"
echo "  Talker path: $TALKER_PATH"
echo "  GPU memory utilization: 0.85"

# Ming uses custom architecture — needs trust-remote-code
# The talker/ directory contains MingTalkerForCausalLM which registers into vLLM
exec "${VENV}/bin/python" -c "
import sys
sys.path.insert(0, '${TALKER_PATH}')
sys.path.insert(0, '${MODEL_PATH}')

from vllm import ModelRegistry
from ming_talker import MingTalkerForCausalLM
ModelRegistry.register_model('MingTalkerForCausalLM', MingTalkerForCausalLM)

from vllm.entrypoints.openai.api_server import run_server
from vllm.engine.arg_utils import AsyncEngineArgs
import asyncio

# Use the talker subdirectory which has the actual model weights for audio
args = AsyncEngineArgs(
    model='${TALKER_PATH}',
    trust_remote_code=True,
    enforce_eager=False,
    gpu_memory_utilization=0.85,
    disable_custom_all_reduce=True,
    tensor_parallel_size=1,
    max_model_len=32768,
    port=${PORT},
)
print('Ming Yardmaster engine args configured. Starting server...')
" 2>&1
