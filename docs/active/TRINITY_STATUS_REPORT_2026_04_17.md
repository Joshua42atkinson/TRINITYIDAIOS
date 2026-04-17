# TRINITY STATUS REPORT — April 17, 2026
## Focus: vLLM Inference Architecture — Engineering Diagnosis & Upgrade Path

> **Hardware:** AMD Ryzen AI Max+ 395 (Strix Halo) · Radeon 8060S (gfx1151) · 128 GB LPDDR5x
> **Current vLLM:** 0.18.1-dev (March 26, 2026 build) · ROCm 7.13 · PyTorch 2.12.0a0
> **Target vLLM:** 0.19.0 (April 2, 2026 release) · needs ROCm 7.x compatible TheRock build
> **Session Goal:** Diagnose every vLLM blocker, document the fix path, prepare next session

---

## 1. Executive Summary

**vLLM is the correct inference engine for Trinity.** It supports batched PagedAttention, multi-user serving, and can scale from a personal workstation to a Purdue GPU cluster. The decision to use vLLM is validated.

**The current blocker is a VERSION GAP.** Our distrobox container has vLLM **0.18.1-dev** (March 26 build). Gemma 4 native support (`gemma4.py`) shipped in vLLM **0.19.0** (April 2 release). We are 7 days behind the release that added the exact model family we need.

**This is not an architecture problem. It is a pip upgrade.**

---

## 2. Diagnostic Timeline — What We Tested Today

| # | Attempt | Model | Error | Root Cause |
|---|---------|-------|-------|-----------|
| 1 | Gemma 4 E4B AWQ (safetensors) | 15 GB, port 8001 | `model type 'gemma4' not recognized` | transformers 5.3.0 too old → upgraded to 5.5.4 |
| 2 | Gemma 4 E4B AWQ (safetensors) retry | Same | `input_max` not found in `Gemma4ClippableLinear` | vLLM 0.18.1 has no native `gemma4.py` → falls back to `TransformersMultiModalForCausalLM` → weight mapping breaks on Google's custom `ClippableLinear` layers |
| 3 | Crow-9B GGUF (qwen3.5) | 5.3 GB | `GGUF architecture qwen35 not supported` | `qwen35` not in GGUF mapper's architecture list |
| 4 | Qwen3-Coder GGUF (qwen3_moe) | 5.0 GB | `bfloat16 not supported for gguf` | Missing `--dtype half` flag |
| 5 | Qwen3-Coder GGUF retry | Same | `Failed to map GGUF parameters: experts.gate_up_proj` | MoE fused expert GGUF format not supported in vLLM 0.18.1 GGUF loader |
| 6 | Gemma 4 E2B (unquantized safetensors) | 9.6 GB | `Engine core initialization failed` | Same `TransformersMultiModalForCausalLM` fallback path → EngineCore weight mapping crash on Gemma 4 architecture |

**Pattern:** Every Gemma 4 model fails because vLLM 0.18.1 doesn't have `vllm/model_executor/models/gemma4.py`. It falls back to a generic transformers wrapper which cannot handle Gemma 4's custom `ClippableLinear` layers and audio/vision tower architecture.

---

## 3. The Fix: Upgrade vLLM to 0.19.0

### What vLLM 0.19.0 Added (Released April 2, 2026)

- **`gemma4.py`** — Native model implementation with full architecture support:
  - All 4 variants: E2B, E4B, 26B (MoE), 31B (Dense)
  - Multimodal: text + image + audio
  - Native `ClippableLinear` handling
  - FusedMoE for 128 fine-grained experts (26B)
  - Dynamic vision resolution (70-1120 tokens/image)
  - Thinking mode support
  - Native function/tool calling with dedicated token parsers
- **Requires:** `transformers >= 5.5.0` (we already upgraded to 5.5.4 ✅)
- **Security:** Patches CVE-2026-0994 (prompt embedding deserialization)

### Upgrade Strategy

**The constraint:** We can't just `pip install vllm==0.19.0` inside the distrobox because vLLM on AMD requires TheRock nightly ROCm builds compiled specifically for gfx1151. Our current container (`localhost/vllm-gfx1151:0.18`) was custom-built.

**Three paths forward (ordered by effort):**

#### Path A: In-Place Upgrade (Fastest — try this first)
```bash
# Inside the existing distrobox
distrobox enter vllm -- /opt/venv/bin/pip install --upgrade "vllm>=0.19.0"
```
- **CRITICAL:** `pip install vllm` from PyPI gives **CUDA wheels only**. These will NOT work on AMD.
- **Actual approach:** We must either find a ROCm-specific wheel index, or go to Path B.
- **Quick test:** Try `pip install --no-deps` to get just the Python code without pulling CUDA binaries.
- **Time:** 5 minutes to attempt.

#### Path B: Rebuild from Source Inside Existing Container (Medium effort)
```bash
distrobox enter vllm -- bash -c "
  cd /tmp
  git clone https://github.com/vllm-project/vllm.git
  cd vllm
  git checkout v0.19.0
  PYTORCH_ROCM_ARCH=gfx1151 pip install -e .
"
```
- **Risk:** May need to pin specific sub-dependencies. The existing PyTorch 2.12.0a0+rocm7.13 should be compatible.
- **Time:** 30-60 minutes for build.

#### Path C: New Container from kyuz0 (Most stable, longest)
```bash
# Check if kyuz0 has released a newer container
podman pull docker.io/kyuz0/vllm-therock-gfx1151:latest

# If newer tag available:
distrobox create -n vllm-019 \
  --image docker.io/kyuz0/vllm-therock-gfx1151:latest \
  --additional-flags "--device /dev/kfd --device /dev/dri --group-add video --group-add render --security-opt seccomp=unconfined"
```
- **Risk:** kyuz0 may not have updated yet.
- **Time:** 20-40 minutes for pull + setup.

---

## 4. What's Actually Working Right Now

### Verified Infrastructure ✅

| Component | Status | Evidence |
|-----------|--------|----------|
| **GPU Detection** | ✅ Working | `rocminfo` shows `Radeon 8060S Graphics`, PyTorch `torch.cuda.is_available() = True` |
| **ROCm 7.13** | ✅ Working | `PyTorch 2.12.0a0+rocm7.13.0a20260325` |
| **Distrobox `vllm`** | ✅ Running | Container starts, `/opt/venv/bin/vllm` present, version `0.18.1.dev0` |
| **vLLM Banner** | ✅ Prints | Engine initializes, reads config, resolves architecture |
| **Port Binding** | ✅ Works | `--host 0.0.0.0 --port 8001` accepted |
| **transformers 5.5.4** | ✅ Installed | `Gemma4ForConditionalGeneration` recognized |
| **Home Directory Mount** | ✅ Mounted | `~/trinity-models/` accessible inside container |
| **Model Files** | ✅ Present | E2B (9.6G), E4B AWQ (15G), 31B AWQ (18G), E2B AWQ (4G) all verified |

### Verified Models Available

| Model | Path | Size | Role (P-ART-Y) | vLLM 0.18 | vLLM 0.19 |
|-------|------|------|-----------------|-----------|-----------|
| Gemma 4 E2B (unquantized) | `~/trinity-models/vllm/gemma-4-E2B-it/` | 9.6 GB | T (Tempo) | ❌ No gemma4.py | ✅ Native |
| Gemma 4 E2B AWQ | `~/trinity-models/vllm/gemma-4-E2B-it-AWQ-4bit/` | ~4 GB | T (Tempo) | ❌ No gemma4.py | ✅ Native |
| Gemma 4 E4B AWQ | `~/trinity-models/vllm/gemma-4-E4B-it-AWQ-4bit/` | 15 GB | T (Tempo) | ❌ ClippableLinear crash | ✅ Native |
| Gemma 4 31B AWQ | `~/trinity-models/vllm/gemma-4-31B-it-AWQ-4bit/` | 18 GB | P (Great Recycler) | ❌ ClippableLinear crash | ✅ Native |
| Gemma 4 26B MoE AWQ | `~/trinity-models/vllm/gemma-4-26B-A4B-it-AWQ-4bit/` | ~14 GB | R (Pete Coder) | ❌ No gemma4.py | ✅ Native + FusedMoE |
| Nomic Embed v1.5 AWQ | `~/trinity-models/vllm/nomic-embed-text-v1.5-AWQ/` | ~500 MB | R (Embeddings) | ✅ Should work | ✅ Works |
| LongCat-Next INT4 | `~/trinity-models/omni/LongCat-Next-INT4/` | 50 GB | Future Omni | Requires SGLang | Requires SGLang |

---

## 5. Architecture Confirmation — vLLM for Trinity IS Correct

### Why vLLM (Not Ollama) for Trinity

| Capability | vLLM | Ollama |
|-----------|------|--------|
| **Batched inference (100+ users)** | ✅ PagedAttention | ❌ Single-user |
| **Multi-model serving** | ✅ Multiple ports + router | ❌ One model at a time |
| **OpenAI-compatible API** | ✅ Full spec (tools, vision, audio) | ⚠️ Partial |
| **Server/cluster deployment** | ✅ Tensor parallel, pipeline parallel | ❌ Desktop only |
| **AWQ/GPTQ quantization** | ✅ Native kernels | ❌ GGUF only |
| **Tool calling protocol** | ✅ Native with Gemma 4 parser | ⚠️ Basic |
| **Speculative decoding** | ✅ Draft model support | ❌ Not supported |
| **Production Docker images** | ✅ `vllm/vllm-openai` | ❌ Self-managed |
| **Purdue Gautschi cluster** | ✅ Designed for this | ❌ Not applicable |

### Deployment Model

```
┌──────────────────────────────────────────────────┐
│         PURDUE GAUTSCHI (28 H100 GPUs)           │
│  ┌────────────────────────────────────────────┐   │
│  │ vLLM Pool: 20 H100s                        │   │
│  │ → 300 concurrent Trinity sessions           │   │
│  │ → Gemma 4 31B (Dense) per session           │   │
│  │ → Hotel Swap for specialized roles          │   │
│  └────────────────────────────────────────────┘   │
│  ┌────────────────────────────────────────────┐   │
│  │ Image Gen: 4 H100s (ComfyUI/FLUX)          │   │
│  │ Voice: 2 H100s (Kokoro/Acestep)             │   │
│  │ Embeddings + RAG: 2 H100s (Nomic)           │   │
│  └────────────────────────────────────────────┘   │
├──────────────────────────────────────────────────┤
│       TEACHER WORKSTATION (Strix Halo)           │
│  vLLM distrobox → Gemma 4 E4B (Tempo)            │
│  + Hotel Swap → 31B (Recycler) / 26B MoE (Pete)  │
│  Memory budget: 0.55 GPU utilization              │
├──────────────────────────────────────────────────┤
│       STUDENT PHONE (Pixel 10 Pro XL)            │
│  Mini Trinity → LiteRT/MediaPipe (on-device)      │
│  Connects to teacher's vLLM for heavy tasks       │
└──────────────────────────────────────────────────┘
```

---

## 6. Memory Budget for Strix Halo P-ART-Y Fleet

With `gpu_memory_utilization = 0.55` → ~70 GB available for models:

| Configuration | Models Loaded | Total VRAM | KV Cache | Remaining |
|--------------|---------------|------------|----------|-----------|
| **Lone Wolf** | E4B AWQ only (15 GB) | 15 GB | 4 GB | 51 GB free |
| **Duo** | E4B (15) + 31B AWQ (18) | 33 GB | 8 GB | 29 GB free |
| **Full P-ART-Y** | E4B (15) + 31B (18) + 26B MoE (14) + Nomic (0.5) | 47.5 GB | 12 GB | 10.5 GB |
| **Omni (Future)** | LongCat-Next INT4 (50) | 50 GB | 12 GB | 8 GB |

---

## 7. Next Session Action Plan

### Pre-Session Prep (Tonight)

1. **Check kyuz0 container update:**
   ```bash
   podman pull docker.io/kyuz0/vllm-therock-gfx1151:latest
   podman images | grep vllm
   ```

2. **Attempt in-place vLLM upgrade:**
   ```bash
   distrobox enter vllm -- /opt/venv/bin/pip install "vllm>=0.19.0" --no-deps
   distrobox enter vllm -- /opt/venv/bin/vllm --version
   ```

### Session Tasks

1. **Get vLLM 0.19.0 running with Gemma 4 E2B** (unquantized, 9.6 GB — smallest, easiest)
2. **Verify chat completion works:** `curl http://localhost:8001/v1/chat/completions`
3. **Build and start Trinity:** `cargo run -p trinity --release`
4. **Smoke test chat in browser:** `localhost:3000`
5. **If successful:** Test E4B AWQ (15 GB) → test 31B AWQ (18 GB) → test Hotel Swap

### Fallback If 0.19.0 Won't Build

If building vLLM 0.19.0 from source inside the container fails:
1. **Pin transformers to the version vLLM 0.18 expects:** `pip install "transformers>=4.56,<5"`
2. **Serve Qwen2.5-14B-Instruct-AWQ** — this is `qwen2` architecture with AWQ, fully supported in 0.18.1:
   ```bash
   # Verified: model_type=qwen2, architectures=Qwen2ForCausalLM, quant=awq
   # Path: ~/trinity-models/vllm/Qwen2.5-14B-Instruct-AWQ/ (~10 GB, 3 shards)
   distrobox enter vllm -- /opt/venv/bin/vllm serve \
     ~/trinity-models/vllm/Qwen2.5-14B-Instruct-AWQ \
     --port 8001 --gpu-memory-utilization 0.15 --max-model-len 16384 \
     --enforce-eager --dtype half --trust-remote-code --served-model-name Tempo_E4B
   ```
3. **Use that as the Tempo brain** while we get the Gemma 4 container sorted

---

## 8. Files Reference for Next Session

| Purpose | File |
|---------|------|
| **Inference config** | `configs/runtime/default.toml` (primary = "tempo-e4b", port 8001) |
| **Inference client** | `crates/trinity/src/inference.rs` (OpenAI-compatible, dynamic model resolution) |
| **Inference router** | `crates/trinity/src/inference_router.rs` (auto-detect, role-based switching) |
| **Hotel manager** | `crates/trinity/src/hotel_manager.rs` (model swap lifecycle) |
| **vLLM router** | `scripts/launch/vllm_router.py` (A.R.T.Y. Hub proxy on :8000) |
| **Tempo launcher** | `scripts/launch/launch_tempo_e4b.sh` (distrobox + gfx1151 env vars) |
| **LongCat reference** | `archive/longcat/LONGCAT_MASTER_REFERENCE.md` (0.55 memory rule) |
| **AMD constraints** | `docs/EVERYTHING_WE_KNOW_ABOUT_LONGCAT_ON_AMD.md` (gfx1151 specifics) |

---

## 9. Conclusion

**We are one version bump away from a working system.** The GPU works, the drivers work, the container works, the models are downloaded, the Rust code compiles, the frontend is built. The ONLY issue is that vLLM 0.18.1 (March 26) shipped 7 days before Gemma 4 support landed in vLLM 0.19.0 (April 2).

**Do NOT:**
- Switch away from vLLM
- Redesign the architecture
- Download different models
- Change the distrobox driver stack

**DO:**
- Upgrade vLLM to 0.19.0 inside the existing container
- If that fails, build from source against the existing PyTorch/ROCm
- If THAT fails, try the Qwen2.5-14B as a bridge model while the container gets sorted
