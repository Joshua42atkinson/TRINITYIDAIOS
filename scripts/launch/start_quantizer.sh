#!/bin/bash
# Dedicated Launcher for OOM-Proof AWQ Quantization
# This ensures we don't pollute the live inference container.

echo "🚀 Entering dedicated 'vllm-quant' toolbox to start quantization..."
distrobox enter vllm-quant -- /home/joshua/Workflow/desktop_trinity/trinity-genesis/scripts/run_quantize.sh
