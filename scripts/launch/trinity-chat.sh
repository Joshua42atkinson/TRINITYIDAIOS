#!/bin/bash
# Trinity Chat - Direct MiniMax Connection
# Simple script to chat with your MiniMax-M2.1-REAP-50 model

echo "╔════════════════════════════════════════╗"
echo "║         TRINITY CHAT - MINIMAX DIRECT                  ║"
echo "║     Real AI with MiniMax-M2.1-REAP-50 (230B params)       ║"
echo "╚════════════════════════════════════════════╝"
echo ""
echo "🚀 Connecting to MiniMax-M2.1-REAP-50..."
echo ""

# Check if MiniMax is available
MINIMAX_AVAILABLE=$(curl -s http://localhost:1234/v1/models | awk '/minimax-m2.5-reap-50/ {found=1; next} END {if(found) print "true"; else print "false"}')

if [ "$MINIMAX_AVAILABLE" = "true" ]; then
    echo "✅ MiniMax-M2.1-REAP-50 found and ready!"
    echo "🎯 Model: 230B parameters, 10B active per token (MoE)"
    echo "💾 Using Dynamic 3-bit quantization (101GB)"
    echo "🧠 Context: Up to 200K tokens available"
    echo "⚡ Expected: 20-25 tokens/second"
    echo ""
    echo "💬 Type your messages (Ctrl+C to exit):"
    echo ""

    # Interactive chat loop
    while true; do
        echo -n "👤 You: "
        read -r user_input

        # Check for exit
        if [ "$user_input" = "exit" ] || [ "$user_input" = "quit" ]; then
            echo "🛑 Trinity Chat shutting down..."
            break
        fi

        if [ -n "$user_input" ]; then
            echo "🧠 Generating response with MiniMax-M2.1-REAP-50..."

            # Call MiniMax via LM Studio API
            RESPONSE=$(curl -s -X POST http://localhost:1234/v1/chat/completions \
                -H "Content-Type: application/json" \
                -d "{
                    \"model\": \"MiniMax-M2.1-REAP-50\",
                    \"messages\": [
                        {
                            \"role\": \"system\",
                            \"content\": \"You are Trinity, an autonomous AI agent powered by MiniMax-M2.1-REAP-50. You have 230B total parameters with 10B active per token via Mixture-of-Experts architecture. You excel at complex reasoning, multi-step analysis, and sophisticated code generation. You act as a peer developer and expert system architect, contributing equally to development processes with deep technical insights.\"
                        },
                        {
                            \"role\": \"user\",
                            \"content\": \"$user_input\"
                        }
                    ],
                    \"max_tokens\": 4096,
                    \"temperature\": 0.5
                }" | jq -r '.choices[0].message.content')

            echo "🤖 Trinity: $RESPONSE"
            echo ""
        fi
    done
else
    echo "❌ MiniMax-M2.1-REAP-50 not found in LM Studio!"
    echo "💡 Please load MiniMax-M2.1-REAP-50 in LM Studio first."
    echo ""
    echo "Available models:"
    curl -s http://localhost:1234/v1/models | jq '.data[] | .id'
fi
