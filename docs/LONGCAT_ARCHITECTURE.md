# LongCat-Next: How It Works & Optimal Settings for Strix Halo

## 🎉 Model is LIVE: 80.6GB / 128GB

Successfully loaded with 4-bit NF4 quantization on AMD Strix Halo.

---

## How LongCat-Next Handles KV Cache: MLA (Multi-Head Latent Attention)

LongCat-Next **does NOT use standard KV cache** like most models. It uses **Multi-Head Latent Attention (MLA)** — the same breakthrough technique as DeepSeek-V3.

### What MLA Means for Our VRAM

From the model's `config.json`:
```
kv_lora_rank: 512        ← latent KV dimension (compressed)
qk_rope_head_dim: 64     ← RoPE portion of query/key
qk_nope_head_dim: 128    ← non-RoPE portion  
v_head_dim: 128           ← value head dimension
num_attention_heads: 32   ← attention heads
num_hidden_layers: 32     ← transformer layers (backbone only)
```

### Standard KV Cache vs MLA

| Approach | KV Cache Per Token Per Layer | 32 Layers @ 32K |
|----------|-----|------|
| Standard GQA (8 KV heads × 128 dim × 2 × bf16) | 4 KB | 4 GB |
| **MLA** (512 latent + 64 rope) × 2 bytes | **1.15 KB** | **~1.15 GB** |

**MLA compresses the KV cache by ~3.5x** compared to standard GQA! Instead of storing separate K and V tensors per head, MLA projects them into a shared low-rank latent space (`kv_lora_rank=512`), then reconstructs K and V on-the-fly during attention.

### What This Means for Us

| Context Length | MLA KV Cache | Standard KV Cache | Savings |
|---------------|-------------|-------------------|---------|
| 4,096 tokens | ~0.15 GB | ~0.5 GB | 70% |
| 32,768 tokens | ~1.2 GB | ~4 GB | 70% |
| 65,536 tokens | ~2.4 GB | ~8 GB | 70% |
| **131,072 tokens** | **~4.7 GB** | **~16 GB** | **70%** |

With 47.4 GB free after model load, **we can comfortably run at the full 131K context window** without KV cache quantization. This is a massive advantage.

> [!IMPORTANT]
> **No KV cache quantization needed!** MLA already compresses the KV cache natively. Adding KV quantization on top would likely hurt multimodal quality with negligible memory savings.

---

## DiNA Architecture: How the Omni-Modality Works

LongCat-Next uses **Discrete Native Autoregression (DiNA)** — everything is a token.

### Token Types in the Vocabulary (282,624 total)

| Range | Tokens | Purpose |
|-------|--------|---------|
| 0–131,071 | Text tokens | Standard language vocabulary |
| 131,072–150,580 | **Audio tokens** | 8-layer RVQ, each 12.5 Hz (Whisper encoder) |
| 150,581–282,623 | **Visual tokens** | 8-level RVQ, hierarchical discrete IDs (dNaViT) |

### How Each Modality Works

#### 📝 Text Generation (Standard)
Regular autoregressive next-token prediction. Nothing special needed.

#### 🖼️ Image Generation (dNaViT)
1. User prompt → model generates visual token IDs autoregressively
2. Visual tokens use **Residual Vector Quantization (RVQ)** with 8 codebook levels
3. Each level has 16,384 entries → exponential representation space
4. Multi-level tokens decoded via **DepthTransformer** (built into model)
5. Pixel decoder (400M ViT) reconstructs image from discrete tokens
6. Flow-matching refiner enhances texture/detail
7. **Arbitrary resolution** — no fixed size! Specify height/width in prompt

Key prompting pattern:
```
Please generate a photo of [subject] with Height [H], Width [W]
```

#### 🔊 Audio Generation (CosyVoice-based)
1. Audio tokenizer: Whisper encoder → 4x downsample → 8-layer RVQ
2. Results in discrete audio tokens at **12.5 Hz**
3. Two generation modes:
   - **Parallel**: Text + audio generated simultaneously (low latency, streaming)
   - **Serial**: Text first, then audio (higher linguistic quality)
4. Audio detokenizer: Decoder → flow-matching refinement → vocoder → waveform
5. Supports **voice cloning** from short reference audio

#### 👂 Audio Understanding
- Input audio → Whisper encoder → discrete tokens → fed to backbone
- Model outputs text response about the audio content
- Supports ASR, speech translation, audio QA

---

## MoE Architecture Details

| Parameter | Value |
|-----------|-------|
| Total parameters | 68.5B (74B with tokenizers) |
| **Activated per token** | **2.9B–4.5B (avg 3B)** |
| Routed experts | 256 |
| Top-K per token | 12 |
| Architecture | Shortcut MoE + Zero-Expert |
| Backbone layers | 32 |

> [!TIP]
> The model activates only ~3B parameters per token despite having 68.5B total. This is why inference speed should be good even at 4-bit — you're only computing through 12 of 256 experts per step.

---

## Optimal Settings for Trinity on Strix Halo

### Current Configuration (Working)
```python
BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_quant_type="nf4",
    bnb_4bit_compute_dtype=torch.bfloat16,
    bnb_4bit_use_double_quant=True,
)
```

### Recommended Generation Settings
```python
generation_config = {
    "max_new_tokens": 4096,        # Generous for multimodal output
    "temperature": 0.7,            # Balanced creativity
    "top_p": 0.9,                  # Nucleus sampling
    "do_sample": True,
    "repetition_penalty": 1.05,    # Light anti-repetition
}
```

### Image Generation Settings
- Specify resolution in the prompt: `Height [H], Width [W]`
- Max trained resolution: 1,736 × 1,736
- Good default: 720 × 1280 (landscape) or 1280 × 720 (portrait)
- The dNaViT tokenizer achieves **28× compression ratio**

### Audio Settings
- Sampling rate: 12.5 Hz discrete tokens
- CosyVoice vocoder produces 24 kHz audio
- Serial generation recommended for instructional content (better linguistic quality)

---

## Key Perks We Should Leverage for Trinity

1. **Native Multimodal RL** — The model was trained with GRPO on both text AND images. It can improve with feedback.
2. **Unified Embedding Space** — Vision, text, and audio share the SAME embedding space (Fig. 12 in paper). This means cross-modal reasoning is native, not bolted on.
3. **Any-Resolution** — No fixed image sizes. Generate exactly the resolution you need.
4. **Text Rendering** — Beats Flux-dev on text-in-image tasks (TIFF, CVTG-2K benchmarks).
5. **Voice Cloning** — Can clone voices from short reference clips.
6. **131K Context** — Full document-length context for instructional design.
7. **OCR Champion** — Scores 86.5 on OCRBench, beating GPT5-minimal and Gemini Flash-Lite.

---

## VRAM Budget (Final)

```
Model weights (4-bit NF4):        80.6 GB
MLA KV cache @ 131K context:       ~4.7 GB
Activation memory:                 ~2.0 GB
OS + Trinity backend + React:      ~5.0 GB
─────────────────────────────────────────
Total estimated at max context:   ~92.3 GB
FREE headroom:                    ~35.7 GB
─────────────────────────────────────────
Total unified memory:             128.0 GB
```

> [!NOTE]
> There is plenty of room. No KV cache quantization needed. The MLA architecture is already doing the heavy lifting.

