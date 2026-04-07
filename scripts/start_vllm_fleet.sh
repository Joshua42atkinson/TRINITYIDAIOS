#!/bin/bash
# ═══════════════════════════════════════════════════════════════
# Trinity ID AI OS — vLLM Fleet Launcher (Strix Halo/gfx1151)
# ═══════════════════════════════════════════════════════════════
# Implements hardware mitigations for ROCm driver freezes, kernel 
# panics, and Triton Attention shared memory limitations on RDNA 3.5.

# ─── 1. Core Hardware Fault Mitigations ────────────────────────
export VLLM_SKIP_WARMUP=true                    # Bypasses profile_run deadlock
export HSA_ENABLE_SDMA=0                        # Prevents Layer Offload Deadlock
export HSA_OVERRIDE_GFX_VERSION=11.5.1          # Forces compatible kernel dispatch
export PYTORCH_ROCM_ARCH="gfx1151"              # Hardware target enforcement
export MIOPEN_FIND_MODE=FAST                    # Bypasses MIOpen exhaustive benchmarking
export PYTORCH_TUNABLEOP_ENABLED=1              # Enables dynamic matrix shape autotuning
export PYTORCH_NO_HIP_MEMORY_CACHING=1          # Eliminates HIP caching latency gaps
export PYTORCH_ALLOC_CONF="expandable_segments:True,garbage_collection_threshold:0.8,max_split_size_mb:512" 

# ─── 2. Triton Kernel Overrides ─────────────────────────────────
export TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL=1
export FLASH_ATTENTION_TRITON_AMD_ENABLE=TRUE
export VLLM_ROCM_USE_AITER=0                    # Fixes missing on_gfx9() gates

# ─── 3. Source Level Python Patching (In-Container) ─────────────
echo -e "\033[0;34mApplying Python source patches inside vLLM distrobox...\033[0m"
distrobox enter vllm -- sh -c "
  # SMEM ceiling restriction (2 warps/SIMD = 8 warps total)
  find /opt/venv -name triton_unified_attention.py -exec sed -i 's/num_warps=.*,/num_warps=8,/' {} + 2>/dev/null || true
  # Disable encoder warmup natively just in case VLLM_SKIP_WARMUP fails
  find /opt/venv -name gpu_model_runner.py -path '*/vllm/v1/worker/*' -exec sed -i '5509,5525s/^/#/' {} + 2>/dev/null || true
"

echo -e "\033[0;32mLaunching vLLM Fleet...\033[0m"

# Instance 1: The Brain & Ears (Socratic Director)
# Model changed to Qwen2.5-14B as per current testing since Gemma-4 31B is broken
distrobox enter vllm -- /opt/venv/bin/vllm serve \
    ~/trinity-models/vllm/Qwen2.5-14B-Instruct-AWQ \
    --port 8001 \
    --gpu-memory-utilization 0.35 \
    --max-model-len 16384 \
    --enforce-eager \
    --disable-pinned-memory \
    --dtype half \
    -np 1 \
    --trust-remote-code \
    --served-model-name Great_Recycler &

# Instance 2: Programmer Pete (Execution Agent)
distrobox enter vllm -- /opt/venv/bin/vllm serve \
    ~/trinity-models/vllm/gemma-4-26B-A4B-it-AWQ-4bit \
    --port 8002 \
    --gpu-memory-utilization 0.30 \
    --max-model-len 16384 \
    --enable-prefix-caching \
    --enforce-eager \
    --disable-pinned-memory \
    --dtype half \
    -np 1 \
    --trust-remote-code \
    --served-model-name Programmer_Pete &

wait
