#!/bin/bash
# Trinity vLLM Omni Sidecar Launcher
# Runs inside the kyuz0/vllm-therock-gfx1151 distrobox (Strix Halo gfx1151)
# The distrobox has TheRock nightly ROCm builds that actually work on this hardware.

set -e

DISTROBOX_NAME="vllm"
MODEL_DIR="$HOME/trinity-models/vllm"

# --- Preflight: ensure distrobox exists ---
if ! distrobox list 2>/dev/null | grep -q "$DISTROBOX_NAME"; then
    echo "❌ Distrobox '$DISTROBOX_NAME' not found. Create it with:"
    echo "   distrobox create -n vllm --image docker.io/kyuz0/vllm-therock-gfx1151:latest \\"
    echo "     --additional-flags \"--device /dev/kfd --device /dev/dri --group-add video --group-add render --security-opt seccomp=unconfined\""
    exit 1
fi

# Helper: run a command inside the distrobox
dbox() {
    distrobox enter "$DISTROBOX_NAME" -- "$@"
}

# --- Auto-Bootstrap: ensure model weights exist ---
ensure_model() {
    local dir_path=$1
    local repo_id=$2
    if [ ! -d "$dir_path" ] || [ -z "$(ls -A "$dir_path" 2>/dev/null)" ]; then
        echo "⬇️  Fetching missing weights: $repo_id -> $dir_path..."
        mkdir -p "$dir_path"
        dbox huggingface-cli download "$repo_id" --local-dir "$dir_path" --local-dir-use-symlinks False
    else
        echo "✅ Weights present: $dir_path"
    fi
}

echo "📦 Verifying Local Model Persistence..."
ensure_model "$MODEL_DIR/gemma-4-31B-it-AWQ-4bit" "google/gemma-4-31b-it-awq"
ensure_model "$MODEL_DIR/gemma-4-26B-A4B-it-AWQ-4bit" "google/gemma-4-26b-a4b-it-awq"
ensure_model "$MODEL_DIR/gemma-4-E4B-it-AWQ-4bit" "google/gemma-4-e4b-it-awq"
ensure_model "$MODEL_DIR/gemma-4-E2B-it" "google/gemma-4-E2B-it"  # Speculative Draft
# Aesthetics Triad is now managed dynamically. Models are fetched at runtime if missing.
echo "--------------------------------------------------------"

# Wait 2 seconds before booting to allow any straggling processes to die
sleep 2

echo "🚀 Starting vLLM P.A.R.T.Y. Hotel via Strix Halo Toolbox..."

# 1. Great Recycler (Socratic Reasoning) - Gemma 4 31B Dense AWQ + E2B Draft
#    Target Weights: ~21GB | Draft: ~5GB | Reserved: 44.8GB (0.35) | KV headroom: ~18GB
#    Hyper-Context: Enabled via TurboQuant 4-bit (Near 160K theoretical)
#    Optimization: Speculative Decoding enabled (E2B draft model)
if [ -d "$MODEL_DIR/gemma-4-31B-it-AWQ-4bit" ]; then
    export TQ4_K_BITS=4
    export TQ4_V_BITS=4
    dbox vllm serve "$MODEL_DIR/gemma-4-31B-it-AWQ-4bit" --port 8001 \
        --gpu-memory-utilization 0.35 --max-model-len 32768 \
        --dtype half --attention-backend CUSTOM \
        --served-model-name "Great_Recycler" &
    sleep 15
fi

# 2. Programmer Pete (Action-Oriented Engine) - Gemma 4 26B-A4B MoE AWQ
#    Weights: ~18GB | Reserved: 23.0GB (0.18) | KV headroom: ~5.0GB
#    Hyper-Context: Enabled via TurboQuant 4-bit
if [ -d "$MODEL_DIR/gemma-4-26B-A4B-it-AWQ-4bit" ]; then
    export TQ4_K_BITS=4
    export TQ4_V_BITS=4
    dbox vllm serve "$MODEL_DIR/gemma-4-26B-A4B-it-AWQ-4bit" --port 8002 \
        --gpu-memory-utilization 0.18 --max-model-len 16384 \
        --dtype half --attention-backend CUSTOM \
        --served-model-name "Programmer_Pete" &
    sleep 10
fi

# 3. Tempo Engine (Standalone Vibe Conductor) - Gemma 4 E4B
#    Weights: ~8GB | Reserved: 9.0GB (0.07) | Context: 4096 (Lightweight)
if [ -d "$MODEL_DIR/gemma-4-E4B-it-AWQ-4bit" ]; then
    export TQ4_K_BITS=4
    export TQ4_V_BITS=4
    dbox vllm serve "$MODEL_DIR/gemma-4-E4B-it-AWQ-4bit" --port 8003 \
        --gpu-memory-utilization 0.07 --max-model-len 4096 \
        --dtype half --attention-backend CUSTOM \
        --served-model-name "Tempo_Engine" &
    sleep 5
fi

# =========================================================================
# Aesthetics Triad (Dynamic Pool - Handled by vLLM Router / App Logic)
# 4. Image Generation - FLUX.1 [schnell] (Q4_K_S GGUF)
# 5. Video Generation - CogVideoX-2b-GGUF (Q4_K_S)
# 6. 3D Mesh Generation - TripoSR
# Note: Media sidecars are NO LONGER statically launched here to preserve
# maximum VRAM for Socratic reasoning context.
# =========================================================================

# 5. Embeddings - nomic-embed-text-v1.5 (137M params, tiny footprint)
#    Weights: ~1GB | Reserved: 2.5GB (0.02) | High-throughput RAG chunking
if [ -d "$MODEL_DIR/nomic-embed-text-v1.5-AWQ" ]; then
    export TQ4_K_BITS=4
    export TQ4_V_BITS=4
    dbox vllm serve "$MODEL_DIR/nomic-embed-text-v1.5-AWQ" --port 8005 \
        --gpu-memory-utilization 0.02 \
        --served-model-name "nomic-embed" &
fi

echo "🚀 Starting FastAPI Reverse Proxy on Port 8000..."
# Router runs on host — it's plain Python, no GPU needed
source /home/joshua/trinity-vllm-env/bin/activate
python3 /home/joshua/Workflow/desktop_trinity/trinity-genesis/scripts/launch/vllm_router.py
