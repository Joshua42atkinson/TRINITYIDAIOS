#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Voxtral-4B TTS Launch Script
# Voice narration engine — 20 preset voices, 9 languages, emotion control
#
# Runs on port 8100 as a separate vLLM instance with --omni
# Trinity's voice.rs auto-detects via check_voxtral_health()
# ═══════════════════════════════════════════════════════════════════════════════
set -e

VENV="${HOME}/trinity-vllm-env"
MODEL="${HOME}/trinity-models/tts/voxtral-4b"
PORT="${VOXTRAL_PORT:-8100}"

# ── AMD Strix Halo (gfx1151 / RDNA 3.5) ──
export HSA_OVERRIDE_GFX_VERSION=11.0.0
export ROCBLAS_USE_HIPBLASLT=1
export HIP_VISIBLE_DEVICES=0
export AMD_LOG_LEVEL=0

# ── Preflight Checks ──
if [ ! -d "$VENV" ]; then
    echo "ERROR: vLLM venv not found at $VENV"
    exit 1
fi

if [ ! -d "$MODEL" ]; then
    echo "ERROR: Voxtral-4B safetensors not found at $MODEL"
    echo "Download: python -c \"from huggingface_hub import snapshot_download; snapshot_download('mistralai/Voxtral-4B-TTS-2603', local_dir='$MODEL')\""
    exit 1
fi

# ── Activate Environment ──
source "${VENV}/bin/activate"

echo "═══════════════════════════════════════════════════"
echo "  TRINITY VOXTRAL — 4B TTS Engine"
echo "═══════════════════════════════════════════════════"
echo "  Model:     ${MODEL}"
echo "  Port:      ${PORT}"
echo "  Voices:    20 presets (causal_male, alloy, echo, nova, ...)"
echo "  Languages: 9 (en, fr, de, es, pt, it, hi, zh, ja)"
echo "  ROCm:      gfx1151 (HSA_OVERRIDE=11.0.0)"
echo "═══════════════════════════════════════════════════"
echo ""

# ── Launch vLLM with Omni flag for audio generation ──
# Lower resource footprint than main MS4 inference:
#   - 4B params (vs 119B MoE) — fits in ~6 GB VRAM with INT4
#   - Short context (max 4096 for TTS chunks)
#   - Low batch size (TTS is sequential per request)
exec vllm serve "$MODEL" \
    --port "$PORT" \
    --max-model-len 4096 \
    --trust-remote-code \
    --enforce-eager \
    --gpu-memory-utilization 0.1 \
    --max-num-seqs 4 \
    "$@"
