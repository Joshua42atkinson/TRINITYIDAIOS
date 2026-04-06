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
ensure_model "$MODEL_DIR/gemma-4-E2B-it" "google/gemma-4-E2B-it"
echo "--------------------------------------------------------"

# Wait 2 seconds before booting to allow any straggling processes to die
sleep 2

echo "🚀 Starting vLLM P.A.R.T.Y. Hotel via Strix Halo Toolbox..."

# 1. Great Recycler (Socratic Reasoning) - Gemma 4 31B Dense AWQ + E2B Draft
if [ -d "$MODEL_DIR/gemma-4-31B-it-AWQ-4bit" ] && [ -d "$MODEL_DIR/gemma-4-E2B-it" ]; then
    # To avoid JSON quote escaping issues across distrobox boundaries, we write the command first
    cat << EOF > /tmp/run_vllm_31b.sh
#!/bin/bash
vllm serve "$HOME/trinity-models/vllm/gemma-4-31B-it-AWQ-4bit" --port 8001 \\
    --gpu-memory-utilization 0.40 --max-model-len 16384 \\
    --served-model-name "Great_Recycler"
EOF
    dbox bash /tmp/run_vllm_31b.sh &
    sleep 15
else
    echo "❌ Missing weights for Gemma 31B or E2B. Please configure models."
fi

# 5. Embeddings - nomic-embed-text-v1.5 (137M params, tiny footprint)
if [ -d "$MODEL_DIR/nomic-embed-text-v1.5-AWQ" ]; then
    export TQ4_K_BITS=4
    export TQ4_V_BITS=4
    dbox vllm serve "$MODEL_DIR/nomic-embed-text-v1.5-AWQ" --port 8005 \
        --gpu-memory-utilization 0.02 \
        --served-model-name "nomic-embed" &
fi

# Note: Media sidecars (FLUX, CogVideoX, TripoSR, Music) are omitted from here 
# and will be managed externally or initialized natively to save VRAM headroom.

echo "🚀 Starting FastAPI Reverse Proxy on Port 8000..."
# Router runs on host — it's plain Python, no GPU needed
source /home/joshua/trinity-vllm-env/bin/activate
python3 /home/joshua/Workflow/desktop_trinity/trinity-genesis/scripts/launch/vllm_router.py
