#!/bin/bash
# Trinity Model Consolidation Script
# Copyright (c) Joshua
# Shared under license for Ask_Pete (Purdue University)

set -e

echo "🗂️  Trinity Model Consolidation"
echo "=============================="
echo "Scanning and consolidating all models to unified location..."

# Create consolidated model directory structure
echo "📁 Creating unified model directory structure..."
mkdir -p models-consolidated/{language_models,diffusion_models,audio_models,vocab_models,vision_models,specialized}

# Function to calculate file size
get_size() {
    if [ -f "$1" ]; then
        stat -f%z "$1" 2>/dev/null || stat -c%s "$1" 2>/dev/null || echo "0"
    else
        echo "0"
    fi
}

# Function to format size
format_size() {
    local size=$1
    if [ "$size" -gt 1073741824 ]; then
        echo "$(echo "scale=1; $size/1073741824" | bc 2>/dev/null || echo "0")GB"
    elif [ "$size" -gt 1048576 ]; then
        echo "$(echo "scale=1; $size/1048576" | bc 2>/dev/null || echo "0")MB"
    elif [ "$size" -gt 1024 ]; then
        echo "$(echo "scale=1; $size/1024" | bc 2>/dev/null || echo "0")KB"
    else
        echo "${size}B"
    fi
}

# Scan for all models
echo "🔍 Scanning for all model files..."

# Create inventory file
echo "# Trinity Model Inventory - $(date)" > model_inventory.txt
echo "# Format: SOURCE_PATH -> DESTINATION_PATH | SIZE | TYPE" >> model_inventory.txt
echo "" >> model_inventory.txt

# Process large language models (>10MB)
echo "📦 Processing Large Language Models..."
find . -type f -name "*.gguf" -size +10M | grep -v ".git" | grep -v "target" | grep -v ".venv" | grep -v "build" | while read model; do
    if [ -f "$model" ]; then
        size=$(get_size "$model")
        formatted_size=$(format_size "$size")
        
        # Determine model category and destination
        if [[ "$model" == *"97B"* ]] || [[ "$model" == *"Qwen3.5"* ]]; then
            category="large_language_models"
            if [[ "$model" == *"conductor"* ]]; then
                dest="models-consolidated/language_models/conductor/$(basename "$model")"
            elif [[ "$model" == *"analyst"* ]]; then
                dest="models-consolidated/language_models/analyst/$(basename "$model")"
            elif [[ "$model" == *"dispatcher"* ]]; then
                dest="models-consolidated/language_models/dispatcher/$(basename "$model")"
            elif [[ "$model" == *"yardmaster"* ]]; then
                dest="models-consolidated/language_models/yardmaster/$(basename "$model")"
            elif [[ "$model" == *"draftsman"* ]]; then
                dest="models-consolidated/language_models/draftsman/$(basename "$model")"
            else
                dest="models-consolidated/language_models/general/$(basename "$model")"
            fi
        elif [[ "$model" == *"27B"* ]] || [[ "$model" == *"Claude"* ]]; then
            dest="models-consolidated/language_models/reasoning/$(basename "$model")"
            category="reasoning_models"
        elif [[ "$model" == *"14B"* ]] || [[ "$model" == *"Rust"* ]]; then
            dest="models-consolidated/language_models/coding/$(basename "$model")"
            category="coding_models"
        else
            dest="models-consolidated/language_models/other/$(basename "$model")"
            category="other_models"
        fi
        
        echo "$model -> $dest | $formatted_size | $category" >> model_inventory.txt
        echo "  📄 $category: $(basename "$model") ($formatted_size)"
    fi
done

# Process diffusion models
echo "🎨 Processing Diffusion Models..."
find . -type f \( -name "*.safetensors" -o -name "*.onnx" \) | grep -i "diffusion" | grep -v ".git" | grep -v "target" | while read model; do
    if [ -f "$model" ]; then
        size=$(get_size "$model")
        formatted_size=$(format_size "$size")
        
        dest="models-consolidated/diffusion_models/$(basename "$model")"
        echo "$model -> $dest | $formatted_size | diffusion_model" >> model_inventory.txt
        echo "  🎨 Diffusion: $(basename "$model") ($formatted_size)"
    fi
done

# Process vocabulary models
echo "📚 Processing Vocabulary Models..."
find ./llama.cpp/models -name "ggml-vocab-*.gguf" | while read model; do
    if [ -f "$model" ]; then
        size=$(get_size "$model")
        formatted_size=$(format_size "$size")
        
        dest="models-consolidated/vocab_models/$(basename "$model")"
        echo "$model -> $dest | $formatted_size | vocab_model" >> model_inventory.txt
        echo "  📚 Vocab: $(basename "$model") ($formatted_size)"
    fi
done

# Process vision models
echo "👁️  Processing Vision Models..."
find . -type f -name "*mmproj*" | grep -v ".git" | grep -v "target" | while read model; do
    if [ -f "$model" ]; then
        size=$(get_size "$model")
        formatted_size=$(format_size "$size")
        
        dest="models-consolidated/vision_models/$(basename "$model")"
        echo "$model -> $dest | $formatted_size | vision_model" >> model_inventory.txt
        echo "  👁️  Vision: $(basename "$model") ($formatted_size)"
    fi
done

# Process specialized models
echo "🔧 Processing Specialized Models..."
find . -type f \( -name "*.pt" -o -name "*.pth" -o -name "*.bin" \) -size +1M | grep -v ".git" | grep -v "target" | grep -v "build" | while read model; do
    if [ -f "$model" ]; then
        size=$(get_size "$model")
        formatted_size=$(format_size "$size")
        
        # Determine type based on name
        if [[ "$model" == *"ng"* ]]; then
            dest="models-consolidated/specialized/nitrogen/$(basename "$model")"
            type="specialized_ng"
        elif [[ "$model" == *"omni"* ]]; then
            dest="models-consolidated/specialized/omni/$(basename "$model")"
            type="specialized_omni"
        else
            dest="models-consolidated/specialized/other/$(basename "$model")"
            type="specialized_other"
        fi
        
        echo "$model -> $dest | $formatted_size | $type" >> model_inventory.txt
        echo "  🔧 Specialized: $(basename "$model") ($formatted_size)"
    fi
done

# Create directory structure
echo "🏗️  Creating directory structure..."
while IFS=" -> " read -r line; do
    if [[ "$line" == "#"* ]] || [[ "$line" == "" ]]; then
        continue
    fi
    
    source_path=$(echo "$line" | cut -d' ' -f1)
    dest_path=$(echo "$line" | cut -d' ' -f2)
    
    if [ -f "$source_path" ]; then
        mkdir -p "$(dirname "$dest_path")"
    fi
done < model_inventory.txt

# Calculate total sizes
echo ""
echo "📊 Model Size Summary by Category:"
echo "================================="

# Language models
llm_size=$(find models-consolidated/language_models -name "*.gguf" -exec ls -l {} \; 2>/dev/null | awk '{sum += $5} END {print sum+0}')
echo "🧠 Language Models: $(format_size $llm_size)"

# Diffusion models
diff_size=$(find models-consolidated/diffusion_models -type f -exec ls -l {} \; 2>/dev/null | awk '{sum += $5} END {print sum+0}')
echo "🎨 Diffusion Models: $(format_size $diff_size)"

# Vision models
vis_size=$(find models-consolidated/vision_models -type f -exec ls -l {} \; 2>/dev/null | awk '{sum += $5} END {print sum+0}')
echo "👁️  Vision Models: $(format_size $vis_size)"

# Vocab models
vocab_size=$(find models-consolidated/vocab_models -name "*.gguf" -exec ls -l {} \; 2>/dev/null | awk '{sum += $5} END {print sum+0}')
echo "📚 Vocab Models: $(format_size $vocab_size)"

# Specialized models
spec_size=$(find models-consolidated/specialized -type f -exec ls -l {} \; 2>/dev/null | awk '{sum += $5} END {print sum+0}')
echo "🔧 Specialized Models: $(format_size $spec_size)"

total_size=$((llm_size + diff_size + vis_size + vocab_size + spec_size))
echo ""
echo "💾 Total Model Storage: $(format_size $total_size)"

# Ask for confirmation before moving
echo ""
echo "🔄 Ready to consolidate models."
echo "   This will move all model files to the unified structure."
echo "   Original files will be preserved as symlinks."
echo ""
read -p "Continue with consolidation? (y/N): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🚀 Starting model consolidation..."
    
    # Move files and create symlinks
    moved_count=0
    total_size_moved=0
    
    while IFS=" -> " read -r line; do
        if [[ "$line" == "#"* ]] || [[ "$line" == "" ]]; then
            continue
        fi
        
        source_path=$(echo "$line" | cut -d' ' -f1)
        dest_path=$(echo "$line" | cut -d' ' -f2)
        size_str=$(echo "$line" | cut -d'|' -f2 | xargs)
        
        if [ -f "$source_path" ] && [ ! -e "$dest_path" ]; then
            # Create destination directory
            mkdir -p "$(dirname "$dest_path")"
            
            # Move the file
            mv "$source_path" "$dest_path"
            
            # Create symlink back to original location
            ln -s "$(realpath --relative-to="$(dirname "$source_path")" "$dest_path")" "$source_path"
            
            moved_count=$((moved_count + 1))
            echo "  ✅ Moved: $(basename "$source_path") -> $dest_path"
        fi
    done < model_inventory.txt
    
    echo ""
    echo "✅ Consolidation Complete!"
    echo "   📁 Models moved: $moved_count"
    echo "   💾 Total storage: $(format_size $total_size)"
    echo "   📂 Unified location: models-consolidated/"
    
else
    echo "❌ Consolidation cancelled."
    echo "   Inventory saved to: model_inventory.txt"
    echo "   Directory structure prepared in: models-consolidated/"
fi

# Create model registry
echo ""
echo "📋 Creating model registry..."
cat > models-consolidated/MODEL_REGISTRY.yaml << 'EOF'
# Trinity Model Registry
# Unified model inventory and metadata

categories:
  language_models:
    description: "Large Language Models for text generation and reasoning"
    subcategories:
      - conductor: "Main 97B educational processing models"
      - analyst: "27B reasoning and analysis models"
      - dispatcher: "14B code generation models"
      - yardmaster: "Backup 97B models"
      - draftsman: "35B instruction following models"
      - general: "Other language models"
      - reasoning: "Specialized reasoning models"
      - coding: "Code generation models"
      - other: "Uncategorized language models"
  
  diffusion_models:
    description: "Image generation and creative models"
    formats: [".safetensors", ".onnx"]
  
  vision_models:
    description: "Multimodal vision processing models"
    formats: [".gguf"]
  
  vocab_models:
    description: "Tokenizer vocabulary files"
    location: "llama.cpp/models/"
    formats: [".gguf"]
  
  specialized:
    description: "Specialized application models"
    subcategories:
      - nitrogen: "Audio processing models"
      - omni: "Multimodal models"
      - other: "Other specialized models"

usage:
  memory_budget: "128GB total system"
  recommended_allocation:
    language_models: "100GB"
    diffusion_models: "4GB"
    vision_models: "2GB"
    vocab_models: "1GB"
    specialized: "1GB"
  
loading_strategy:
  primary_models: "conductor/Qwen3.5-REAP-97B-A10B"
  backup_models: "yardmaster/"
  cache_frequently_used: true
  lazy_load_specialized: true

integration:
  trinity_kernel: "language_models/"
  trinity_body: "diffusion_models/"
  trinity_ui: "specialized/"
  llama_cpp: "vocab_models/"

last_updated: "$(date)"
total_models: "$(find models-consolidated -type f | wc -l)"
total_storage: "$(format_size $total_size)"
EOF

echo "✅ Model registry created: models-consolidated/MODEL_REGISTRY.yaml"
echo ""
echo "🎯 Next Steps:"
echo "   1. Review model_inventory.txt for complete inventory"
echo "   2. Update Trinity configuration to use models-consolidated/"
echo "   3. Remove old model directories after verification"
echo "   4. Update documentation with new model paths"
echo ""
echo "📚 Documentation:"
echo "   📄 Inventory: model_inventory.txt"
echo "   📋 Registry: models-consolidated/MODEL_REGISTRY.yaml"
echo "   🗂️  Models: models-consolidated/"
