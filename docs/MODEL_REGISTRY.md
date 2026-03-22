# Trinity Model Registry

**Purpose:** Single source of truth for AI model locations and launch config.
**Updated:** 2026-03-20

## Model Location

All models in:
```
~/trinity-models/
├── gguf/           # LLM weights (llama.cpp)
├── vision/         # mmproj vision projectors
├── music/          # AceStep music generation
└── (safetensors/)  # Diffusion models (if needed)
```

## Active Models

| Model | File | Size | Role | Status |
|-------|------|------|------|--------|
| **Mistral Small 4 119B** | `Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf` | 68GB (2 shards) | Primary LLM — Pete, orchestration, coding | ✅ Loads via llama-server |
| **Crow 9B** | `Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf` | ~6GB | Fast dev/testing model | ✅ Verified |
| **Qwen3.5-35B-A3B** | `Qwen3.5-35B-A3B-Q4_K_M.gguf` | ~20GB | Medium model option | Available |
| **Qwen3-Coder-REAP-25B** | `Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf` | ~15GB | Rust code specialist | Available |
| **Step-3.5-Flash-REAP-121B** | `Step-3.5-Flash-REAP-121B-A11B.Q4_K_S.gguf` | ~70GB | Large reasoning model | Available |
| **OmniCoder 9B** | `OmniCoder-9B-Q4_K_M.gguf` | ~6GB | Code assistant | Available |

### Vision Projectors

| File | Size | Compatible With |
|------|------|-----------------|
| `mmproj-Qwen3.5-35B-A3B-BF16.gguf` | ~861MB | Qwen3.5-35B-A3B |
| `mmproj-qwen3.5-35b-bf16.gguf` | ~861MB | Qwen3.5-35B (full) |

### Music

| File | Size | Role |
|------|------|------|
| `ace-step/acestep-1.7b-q8_0.gguf` | ~1.7GB | Music generation |
| `ace-step/ace-vae-bf16.gguf` | ~varies | Music VAE |

## Launch Config (Verified Working)

```bash
# Terminal 1: Start llama-server with RADV fix for Strix Halo
RADV_PERFMODE=nogttspill \
~/Workflow/desktop_trinity/trinity-genesis/llama.cpp/build-vulkan/bin/llama-server \
  --model ~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
  --n-gpu-layers 999 --no-mmap --port 8000 --ctx-size 262144 --flash-attn on -np 1

# Terminal 2: Start Trinity
LLM_URL=http://127.0.0.1:8000 cargo run -p trinity
```

### Strix Halo Environment

| Setting | Value | Purpose |
|---------|-------|---------|
| `RADV_PERFMODE` | `nogttspill` | Allow >64GB Vulkan allocations |
| `--no-mmap` | (flag) | Prevent mmap hangs on unified memory |
| `--flash-attn on` | (flag) | Reduce memory, faster attention |

### Kernel Parameters (GRUB)

```
iommu=pt amdgpu.gttsize=126976 ttm.pages_limit=33554432
```

## Removed Models (Historical)

| Model | Why Removed |
|-------|-------------|
| GPT-OSS-20B | Replaced by Mistral Small 4 |
| PersonaPlex-7B | Voice pipeline deferred |
| MiniMax-M2.5-REAP-50 | Not needed with Mistral |
| Opus-27B variants | Superseded |
| SDXL-Turbo | ComfyUI handles diffusion |
