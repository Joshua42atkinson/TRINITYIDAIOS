#!/bin/bash
# --- 1. MIOpen Live Inference Anti-Freeze ---
export MIOPEN_FIND_MODE=FAST
export MIOPEN_DISABLE_CACHE=1
export VLLM_SKIP_WARMUP=true

# --- 2. 3-Tier Multi-Agent SDMA Bypass ---
export HSA_ENABLE_SDMA=0                  # Mandated for >1 instance on an APU
export PYTORCH_NO_HIP_MEMORY_CACHING=1    # Prevents Instances from locking unused VRAM

# --- 3. Strix Halo Strict Targeting ---
export HSA_OVERRIDE_GFX_VERSION=11.5.1
export PYTORCH_ROCM_ARCH="gfx1151"
export HIP_FORCE_DEV_KERNARG=1
export VLLM_ROCM_SHUFFLE_KV_CACHE_LAYOUT=0
export PYTORCH_TUNABLEOP_ENABLED=1

# Launch E2B Omni:
# (Using the AWQ model path here, once quantized. If not quantized yet, switch to full model path)
distrobox enter vllm -- /opt/venv/bin/vllm serve ~/trinity-models/vllm/gemma-4-E2B-it-AWQ-4bit \
    --port 8003 \
    --gpu-memory-utilization 0.15 \
    --max-num-batched-tokens 1024 \
    --dtype half \
    --enforce-eager \
    --trust-remote-code \
    --disable-custom-all-reduce \
    --served-model-name Omni_Chat
