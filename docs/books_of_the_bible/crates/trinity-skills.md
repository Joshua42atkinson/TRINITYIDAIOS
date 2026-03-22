# 📖 Book of trinity-skills

## Specifications
- **Crate**: `trinity-skills`
- **Version**: 
- **Description**: Trinity AI Agent System
- **Location**: `crates/trinity-skills`

## Technical Architecture
*(Auto-extracted from module level documentation)*

```rust
// Source: crates/trinity-skills/src/lib.rs
# Trinity Skills (The Specialists)
//!
## Philosophy (Architectonics)
"Skills are the Hands of Trinity. Each Skill is a specialist capable of one thing
 done excellently. The Orchestrator assigns tasks; Skills execute them."
//!
## User Preference: Pure Rust
This crate uses **pure Rust** implementations only:
- `candle-transformers` for diffusion models (no Python)
- `llama-cpp-2` bindings for LLM (no Python)
- ROCm/Vulkan GPU acceleration via native code
//!
## Instructions for Developers
1. **Single Responsibility**: Each Skill does ONE category of work well.
2. **Async by Default**: Skills may take seconds or minutes. Never block.
3. **Graceful Failure**: Skills return Result<T, E>. Never panic.
4. **No Python**: Use Rust crates or FFI bindings, never subprocess Python.
//!
## Modules
- `tools`: File and shell operations for self-coding
- `coder`: Code generation and execution
- `writer`: Document and content generation
- `media`: Image/video/audio generation (candle SDXL, future: Cosmos via Rust)
- `web`: Web scraping and research
//!
## NPU Notes (Strix Halo)
- LLM: FastFlowLM for XDNA 2 NPU (50 TOPS)
- Image: GPU (ROCm/CUDA) via candle-transformers
- TTS: Zonos working on ONNX for future NPU support
```

