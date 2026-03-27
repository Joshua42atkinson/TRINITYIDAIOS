#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — vLLM MS4 + P-EAGLE Launch Script
# Host-local execution — this whole computer is Trinity
#
# Based on official Mistral serve command:
# https://huggingface.co/mistralai/Mistral-Small-4-119B-2603-eagle
# ═══════════════════════════════════════════════════════════════════════════════
set -e

VENV="${HOME}/trinity-vllm-env"
MODEL="${HOME}/trinity-models/safetensors/Mistral-Small-4-119B-2603"
EAGLE="${HOME}/trinity-models/safetensors/Mistral-Small-4-119B-2603-eagle"
PORT="${VLLM_PORT:-8000}"
MAX_LEN="${VLLM_MAX_LEN:-32768}"

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
    echo "ERROR: MS4 safetensors not found at $MODEL"
    echo "Download: huggingface-cli download mistralai/Mistral-Small-4-119B-2603 --local-dir $MODEL"
    exit 1
fi

# ── Activate Environment ──
source "${VENV}/bin/activate"

# ── Build Speculative Config (EAGLE ~390 MB) ──
SPEC_ARGS=""
if [ -d "$EAGLE" ] && [ "$(ls -A $EAGLE/*.safetensors 2>/dev/null)" ]; then
    echo "🦅 EAGLE head found — enabling speculative decoding (3 draft tokens)"
    SPEC_ARGS="--speculative-config {\"model\":\"${EAGLE}\",\"num_speculative_tokens\":3,\"method\":\"eagle\",\"max_model_len\":\"16384\"}"
else
    echo "⚠️  EAGLE head not found — running without speculative decoding"
    echo "    Download (~390 MB): python -c \"from huggingface_hub import snapshot_download; snapshot_download('mistralai/Mistral-Small-4-119B-2603-eagle', local_dir='${EAGLE}')\""
fi

echo "═══════════════════════════════════════════════════"
echo "  TRINITY vLLM — Mistral Small 4 (119B MoE)"
echo "═══════════════════════════════════════════════════"
echo "  Model:     ${MODEL}"
echo "  Port:      ${PORT}"
echo "  Max Len:   ${MAX_LEN}"
echo "  Quant:     fp8"
echo "  EAGLE:     $([ -n "$SPEC_ARGS" ] && echo 'enabled (3 tokens)' || echo 'disabled')"
echo "  Tool Call: Mistral native (--tool-call-parser mistral)"
echo "  Reasoning: Mistral native (--reasoning-parser mistral)"
echo "  ROCm:      gfx1151 (HSA_OVERRIDE=11.0.0)"
echo "═══════════════════════════════════════════════════"
echo ""

# ── Launch vLLM ──
# Based on official Mistral model card recommendations
# Strix Halo: single GPU (tensor-parallel-size 1), UMA 128GB
exec vllm serve "$MODEL" \
    --port "$PORT" \
    --max-model-len "$MAX_LEN" \
    --quantization fp8 \
    --trust-remote-code \
    --enforce-eager \
    --tool-call-parser mistral \
    --enable-auto-tool-choice \
    --reasoning-parser mistral \
    --max-num-batched-tokens 16384 \
    --max-num-seqs 128 \
    --gpu-memory-utilization 0.8 \
    $SPEC_ARGS \
    "$@"
