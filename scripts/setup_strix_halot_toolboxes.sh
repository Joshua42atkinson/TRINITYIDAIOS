#!/bin/bash
# Trinity Strix Halo Toolbox Setup Script
# Based on Kryzo's proven configurations for AMD Strix Halo

set -e

echo "🚀 Setting up Trinity Strix Halo toolboxes..."

# Check if we're on a compatible system
if ! command -v toolbox &> /dev/null; then
    echo "❌ Error: toolbox command not found. Please install Toolbox (Fedora) or Distrobox (Ubuntu)."
    exit 1
fi

echo "📦 Creating ROCm 7.2 toolbox for llama.cpp..."
toolbox create trinity-llama-rocm \
  --image docker.io/kyuz0/amd-strix-halo-toolboxes:rocm-7.2 \
  -- --device /dev/dri \
  --device /dev/kfd \
  --group-add video \
  --group-add render \
  --security-opt seccomp=unconfined

echo "📦 Creating vLLM toolbox for high-performance inference..."
toolbox create trinity-vllm \
  --image docker.io/kyuz0/vllm-therock-gfx1151:latest \
  -- --device /dev/dri \
  --device /dev/kfd \
  --group-add video \
  --group-add render \
  --security-opt seccomp=unconfined

echo "📦 Creating Vulkan toolbox for compatibility..."
toolbox create trinity-vulkan-radv \
  --image docker.io/kyuz0/amd-strix-halo-toolboxes:vulkan-radv \
  -- --device /dev/dri \
  --group-add video \
  --security-opt seccomp=unconfined

echo "✅ Strix Halo toolboxes created successfully!"
echo ""
echo "🎯 Available toolboxes:"
echo "   - trinity-llama-rocm (ROCm 7.2 - Recommended for performance)"
echo "   - trinity-vllm (High-performance vLLM)"
echo "   - trinity-vulkan-radv (Compatibility mode)"
echo ""
echo "🔧 To enter a toolbox:"
echo "   toolbox enter trinity-llama-rocm"
echo ""
echo "📚 Next steps:"
echo "   1. Enter the ROCm toolbox: toolbox enter trinity-llama-rocm"
echo "   2. Test GPU access: llama.cpp --help"
echo "   3. Load your models: llama.cpp -m /path/to/model.gguf"
