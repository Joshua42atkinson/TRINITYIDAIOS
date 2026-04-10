#!/bin/bash
set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR"

if [ ! -d ".venv" ]; then
    echo "❌ Error: Virtual environment not found. Run ./setup_amd_env.sh first."
    exit 1
fi

source .venv/bin/activate

# Aggressively disable Flash Attention to prevent any AMD segfaults
export FLASH_ATTENTION_DISABLE=1
export USE_FLASH_ATTENTION=0
export SGLANG_FLASHINFER=0

# AMD ROCm compatibility for Strix Halo (gfx1151)
export HSA_OVERRIDE_GFX_VERSION=11.0.0
export PYTORCH_HIP_ALLOC_CONF=expandable_segments:True

echo "================================================="
echo " Starting LongCat-Next Omni Brain (AMD SDPA Backend)"
echo " Port: 8010"
echo "================================================="

python3 server.py
