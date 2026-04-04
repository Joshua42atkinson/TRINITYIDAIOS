#!/bin/bash
# Trinity vLLM Omni Sidecar Launcher
# Optimized for AMD Strix Halo (128GB Unified Memory)

export HSA_OVERRIDE_GFX_VERSION=11.5.1
export VLLM_USE_TRITON_FLASH_ATTN=0

source /home/joshua/trinity-vllm-env/bin/activate

# --- Auto-Bootstrap Distribution Logic ---
# Ensure the OS "just works" by automatically fetching missing model weights.
ensure_model() {
    local dir_path=$1
    local repo_id=$2
    if [ ! -d "$dir_path" ]; then
        echo "⬇️  Fetching missing weights: $repo_id -> $dir_path..."
        mkdir -p "$dir_path"
        huggingface-cli download "$repo_id" --local-dir "$dir_path" --local-dir-use-symlinks False
    else
        echo "✅ Weights present: $dir_path"
    fi
}

echo "📦 Verifying Local Model Persistence..."
# Ensure core Omni models are present for pure, local AI pipeline
ensure_model "$HOME/trinity-models/vllm/gemma-4-31B-it-AWQ-4bit" "google/gemma-4-31b-it-awq"
ensure_model "$HOME/trinity-models/vllm/gemma-4-26B-A4B-it-AWQ-4bit" "google/gemma-4-26b-a4b-it-awq"
ensure_model "$HOME/trinity-models/vllm/gemma-4-E4B-it-AWQ-4bit" "google/gemma-4-e4b-it-awq"
ensure_model "$HOME/trinity-models/vllm/HunyuanImage-vLLM-AWQ" "Kijai/HunyuanImage-vLLM-AWQ-4bit"
ensure_model "$HOME/trinity-models/vllm/nomic-embed-text-v1.5-AWQ" "nomic-ai/nomic-embed-text-v1.5-awq"
echo "--------------------------------------------------------"

echo "🚀 Starting vLLM P.A.R.T.Y. Hotel on unified memory..."

# 1. Start the Great Recycler (Heavy Logic / Architectural Engine) - Gemma 4 31B
if [ -d ~/trinity-models/vllm/gemma-4-31B-it-AWQ-4bit ]; then
    vllm serve ~/trinity-models/vllm/gemma-4-31B-it-AWQ-4bit --port 8001 \
        --gpu-memory-utilization 0.40 --max-model-len 16384 --served-model-name "Great_Recycler" &
fi

# 2. Start Programmer Pete (UI / UX / React Engine) - Gemma 4 26B A4B
if [ -d ~/trinity-models/vllm/gemma-4-26B-A4B-it-AWQ-4bit ]; then
    vllm serve ~/trinity-models/vllm/gemma-4-26B-A4B-it-AWQ-4bit --port 8002 \
        --gpu-memory-utilization 0.25 --max-model-len 16384 --served-model-name "Programmer_Pete" &
fi

# 3. Start E4B Fast Inference (NPCs) - Gemma 4 E4B
if [ -d ~/trinity-models/vllm/gemma-4-E4B-it-AWQ-4bit ]; then
    vllm serve ~/trinity-models/vllm/gemma-4-E4B-it-AWQ-4bit --port 8003 \
        --gpu-memory-utilization 0.10 --max-model-len 8192 --served-model-name "Omni_NPC" &
fi

# 4. Start HunyuanImage (Creative Visual Engine) - Native 4-bit AWQ Rendering
if [ -d ~/trinity-models/vllm/HunyuanImage-vLLM-AWQ ]; then
    vllm serve ~/trinity-models/vllm/HunyuanImage-vLLM-AWQ --port 8004 \
        --gpu-memory-utilization 0.20 --served-model-name "HunyuanImage" &
fi

echo "🚀 Starting FastAPI Reverse Proxy on Port 8000..."
python3 /home/joshua/Workflow/desktop_trinity/trinity-genesis/scripts/launch/vllm_router.py
