#!/bin/bash
set -e

echo "================================================="
echo " Setting up Fresh SGLang Environment (ROCm/AMD)"
echo "================================================="

# 1. Create directory structure and enter it
cd "$(dirname "$0")"

# ─── ROCm Environment for Strix Halo (gfx1151) ───
export HSA_OVERRIDE_GFX_VERSION=11.0.0
export PYTORCH_ROCM_ARCH="gfx1151"

# 2. Re-create pristine virtual environment
echo "[1/3] Creating isolated Python virtual environment..."
python3 -m venv .venv
source .venv/bin/activate

# 3. Upgrade pip
pip install --upgrade pip

# 4. Install ROCm-specific PyTorch
# Using standard rocm package indexing from pytorch.
echo "[2/3] Installing PyTorch for ROCm 6.2..."
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/rocm6.2

# 5. Install SGLang (AMD compatible)
echo "[3/3] Installing SGLang and bridge requirements..."
# NOTE: Removed the CUDA-only --find-links https://flashinfer.ai/whl/... flag to prevent conflicts. 
# SGLang will natively fall back to Triton kernels under ROCm.
pip install "sglang[all]"

# Install requirements for the Omni-Bridge and API tests
pip install soundfile fastapi uvicorn httpx pyyaml

echo "================================================="
echo " Environment setup complete for AMD Strix Halo."
echo " Activation command: source .venv/bin/activate"
echo "================================================="
