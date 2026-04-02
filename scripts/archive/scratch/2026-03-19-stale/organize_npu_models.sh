#!/bin/bash
# Trinity NPU Model Organization Script
# Copyright (c) Joshua
# Shared under license for Ask_Pete (Purdue University)

echo "Trinity NPU Model Organization"
echo "=============================="

# Define paths
TRINITY_ROOT="/home/joshua/Workflow/desktop_trinity/trinity-genesis"
NPU_MODELS_DIR="$TRINITY_ROOT/models/npu"
SEARCH_ROOT="/home/joshua"

# Create NPU models directory structure
mkdir -p "$NPU_MODELS_DIR"/{granite,qwen-vl,embeddings,code,multilingual}

echo "Created NPU models directory structure at:"
echo "  $NPU_MODELS_DIR"

# Function to move model if found
move_model() {
    local model_name="$1"
    local target_dir="$2"
    local pattern="$3"
    
    echo -e "\nSearching for $model_name..."
    
    # Find the model
    local model_path=$(find "$SEARCH_ROOT" -path "*/$pattern" -type f 2>/dev/null | grep -v "__pycache__" | grep -v ".git" | head -1)
    
    if [ -n "$model_path" ]; then
        echo "Found: $model_path"
        
        # Get the directory containing the model
        local model_dir=$(dirname "$model_path")
        
        # Move to target location
        if [ -d "$model_dir" ] && [ "$(ls -A "$model_dir" 2>/dev/null)" ]; then
            echo "Moving to: $NPU_MODELS_DIR/$target_dir/"
            mv "$model_dir" "$NPU_MODELS_DIR/$target_dir/"
            echo "✅ Moved successfully"
        else
            echo "⚠️  Model directory is empty or doesn't exist"
        fi
    else
        echo "❌ Model not found"
        echo "   Searched for pattern: $pattern"
        echo "   Please ensure the model is downloaded"
    fi
}

# Look for Granite 4.0 model
echo -e "\n=== Granite 4.0 Model ==="
move_model "Granite 4.0 1B" "granite/granite-4.0-1b-converted" "*granite*4*0*1b*"

# Look for Qwen3-VL model (when downloaded)
echo -e "\n=== Qwen3-VL Model ==="
move_model "Qwen3-VL 4B" "qwen-vl/qwen3-vl-4b-instruct" "*qwen*vl*4b*"

# Look for embedding models
echo -e "\n=== Embedding Models ==="
move_model "MiniLM-L6-v2" "embeddings/all-MiniLM-L6-v2" "*minilm*l6*v2*"

# Look for code models
echo -e "\n=== Code Models ==="
move_model "Code Model" "code/code-1b-onnx" "*code*1b*"

# Look for multilingual models
echo -e "\n=== Multilingual Models ==="
move_model "Multilingual Model" "multilingual/multilingual-1b" "*multilingual*1b*"

# Show final structure
echo -e "\n=== Final NPU Models Structure ==="
tree -L 3 "$NPU_MODELS_DIR" 2>/dev/null || ls -la "$NPU_MODELS_DIR"

# Calculate total size
echo -e "\n=== Disk Usage ==="
if [ -d "$NPU_MODELS_DIR" ]; then
    du -sh "$NPU_MODELS_DIR"/* 2>/dev/null
fi

echo -e "\n✅ Organization complete!"
echo -e "\nNext steps:"
echo "1. Download Qwen3-VL-4B-Instruct model"
echo "2. Run: cargo run --release -p trinity-kernel --bin test_npu_dispatch"
echo "3. Run: cargo run --release -p trinity-kernel --bin test_npu_vision"
