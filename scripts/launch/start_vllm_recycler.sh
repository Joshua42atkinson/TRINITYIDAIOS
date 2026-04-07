#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════
# Trinity vLLM Launch — Great Recycler (port 8001)
# ═══════════════════════════════════════════════════════════════
# Run via:  distrobox enter vllm -- bash scripts/launch/start_vllm_recycler.sh
#
# Required env fix for gfx1151 (Strix Halo RDNA 3.5):
#   VLLM_ROCM_SHUFFLE_KV_CACHE_LAYOUT=0  — disables shuffled KV cache
#     that causes shape mismatch crash in V1 engine
# ═══════════════════════════════════════════════════════════════

export VLLM_ROCM_SHUFFLE_KV_CACHE_LAYOUT=0
export PYTORCH_TUNABLEOP_ENABLED=1

MODEL_DIR="$HOME/trinity-models/vllm/Qwen2.5-14B-Instruct-AWQ"
PORT=8001
GPU_MEM=0.35
MAX_LEN=16384
MODEL_NAME="Great_Recycler"

echo "═══ Trinity vLLM Recycler ═══"
echo "  Model:    $MODEL_DIR"
echo "  Port:     $PORT"
echo "  GPU mem:  $GPU_MEM"
echo "  Max len:  $MAX_LEN"
echo "  KV shuffle: $VLLM_ROCM_SHUFFLE_KV_CACHE_LAYOUT"
echo "═════════════════════════════"

exec vllm serve "$MODEL_DIR" \
  --port "$PORT" \
  --gpu-memory-utilization "$GPU_MEM" \
  --max-model-len "$MAX_LEN" \
  --dtype half \
  --served-model-name "$MODEL_NAME" \
  2>&1 | tee /tmp/vllm-31b.log
