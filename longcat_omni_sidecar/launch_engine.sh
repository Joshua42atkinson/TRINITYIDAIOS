#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — launch_engine.sh
# ═══════════════════════════════════════════════════════════════════════════════
#
# PURPOSE:  Launch the LongCat-Next 74B MoE Omni-Brain sidecar
# HARDWARE: AMD Strix Halo (APU) — gfx1151, 128GB unified LPDDR5x
#
# QUANTIZATION: 4-bit NF4 via bitsandbytes
#   - Full bf16 model: ~151GB
#   - 4-bit NF4 model: ~38GB in-flight VRAM
#   - Multimodal decoders (dNaViT, CosyVoice) remain at bf16 precision
#
# ARCHITECTURE:
#   The model is served via transformers + FastAPI (not sglang) because:
#   1. Stock sglang 0.5.x doesn't support 'longcat_next' model type
#   2. The FluentLLM sglang fork requires CUDA (NVIDIA only)
#   3. Transformers with trust_remote_code=True handles everything natively
#   4. All multimodal decode (image gen, audio gen) works out of the box
#
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

echo "🐱 LongCat-Next Omni-Brain — Trinity ID AI OS"
echo "═══════════════════════════════════════════════"

# ─── ROCm Environment for Strix Halo (gfx1151) ──────────────────────────────
export HSA_ENABLE_SDMA=0
export MIOPEN_FIND_MODE=FAST
export PYTORCH_ROCM_ARCH="gfx1151"
export TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL=1

# ─── HuggingFace Cache (project-local, avoids ~/.cache permission conflicts) ─
export HF_HOME="$HOME/trinity-models/sglang/LongCat-Next/.cache"
export TRANSFORMERS_CACHE="$HOME/trinity-models/sglang/LongCat-Next/.cache"
mkdir -p "$HF_HOME/modules" 2>/dev/null

# Fix bitsandbytes ROCm version detection (rocm713 binary, not rocm72)
BNB_DIR="/opt/venv/lib64/python3.12/site-packages/bitsandbytes"
if [ -f "$BNB_DIR/libbitsandbytes_rocm713.so" ] && [ ! -f "$BNB_DIR/libbitsandbytes_rocm72.so" ]; then
    echo "🔧 Symlinking bitsandbytes ROCm library..."
    ln -sf "$BNB_DIR/libbitsandbytes_rocm713.so" "$BNB_DIR/libbitsandbytes_rocm72.so"
    ln -sf "$BNB_DIR/libbitsandbytes_rocm713.so" "$BNB_DIR/libbitsandbytes_cpu.so"
fi

# ─── Verify Model ───────────────────────────────────────────────────────────
MODEL_DIR="$HOME/trinity-models/sglang/LongCat-Next"

if [ ! -d "$MODEL_DIR" ]; then
    echo "⚠️  Model directory not found at $MODEL_DIR"
    echo "   Download with: huggingface-cli download meituan-longcat/LongCat-Next --local-dir $MODEL_DIR"
    exit 1
fi

SHARD_COUNT=$(ls "$MODEL_DIR"/model-*.safetensors 2>/dev/null | wc -l)
echo "📦 Model shards found: $SHARD_COUNT/15"
if [ "$SHARD_COUNT" -lt 15 ]; then
    echo "⚠️  Download incomplete! Only $SHARD_COUNT/15 shards present."
    echo "   The server will start in mock mode."
fi

# ─── Launch the Omni Sidecar ────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
echo ""
echo "🚀 Starting LongCat-Next Omni Sidecar on port 8010..."
echo "   4-bit NF4 quantization via bitsandbytes"
echo "   Endpoints: /v1/chat/completions, /v1/images/generations, /tts"
echo ""

# Use the container's venv Python (not system python)
exec /opt/venv/bin/python3 "$SCRIPT_DIR/server.py"
