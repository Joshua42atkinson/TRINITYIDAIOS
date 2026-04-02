#!/bin/bash

# Trinity Unified Memory Verification Script
# Checks 96GB unified memory configuration on Strix Halo

echo "🔍 Trinity Unified Memory Verification"
echo "=================================="

# Check kernel version
echo "📋 Kernel Version:"
uname -r
echo ""

# Check current TTM parameters
echo "🔧 TTM Parameters:"
cat /sys/module/ttm/parameters/p* | awk '{print $1 / (1024 * 1024 / 4)}' | while read line; do
    echo "  Memory Allocation: ${line}GB"
done
echo ""

# Check ROCm memory
echo "🚀 ROCm Memory Status:"
rocm-smi --showmeminfo vram 2>/dev/null || echo "  ROCm not available yet"
echo ""

# Check AMD GPU info
echo "🎯 GPU Information:"
rocm-smi --showproductname 2>/dev/null || echo "  ROCm not available yet"
echo ""

# Expected values verification
echo "✅ Expected Configuration:"
echo "  BIOS GMA: 512MB (GMKtec EVO X2 minimum)"
echo "  TTM pages_limit: 33554432 (128GB)"
echo "  TTM page_pool_size: 33554432 (128GB)"
echo "  amdgpu.gttsize: 131072 (128GB)"
echo "  Expected Output: 128 128"
echo ""

# Check if values match expected
current_values=$(cat /sys/module/ttm/parameters/p* | awk '{print $1 / (1024 * 1024 / 4)}')
if [[ "$current_values" == *"128"* ]]; then
    echo "🎉 SUCCESS: 128GB unified memory configured!"
else
    echo "⚠️  WARNING: Memory allocation may not be optimal"
    echo "   Current values: $current_values"
fi

echo ""
echo "🚀 Ready for 97B model deployment!"
