#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — P (Programming) Launcher
# ═══════════════════════════════════════════════════════════════════════════════
#
# PURPOSE:  Launch Pete Coder — the MoE coding brain
#
# P-ART-Y ROLE: P (Programming)
# MODEL:    Gemma-4-26B-A4B-it AWQ 4-bit (MoE: 26B total, 4B active/token)
# PORT:     8000
# VRAM:     ~16 GB (AWQ INT4)
# CONTEXT:  256K tokens
# LOADING:  HOTEL SWAP — loaded on-demand during Design, Development, Yoke phases
#
# CAPABILITIES:
#   • Code generation (React, Rust, Python, etc.)
#   • Agentic tool calling (native Gemma 4 function calling)
#   • Multi-file reasoning at 26B quality
#   • 4B inference speed via MoE architecture
#   • Vision understanding (screenshot analysis)
#
# HARDWARE: AMD Strix Halo (APU) — gfx1151, 128GB unified LPDDR5x
#
# USAGE:
#   ./scripts/launch/launch_pete_coder.sh          # foreground
#   ./scripts/launch/launch_pete_coder.sh --bg     # background (for Hotel swap)
#
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DISTROBOX_NAME="vllm-019"
MODEL_DIR="$HOME/trinity-models/vllm/gemma-4-26B-A4B-it-AWQ-4bit"
PORT=8000
SERVED_NAME="Pete_Coder_26B"
BG_MODE="${1:-}"

echo "═══════════════════════════════════════════════"
echo "  TRINITY P-ART-Y — P (Programming)"
echo "  Gemma 4 26B A4B AWQ (MoE) — Port $PORT"
echo "  Role: Hotel swap — coding brain"
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
    echo "   distrobox enter $DISTROBOX_NAME -- huggingface-cli download google/gemma-4-26b-a4b-it-awq --local-dir $MODEL_DIR"
    exit 1
fi

echo "📦 Model: $MODEL_DIR"
echo "   Size:  $(du -sh "$MODEL_DIR" | cut -f1)"
echo ""

# ─── Kill any existing process on this port ──────────────────────────────────
if lsof -ti:$PORT >/dev/null 2>&1; then
    echo "⚠️  Killing existing process on port $PORT (Hotel checkout)..."
    lsof -ti:$PORT | xargs kill -9 2>/dev/null || true
    sleep 2
fi

# ─── Write launch script (avoids quoting issues across distrobox boundary) ───
LAUNCH_SCRIPT="/tmp/trinity_launch_pete_coder.sh"
cat <<LAUNCH_EOF > "$LAUNCH_SCRIPT"
#!/bin/bash
# ── AMD Strix Halo gfx1151 Environment ──
export HSA_ENABLE_SDMA=0
export MIOPEN_FIND_MODE=FAST
export PYTORCH_ROCM_ARCH="gfx1151"
export TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL=1
export VLLM_SKIP_WARMUP=true
export HSA_OVERRIDE_GFX_VERSION=11.5.1
export PYTORCH_HIP_ALLOC_CONF=expandable_segments:True
export NCCL_P2P_DISABLE=1
export VLLM_USE_V1=0

/opt/venv/bin/vllm serve "\$HOME/trinity-models/vllm/gemma-4-26B-A4B-it-AWQ-4bit" \\
    --port $PORT \\
    --gpu-memory-utilization 0.20 \\
    --max-model-len 8192 \\
    --enforce-eager \\
    --dtype half \\
    --quantization compressed-tensors \\
    --trust-remote-code \\
    --served-model-name "$SERVED_NAME" \\
    --enable-auto-tool-choice \\
    --tool-call-parser gemma4
LAUNCH_EOF

chmod +x "$LAUNCH_SCRIPT"

# ─── Launch inside distrobox ─────────────────────────────────────────────────
echo "🚀 Starting P (Programming) on port $PORT..."
echo "   Using distrobox: $DISTROBOX_NAME"
echo "   Served as: $SERVED_NAME"
echo ""

if [ "$BG_MODE" = "--bg" ]; then
    echo "   Mode: background (Hotel swap)"
    nohup distrobox enter "$DISTROBOX_NAME" -- bash "$LAUNCH_SCRIPT" > /tmp/trinity_pete_coder.log 2>&1 &
    echo "   PID: $!"
    echo "   Log: /tmp/trinity_pete_coder.log"

    # Wait for health
    echo -n "   Waiting for health"
    for i in $(seq 1 90); do
        if curl -s --connect-timeout 2 http://127.0.0.1:$PORT/health > /dev/null 2>&1; then
            echo ""
            echo "   ✅ P (Programming) ONLINE on port $PORT"
            exit 0
        fi
        echo -n "."
        sleep 2
    done
    echo ""
    echo "   ⚠️  Health check timed out. Check /tmp/trinity_pete_coder.log"
    exit 1
else
    distrobox enter "$DISTROBOX_NAME" -- bash "$LAUNCH_SCRIPT"
fi
