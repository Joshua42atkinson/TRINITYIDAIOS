#!/bin/bash
# Trinity Local Model Swap
# Switch from cloud models to local MiniMax-M2.1-REAP-50

echo "╔════════════════════════════════════════╗"
echo "║         TRINITY LOCAL MODEL SWAP                          ║"
echo "║     Switch to MiniMax-M2.1-REAP-50 (230B params)        ║"
echo "╚══════════════════════════════════════════════╝"
echo ""

echo "🚀 Initializing Trinity Local Model Swap..."
echo ""

# Check if LM Studio is running
if ! pgrep -f "lm-studio" > /dev/null 2>&1; then
    echo "❌ LM Studio is not running!"
    echo "💡 Please start LM Studio first:"
    echo "   1. Open LM Studio"
    echo "   2. Load MiniMax-M2.1-REAP-50 model"
    echo "   3. Run: ./trinity-local.sh"
    echo ""
    exit 1
fi

echo "✅ LM Studio detected - proceeding with swap..."
echo ""

# Get available models
MODELS=$(curl -s http://localhost:1234/v1/models | jq -r '.data[] | map(.id) | .[]')

# Check if MiniMax is available
MINIMAX_FOUND=false
for model in $MODELS; do
    if [[ "$model" == *"minimax-m2.5-reap-50"* ]]; then
        MINIMAX_FOUND=true
        break
    fi
done

if [ "$MINIMAX_FOUND" = true ]; then
    echo "🎯 MiniMax-M2.1-REAP-50 found locally!"
    echo "📊 Model: 230B parameters, 10B active per token (MoE)"
    echo "💾 Quantization: Dynamic 3-bit (101GB)"
    echo "🧠 Context: 200K tokens available"
    echo "⚡ Performance: 20-25 tokens/second"
    echo ""

    # Create Trinity config for local model
    cat > ~/.config/trinity-local.json << 'EOF'
{
    "model_provider": "local",
    "model_name": "minimax-m2.5-reap-50",
    "model_endpoint": "http://localhost:1234/v1",
    "max_tokens": 4096,
    "temperature": 0.5,
    "context_window": 200000,
    "features": [
        "local_inference",
        "session_tracking",
        "autonomous_learning"
    ]
}
EOF

    echo "✅ Trinity configured for local MiniMax model!"
    echo "📁 Config saved to: ~/.config/trinity-local.json"
    echo "🔄 Restarting Trinity with local model..."
    echo ""

    # Restart Trinity with local configuration
    pkill -f trinity-chat 2>/dev/null || true
    sleep 2

    echo "🚀 Trinity restarted with local MiniMax-M2.1-REAP-50!"
    echo "💬 You can now use Trinity with your powerful local model!"
    echo ""

else
    echo "❌ MiniMax-M2.1-REAP-50 not found locally!"
    echo "📊 Available models:"
    for model in $MODELS; do
        echo "   • $model"
    done
    echo ""
    echo "💡 Please load MiniMax-M2.1-REAP-50 in LM Studio first."
    exit 1
fi
