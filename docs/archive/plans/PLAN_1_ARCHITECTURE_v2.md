# PLAN 1 v2: Architecture — vLLM as Sole Inference Engine
## Trinity ID AI OS — Unified Inference via vLLM

*Supersedes PLAN_1_ARCHITECTURE.md (llama.cpp + vLLM hybrid)*

---

## 1. The Core Simplification

**Before (v1):** Two inference engines — llama-server for GGUF, vLLM for safetensors.  
**After (v2):** One inference engine — vLLM serves EVERYTHING.

### Why This Is Better

| Concern | v1 (Hybrid) | v2 (vLLM Only) |
|---------|-------------|-----------------|
| Inference engines to maintain | 2 (llama.cpp + vLLM) | 1 (vLLM) |
| API protocols | 2 (OpenAI-compat + Ming custom) | 1 (OpenAI-compat for all, Ming custom for talker) |
| Model hot-swap complexity | Must track which engine owns which model | Single engine, single swap protocol |
| Continuous batching | Only vLLM | Everything |
| Diffusion support | None | vLLM diffusion pipeline (Layer 3 expansion) |
| Bevy ECS harmony | Awkward — two async patterns | Natural — batch processing aligns with ECS tick |
| Python isolation | PyO3 only for Ming/ART | PyO3 for all inference (clean single boundary) |
| Scalability to multi-GPU/cluster | Fragmented | vLLM's tensor parallelism works across all models |

---

## 2. Unified Inference Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    TRINITY MAIN SERVER (:3000)                  │
│                         Axum / Rust                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  All inference routes → single vLLM endpoint                   │
│                                                                 │
│  LLAMA_URL removed. Replaced with:                             │
│  VLLM_URL = http://127.0.0.1:8000                             │
│                                                                 │
│  Every model served via /v1/chat/completions                   │
│  (Ming talker uses custom /generate for audio)                 │
│                                                                 │
└──────────────────────────┬──────────────────────────────────────┘
                           │
              ┌────────────▼────────────────────────┐
              │     vLLM ENGINE (Python, :8000)      │
              │                                      │
              │  Serves ALL models:                  │
              │  • Mistral Small 4 (GGUF, merged)    │
              │  • Ming-flash-omni-2.0 (safetensors) │
              │  • Crow 9B (GGUF)                    │
              │  • REAP 25B (GGUF)                   │
              │  • OmniCoder 9B (GGUF)               │
              │  • Reserve models on demand           │
              │                                      │
              │  API: OpenAI-compatible               │
              │  /v1/chat/completions                 │
              │  /v1/models                           │
              │                                      │
              │  Continuous batching                  │
              │  KV cache management                  │
              │  Tensor parallelism ready             │
              │                                      │
              │  FUTURE: Diffusion pipeline           │
              │  /v1/images/generations (SDXL, etc.)  │
              └──────────────────────────────────────┘
```

---

## 3. Model Serving Strategy

### 3.1 GGUF Models (merge to single file first)

vLLM requires single-file GGUFs. One-time merge operation:

```bash
# Merge Mistral Small 4 split GGUF into single file
gguf-split --merge \
  ~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
  ~/trinity-models/gguf/Mistral-Small-4-119B-2603-merged.gguf
```

All other GGUF models are already single-file.

### 3.2 Serving Commands

```bash
# Pete (Conductor) — always loaded
vllm serve ~/trinity-models/gguf/Mistral-Small-4-119B-2603-merged.gguf \
  --tokenizer mistralai/Mistral-Small-4-Base \
  --port 8000 \
  --gpu-memory-utilization 0.5

# Swap to Crow 9B (Research)
vllm serve ~/trinity-models/gguf/Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf \
  --tokenizer Qwen/Qwen3.5-7B \
  --port 8001

# Ming (Yardmaster) — safetensors, native vLLM
vllm serve ~/trinity-models/safetensors/Ming-flash-omni-2.0 \
  --trust-remote-code \
  --port 8002 \
  --gpu-memory-utilization 0.8
```

### 3.3 Hotel Pattern (Model Swapping)

Only ONE heavyweight model loaded at a time. The Conductor manages swaps:

| Phase | Active Model | Port | Memory |
|-------|-------------|------|--------|
| Analysis, Review, Assessment, Yield, Execution | Pete (Mistral Small 4) | 8000 | ~68GB |
| Development, Implementation, Evaluation, Correction | Ming (Yardmaster) | 8000 | ~needs vLLM offloading |
| Design, Extension | ART models (Crow/REAP/OmniCoder) | 8001 | 5-15GB |

---

## 4. Layer Expansion via vLLM

### Layer 1: Headless Server
- vLLM serves Pete for audio-only Iron Road narration
- Ming handles omni-modal (text+audio+vision) worldbuilding
- All via OpenAI-compatible API

### Layer 2: Web UI + 2D/3D Bevy + eLearning
- Same vLLM API serves the web UI chat
- Bevy games call vLLM for NPC dialogue, quest generation
- React/Vite lesson plans generated by Pete via API
- vLLM's continuous batching handles concurrent requests from all frontends

### Layer 3: XR Sandbox
- vLLM diffusion pipeline for real-time texture/asset generation
- Bevy ECS ticks align with vLLM batch inference cycles
- XR interactions → vLLM inference → world state updates
- This is where the Bevy ECS + vLLM batching harmony really shines

---

## 5. PyO3 Sidecar Pattern (Simplified)

With vLLM as sole engine, the PyO3 boundary becomes cleaner:

```
┌──────────────────────────────────────┐
│        SIDECAR PROCESS (Rust)        │
│                                      │
│  ┌────────────────────────────────┐  │
│  │  Axum API (:8090)              │  │
│  │  Quest management              │  │
│  │  Tool execution                │  │
│  └──────────┬─────────────────────┘  │
│             │                        │
│  ┌──────────▼─────────────────────┐  │
│  │  PyO3 Bridge                   │  │
│  │                                │  │
│  │  • vLLM engine management      │  │
│  │    (start/stop/swap models)    │  │
│  │  • ComfyUI workflow dispatch   │  │
│  │  • Blender script execution    │  │
│  │  • MusicUI generation          │  │
│  │  • Ming talker (audio I/O)     │  │
│  │                                │  │
│  │  ALL Python lives here.        │  │
│  └────────────────────────────────┘  │
└──────────────────────────────────────┘
```

---

## 6. What Changes From v1

### Code Changes Required

| File | Change |
|------|--------|
| `main.rs` | Replace `LLAMA_URL` with `VLLM_URL` as the sole inference endpoint |
| `inference.rs` | Point to vLLM instead of llama-server (same OpenAI-compat API, minimal change) |
| `tools.rs` | Remove `llama_server_binary()`, `conductor-llama` launch. Replace with vLLM management via PyO3 |
| `conductor_leader.rs` | `call_pete()` already uses HTTP — just change the URL |
| `vllm_batcher.rs` | Already talks OpenAI-compat — becomes the primary client |
| `pete_core.rs` | Already uses `VllmEngineClient` — no change needed |

### Infrastructure Required

1. **Install PyTorch with ROCm** for AMD 395+
2. **Install vLLM from source** with ROCm backend
3. **Merge Mistral Small 4 split GGUF** into single file (one-time)
4. **Test vLLM with each model** before wiring into Trinity

### What Gets Deleted

- `llama_server_binary()` function in `tools.rs`
- `LLAMA_URL` env var (replaced by `VLLM_URL`)
- Any references to llama-server launch commands
- The `bin/llama-server` binary dependency

---

## 7. Risk Assessment

| Risk | Mitigation |
|------|------------|
| vLLM GGUF is "experimental and under-optimized" | Keep llama-server binary as fallback. Can always revert. |
| vLLM ROCm support for AMD 395+ may have issues | Strix Halo is new silicon — test thoroughly before committing |
| Single-file GGUF merge may fail for MoE models | Test merge first. If fails, convert to safetensors instead. |
| vLLM startup is slower than llama-server | Acceptable — models stay loaded for long sessions |

---

## 8. Prerequisite: vLLM + ROCm Installation

This is now **Phase 0** of Plan 3 — must happen before anything else.

```bash
# 1. Install PyTorch with ROCm
pip3 install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/rocm6.2

# 2. Install vLLM from source
pip3 install vllm

# 3. Verify
python3 -c "import torch; print(torch.cuda.is_available()); print(torch.version.hip)"
vllm serve --help

# 4. Merge Mistral Small 4 GGUF
# (requires gguf-split from llama.cpp)
gguf-split --merge \
  ~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
  ~/trinity-models/gguf/Mistral-Small-4-119B-2603-merged.gguf

# 5. Test Pete
vllm serve ~/trinity-models/gguf/Mistral-Small-4-119B-2603-merged.gguf \
  --tokenizer mistralai/Mistral-Small-4-Base \
  --port 8000

# 6. Test Ming
vllm serve ~/trinity-models/safetensors/Ming-flash-omni-2.0 \
  --trust-remote-code \
  --port 8000
```
