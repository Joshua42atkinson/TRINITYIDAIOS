#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — A.R.T.Y. Hub Launcher
# ═══════════════════════════════════════════════════════════════════════════════
#
# PURPOSE:  Launch the vLLM A.R.T.Y. Hub — the secondary brain sidecar
#
# ARCHITECTURE (P.A.R.T.Y. Framework):
#   Port 8000 — A.R.T.Y. Hub (FastAPI reverse proxy)
#               Routes model-specific requests to downstream vLLM instances:
#     Port 8005 — R: nomic-embed-text-v1.5-AWQ (embeddings for RAG semantic search)
#     Port 8009 — Y: Yardmaster (Qwen/Gemma coding subagent — optional, add later)
#
#   Port 8001 — Pete/Gemma 4 E4B AWQ (vLLM — launched SEPARATELY via launch_pete.sh)
#               NOT managed by this script.
#
# HARDWARE: AMD Strix Halo (APU) — gfx1151, 128GB unified LPDDR5x
# VRAM:     nomic-embed ~500MB, leaves ~100GB+ free alongside Pete
#
# USAGE:
#   ./scripts/launch/launch_arty_hub.sh
#
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DISTROBOX_NAME="vllm"
MODEL_DIR="$HOME/trinity-models/vllm"

echo "═══════════════════════════════════════════════"
echo "  TRINITY ID AI OS — A.R.T.Y. Hub Launcher"
echo "═══════════════════════════════════════════════"
echo ""

# ─── Preflight: ensure distrobox exists ──────────────────────────────────────
if ! distrobox list 2>/dev/null | grep "$DISTROBOX_NAME" > /dev/null; then
    echo "❌ Distrobox '$DISTROBOX_NAME' not found. Create it with:"
    echo "   distrobox create -n vllm --image docker.io/kyuz0/vllm-therock-gfx1151:latest \\"
    echo "     --additional-flags \"--device /dev/kfd --device /dev/dri --group-add video --group-add render --security-opt seccomp=unconfined\""
    exit 1
fi

# ─── ROCm Environment for Strix Halo (gfx1151) ──────────────────────────────
export HSA_ENABLE_SDMA=0
export MIOPEN_FIND_MODE=FAST
export PYTORCH_ROCM_ARCH="gfx1151"
export TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL=1
export VLLM_SKIP_WARMUP=true
export HSA_OVERRIDE_GFX_VERSION=11.5.1

# Helper: run a command inside the distrobox
dbox() {
    distrobox enter "$DISTROBOX_NAME" -- "$@"
}

# ─── 1. Launch nomic-embed (R — Research/Embeddings) ────────────────────────
NOMIC_DIR="$MODEL_DIR/nomic-embed-text-v1.5-AWQ"

if [ -d "$NOMIC_DIR" ]; then
    echo "🧠 Starting nomic-embed-text-v1.5 on port 8005..."
    echo "   Model: $NOMIC_DIR"
    echo "   VRAM:  ~500MB"
    echo ""

    # Write launch script to avoid quoting issues across distrobox boundary
    cat <<'LAUNCH_EOF' > "$SCRIPT_DIR/run_nomic_embed.sh"
#!/bin/bash
export HSA_ENABLE_SDMA=0
export MIOPEN_FIND_MODE=FAST
export PYTORCH_ROCM_ARCH="gfx1151"
export TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL=1
export VLLM_SKIP_WARMUP=true
export HSA_OVERRIDE_GFX_VERSION=11.5.1

# CPU Thread Restrictions (Antigravity coexistence on UMA)
export OMP_NUM_THREADS=4
export MKL_NUM_THREADS=4
export TORCH_NUM_THREADS=4
export NUMEXPR_MAX_THREADS=4
export OPENBLAS_NUM_THREADS=4
export VECLIB_MAXIMUM_THREADS=4

/opt/venv/bin/vllm serve "$HOME/trinity-models/vllm/nomic-embed-text-v1.5-AWQ" \
    --port 8005 \
    --gpu-memory-utilization 0.05 \
    --max-model-len 2048 \
    --enforce-eager \
    --dtype half \
    --trust-remote-code \
    --served-model-name nomic-embed-text-v1.5-AWQ
LAUNCH_EOF

    chmod +x "$SCRIPT_DIR/run_nomic_embed.sh"
    dbox bash "$SCRIPT_DIR/run_nomic_embed.sh" &
    NOMIC_PID=$!

    # Wait for nomic-embed to become healthy
    echo -n "   Waiting for nomic-embed health"
    for i in $(seq 1 60); do
        if curl -s --connect-timeout 2 http://127.0.0.1:8005/health > /dev/null 2>&1; then
            echo ""
            echo "   ✅ nomic-embed ONLINE (port 8005)"
            break
        fi
        echo -n "."
        sleep 2
    done
    echo ""
else
    echo "⚠️  nomic-embed model not found at $NOMIC_DIR"
    echo "   Download with: huggingface-cli download nomic-ai/nomic-embed-text-v1.5 --local-dir $NOMIC_DIR"
    echo "   A.R.T.Y. Hub will start without embeddings (RAG falls back to text search)"
fi

# ─── 2. Launch A.R.T.Y. Router Proxy (Port 8000) ────────────────────────────
echo ""
echo "🔀 Starting A.R.T.Y. Hub reverse proxy on port 8000..."
echo "   Routes to: nomic-embed (:8005), future models (:8009, etc.)"
echo ""

# The router is plain Python — runs on the host, no GPU needed
ROUTER_SCRIPT="$PROJECT_ROOT/scripts/launch/vllm_router.py"

if [ -f "$ROUTER_SCRIPT" ]; then
    # Try to find a working Python with fastapi + uvicorn
    PYTHON_CMD=""
    if [ -d "$PROJECT_ROOT/.venv" ] && [ -f "$PROJECT_ROOT/.venv/bin/python3" ]; then
        PYTHON_CMD="$PROJECT_ROOT/.venv/bin/python3"
    elif [ -d "$HOME/trinity-vllm-env" ] && [ -f "$HOME/trinity-vllm-env/bin/python3" ]; then
        PYTHON_CMD="$HOME/trinity-vllm-env/bin/python3"
    elif command -v python3 &>/dev/null; then
        PYTHON_CMD="python3"
    fi

    if [ -z "$PYTHON_CMD" ]; then
        echo "❌ No Python 3 found. Install fastapi + uvicorn + httpx."
        exit 1
    fi

    echo "   Using Python: $PYTHON_CMD"
    exec $PYTHON_CMD "$ROUTER_SCRIPT"
else
    echo "❌ Router script not found at $ROUTER_SCRIPT"
    exit 1
fi
