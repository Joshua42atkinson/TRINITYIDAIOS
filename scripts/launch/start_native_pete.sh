#!/bin/bash
export HSA_OVERRIDE_GFX_VERSION=11.5.1
export PYTORCH_ROCM_ARCH="gfx1151"
export NCCL_P2P_DISABLE=1
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis/longcat_amd_sidecar
source .venv/bin/activate
nohup python3 server.py > pete_engine.log 2>&1 &
echo $! > pete_engine.pid
