#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — T (Tempo) Launcher
# ═══════════════════════════════════════════════════════════════════════════════
#
# PURPOSE:  Launch Tempo — the always-on fast-twitch brain
#
# P-ART-Y ROLE: T (Tempo)
# MODEL:    Gemma-4-E4B-it AWQ 4-bit (Dense, 4B effective)
# PORT:     8001
# VRAM:     ~6 GB (AWQ INT4)
# CONTEXT:  128K tokens
# LOADING:  ALWAYS ON — permanent resident, never Hotel-swapped
#
# CAPABILITIES:
#   • Real-time Socratic chat (fast-twitch, <100ms/token)
#   • NPC dialog generation
#   • TTS routing (dispatches to Kokoro ORT)
#   • Phase transition responses
#   • Multimodal input (text + image + audio)
#
# HARDWARE: AMD Strix Halo (APU) — gfx1151, 128GB unified LPDDR5x
#
# USAGE:
#   ./scripts/launch/launch_tempo_e4b.sh
#
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DISTROBOX_NAME="vllm-019"
MODEL_DIR="$HOME/trinity-models/vllm/gemma-4-E4B-it-AWQ-4bit"
PORT=8001
SERVED_NAME="Tempo_E4B"

echo "═══════════════════════════════════════════════"
echo "  TRINITY P-ART-Y — T (Tempo)"
echo "  Gemma 4 E4B AWQ — Port $PORT"
echo "  Role: Always-on fast brain"
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

# ─── Kill any existing process on this port ──────────────────────────────────
if lsof -ti:$PORT >/dev/null 2>&1; then
    echo "⚠️  Killing existing process on port $PORT..."
    lsof -ti:$PORT | xargs kill -9 2>/dev/null || true
    sleep 2
fi

# ─── Write launch script (avoids quoting issues across distrobox boundary) ───
LAUNCH_SCRIPT="/tmp/trinity_launch_tempo.sh"
cat <<LAUNCH_EOF > "$LAUNCH_SCRIPT"
#!/bin/bash
# ── AMD Strix Halo gfx1151 Environment ──
export HSA_ENABLE_SDMA=0
export MIOPEN_FIND_MODE=FAST
export PYTORCH_ROCM_ARCH="gfx1151"
export TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL=1
export VLLM_SKIP_WARMUP=true
export HSA_OVERRIDE_GFX_VERSION=11.5.1
# NOTE: expandable_segments not supported on gfx1151 (silently ignored)
# NOTE: VLLM_USE_V1 was removed in vLLM 0.19 — V1 engine is the only engine
export NCCL_P2P_DISABLE=1

# ── CPU Thread Restrictions (Antigravity coexistence on UMA) ──
# Without these, PyTorch spawns OMP_NUM_THREADS=nproc (32) per operator,
# causing 170+ threads per vLLM process and massive context-switch overhead.
export OMP_NUM_THREADS=4
export MKL_NUM_THREADS=4
export TORCH_NUM_THREADS=4
export NUMEXPR_MAX_THREADS=4
export OPENBLAS_NUM_THREADS=4
export VECLIB_MAXIMUM_THREADS=4

/opt/venv/bin/vllm serve "\$HOME/trinity-models/vllm/gemma-4-E4B-it-AWQ-4bit" \\
    --port $PORT \\
    --gpu-memory-utilization 0.25 \\
    --max-model-len 32768 \\
    --enforce-eager \\
    --dtype half \\
    --quantization awq \\
    --trust-remote-code \\
    --served-model-name "$SERVED_NAME" \\
    --enable-auto-tool-choice \\
    --tool-call-parser gemma4
LAUNCH_EOF

chmod +x "$LAUNCH_SCRIPT"

# ─── Launch inside distrobox ─────────────────────────────────────────────────
echo "🚀 Starting T (Tempo) on port $PORT..."
echo "   Using distrobox: $DISTROBOX_NAME"
echo "   Served as: $SERVED_NAME"
echo ""

distrobox enter "$DISTROBOX_NAME" -- bash "$LAUNCH_SCRIPT"
