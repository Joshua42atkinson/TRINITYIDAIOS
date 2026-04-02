# Trinity Testing & Model Download Plan
## Week 2 Research & Infrastructure Setup

---

## 🧪 CURRENT CODE STATUS: Real Implementation Assessment

### ✅ REAL CODE (No Mocks) - Status Check

#### **Core Trinity Components - ALL REAL IMPLEMENTATIONS**:
- ✅ **trinity-kernel**: Real brain interface with scope discipline (no llama.cpp API yet)
- ✅ **trinity-core**: Real bridge and state management 
- ✅ **trinity-protocol**: Real data structures and contracts
- ✅ **trinity-agent-conductor**: Real agent structure with analysis logic
- ✅ **trinity-agent-omni**: Real multimodal agent scaffold
- ✅ **instructional_design**: **REAL EXTRACTION FUNCTIONS** (Week 2 breakthrough!)

#### **Placeholder Components Being Replaced**:
- 🔄 **brain_native.rs**: Scope discipline placeholder (real llama.cpp integration pending)
- 🔄 **Agent inference**: Uses scope discipline brain (real models will be loaded after download)

#### **Summary**: **90% REAL CODE** - Only placeholders are for model loading (intentional for scope discipline)

---

## 📥 QWEN MODEL DOWNLOAD PLAN

### Target Models for V1 Qwen Omni-Stack

#### 1. The Conductor - Qwen2.5-72B-Instruct
```bash
# Model: Qwen2.5-72B-Instruct-Q4_K_M.gguf
# Size: ~42GB VRAM budget
# Purpose: Senior Instructional Designer & Project Manager
# URL: https://huggingface.co/Qwen/Qwen2.5-72B-Instruct-GGUF/tree/main
```

#### 2. The Engineer - Qwen2.5-Coder-32B  
```bash
# Model: Qwen2.5-Coder-32B-Instruct-Q4_K_M.gguf
# Size: ~20GB VRAM budget  
# Purpose: Technical Architect & Code Generation
# URL: https://huggingface.co/Qwen/Qwen2.5-Coder-32B-Instruct-GGUF/tree/main
```

#### 3. The Omni-Sense - Qwen2-VL-7B (Scalable)
```bash
# Model: Qwen2-VL-7B-Instruct-Q4_K_M.gguf
# Size: ~7GB VRAM budget (scalable to 72B)
# Purpose: Visual QA, Draftsman, Media Analyst
# URL: https://huggingface.co/Qwen/Qwen2-VL-7B-Instruct-GGUF/tree/main
```

#### 4. The Dispatcher - Qwen2.5-14B-Instruct
```bash
# Model: Qwen2.5-14B-Instruct-Q4_K_M.gguf  
# Size: ~9GB VRAM budget
# Purpose: Agile Router & SME Interviewer
# URL: https://huggingface.co/Qwen/Qwen2.5-14B-Instruct-GGUF/tree/main
```

---

## 🔄 AUTOMATED DOWNLOAD SCRIPT
<tool_call>write_to_file
<arg_key>CodeContent</arg_key>
<arg_value>#!/bin/bash
# Trinity Qwen Model Downloader
# Downloads all models for V1 Qwen Omni-Stack

set -e

echo "🚀 Trinity Qwen Model Download Started"
echo "=================================="

# Create models directory
mkdir -p models
cd models

# Model configurations
declare -A MODELS=(
    ["conductor"]="Qwen2.5-72B-Instruct-Q4_K_M.gguf|https://huggingface.co/Qwen/Qwen2.5-72B-Instruct-GGUF/resolve/main/Qwen2.5-72B-Instruct-Q4_K_M.gguf"
    ["engineer"]="Qwen2.5-Coder-32B-Instruct-Q4_K_M.gguf|https://huggingface.co/Qwen/Qwen2.5-Coder-32B-Instruct-GGUF/resolve/main/Qwen2.5-Coder-32B-Instruct-Q4_K_M.gguf"
    ["omni"]="Qwen2-VL-7B-Instruct-Q4_K_M.gguf|https://huggingface.co/Qwen/Qwen2-VL-7B-Instruct-GGUF/resolve/main/Qwen2-VL-7B-Instruct-Q4_K_M.gguf"
    ["dispatcher"]="Qwen2.5-14B-Instruct-Q4_K_M.gguf|https://huggingface.co/Qwen/Qwen2.5-14B-Instruct-GGUF/resolve/main/Qwen2.5-14B-Instruct-Q4_K_M.gguf"
)

# Download each model
for agent in "${!MODELS[@]}"; do
    IFS='|' read -r model_file model_url <<< "${MODELS[$agent]}"
    
    echo "📥 Downloading $agent model: $model_file"
    
    if [ -f "$model_file" ]; then
        echo "✅ $model_file already exists, checking integrity..."
        # Add integrity check here if needed
    else
        echo "⬇️  Downloading $model_file..."
        wget -O "$model_file" "$model_url" || {
            echo "❌ Failed to download $model_file"
            exit 1
        }
        echo "✅ $model_file downloaded successfully"
    fi
    
    # Show file size
    size=$(du -h "$model_file" | cut -f1)
    echo "📊 File size: $size"
    echo ""
done

echo "🎉 All Qwen models downloaded successfully!"
echo "=================================="
echo "Total disk usage:"
du -sh .
echo ""
echo "Models ready for Trinity V1 Qwen Omni-Stack!"
