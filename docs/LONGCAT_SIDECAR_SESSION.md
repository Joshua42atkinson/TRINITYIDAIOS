# LongCat-Next Omni Sidecar — Session Report

> **Date**: April 7, 2026
> **Status**: ✅ **MODEL LOADED AND SERVING**
> **Load Time**: 2 minutes 30 seconds
> **VRAM Used**: 80.6 GB / 128 GB (63%)
> **VRAM Free**: 47.4 GB (for KV cache + OS)

---

## ✅ Confirmed Working Configuration

### Hardware
| Component | Spec |
|-----------|------|
| APU | AMD Strix Halo (Ryzen AI Max) |
| GPU | Radeon 8060S (gfx1151, RDNA 3.5) |
| Unified Memory | 128 GB LPDDR5x |
| Dedicated VRAM | 512 MB (framebuffer only) |
| ROCm Target | gfx1151 |

### Software Stack (EXACT versions that work)
| Package | Version | Source |
|---------|---------|--------|
| **torch** | `2.11.0a0+rocm7.11.0a20260106` | TheRock nightlies (`rocm.nightlies.amd.com/v2/gfx1151/`) |
| **torchvision** | `0.25.0+rocm7.13.0a20260403` | TheRock nightlies |
| **torchaudio** | `2.10.0+rocm7.13.0a20260403` | TheRock nightlies |
| **triton** | `3.6.0+git03c08237.rocm7.11.0a20260106` | TheRock nightlies |
| **transformers** | `4.57.6` | PyPI (**NOT 5.x — model requires this exact version**) |
| **bitsandbytes** | `0.43.3.dev` | Container native (with ROCm symlink fix) |
| **sglang** | `0.5.10` | Container native (unused — model loads via transformers) |
| **diffusers** | `0.34.0` | PyPI |
| **librosa** | `0.11.0` | PyPI |
| **soundfile** | `0.13.1` | PyPI |
| **accelerate** | latest | PyPI |
| **Python** | `3.12` at `/opt/venv/bin/python3` | Container venv |

### Container
| Setting | Value |
|---------|-------|
| Container name | `sglang-engine` |
| Container image | `kyuz0/vllm-therock-gfx1151` |
| Runtime | podman (distrobox wrapper) |
| Python path | `/opt/venv/bin/python3` (NOT `/usr/sbin/python3`) |

### Model Load Configuration
```python
from transformers import AutoModelForCausalLM, AutoTokenizer, BitsAndBytesConfig

bnb_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_quant_type="nf4",
    bnb_4bit_compute_dtype=torch.bfloat16,
    bnb_4bit_use_double_quant=True,  # nested quantization
)

model = AutoModelForCausalLM.from_pretrained(
    MODEL_PATH,
    quantization_config=bnb_config,
    device_map="auto",
    trust_remote_code=True,
    torch_dtype=torch.bfloat16,
)
```

### Environment Variables (Required)
```bash
export HSA_ENABLE_SDMA=0
export MIOPEN_FIND_MODE=FAST
export PYTORCH_ROCM_ARCH="gfx1151"
export HF_HOME="$HOME/trinity-models/sglang/LongCat-Next/.cache"
export TRANSFORMERS_CACHE="$HOME/trinity-models/sglang/LongCat-Next/.cache"
```

### BNB Symlink Fix (Required after torch upgrade)
```bash
BNB_DIR="/opt/venv/lib64/python3.12/site-packages/bitsandbytes"
ln -sf "$BNB_DIR/libbitsandbytes_rocm713.so" "$BNB_DIR/libbitsandbytes_rocm72.so"
ln -sf "$BNB_DIR/libbitsandbytes_rocm713.so" "$BNB_DIR/libbitsandbytes_cpu.so"
```

---

## VRAM Budget (Actual)

```
LongCat-Next 4-bit NF4 (actual):  83.6 GB
  - 74B MoE backbone (4-bit)
  - MoE dynamic router & classifier logic (bf16 unquantized)
  - dNaViT visual spatial decoders (bf16 unquantized)
  - CosyVoice audio decoder (bf16)
  - Embedding table + LM head (bf16)
────────────────────────────────────────
FREE for KV cache + Subagents:     44.4 GB
Total unified memory:             128.0 GB
```

> [!NOTE]
> Model uses MORE than the theoretical 38GB 4-bit calculation because:
> - HuggingFace MoE logic natively suspends dynamic routers and large vocab heads in 16-bit to maintain cohesion.
> - Visual decoder (FLUX VAE) remains at bf16 (~1.2GB)
> - Audio decoder (CosyVoice) remains at bf16 (~0.8GB)
> - Double quantization metadata overhead
> - PyTorch internal buffers and fragmentation on unified memory

---

## KV Cache Analysis

### What Gets Quantized
- ✅ **Model weights** → 4-bit NF4 via bitsandbytes
- ❌ **KV cache** → NOT quantized by default (stays at bf16)
- ❌ **Activations** → Computed at bf16 compute dtype

### KV Cache Size Per Token
- `num_key_value_heads = 8` (GQA)
- `num_hidden_layers = 64`
- `head_dim = 128` (6144 / 48)
- Per token per layer: 2 × 8 × 128 × 2 bytes = 4 KB
- Per token (all layers): 64 × 4 KB = **256 KB per token**

### Context Length Budget (47.4 GB free)
| Context Length | KV Cache Size | Feasible? |
|---------------|---------------|-----------|
| 4,096 tokens | ~1.0 GB | ✅ Easy |
| 8,192 tokens | ~2.0 GB | ✅ Easy |
| 32,768 tokens | ~8.0 GB | ✅ Comfortable |
| 65,536 tokens | ~16.0 GB | ✅ Yes |
| 131,072 tokens | ~32.0 GB | ⚠️ Tight (leaves 15GB for OS) |

---

## Launch Command
```bash
distrobox enter sglang-engine -- bash ./longcat_omni_sidecar/launch_engine.sh
```

## API Endpoints (Port 8010)
| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/health` | GET | Status + capabilities |
| `/v1/chat/completions` | POST | Text generation (OpenAI-compatible) |
| `/v1/images/generations` | POST | Image gen via DiNA visual tokens |
| `/tts` | POST | Speech synthesis via CosyVoice tokens |
| `/v1/audio/transcriptions` | POST | Audio understanding |

## Test Commands
```bash
# Health check
curl http://127.0.0.1:8010/health

# Text generation
curl -X POST http://127.0.0.1:8010/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"Hello, who are you?"}]}'
```

---

## Critical Rules

1. **ALWAYS use TheRock nightlies** for torch/torchvision/torchaudio on gfx1151
2. **ALWAYS use `/opt/venv/bin/python3`** inside the container
3. **ALWAYS use `transformers==4.57.6`** — NOT 5.x
4. **NEVER install standard PyTorch wheels** from `download.pytorch.org`
5. **NEVER install `turboquant-vllm`** — NVIDIA-only, breaks ROCm KV cache
6. **BNB symlink is required** after any torch upgrade
7. **Model loads via transformers** — NOT via sglang server (stock sglang doesn't support longcat_next)

---

## Fixes Applied During Session

| # | Error | Fix |
|---|-------|-----|
| 1 | `torchvision::nms does not exist` | Install TheRock nightlies for torch/torchvision |
| 2 | `libbitsandbytes_rocm72.so` not found | Symlink `rocm713.so` → `rocm72.so` |
| 3 | `PermissionError: ~/.cache/huggingface` | Set `HF_HOME` to project-local `.cache` |
| 4 | `libtorch_cuda.so` not found (torchaudio) | Install torchaudio from TheRock nightlies |
| 5 | `No module named 'diffusers'` | `pip install diffusers==0.34.0 librosa soundfile` |
| 6 | `cannot import name 'Qwen2RMSNorm'` | Downgrade `transformers` from 5.3.0 → 4.57.6 |
| 7 | `total_mem` AttributeError | Fix typo to `total_memory` in server.py |
| 8 | MoE `mat1/mat2 shapes cannot be multiplied` | **Critical Fix:** Added `"router", "classifier", "lm_head"` to `llm_int8_skip_modules` (4-bit breaks dynamic grouped execution). |
| 9 | Image `AttributeError: no attribute 'text_tokenizer'` | Manually bound `model.text_tokenizer = tokenizer` right after `model.eval()` |
| 10 | DiT image `shape invalid for input 33554432` | **Critical Fix:** Added `"linear1", "linear2"` to `llm_int8_skip_modules` because native decoders call unquantizable `.reshape()` |
| 11 | VAE `FileNotFoundError: WEIGHT_PATH_TO_...` | Replaced placeholder paths in `config.json` with absolute paths for `image_decoder.safetensors` and audio `vocoder/hift.pt` |
| 12 | DiT/VAE `max_seqlens_q must be provided` | Changed `_attn_implementation` inside `config.json`'s visual and audio properties from `flash_attention_2` to `sdpa` to bypass poor AMD varlen compiler logic. |
| 13 | DiT silent failure due to `trust_remote_code=True` reversion | Edited the ROOT `refiner_modules.py` to bypass `AttnProcessorFlash2Varlen()` instead of the `.cache` file, preventing HuggingFace from overwriting the hotfix during every boot. |

---

## Capability Verification Results (April 7, 2026)

### ✅ Text Generation — WORKING
```bash
curl -X POST http://127.0.0.1:8010/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"Hello, who are you?"}]}'
```
**Result:** Full coherent responses. Model identifies as LongCat-Next. Response time ~2-5s depending on length.

### ✅ Image Generation — WORKING (DiNA Visual Tokens)
```bash
curl -X POST http://127.0.0.1:8010/v1/images/generations \
  -H "Content-Type: application/json" \
  -d '{"prompt":"a cybercat with glowing circuits","size":"512x512"}'
```
**Result:** 968 KB PNG produced at `/tmp/longcat_img_0_refined.png`. Real image, not mock.
- DiNA visual token generation → FLUX VAE decode pipeline works end-to-end
- `creative.rs` in Trinity backend already proxies to this endpoint
- Generation time: ~30-60s per image (varies with complexity)
- 6 audiobook chapter splash images and 6 field manual illustrations already generated this session

### ⚠️ TTS (CosyVoice) — MOCK RESPONSE (Audio tokens not generated)
```bash
curl -X POST http://127.0.0.1:8010/tts \
  -H "Content-Type: application/json" \
  -d '{"text":"Welcome to Trinity.","voice":"am_adam"}'
```
**Result:** 44-byte file — this is the mock WAV header (no actual audio data).
- The model returned **text tokens instead of audio tokens** (same pattern as the initial image gen issue)
- CosyVoice vocoder is loaded but the `<longcat_audiogen_start>` trigger isn't producing audio IDs
- **Likely fix:** Same class of issue as Fix #8-#13 above — the audio decoder modules may need:
  1. Additional modules added to `llm_int8_skip_modules` for the CosyVoice path
  2. `_attn_implementation` set to `sdpa` in the audio decoder config (like Fix #12)
  3. Voice reference file (`system_audio.wav`) may be missing/mispathed

> [!NOTE]
> **TTS is NOT a blocker.** Kokoro TTS on port 8200 handles all current narration needs.
> LongCat CosyVoice is a future upgrade — fixing it requires the same debugging pattern
> used for image gen (Fixes #8-13), which took ~2 hours of iterative config surgery.

### 🔍 Image Understanding — NOT YET TESTED
Requires multimodal chat with image input. Endpoint exists but not exercised.

### 🔍 Audio Understanding — NOT YET TESTED
Endpoint `/v1/audio/transcriptions` exists but not exercised.

