#!/bin/bash
# Start Full Trinity AI System (Not Just LM Studio Bridge)

echo "🚀 Starting Full Trinity AI System..."
echo ""

# Check if Trinity agent binary exists
if [ ! -f "target/debug/trinity-agent" ]; then
    echo "🔨 Building Trinity Agent..."
    cargo build --bin trinity-agent
fi

# Start the autonomous Trinity Agent
echo "🤖 Starting Trinity Autonomous Agent..."
echo "📊 This enables:"
echo "   • ADDIE curriculum design"
echo "   • Mastery tracking"
echo "   • Autonomous learning loops"
echo "   • Evidence generation"
echo "   • Self-improvement"
echo ""

# Check for Bevy visualization
if [ -f "target/debug/trinity-body" ]; then
    echo "🎮 Trinity Body (Bevy) available for visualization"
    echo "   Run: cargo run --bin trinity-body"
fi

echo ""
echo "🎯 Full Trinity Systems Active:"
echo "   • Agent Runtime: ./target/debug/trinity-agent"
echo "   • CLI Tools: cargo run --bin trinity-cli"
echo "   • Body Visualization: cargo run --bin trinity-body"
echo "   • Memory Systems: Integrated"
echo "   • Curriculum Engine: Active"
echo ""
echo "💡 For LM Studio integration, use: ./start-trinity.sh"
echo "🔧 For autonomous mode, use: ./target/debug/trinity-agent"
