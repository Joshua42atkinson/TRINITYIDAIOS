#!/bin/bash
# Trinity HuggingFace Model Data Extraction Workflow
# Pulls complete model information for all Trinity models

set -e

echo "🔍 Trinity HuggingFace Model Data Extraction"
echo "============================================"

# Create data directory
mkdir -p huggingface_data/{model_cards,configs,examples,specs}
cd huggingface_data

# List of all Trinity models with their HuggingFace URLs
declare -A MODELS=(
    # Voice Models
    ["personaplex-7b"]="nvidia/personaplex-7b-v1"
    
    # Language Models
    ["qwen-97b-conductor"]="Qwen/Qwen2.5-72B-Instruct"
    ["qwen-27b-analyst"]="Qwen/Qwen2.5-27B-Instruct"
    ["qwen-9b-thinking"]="Qwen/Qwen2.5-9B-Instruct"
    ["rust-coder-14b"]="Fortytwo-AI/Fortytwo_Strand-Rust-Coder-14B-v1"
    ["qwen-35b-instruct"]="Qwen/Qwen2.5-32B-Instruct"
    
    # Vision Models
    ["qwen-vision-projector"]="Qwen/Qwen2-VL-72B"
    
    # Diffusion Models
    ["sdxl-turbo"]="stabilityai/sdxl-turbo"
    
    # Embedding Models
    ["nomic-embed"]="nomic-ai/nomic-embed-text-v1.5"
    
    # Tokenizer Models (representative)
    ["qwen2-tokenizer"]="Qwen/Qwen2.5-72B-Instruct"
    ["llama-tokenizer"]="meta-llama/Meta-Llama-3.1-8B-Instruct"
    ["falcon-tokenizer"]="tiiuae/falcon-7b"
)

echo "📥 Extracting model data from HuggingFace..."

# Function to extract model data
extract_model_data() {
    local model_name="$1"
    local hf_repo="$2"
    local output_dir="$3"
    
    echo "  📥 Extracting: $model_name ($hf_repo)"
    
    # Create model directory
    mkdir -p "$output_dir/$model_name"
    
    # Get model card
    echo "    📄 Getting model card..."
    curl -s "https://huggingface.co/api/models/$hf_repo" | jq '.' > "$output_dir/$model_name/api_info.json"
    
    # Get README/model card
    echo "    📖 Getting README..."
    curl -s "https://huggingface.co/$hf_repo/raw/main/README.md" > "$output_dir/$model_name/README.md" 2>/dev/null || echo "README not found" > "$output_dir/$model_name/README.md"
    
    # Get config
    echo "    ⚙️ Getting config..."
    curl -s "https://huggingface.co/$hf_repo/raw/main/config.json" > "$output_dir/$model_name/config.json" 2>/dev/null || echo "Config not found" > "$output_dir/$model_name/config.json"
    
    # Get tokenizer config
    echo "    🗣️ Getting tokenizer config..."
    curl -s "https://huggingface.co/$hf_repo/raw/main/tokenizer_config.json" > "$output_dir/$model_name/tokenizer_config.json" 2>/dev/null || echo "Tokenizer config not found" > "$output_dir/$model_name/tokenizer_config.json"
    
    # Get model card (if available)
    echo "    🎴 Getting model card..."
    curl -s "https://huggingface.co/$hf_repo/raw/main/model_card.md" > "$output_dir/$model_name/model_card.md" 2>/dev/null || echo "Model card not found" > "$output_dir/$model_name/model_card.md"
    
    # Get usage examples from files
    echo "    💡 Checking for usage examples..."
    curl -s "https://huggingface.co/$hf_repo/raw/main/example.py" > "$output_dir/$model_name/example.py" 2>/dev/null || echo "Python example not found" > "$output_dir/$model_name/example.py"
    curl -s "https://huggingface.co/$hf_repo/raw/main/example.txt" > "$output_dir/$model_name/example.txt" 2>/dev/null || echo "Text example not found" > "$output_dir/$model_name/example.txt"
    
    echo "    ✅ Complete: $model_name"
}

# Extract data for all models
for model_name in "${!MODELS[@]}"; do
    extract_model_data "$model_name" "${MODELS[$model_name]}" "model_cards"
done

echo ""
echo "🔍 Getting additional model information..."

# Get model lists and categories
echo "📋 Getting model categories..."
curl -s "https://huggingface.co/api/models" | jq '.[:10]' > "specs/recent_models.json"

# Get specific model information with filters
echo "🎯 Getting task-specific models..."

# Text-to-Text models
echo "  📝 Text-to-Text models..."
curl -s "https://huggingface.co/api/models?pipeline=text-generation&limit=20" | jq '.' > "specs/text_generation_models.json"

# Text-to-Speech models  
echo "  🎤 Text-to-Speech models..."
curl -s "https://huggingface.co/api/models?pipeline=text-to-speech&limit=10" | jq '.' > "specs/text_to_speech_models.json"

# Speech-to-Text models
echo "  🎧 Speech-to-Text models..."
curl -s "https://huggingface.co/api/models?pipeline=automatic-speech-recognition&limit=10" | jq '.' > "specs/speech_to_text_models.json"

# Image-to-Text models
echo "  👁️ Image-to-Text models..."
curl -s "https://huggingface.co/api/models?pipeline=image-to-text&limit=10" | jq '.' > "specs/image_to_text_models.json"

# Text-to-Image models
echo "  🎨 Text-to-Image models..."
curl -s "https://huggingface.co/api/models?pipeline=text-to-image&limit=10" | jq '.' > "specs/text_to_image_models.json"

echo ""
echo "📊 Generating comprehensive model report..."

# Generate comprehensive report
cat > "TRINITY_MODEL_REPORT.md" << 'EOF'
# 🔍 Trinity HuggingFace Model Data Report
## **Complete Model Information Extracted from HuggingFace**

---

## 📊 **Model Portfolio Overview**

This report contains complete information extracted directly from HuggingFace for all Trinity models.

EOF

# Add model information to report
for model_name in "${!MODELS[@]}"; do
    hf_repo="${MODELS[$model_name]}"
    
    cat >> "TRINITY_MODEL_REPORT.md" << EOF

### 📋 $model_name
**HuggingFace:** [$hf_repo](https://huggingface.co/$hf_repo)

EOF
    
    # Extract key information from API
    if [ -f "model_cards/$model_name/api_info.json" ]; then
        model_id=$(jq -r '.id // "Unknown"' "model_cards/$model_name/api_info.json")
        model_type=$(jq -r '.pipeline_tag // "Unknown"' "model_cards/$model_name/api_info.json")
        downloads=$(jq -r '.downloads // 0' "model_cards/$model_name/api_info.json")
        likes=$(jq -r '.likes // 0' "model_cards/$model_name/api_info.json")
        
        cat >> "TRINITY_MODEL_REPORT.md" << EOF
- **Model ID:** $model_id
- **Type:** $model_type
- **Downloads:** $downloads
- **Likes:** $likes

EOF
    fi
    
    # Add README excerpt
    if [ -f "model_cards/$model_name/README.md" ] && [ -s "model_cards/$model_name/README.md" ]; then
        echo "**README Excerpt:**" >> "TRINITY_MODEL_REPORT.md"
        echo '```' >> "TRINITY_MODEL_REPORT.md"
        head -20 "model_cards/$model_name/README.md" >> "TRINITY_MODEL_REPORT.md"
        echo '```' >> "TRINITY_MODEL_REPORT.md"
        echo "" >> "TRINITY_MODEL_REPORT.md"
    fi
    
    echo "---" >> "TRINITY_MODEL_REPORT.md"
    echo "" >> "TRINITY_MODEL_REPORT.md"
done

echo "✅ Model data extraction complete!"
echo ""
echo "📂 Generated Files:"
echo "  - huggingface_data/TRINITY_MODEL_REPORT.md (comprehensive report)"
echo "  - huggingface_data/model_cards/ (individual model data)"
echo "  - huggingface_data/spec/ (model categories and types)"
echo ""
echo "🎯 Next Steps:"
echo "  1. Review the comprehensive model report"
echo "  2. Update Trinity model cards with real data"
echo "  3. Create usage examples from extracted information"
echo "  4. Optimize model selection based on actual capabilities"
