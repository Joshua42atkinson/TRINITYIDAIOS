#!/bin/bash
set -e

echo "Trinity Model Downloader"
echo "Target: Ming-flash-omni-2.0"
echo "We need the 4-bit quantized version for the Strix Halo UMA (128GB)."

# Define paths
MODEL_DIR="$HOME/trinity-models/sglang/Ming-flash-omni-2.0-AWQ"

mkdir -p "$MODEL_DIR"

# Note: The official inclusionAI repo currently only hosts the full fp16 safetensors (200GB+). 
# For Strix Halo (128GB), we will eventually need an AWQ/GPTQ or GGUF quantized version.
# For now, we will download a stub or smaller proxy to test SGLang infrastructure, 
# or wait for the quantized release.

echo "Warning: Full Ming-flash-omni-2.0 weights exceed 200GB in FP16."
echo "Since you have 128GB of Strix Halo memory, attempting to download the full unquantized model will result in OOM."
echo "Please find a community quantized Q4 (AWQ or GGUF) version, or run quantization locally using a cloud node."

# E.g. hf download some-community/Ming-flash-omni-2.0-AWQ --local-dir "$MODEL_DIR"

