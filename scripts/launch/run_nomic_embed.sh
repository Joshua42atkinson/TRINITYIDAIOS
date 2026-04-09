#!/bin/bash
export HSA_ENABLE_SDMA=0
export MIOPEN_FIND_MODE=FAST
export PYTORCH_ROCM_ARCH="gfx1151"
export TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL=1
export VLLM_SKIP_WARMUP=true
export HSA_OVERRIDE_GFX_VERSION=11.5.1

/opt/venv/bin/vllm serve "$HOME/trinity-models/vllm/nomic-embed-text-v1.5-AWQ" \
    --port 8005 \
    --gpu-memory-utilization 0.05 \
    --max-model-len 2048 \
    --enforce-eager \
    --dtype half \
    --trust-remote-code \
    --served-model-name nomic-embed-text-v1.5-AWQ
