# 🤖 Trinity AI Model Inventory

## **Model Storage Location**
All models stored in: `models/` directory

## **🎯 Primary Trinity Brain (97B Models)**

### **Main 97B Model**
- **Path**: `models/Qwen3.5-REAP-97B-A10B/Q4_K_M/`
- **Files**: 
  - `Qwen3.5-REAP-97B-A10B-Q4_K_M-00001-of-00002.gguf`
  - `Qwen3.5-REAP-97B-A10B-Q4_K_M-00002-of-00002.gguf`
- **Size**: ~55GB total
- **Purpose**: Primary Trinity reasoning engine
- **Format**: GGUF Q4_K_M (4-bit quantized)

### **Multimodal Projector**
- **Path**: `models/Qwen3.5-REAP-97B-A10B/mmproj-BF16.gguf`
- **Size**: ~335MB
- **Purpose**: Vision-language projection
- **Format**: BF16

### **Backup Copies**
- **Path**: `models/yardmaster/`
- **Contents**: Duplicate 97B model files
- **Purpose**: Redundancy for different agents

## **🤖 Specialist Agent Models**

### **NEW: Analyst Agent** 🆕
- **Path**: `models/analyst/Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled.i1-Q4_K_M.gguf`
- **Size**: 16.5GB
- **Added**: March 8, 2026
- **Purpose**: Advanced analysis and reasoning
- **Specialty**: Claude-style reasoning capabilities

### **Conductor Agent**
- **Path**: `models/conductor/`
- **Model**: Qwen3.5-REAP-97B-A10B-Q4_K_M
- **Purpose**: Orchestration and planning
- **Size**: ~55GB

### **Draftsman Agent**
- **Path**: `models/draftsman/Qwen3.5-35B-Instruct-Q4_K_M.gguf`
- **Size**: ~20GB
- **Purpose**: Content creation and drafting
- **Specialty**: Instruction following

### **Dispatcher Agent**
- **Path**: `models/dispatcher/Fortytwo_Strand-Rust-Coder-14B-v1-Q4_K_M.gguf`
- **Size**: ~8GB
- **Purpose**: Code task distribution
- **Specialty**: Rust code generation

### **Code Generation Models**
- **Path**: `models/nitrogen/`
- **Contents**: Various code generation models
- **Purpose**: Specialized coding tasks

### **Image Generation Models**
- **Path**: `models/diffusion/`
- **Contents**: Stable Diffusion models
- **Purpose**: Visual content creation

## **📊 Storage Statistics**

### **Total Model Storage**
- **Primary Models**: ~55GB (97B + projector)
- **Specialist Models**: ~45GB
- **Backups**: ~55GB
- **Total**: ~155GB

### **Model Formats**
- **GGUF**: All models (CPU-optimized)
- **Quantization**: Q4_K_M (4-bit, balanced quality/size)
- **Compatibility**: llama.cpp, rust-cpp, trinity-llama

## **🔧 Model Usage in Trinity**

### **Current Integration**
- 97B model referenced in ProductionBrain
- Model paths configured in various crates
- Loading mechanisms in place

### **Model Loading Code**
```rust
// Example model loading
let model_path = "models/Qwen3.5-REAP-97B-A10B/Q4_K_M/";
let model = LlamaModel::from_file(model_path)?;
```

### **Agent Model Assignment**
```rust
// Agent-specific models
match agent_type {
    AgentType::Conductor => "models/conductor/",
    AgentType::Analyst => "models/analyst/",
    AgentType::Draftsman => "models/draftsman/",
    AgentType::Dispatcher => "models/dispatcher/",
}
```

## **🚀 Model Management**

### **Adding New Models**
1. Download model to Downloads/
2. Create appropriate folder in models/
3. Move model file to folder
4. Update Technical Bible
5. Configure in Trinity if needed

### **Model Verification**
```bash
# Check model exists
ls -la models/analyst/

# Verify model format
file models/analyst/*.gguf

# Test loading (when Trinity ready)
cargo run --example test_model_loading
```

## **📝 Notes**
- All models are quantized for CPU inference
- No GPU required for basic operation
- Models can be loaded dynamically per agent
- Storage space: ensure adequate disk space (~200GB recommended)

---
*Last updated: March 8, 2026*
*Total models: 10+ across 6 agent types*
