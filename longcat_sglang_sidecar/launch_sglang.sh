#!/bin/bash
set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR"

if [ ! -d ".venv" ]; then
    echo "❌ Error: Virtual environment not found. Run ./setup_env.sh first."
    exit 1
fi

source .venv/bin/activate

echo "================================================="
echo " Starting SGLang Raw Server (Port 8011)"
echo " Model: LongCat-Next"
echo "================================================="

# Crucial AMD APU overrides
export HSA_OVERRIDE_GFX_VERSION=11.0.0
# Prevent potential triton/flash_attention compilation segfaults by using SDPA if needed
# SGLang natively manages memory and attention.

MODEL_PATH="${HOME}/trinity-models/sglang/LongCat-Next"

if [ ! -d "$MODEL_PATH" ]; then
    echo "❌ Error: Model directory not found at $MODEL_PATH"
    exit 1
fi

# We use port 8011 for the RAW SGLang server to keep 8010 available for our bridge
# memory fraction set to slightly lower if using unified system RAM
python3 -m sglang.launch_server \
    --model-path "$MODEL_PATH" \
    --port 8011 \
    --host 127.0.0.1 \
    --trust-remote-code \
    --mem-fraction-static 0.85
