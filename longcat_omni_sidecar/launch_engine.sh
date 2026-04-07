#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — start_sglang_omni.sh
# ═══════════════════════════════════════════════════════════════════════════════
#
# PURPOSE:  Launch SGLang engine for LongCat-Next (74B MoE)
# HARDWARE: AMD Strix Halo (APU) — gfx1151
#
# SGLANG APU CRITICAL PROTECTIONS:
#   1. `--attention-backend triton` is MANDATORY. Flash-infer will crash ROCm 7.x
#   2. `--tp 1` disables Tensor Parallelism (LPDDR5x is single monolithic block)
#   3. `--quantization bitsandbytes` compresses the 150GB model to ~36GB in-flight
#
# ═══════════════════════════════════════════════════════════════════════════════

echo "🚀 Booting SGLang Inference Engine for LongCat-Next..."

# Force ROCm paths and MIOpen fixes for Zen 5 APU stability (same as vLLM fixes)
export HSA_ENABLE_SDMA=0
export MIOPEN_FIND_MODE=FAST
export PYTORCH_ROCM_ARCH="gfx1151"

# The location of the in-progress background download
MODEL_DIR="$HOME/trinity-models/sglang/LongCat-Next"

if [ ! -d "$MODEL_DIR" ]; then
    echo "⚠️  Model directory not found at $MODEL_DIR"
    echo "Please wait for the background huggingface-cli download to finish!"
    exit 1
fi

echo "🧠 Engaging DiNA Token Architecture on port 30000"

# Note: SGLang has native OpenAPI-compatible chat completions
python3 -m sglang.launch_server \
  --model-path "$MODEL_DIR" \
  --port 30000 \
  --attention-backend triton \
  --quantization bitsandbytes \
  --trust-remote-code \
  --tp 1

# Alternative Option:
# If bitsandbytes fails on ROCm, swap the flag above for `--quantization fp8`
# and pass `--kv-cache-dtype fp8_e4m3` for balanced memory packing.
