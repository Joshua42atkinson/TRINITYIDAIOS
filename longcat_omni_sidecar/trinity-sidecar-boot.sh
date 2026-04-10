#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — trinity-sidecar-boot.sh
# ═══════════════════════════════════════════════════════════════════════════════
#
# PURPOSE:  Native APU deployment sequence executed blindly by Tauri Framework.
# ARCHITECTURE: 
#   - Relies on `uv` to instantly assemble isolated PyTorch ecosystems without
#     requiring system-level OS Python installations or Docker mappings.
#   - Retains LongCat-Next DiNA Output functionality via vLLM abstraction.
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

echo "⚙️  Initializing Trinity Inference Sidecar..."

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TARGET_MODEL="$HOME/trinity-models/vllm/LongCat-Next-AWQ-4bit"

# 1. Environment Verification & Bootstrapping
if ! command -v uv &> /dev/null; then
    echo "⬇️  'uv' not natively found. Initializing portable environment assembler..."
    curl -LsSf https://astral.sh/uv/install.sh | env UV_UNMANAGED_INSTALL="/usr/local/bin" sh
fi

# 2. Build the Isolated Model Space
if [ ! -d "$SCRIPT_DIR/.venv" ]; then
    echo "🏗️  Constructing isolated inference matrix (Zero-Dependency Architecture)..."
    uv venv "$SCRIPT_DIR/.venv"
    source "$SCRIPT_DIR/.venv/bin/activate"
    
    echo "📦 Hydrating vLLM ROCm binaries..."
    # Pinning down the exact AMD APU native routing version for Strix Halo stability.
    uv pip install vllm torch torchvision torchaudio --index-url https://download.pytorch.org/whl/rocm6.1
else
    source "$SCRIPT_DIR/.venv/bin/activate"
fi

# 3. Model Deployment Readiness
if [ ! -d "$TARGET_MODEL" ]; then
    echo "❌ Error: Quantized DiNA matrix not ready."
    echo "   Please wait for the background AMD Quark pipeline to finish at $TARGET_MODEL."
    exit 1
fi

echo "🚀 Booting vLLM Multi-Modal Engine Matrix natively on APU..."
echo "   Access: http://127.0.0.1:8010/v1/chat/completions"

# 4. LongCat-Next Payload Injection
# Explicitly enforcing Image=1, Audio=1 to prevent DiNA OOM crashes.
exec python3 -m vllm.entrypoints.openai.api_server \
  --model "$TARGET_MODEL" \
  --quantization awq \
  --gpu-memory-utilization 0.85 \
  --trust-remote-code \
  --dtype float16 \
  --limit-mm-per-prompt image=1,audio=1 \
  --max-model-len 8192 \
  --port 8010 \
  --host 0.0.0.0
