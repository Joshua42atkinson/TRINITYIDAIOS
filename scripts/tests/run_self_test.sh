#!/bin/bash

# Trinity Self-Improvement Test Runner
echo "🧪 Starting Trinity Self-Improvement Test"
echo "🎯 Testing if Trinity can wake up, evaluate itself, and improve its UI"
echo ""

# Change to the correct directory
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis

# First check compilation
echo "📦 Checking compilation..."
cargo check --bin simple_self_test

if [ $? -ne 0 ]; then
    echo "❌ Compilation failed!"
    exit 1
fi

echo "✅ Compilation successful!"
echo ""

# Run the simple test
echo "🚀 Running simple self-improvement test..."
echo "📝 A window should open showing Trinity changing its own colors"
echo "📝 Watch for the phases: Starting → Evaluating → Improving → Complete"
echo ""

cargo run --bin simple_self_test

echo ""
echo "✅ Test complete!"
echo "📝 Did you see Trinity change its colors?"
