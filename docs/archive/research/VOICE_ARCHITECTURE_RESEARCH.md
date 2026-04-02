# Voice Architecture Research — Trinity ID AI OS
## March 18, 2026

> [!WARNING]
> **ARCHIVED (March 28, 2026)**: The vLLM experiments referenced in this document were abandoned in favor of `llama.cpp` due to UMA memory constraints on the AMD Strix Halo architecture. This document remains for historical reference.

---

## The Problem

The walkie-talkie voice loop (sox → whisper → piper) uses silence timers to detect
when the user is done talking. This fails in real-world conditions:
- Babies and background noise trigger false responses
- Pausing to think gets cut off
- No echo cancellation (Trinity talks to itself)
- Not context-aware (doesn't understand conversational rhythm)

## Options Researched

### Option 1: Moshi/Moshiko (Kyutai Labs) — Rust/Candle
**What it is**: Full-duplex speech-to-speech model. Audio in → audio out, no text intermediate.
**Architecture**: Mimi encoder → Helium 7B LM → Mimi decoder
**Key features**:
- True full-duplex (listens while speaking)
- Backchanneling ("uh-huh" while user talks)
- Echo cancellation built into the web client
- Inner Monologue (predicts text tokens before audio for better quality)
- 240ms interruption latency, 170ms TTFT

**Rust implementation**: `cargo run --features cuda --bin moshi-backend` from the official repo.
Uses Candle framework. Config files for q8 quantized models.

**Model downloaded**: `~/trinity-models/moshiko-candle-q8/` (7.7GB q8 GGUF + tokenizer)

**Pros**:
- Pure Rust inference via Candle ✅
- Full-duplex solves ALL the timer/turn-detection problems
- Small model (7B, 8GB quantized)
- WebSocket-based client protocol (easy to integrate)

**Cons**:
- Moshi is a 7B model — not as smart as Mistral Small 4
- Fixed persona (can't easily swap system prompts like text LLMs)
- ROCm/HIP support in Candle is experimental
- Cannot be served via vLLM (custom architecture)

**Trinity fit**: EXCELLENT for the "always-on listener" layer. Moshi handles the conversation
rhythm, echo cancellation, and turn detection. For deep thinking, it hands off to Mistral.

---

### Option 2: Qwen2.5-Omni-3B/7B — vLLM Compatible
**What it is**: End-to-end multimodal model. Text + audio + image + video in → text + speech out.
**Architecture**: Thinker-Talker with TMRoPE (time-aligned multimodal position encoding)
**Key features**:
- Audio INPUT understood natively (not just STT)
- Speech OUTPUT generated natively (not just TTS)
- Also handles images and video
- Streaming chunked input/output for real-time interaction
- 3B version available for lower hardware requirements

**vLLM support**: YES — via `vllm-omni` or forked vllm branch.
- Text output: fully supported
- Audio output: supported in offline mode, serve mode still text-only

**ROCm**: YES — alexhegit/Qwen2.5-Omni-ROCm repo exists with instructions.

**Memory**: 3B model ~6GB, 7B model ~14GB

**Pros**:
- Runs on vLLM (batch processing, OpenAI-compat API) ✅
- Understands audio natively (better than STT→text pipeline)
- Also understands images/video (future: screen sharing)
- 3B version fits easily alongside Mistral Small 4
- Can be system-prompted like a normal LLM ✅

**Cons**:
- Not full-duplex (turn-based like current system)
- vLLM audio output not yet in serve mode (only offline)
- Still needs external echo cancellation
- Not as natural as Moshi for conversation rhythm

**Trinity fit**: EXCELLENT for the "smart understanding" layer. Qwen2.5-Omni understands
what the user MEANS from their voice (tone, emphasis, hesitation) better than text STT.

---

### Option 3: Qwen3-Omni-30B-A3B — vLLM Compatible (Future)
**What it is**: MoE version of Qwen Omni. 30B total, 3B active.
**vLLM support**: YES via vllm-omni
**Status**: Very new, MoE inference can be slow on HF Transformers.
**Note**: vLLM recommended for inference speed.
**Trinity fit**: Future upgrade path when vllm-omni matures.

---

### Option 4: PersonaPlex-7B (NVIDIA) — ONNX/NPU
**Already on disk**: `~/trinity-models/voice/personaplex-7b/` (14GB)
**Architecture**: Mimi encoder → 7B LM → Mimi decoder (similar to Moshi)
**Pros**: Full-duplex, ONNX format for NPU
**Cons**: Requires ONNX Runtime + Vitis AI EP (not validated on Strix Halo)
**Trinity fit**: Future NPU offload path. Same architecture as Moshi.

---

## RECOMMENDATION: Two-Layer Architecture

```
┌─────────────────────────────────────────────────────┐
│  Layer 1: Conversation Manager (always on, ~8GB)    │
│  Moshiko Candle Q8 — Rust native                    │
│  - Full-duplex audio stream                         │
│  - Echo cancellation (knows what it just said)      │
│  - Natural turn detection (no timers)               │
│  - Backchanneling ("I see", "go on")                │
│  - Transcribes user speech to text internally        │
│  - Generates quick verbal acknowledgments            │
│  - Routes complex questions to Layer 2               │
├─────────────────────────────────────────────────────┤
│  Layer 2: Deep Thinking Brain (on demand)            │
│  Mistral Small 4 119B via vLLM — or llama.cpp        │
│  - Receives transcribed text from Layer 1            │
│  - Full ADDIECRAPEYE orchestration                   │
│  - QM Rubric evaluation                              │
│  - Backward Design enforcement                       │
│  - Returns structured response to Layer 1            │
│  - Layer 1 speaks the response naturally             │
├─────────────────────────────────────────────────────┤
│  Layer 3: Audio Understanding (future, optional)     │
│  Qwen2.5-Omni-3B via vLLM                           │
│  - Understands tone, emphasis, hesitation            │
│  - "The user sounds uncertain" → adjust approach     │
│  - Screen/image understanding for context            │
│  - Adds emotional intelligence to Layer 2            │
└─────────────────────────────────────────────────────┘
```

### Why This Works
- **Moshi handles conversation rhythm** — no more timers, no more babies triggering responses
- **Mistral handles thinking** — 119B MoE brain for real instructional design work
- **Qwen Omni adds understanding** — future layer for emotional/tonal awareness
- **Memory budget**: Moshi 8GB + Mistral 68GB + OS 30GB = 106GB (fits in 128GB)
- **All Rust except vLLM** — Moshi via Candle, llama.cpp for GGUF, vLLM for safetensors

### "Go Dark" Mode
When user steps away:
1. Moshi stops listening (mic off)
2. Mistral gets full 128GB for batch processing (quest board, worldbuilding, code generation)
3. User says "Hey Trinity" → Moshi wakes up → normal conversation resumes

---

## Implementation Priority

1. **NOW**: Moshiko Candle integration (model downloaded, Rust repo exists)
2. **SOON**: Mistral Small 4 via vLLM (downloading)
3. **FUTURE**: Qwen2.5-Omni-3B as emotional intelligence layer
4. **FUTURE**: PersonaPlex on NPU when ONNX Runtime + Vitis AI validates

---

*Research conducted March 18, 2026 for Trinity ID AI OS voice architecture.*

---
---

# UPDATE: March 19, 2026 — GPU Reality Check + New Options

## Hardware Reality (Tested)

The Radeon 8060S iGPU has **512MB dedicated VRAM** (gfx1151). Unified memory
allows PyTorch/ROCm to allocate from the 128GB system RAM pool, BUT the iGPU
only has ~40 CUs — not enough raw compute for real-time 7B audio inference.

**Tested and failed on GPU:**
- Moshi 7B (bf16) → choppy, unusable (needs ~14GB + fast compute)
- LFM2.5-Audio → segfault on gfx1151 (experimental ROCm attention)

**Tested and works on CPU:**
- LFM2.5-Audio via llama-liquid-audio-server → 1.7s latency, 138 tok/s, voice heard
- Piper TTS ONNX → fast, but robotic voice quality

**Conclusion**: The Strix Halo is a CPU beast (16× Zen5, AVX-512) but its iGPU
cannot sustain real-time audio model inference. All voice inference must target CPU
or wait for discrete GPU / validated NPU path.

---

## Option 5: Chatterbox Turbo (Resemble AI) — TTS Only

**What it is**: State-of-the-art open-source Text-to-Speech. 350M params.
MIT licensed. Consistently beats ElevenLabs in blind tests (63.8% preference).
Voice cloning from 5-10 second reference clip.

**Key features**:
- Paralinguistic tags: `[laugh]`, `[chuckle]`, `[sigh]` for emotional expression
- PerTh watermarking built in (responsible AI)
- Turbo variant: single-step decoding, <150ms latency (on GPU)
- 23 languages supported via Multilingual variant
- `pip install chatterbox-tts`

**Hardware requirements**:
- Default: requires PyTorch with CUDA (even CPU mode needs CUDA-enabled PyTorch)
- ROCm: untested on gfx1151, likely needs unified memory workaround
- CPU: possible but latency will be higher than 150ms (maybe 500ms-1s for 350M)

**Pros**:
- Best open-source voice quality available ✅
- Voice cloning: record 10s of Pete's voice → Trinity sounds like Pete
- Emotional tags for expressive responses
- Small model (350M) — fits alongside everything else

**Cons**:
- TTS only — still needs separate ASR and Brain
- PyTorch dependency (not pure Rust)
- ROCm on gfx1151 untested
- CPU latency unknown (needs benchmarking)

**Trinity fit**: EXCELLENT for voice output quality. Replaces Piper as the TTS
layer in a separated pipeline. Could give Trinity cinematic voice quality.

---

## Option 6: Separated Pipeline (How Professional Phone Systems Work)

This is how **every** commercial AI phone system works — from Domino's pizza
ordering to Marriott hotel concierge to dental appointment bots:

```
┌──────────────────────────────────────────────────────────┐
│  Layer 0: Wake Word (always on, ~2MB CPU)                │
│  openwakeword — listens for "Hey Trinity"                │
│  Negligible CPU, always running                          │
│  Immediately plays pre-cached "Loading voice..." WAV     │
├──────────────────────────────────────────────────────────┤
│  Layer 1: ASR — Speech to Text                           │
│  whisper.cpp / faster-whisper on CPU (AVX-512)           │
│  ~300ms for short utterances, battle-tested              │
│  Outputs clean text of what user said                    │
├──────────────────────────────────────────────────────────┤
│  Layer 2: Brain — Understanding + Response               │
│  Mistral Small 4 (when ready) or current LLM             │
│  Full Pete persona, ADDIECRAPEYE, all Trinity logic      │
│  Outputs text response                                   │
├──────────────────────────────────────────────────────────┤
│  Layer 3: TTS — Text to Speech                           │
│  Chatterbox Turbo 350M (cinematic quality)               │
│  OR Piper (fast, lower quality)                          │
│  Voice-cloned Pete with emotional expression             │
│  Streams audio to speaker as it generates                │
├──────────────────────────────────────────────────────────┤
│  Layer 4: Echo Suppression                               │
│  Mutes mic while Trinity speaks                          │
│  Resumes listening when TTS finishes                     │
│  Simple but effective (walkie-talkie style)              │
└──────────────────────────────────────────────────────────┘
```

**Total latency estimate (CPU)**:
- Wake word detection: ~50ms
- ASR (whisper.cpp tiny/base): 300-500ms
- Brain (LLM generation start): 200-500ms
- TTS first audio chunk: 200-500ms
- **Total time-to-first-audio: ~1-2 seconds**

This is comparable to Alexa/Google Home (~1.5s) and faster than most
AI phone systems (~2-3s). Perfectly acceptable for a teaching assistant.

**Pros**:
- Each component is best-in-class and independently upgradeable ✅
- Works entirely on CPU ✅
- Proven architecture (billions of calls handled daily worldwide)
- Pete's voice can be customized via Chatterbox voice cloning
- Wake word enables hands-free ✅
- Can upgrade individual layers (e.g., swap Piper → Chatterbox)

**Cons**:
- Not full-duplex (can't interrupt Trinity mid-sentence)
- Higher total latency than end-to-end models
- Multiple processes to manage (orchestrator needed)

---

## REVISED RECOMMENDATION

### Immediate Path (works NOW on this hardware):

**Separated Pipeline with progressive upgrades:**

1. **Week 1**: Wake word (openwakeword) + ASR (whisper.cpp) + Brain (current LLM) + TTS (Piper)
   - Gets hands-free working immediately
   - Piper is fast and already on disk
   - "Hey Trinity" → transcribe → think → speak

2. **Week 2**: Upgrade TTS to Chatterbox Turbo
   - Benchmark CPU latency
   - Voice-clone Pete with a 10s reference recording
   - If ROCm works on gfx1151, even better

3. **Week 3**: PersonaPlex 7B ONNX on CPU as alternative conversation engine
   - ONNX Runtime with AVX-512 EP may handle 7B better than PyTorch
   - Already on disk (14GB)
   - Full-duplex Moshi architecture without GPU

### Future Path (when hardware allows):

4. Moshiko Candle Q8 on ROCm (when Candle adds ROCm support)
5. PersonaPlex on NPU (when ONNX Runtime + Vitis AI validates for Strix Halo)
6. Qwen2.5-Omni-3B as emotional intelligence layer

### Why This Is Professional

- **Google Home / Alexa**: Wake word → ASR → NLU → TTS (exactly this pipeline)
- **Apple Siri**: Wake word → on-device ASR → cloud LLM → on-device TTS
- **Domino's AI Phone**: Wake word → ASR → intent engine → TTS
- **We are building the same thing, fully local, no cloud.**

---

*Updated March 19, 2026 after GPU testing and Chatterbox Turbo research.*
