#!/bin/bash
# Trinity vLLM Setup using Distrobox
# Based on kyuz0's vLLM containers for AMD Strix Halo

set -e

echo "🚀 Setting up Trinity vLLM with distrobox..."

# Create vLLM container
echo "📦 Creating vLLM container for high-performance inference..."
distrobox create trinity-vllm \
  --image docker.io/kyuz0/vllm-therock-gfx1151:latest \
  --home /home/joshua/trinity-vllm-home \
  --additional-flags "--device /dev/dri --device /dev/kfd --group-add video --group-add render"

echo "✅ vLLM container created successfully!"
echo ""
echo "🎯 To start using vLLM:"
echo "   distrobox enter trinity-vllm"
echo ""
echo "📚 Once inside the container:"
echo "   1. Test vLLM: vllm --help"
echo "   2. Start server: vllm serve /path/to/model"
echo ""
echo "🔧 Available models in /home/joshua/trinity-models/gguf/:"
ls -la /home/joshua/trinity-models/gguf/ | grep -E "(Crow|Qwen|MiniMax)" | awk '{print "   " $9 " (" $5 " bytes)"}'
