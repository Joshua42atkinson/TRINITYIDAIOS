#!/bin/bash
# Download Qwen3.5-REAP-97B-A10B-Q4_K_M model from HuggingFace

echo "🚀 Downloading Qwen3.5-REAP-97B-A10B-Q4_K_M..."

# Create conductor directory
mkdir -p models/conductor/

# Download mmproj file (available now)
echo "📥 Downloading vision projector..."
curl -L -o models/conductor/Qwen3.5-97B-A10B-mmproj-BF16.gguf \
  https://huggingface.co/OpenMOSE/Qwen3.5-REAP-97B-A10B-GGUF/resolve/main/Qwen3.5-97B-A10B-mmproj-BF16.gguf

# Check if GGUF files are available (they're not uploaded yet)
echo "🔍 Checking GGUF file availability..."
if curl -s -f "https://huggingface.co/OpenMOSE/Qwen3.5-REAP-97B-A10B-GGUF/resolve/main/Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf" > /dev/null; then
    echo "📥 Downloading GGUF part 1..."
    curl -L -o models/conductor/Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf \
      https://huggingface.co/OpenMOSE/Qwen3.5-REAP-97B-A10B-GGUF/resolve/main/Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf
    
    echo "📥 Downloading GGUF part 2..."
    curl -L -o models/conductor/Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf \
      https://huggingface.co/OpenMOSE/Qwen3.5-REAP-97B-A10B-GGUF/resolve/main/Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf
else
    echo "⚠️  GGUF files not yet uploaded to HuggingFace"
    echo "📋 You'll need to download them when they become available:"
    echo "   - Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf"
    echo "   - Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf"
fi

# Verify files
echo "🔍 Verifying downloaded files..."
for file in models/conductor/*.gguf; do
    if [ -f "$file" ]; then
        size_mb=$(du -h "$file" | cut -f1)
        echo "✅ $(basename "$file"): $size_mb"
    else
        echo "❌ Missing: $(basename "$file")"
    fi
done

echo ""
echo "📊 Expected file sizes:"
echo "   - Qwen3.5-97B-A10B-mmproj-BF16.gguf: ~912 MB ✅"
echo "   - Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf: ~25 GB (when available)"
echo "   - Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf: ~25 GB (when available)"
echo ""
echo "🎉 Download complete!"
echo "💡 Total expected size: ~50 GB when GGUF files are available"
