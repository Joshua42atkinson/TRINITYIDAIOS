#!/bin/bash
# Download AMD-OLMo Model for Trinity NPU
# Copyright (c) Joshua
# Shared under license for Ask_Pete (Purdue University)

set -e

echo "🚀 Downloading AMD-OLMo-1B for Trinity NPU..."

# Model details
MODEL_NAME="amd/AMD-OLMo-1B-SFT-DPO-awq-g128-int4-asym-bf16-onnx-ryzen-strix"
LOCAL_DIR="models/npu/amd-olmo-1b"

# Create directory
mkdir -p "$LOCAL_DIR"
echo "✅ Created directory: $LOCAL_DIR"

# Check if huggingface-cli is installed
if ! command -v huggingface-cli &> /dev/null; then
    echo "⚠️  huggingface-cli not found. Installing..."
    pip install -U "huggingface_hub[cli]"
fi

# Download model
echo "📥 Downloading model: $MODEL_NAME"
huggingface-cli download "$MODEL_NAME" \
    --local-dir "$LOCAL_DIR" \
    --include="*.onnx" \
    --include="*.json" \
    --include="*.txt" \
    --resume-download

# Verify download
echo "🔍 Verifying download..."
if [ -f "$LOCAL_DIR/model.onnx" ]; then
    MODEL_SIZE=$(du -h "$LOCAL_DIR/model.onnx" | cut -f1)
    echo "✅ Model downloaded successfully"
    echo "   Size: $MODEL_SIZE"
else
    echo "❌ Model file not found"
    exit 1
fi

# List all files
echo "📁 Downloaded files:"
ls -la "$LOCAL_DIR/"

# Check total size
TOTAL_SIZE=$(du -sh "$LOCAL_DIR" | cut -f1)
echo "💾 Total download size: $TOTAL_SIZE"

# Create test script
cat > test_amd_olmo.sh << 'EOF'
#!/bin/bash
echo "🧪 Testing AMD-OLMo-1B on NPU..."
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis
LD_LIBRARY_PATH=/opt/xilinx/xrt/lib:$LD_LIBRARY_PATH \
cargo run --release -p trinity-kernel --bin test_amd_olmo
EOF

chmod +x test_amd_olmo.sh

echo ""
echo "🎯 Download complete!"
echo "   Model: $MODEL_NAME"
echo "   Location: $LOCAL_DIR"
echo "   Test: ./test_amd_olmo.sh"
echo ""
echo "📊 Model Info:"
echo "   - Parameters: 1.2B"
echo "   - Format: ONNX (NPU compatible)"
echo "   - Quantization: INT4 weights, BF16 activations"
echo "   - Memory: ~750MB (fits in 2GB NPU)"
echo "   - License: MIT + Apache 2.0"
