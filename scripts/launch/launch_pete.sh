#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Pete / Great Recycler Launcher
# ═══════════════════════════════════════════════════════════════════════════════
#
# PURPOSE:  Launch Pete (Gemma 4 E4B AWQ) — the primary Socratic brain
#
# ARCHITECTURE:
#   Port 8001 — Pete / Great Recycler (vLLM inside distrobox)
#               Gemma 4 E4B AWQ 4-bit: vision + text, ~15GB VRAM
#               Handles: Socratic dialogue, instruction design, narration, vision
#
# HARDWARE: AMD Strix Halo (APU) — gfx1151, 128GB unified LPDDR5x
# VRAM:     ~15GB (leaves 100GB+ free for other models + OS)
#
# USAGE:
#   ./scripts/launch/launch_pete.sh
#
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DISTROBOX_NAME="vllm"
MODEL_DIR="$HOME/trinity-models/vllm/gemma-4-E4B-it-AWQ-4bit"

echo "═══════════════════════════════════════════════"
echo "  TRINITY ID AI OS — Pete / Great Recycler"
echo "  Gemma 4 E4B AWQ — Port 8001"
echo "═══════════════════════════════════════════════"
echo ""

# ─── Preflight: ensure distrobox exists ──────────────────────────────────────
if ! distrobox list 2>/dev/null | grep "$DISTROBOX_NAME" > /dev/null; then
    echo "❌ Distrobox '$DISTROBOX_NAME' not found. Create it with:"
    echo "   distrobox create -n vllm --image docker.io/kyuz0/vllm-therock-gfx1151:latest \\"
    echo "     --additional-flags \"--device /dev/kfd --device /dev/dri --group-add video --group-add render --security-opt seccomp=unconfined\""
    exit 1
fi

# ─── Preflight: ensure model exists ──────────────────────────────────────────
if [ ! -d "$MODEL_DIR" ]; then
    echo "❌ Model not found at: $MODEL_DIR"
    echo "   Download with:"
    echo "   distrobox enter $DISTROBOX_NAME -- huggingface-cli download google/gemma-4-E4B-it-awq --local-dir $MODEL_DIR"
    exit 1
fi

echo "📦 Model: $MODEL_DIR"
echo "   Size:  $(du -sh "$MODEL_DIR" | cut -f1)"
echo ""

# ─── Write launch script (avoids quoting issues across distrobox boundary) ───
LAUNCH_SCRIPT="/tmp/trinity_launch_pete.sh"
cat <<LAUNCH_EOF > "$LAUNCH_SCRIPT"
#!/bin/bash
# ── AMD Strix Halo gfx1151 Environment ──
export HSA_ENABLE_SDMA=0
export MIOPEN_FIND_MODE=FAST
export PYTORCH_ROCM_ARCH="gfx1151"
export TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL=1
export VLLM_SKIP_WARMUP=true
export HSA_OVERRIDE_GFX_VERSION=11.5.1
export PYTORCH_HIP_ALLOC_CONF=garbage_collection_threshold:0.8\n# NOTE: expandable_segments not supported on gfx1151 (silently ignored)
export NCCL_P2P_DISABLE=1

# ── CPU Thread Restrictions (Antigravity coexistence on UMA) ──
export OMP_NUM_THREADS=4
export MKL_NUM_THREADS=4
export TORCH_NUM_THREADS=4
export NUMEXPR_MAX_THREADS=4
export OPENBLAS_NUM_THREADS=4
export VECLIB_MAXIMUM_THREADS=4

/opt/venv/bin/vllm serve "$HOME/trinity-models/vllm/gemma-4-E4B-it-AWQ-4bit" \\
    --port 8001 \\
    --gpu-memory-utilization 0.15 \\
    --max-model-len 32768 \\
    --enforce-eager \\
    --dtype half \\
    --trust-remote-code \\
    --served-model-name "Great_Recycler"
LAUNCH_EOF

chmod +x "$LAUNCH_SCRIPT"

# ─── Launch inside distrobox ─────────────────────────────────────────────────
echo "🚀 Starting Pete (Gemma 4 E4B AWQ) on port 8001..."
echo "   Using distrobox: $DISTROBOX_NAME"
echo ""

distrobox enter "$DISTROBOX_NAME" -- bash "$LAUNCH_SCRIPT"
