#!/bin/bash

# Trinity Standalone Self-Improvement Test Runner
echo "🧪 Trinity Standalone Self-Improvement Test"
echo "🎯 Testing if Trinity can change its own colors"
echo "📝 This test has NO dependencies on our complex systems"
echo ""

# Change to the correct directory
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis

# First check compilation
echo "📦 Checking compilation..."
cargo check --bin standalone_test

if [ $? -ne 0 ]; then
    echo "❌ Compilation failed!"
    echo "📝 This should work - it's a minimal Bevy + Egui app"
    exit 1
fi

echo "✅ Compilation successful!"
echo ""

# Run the standalone test
echo "🚀 Running standalone self-improvement test..."
echo "📝 A window should open showing Trinity changing its own colors"
echo "📝 Watch for the phases: Starting → Evaluating → Improving → Complete"
echo "📝 Colors should change from blue/orange to green/red after 5 seconds"
echo ""

cargo run --bin standalone_test

echo ""
echo "✅ Test complete!"
echo ""
echo "🤔 IMPORTANT QUESTIONS:"
echo "1. Did the window open successfully?"
echo "2. Did you see the colors change after 5 seconds?"
echo "3. Did it say 'TEST COMPLETE!'?"
echo ""
echo "If YES to all three - Trinity can improve itself! 🎉"
echo "If NO to any - we need to debug further"
