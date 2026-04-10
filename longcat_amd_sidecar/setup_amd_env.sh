#!/bin/bash
set -e

echo "================================================="
echo " Setting up Fresh AMD Transformers Environment"
echo "================================================="

# 1. Create directory structure and enter it
cd "$(dirname "$0")"

# 2. Create pristine virtual environment
echo "[1/3] Creating isolated Python virtual environment..."
python3 -m venv .venv
source .venv/bin/activate

# 3. Upgrade pip
pip install --upgrade pip

# 4. Install ROCm-specific PyTorch (using standard pip wheels for 6.2 compatibility)
echo "[2/3] Installing PyTorch for ROCm..."
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/rocm6.2

# 5. Install standard multimodal and quantization libraries
echo "[3/3] Installing transformers, bitsandbytes, and API tools..."
pip install transformers accelerate soundfile fastapi uvicorn httpx pydantic

# Specifically install bitsandbytes compiled for ROCm if standard pip wheel fails, 
# but usually standard bitsandbytes auto-compiles or has ROCm binaries natively now
# We will use the latest pip release of bitsandbytes which has better multi-platform support.
pip install bitsandbytes>=0.43.0

echo "================================================="
echo " Environment setup complete."
echo "================================================="
