#!/bin/bash
# Find and Move Granite Model Script
# Copyright (c) Joshua
# Shared under license for Ask_Pete (Purdue University)

echo "Searching for Granite model..."

# Search for granite directories
GRANITE_DIRS=$(find /home/joshua -type d -iname "*granite*" 2>/dev/null | grep -v "__pycache__" | grep -v "site-packages")

if [ -z "$GRANITE_DIRS" ]; then
    echo "No granite directories found. Searching for model files..."
    # Search for model files
    MODEL_FILES=$(find /home/joshua -name "*.onnx" -o -name "*.safetensors" -o -name "*.bin" 2>/dev/null | grep -i granite | head -5)
    
    if [ -z "$MODEL_FILES" ]; then
        echo "No granite model files found."
        echo "Please ensure the model is downloaded and extract the path."
        exit 1
    fi
    
    echo "Found granite model files:"
    echo "$MODEL_FILES"
    
    # Get the directory of the first file
    FIRST_FILE=$(echo "$MODEL_FILES" | head -1)
    MODEL_DIR=$(dirname "$FIRST_FILE")
    echo "Model directory: $MODEL_DIR"
else
    echo "Found granite directories:"
    echo "$GRANITE_DIRS"
    
    # Use the first directory found
    MODEL_DIR=$(echo "$GRANITE_DIRS" | head -1)
fi

# Create target directory
TARGET_DIR="/home/joshua/Workflow/desktop_trinity/trinity-genesis/models/npu/granite/granite-4.0-1b-converted"
echo "Creating target directory: $TARGET_DIR"
mkdir -p "$TARGET_DIR"

# Move the model
if [ -d "$MODEL_DIR" ]; then
    echo "Moving model from $MODEL_DIR to $TARGET_DIR"
    cp -r "$MODEL_DIR"/* "$TARGET_DIR/"
    echo "✅ Model moved successfully"
else
    echo "Model directory not found: $MODEL_DIR"
    exit 1
fi

# List the moved files
echo "Files in target directory:"
ls -la "$TARGET_DIR"

# Check for ONNX file
ONNX_FILE=$(find "$TARGET_DIR" -name "*.onnx" | head -1)
if [ -n "$ONNX_FILE" ]; then
    echo "✅ Found ONNX file: $(basename "$ONNX_FILE")"
    echo "File size: $(du -h "$ONNX_FILE" | cut -f1)"
else
    echo "⚠️  No ONNX file found. Checking for other formats..."
    SAFETENSORS_FILE=$(find "$TARGET_DIR" -name "*.safetensors" | head -1)
    if [ -n "$SAFETENSORS_FILE" ]; then
        echo "✅ Found Safetensors file: $(basename "$SAFETENSORS_FILE")"
        echo "File size: $(du -h "$SAFETENSORS_FILE" | cut -f1)"
    fi
fi

echo "Model setup complete!"
