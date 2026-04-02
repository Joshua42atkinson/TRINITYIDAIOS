#!/bin/bash

# Organize models in Trinity project structure

echo "🗂️ Organizing models for Trinity..."

# Create real model directories
mkdir -p models/real/{general,coding,creative,multimodal}

# MiniMax model - General purpose (66GB)
MINIMAX_SOURCE="/home/joshua/.lmstudio/models/RemySkye/MiniMax-M2.5-REAP-50-GGUF/MiniMax-M2-5-REAP-50-Q4_K_M.gguf"
MINIMAX_DEST="models/real/general/MiniMax-M2.5-REAP-50-Q4_K_M.gguf"

if [ -f "$MINIMAX_SOURCE" ]; then
    echo "📦 Moving MiniMax model (66GB)..."
    # Create symlink instead of copying to save space
    ln -sf "$MINIMAX_SOURCE" "$MINIMAX_DEST"
    echo "✅ MiniMax linked to general/"
else
    echo "❌ MiniMax model not found"
fi

# Qwen3.5-35B model - Multimodal (20GB)
QWEN_SOURCE="/home/joshua/.lmstudio/models/lmstudio-community/Qwen3.5-35B-A3B-GGUF/Qwen3.5-35B-A3B-Q4_K_M.gguf"
QWEN_DEST="models/real/multimodal/Qwen3.5-35B-A3B-Q4_K_M.gguf"

if [ -f "$QWEN_SOURCE" ]; then
    echo "📦 Moving Qwen3.5-35B model (20GB)..."
    ln -sf "$QWEN_SOURCE" "$QWEN_DEST"
    echo "✅ Qwen3.5-35B linked to multimodal/"
else
    echo "❌ Qwen3.5-35B model not found"
fi

# Create model configuration file
cat > models/real/model_config.yaml << EOF
# Trinity Model Configuration
models:
  minimax:
    path: models/real/general/MiniMax-M2.5-REAP-50-Q4_K_M.gguf
    size_gb: 66
    type: general
    description: "50B parameter REAP model for general tasks"
    vram_gb: 48
    context_size: 8192
    
  qwen35b:
    path: models/real/multimodal/Qwen3.5-35B-A3B-Q4_K_M.gguf
    size_gb: 20
    type: multimodal
    description: "35B parameter multimodal model"
    vram_gb: 16
    context_size: 8192

subagent_assignments:
  conductor: minimax
  dispatcher: minimax
  draftsman: qwen35b
  engineer: minimax
  yardmaster: minimax
  brakeman: minimax
  diffusion: qwen35b
  nitrogen: minimax
  omni: qwen35b
EOF

echo ""
echo "📊 Model Organization Complete:"
echo "  - models/real/general/ - General purpose models"
echo "  - models/real/multimodal/ - Multimodal models"
echo "  - models/real/coding/ - Coding specialized models"
echo "  - models/real/creative/ - Creative task models"
echo ""
echo "🚀 Models organized and ready!"
