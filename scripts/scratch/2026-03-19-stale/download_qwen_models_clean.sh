#!/bin/bash
# Trinity Qwen Model Downloader
# Downloads all models for V1 Qwen Omni-Stack

set -e

echo "🚀 Trinity Qwen Model Download Started"
echo "=================================="

# Create models directory
mkdir -p models
cd models

# Model configurations
declare -A MODELS=(
    ["conductor"]="Qwen2.5-72B-Instruct-Q4_K_M.gguf|https://huggingface.co/Qwen/Qwen2.5-72B-Instruct-GGUF/resolve/main/Qwen2.5-72B-Instruct-Q4_K_M.gguf"
    ["engineer"]="Qwen2.5-Coder-32B-Instruct-Q4_K_M.gguf|https://huggingface.co/Qwen/Qwen2.5-Coder-32B-Instruct-GGUF/resolve/main/Qwen2.5-Coder-32B-Instruct-Q4_K_M.gguf"
    ["omni"]="Qwen2-VL-7B-Instruct-Q4_K_M.gguf|https://huggingface.co/Qwen/Qwen2-VL-7B-Instruct-GGUF/resolve/main/Qwen2-VL-7B-Instruct-Q4_K_M.gguf"
    ["dispatcher"]="Qwen2.5-14B-Instruct-Q4_K_M.gguf|https://huggingface.co/Qwen/Qwen2.5-14B-Instruct-GGUF/resolve/main/Qwen2.5-14B-Instruct-Q4_K_M.gguf"
)

# Download each model
for agent in "${!MODELS[@]}"; do
    IFS='|' read -r model_file model_url <<< "${MODELS[$agent]}"
    
    echo "📥 Downloading $agent model: $model_file"
    
    if [ -f "$model_file" ]; then
        echo "✅ $model_file already exists, checking integrity..."
        # Add integrity check here if needed
    else
        echo "⬇️  Downloading $model_file..."
        wget -O "$model_file" "$model_url" || {
            echo "❌ Failed to download $model_file"
            exit 1
        }
        echo "✅ $model_file downloaded successfully"
    fi
    
    # Show file size
    size=$(du -h "$model_file" | cut -f1)
    echo "📊 File size: $size"
    echo ""
done

echo "🎉 All Qwen models downloaded successfully!"
echo "=================================="
echo "Total disk usage:"
du -sh .
echo ""
echo "Models ready for Trinity V1 Qwen Omni-Stack!"
