#!/bin/bash
export HSA_OVERRIDE_GFX_VERSION=11.5.1
export PYTORCH_ROCM_ARCH="gfx1151"
export VLLM_WORKER_MULTIPROC_METHOD=spawn
export NCCL_P2P_DISABLE=1

distrobox enter vllm-quant -- /opt/venv/bin/python3 -m vllm.entrypoints.openai.api_server \
  --model /home/joshua/trinity-models/omni/LongCat-Next-INT4 \
  --quantization awq \
  --trust-remote-code \
  --gpu-memory-utilization 0.55 \
  --tensor-parallel-size 1 \
  --enforce-eager \
  --port 8010
