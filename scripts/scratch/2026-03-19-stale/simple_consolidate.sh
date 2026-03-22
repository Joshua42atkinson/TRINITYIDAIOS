#!/bin/bash
# Simple Trinity Model Consolidation
set -e

echo "🗂️  Trinity Model Consolidation"
echo "=============================="

# Create consolidated structure
mkdir -p models-consolidated/{language_models/{conductor,analyst,dispatcher,yardmaster,draftsman,general},diffusion_models,vision_models,vocab_models,specialized}

# Move large language models
echo "📦 Moving Language Models..."

# Conductor models
if [ -f "models/conductor/Qwen3.5-97B-A10B-mmproj-BF16.gguf" ]; then
    mv models/conductor/Qwen3.5-97B-A10B-mmproj-BF16.gguf models-consolidated/language_models/conductor/
    echo "  ✅ Conductor: Qwen3.5-97B-A10B-mmproj-BF16.gguf"
fi

if [ -f "models/conductor/Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf" ]; then
    mv models/conductor/Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf models-consolidated/language_models/conductor/
    echo "  ✅ Conductor: Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf"
fi

if [ -f "models/conductor/Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf" ]; then
    mv models/conductor/Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf models-consolidated/language_models/conductor/
    echo "  ✅ Conductor: Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf"
fi

# Analyst models
if [ -f "models/analyst/Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled.i1-Q4_K_M.gguf" ]; then
    mv models/analyst/Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled.i1-Q4_K_M.gguf models-consolidated/language_models/analyst/
    echo "  ✅ Analyst: Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled.i1-Q4_K_M.gguf"
fi

# Dispatcher models
if [ -f "models/dispatcher/Fortytwo_Strand-Rust-Coder-14B-v1-Q4_K_M.gguf" ]; then
    mv models/dispatcher/Fortytwo_Strand-Rust-Coder-14B-v1-Q4_K_M.gguf models-consolidated/language_models/dispatcher/
    echo "  ✅ Dispatcher: Fortytwo_Strand-Rust-Coder-14B-v1-Q4_K_M.gguf"
fi

# Yardmaster models
if [ -f "models/yardmaster/Qwen3.5-97B-A10B-mmproj-BF16.gguf" ]; then
    mv models/yardmaster/Qwen3.5-97B-A10B-mmproj-BF16.gguf models-consolidated/language_models/yardmaster/
    echo "  ✅ Yardmaster: Qwen3.5-97B-A10B-mmproj-BF16.gguf"
fi

if [ -f "models/yardmaster/Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf" ]; then
    mv models/yardmaster/Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf models-consolidated/language_models/yardmaster/
    echo "  ✅ Yardmaster: Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf"
fi

if [ -f "models/yardmaster/Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf" ]; then
    mv models/yardmaster/Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf models-consolidated/language_models/yardmaster/
    echo "  ✅ Yardmaster: Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf"
fi

# Draftsman models
if [ -f "models/draftsman/Qwen3.5-35B-Instruct-Q4_K_M.gguf" ]; then
    mv models/draftsman/Qwen3.5-35B-Instruct-Q4_K_M.gguf models-consolidated/language_models/draftsman/
    echo "  ✅ Draftsman: Qwen3.5-35B-Instruct-Q4_K_M.gguf"
fi

# Main Qwen3.5-REAP-97B-A10B models
if [ -f "models/Qwen3.5-REAP-97B-A10B/Q4_K_M/Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf" ]; then
    mv models/Qwen3.5-REAP-97B-A10B/Q4_K_M/Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf models-consolidated/language_models/general/
    echo "  ✅ General: Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf"
fi

if [ -f "models/Qwen3.5-REAP-97B-A10B/Q4_K_M/Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf" ]; then
    mv models/Qwen3.5-REAP-97B-A10B/Q4_K_M/Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf models-consolidated/language_models/general/
    echo "  ✅ General: Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf"
fi

if [ -f "models/Qwen3.5-REAP-97B-A10B/Qwen3.5-97B-A10B-mmproj-BF16.gguf" ]; then
    mv models/Qwen3.5-REAP-97B-A10B/Qwen3.5-97B-A10B-mmproj-BF16.gguf models-consolidated/vision_models/
    echo "  ✅ Vision: Qwen3.5-97B-A10B-mmproj-BF16.gguf"
fi

# Move diffusion models
echo "🎨 Moving Diffusion Models..."
if [ -f "models/diffusion/sdxl_turbo/text_encoder/model.safetensors" ]; then
    mv models/diffusion/sdxl_turbo/text_encoder/model.safetensors models-consolidated/diffusion_models/
    echo "  ✅ Diffusion: text_encoder/model.safetensors"
fi

if [ -f "models/diffusion/sdxl_turbo/text_encoder_2/model.safetensors" ]; then
    mv models/diffusion/sdxl_turbo/text_encoder_2/model.safetensors models-consolidated/diffusion_models/
    echo "  ✅ Diffusion: text_encoder_2/model.safetensors"
fi

if [ -f "models/diffusion/sdxl_turbo/unet/dd/replaced.onnx" ]; then
    mv models/diffusion/sdxl_turbo/unet/dd/replaced.onnx models-consolidated/diffusion_models/unet_replaced.onnx
    echo "  ✅ Diffusion: unet_replaced.onnx"
fi

if [ -f "models/diffusion/sdxl_turbo/vae_decoder/dd/replaced.onnx" ]; then
    mv models/diffusion/sdxl_turbo/vae_decoder/dd/replaced.onnx models-consolidated/diffusion_models/vae_decoder_replaced.onnx
    echo "  ✅ Diffusion: vae_decoder_replaced.onnx"
fi

# Move specialized models
echo "🔧 Moving Specialized Models..."
if [ -f "models/nitrogen/ng.pt" ]; then
    mv models/nitrogen/ng.pt models-consolidated/specialized/
    echo "  ✅ Specialized: ng.pt"
fi

# Move vocab models
echo "📚 Moving Vocabulary Models..."
for vocab in llama.cpp/models/ggml-vocab-*.gguf; do
    if [ -f "$vocab" ]; then
        mv "$vocab" models-consolidated/vocab_models/
        echo "  ✅ Vocab: $(basename "$vocab")"
    fi
done

# Create model inventory
echo "📋 Creating Model Inventory..."
cat > models-consolidated/MODEL_INVENTORY.txt << 'EOF'
Trinity Model Inventory
======================

Language Models (Total: ~150GB)
├── conductor/
│   ├── Qwen3.5-97B-A10B-mmproj-BF16.gguf (870MB) - Vision projector
│   ├── Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf (29.6GB) - Part 1
│   └── Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf (25.1GB) - Part 2
├── analyst/
│   └── Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled.i1-Q4_K_M.gguf (15.4GB)
├── dispatcher/
│   └── Fortytwo_Strand-Rust-Coder-14B-v1-Q4_K_M.gguf (8.3GB)
├── yardmaster/
│   ├── Qwen3.5-97B-A10B-mmproj-BF16.gguf (870MB) - Vision projector
│   ├── Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf (29.6GB) - Part 1
│   └── Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf (25.1GB) - Part 2
├── draftsman/
│   └── Qwen3.5-35B-Instruct-Q4_K_M.gguf (20GB)
└── general/
    ├── Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf (29.6GB) - Part 1
    └── Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf (25.1GB) - Part 2

Diffusion Models (Total: ~3GB)
├── text_encoder/model.safetensors (469MB)
├── text_encoder_2/model.safetensors (2.5GB)
├── unet_replaced.onnx (44KB)
└── vae_decoder_replaced.onnx (1.7MB)

Vision Models (Total: ~870MB)
└── Qwen3.5-97B-A10B-mmproj-BF16.gguf (870MB)

Vocabulary Models (Total: ~50MB)
├── ggml-vocab-aquila.gguf (4.6MB)
├── ggml-vocab-baichuan.gguf (1.2MB)
├── ggml-vocab-bert-bge.gguf (613KB)
├── ggml-vocab-command-r.gguf (10.3MB)
├── ggml-vocab-deepseek-coder.gguf (1.1MB)
├── ggml-vocab-deepseek-llm.gguf (3.7MB)
├── ggml-vocab-falcon.gguf (2.1MB)
├── ggml-vocab-gpt-2.gguf (1.6MB)
├── ggml-vocab-gpt-neox.gguf (1.6MB)
├── ggml-vocab-llama-bpe.gguf (7.4MB)
├── ggml-vocab-llama-spm.gguf (707KB)
├── ggml-vocab-mpt.gguf (1.6MB)
├── ggml-vocab-nomic-bert-moe.gguf (6.5MB)
├── ggml-vocab-phi-3.gguf (709KB)
├── ggml-vocab-qwen2.gguf (5.6MB)
├── ggml-vocab-refact.gguf (1.6MB)
└── ggml-vocab-starcoder.gguf (1.6MB)

Specialized Models (Total: ~1.8GB)
└── ng.pt (1.8GB) - Nitrogen audio processing

Summary:
- Total Models: 30+ files
- Total Storage: ~156GB
- Primary Models: Qwen3.5-REAP-97B-A10B (56GB per copy)
- Backup Models: Yardmaster duplicates (56GB)
- Specialized: Diffusion, Vision, Audio, Vocab

Usage:
- Main: models-consolidated/language_models/conductor/
- Backup: models-consolidated/language_models/yardmaster/
- Vision: models-consolidated/vision_models/
- Diffusion: models-consolidated/diffusion_models/
EOF

# Calculate sizes
echo "📊 Calculating Sizes..."
llm_size=$(find models-consolidated/language_models -type f -exec ls -l {} \; 2>/dev/null | awk '{sum += $5} END {print sum+0}')
diff_size=$(find models-consolidated/diffusion_models -type f -exec ls -l {} \; 2>/dev/null | awk '{sum += $5} END {print sum+0}')
vis_size=$(find models-consolidated/vision_models -type f -exec ls -l {} \; 2>/dev/null | awk '{sum += $5} END {print sum+0}')
vocab_size=$(find models-consolidated/vocab_models -type f -exec ls -l {} \; 2>/dev/null | awk '{sum += $5} END {print sum+0}')
spec_size=$(find models-consolidated/specialized -type f -exec ls -l {} \; 2>/dev/null | awk '{sum += $5} END {print sum+0}')

total_size=$((llm_size + diff_size + vis_size + vocab_size + spec_size))

echo ""
echo "✅ Model Consolidation Complete!"
echo "================================"
echo "🧠 Language Models: $(echo "scale=1; $llm_size/1073741824" | bc 2>/dev/null || echo "?")GB"
echo "🎨 Diffusion Models: $(echo "scale=1; $diff_size/1073741824" | bc 2>/dev/null || echo "?")GB"
echo "👁️  Vision Models: $(echo "scale=1; $vis_size/1073741824" | bc 2>/dev/null || echo "?")GB"
echo "📚 Vocab Models: $(echo "scale=1; $vocab_size/1048576" | bc 2>/dev/null || echo "?")MB"
echo "🔧 Specialized: $(echo "scale=1; $spec_size/1073741824" | bc 2>/dev/null || echo "?")GB"
echo ""
echo "💾 Total Storage: $(echo "scale=1; $total_size/1073741824" | bc 2>/dev/null || echo "?")GB"
echo "📂 Unified Location: models-consolidated/"
echo ""
echo "🎯 Next Steps:"
echo "   1. Update Trinity config to use models-consolidated/ paths"
echo "   2. Test model loading with new locations"
echo "   3. Remove empty old directories after verification"
echo "   4. Update documentation with new paths"
