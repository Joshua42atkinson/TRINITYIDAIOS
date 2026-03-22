#!/bin/bash
# Rebuild llama.cpp for AMD ROCm (Strix Halo - gfx1151)
# Trinity ID AI OS - Performance Optimization Script

set -e

LLAMA_DIR="/home/joshua/Workflow/desktop_trinity/trinity-genesis/llama.cpp"
BUILD_DIR="$LLAMA_DIR/build"
ROCM_PATH="/opt/rocm"

echo "🚀 Starting Trinity optimization for Strix Halo GPU (gfx1151)..."
echo "   ROCm Path: $ROCM_PATH"
echo "   LLAMA DIR: $LLAMA_DIR"

if [ ! -d "$LLAMA_DIR" ]; then
    echo "❌ llama.cpp directory not found! Cloning..."
    git clone https://github.com/ggerganov/llama.cpp.git "$LLAMA_DIR"
fi

cd "$LLAMA_DIR"

# Ensure we're in a clean state
# echo "🧹 Cleaning build directory..."
# rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

echo "⚙️ Configuring CMake for ROCm/HIP (gfx1151)..."
cmake .. \
    -DGGML_HIP=ON \
    -DAMDGPU_TARGETS=gfx1151 \
    -DCMAKE_HIP_COMPILER=$ROCM_PATH/llvm/bin/clang++ \
    -DCMAKE_C_COMPILER=$ROCM_PATH/llvm/bin/clang \
    -DCMAKE_CXX_COMPILER=$ROCM_PATH/llvm/bin/clang++ \
    -DCMAKE_BUILD_TYPE=Release \
    -DGGML_HIPBLAS=ON \
    -DGGML_OPENMP=ON \
    -DGGML_AVX=ON \
    -DGGML_AVX2=ON \
    -DGGML_FMA=ON \
    -DGGML_F16C=ON \
    -DGGML_NATIVE=ON

echo "🏗️ Building llama-cli for ROCm..."
cmake --build . --config Release --target llama-cli -j$(nproc)

echo "✅ llama-cli rebuilt with ROCm support!"
ls -l bin/llama-cli

echo "🚀 Trinity is now ready for 96GB VRAM offload on Strix Halo."
