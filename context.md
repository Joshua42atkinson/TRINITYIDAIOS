# Trinity AI OS — Session Context & Fix List

> **Last Updated**: April 8, 2026
> **Goal**: LongCat-Next 74B MoE Omni-Brain integration — multimedia Iron Road with LitRPG storytelling, voice-to-text document filling, and inline image/audio generation.



## Current System State

| Component | Status | Detail |
|-----------|--------|--------|
| **LongCat-Next 74B MoE** | ✅ LOADED & SERVING | Port 8010. Text ✅, Image Gen ✅, TTS 🔍, Audio Understanding 🔍. ~84GB VRAM (NF4). |
| **Trinity Server** | ✅ Running | Port 3000, Axum headless server |
| **LDT Portfolio UI** | ✅ Running | React Web App on Port 3001 via Vite |
| **Iron Road UI** | ✅ Working | Zen-mode chat with ADDIECRAPEYE phase navigation |
| **Rust REAP (Pete)** | ⬚ NOT YET WIRED | Qwen3-Coder-REAP-25B GGUF available. Needs llama-server on port 8000. |
| **Kokoro TTS** | ⬚ Available | Port 8200, not currently running |

## Three-Model Architecture (April 8, 2026)

Trinity runs a three-model stack mapped to the ID-AI-OS paradigm:

| Slot | Role | Model | Port | Hardware | VRAM |
|------|------|-------|------|----------|------|
| **ID** | Instructional Designer | LongCat-Next 74B MoE (NF4) | 8010 | GPU (ROCm) | ~84 GB |
| **AI** | Storyteller / Narrator | LongCat via system prompt | 8010 | GPU (shared) | 0 extra |
| **OS** | Coding Subagent | Qwen3-Coder REAP 25B A3B | 8000 | CPU (llama-server) | 0 GB |

**Key insight**: ID and AI are the SAME model (LongCat) using different system prompts. The Great Recycler has two modes:
- **Instructional Mode**: Standard Socratic ADDIECRAPEYE scaffolding
- **Narrative Mode**: LitRPG storyteller where the USER is the protagonist
- **Hybrid Mode (default)**: Weaves instruction into narrative seamlessly

## System Full Startup Sequence

1. **LongCat Omni Engine (Port 8010)**
   ```bash
   distrobox enter sglang-engine -- bash ./longcat_omni_sidecar/launch_engine.sh
   ```
   *(Uses transformers + bitsandbytes NF4. Takes ~2.5 min, uses ~84GB unified memory)*

2. **Programmer Pete (Port 8000) — OPTIONAL, NOT YET WIRED**
   ```bash
   llama-server \
     --model ~/trinity-models/gguf/Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf \
     --port 8000 --ctx-size 32768 --n-gpu-layers 0 --threads 16
   ```

3. **Trinity Backend (Port 3000)**
   ```bash
   cargo run --release -p trinity
   ```

4. **LDT Portfolio Web UI (Port 3001)**
   ```bash
   cd LDTAtkinson/client && npm run dev -- --port 3001
   ```

---

## LongCat Capabilities (Verified April 8, 2026)

| Capability | Endpoint | Status |
|-----------|----------|--------|
| **Text generation** | `POST /v1/chat/completions` | ✅ Verified |
| **Image generation (DiNA)** | `POST /v1/images/generations` | ✅ Verified |
| **TTS (CosyVoice)** | `POST /tts` | 🔍 Ready (not yet smoke-tested) |
| **Audio understanding** | `POST /v1/audio/transcriptions` | 🔍 Ready (not yet tested) |
| **Image understanding** | Via multimodal chat | 🔍 Ready (not yet tested) |
| **Voice cloning** | `/tts` with voice reference WAV | 🔍 Ready (joshua.wav available) |

## Fix List (Priority Order)

### 🔴 P0 — Iron Road Multimedia Delivery

- [ ] **Conductor mode switching** — Add Narrative/Instructional/Hybrid modes to `conductor_leader.rs`
- [ ] **Inline multimedia** — Parse `[IMG:]` and `[VOICE:]` markers in AI responses
- [ ] **SSE multimedia events** — Add `image` and `audio` event types to chat stream
- [ ] **LitRPG system prompts** — Rewrite 12 ADDIECRAPEYE prompts as narrative scenes

### 🟠 P1 — Voice Pipeline Activation

- [ ] **LongCat TTS priority** — Make CosyVoice (:8010) Tier 1, Kokoro (:8200) Tier 2
- [ ] **Voice cloning** — Wire joshua.wav reference for Recycler narrator voice
- [ ] **STT integration** — Wire LongCat `/v1/audio/transcriptions` for speech-to-text
- [ ] **Document filling** — Voice-to-text mode for structured document completion

### 🟡 P2 — Frontend Multimedia

- [ ] **Inline image rendering** — Render `<img>` in chat when image events received
- [ ] **Inline audio player** — Render `<audio>` controls for narrated responses
- [ ] **Microphone button** — Browser MediaRecorder → upload → transcribe → send
- [ ] **Narrator voice toggle** — on/off switch for ambient narration

### 🟡 P3 — Legacy Cleanup

- [ ] **`CONTEXT.md`** in docs/ — sync with this file
- [ ] **Fancy Bible** — Update P-ART-Y framework, hardware section, model inventory

---

## Key Files

| Purpose | File |
|---------|------|
| Inference client | `crates/trinity/src/inference.rs` |
| Backend router | `crates/trinity/src/inference_router.rs` |
| Conductor (phase orchestrator) | `crates/trinity/src/conductor_leader.rs` |
| Agent chat (Yardmaster) | `crates/trinity/src/agent.rs` |
| Iron Road chat (SSE) | `crates/trinity/src/main.rs` → `chat_stream()` |
| Voice synthesis | `crates/trinity/src/voice.rs` |
| Telephone (WebSocket audio) | `crates/trinity/src/telephone.rs` |
| Creative pipeline | `crates/trinity/src/creative.rs` |
| Narrative generation | `crates/trinity/src/narrative.rs` |
| Runtime config | `configs/runtime/default.toml` |
| LongCat sidecar | `longcat_omni_sidecar/server.py` |
| LongCat launch | `longcat_omni_sidecar/launch_engine.sh` |

## Architecture Reminders

- **One Brain, Two Personas**: LongCat handles both Instructional Design (Socratic) and Storytelling (LitRPG narrative) via system prompt switching. No model swap needed.
- **The User IS the protagonist**: Every ADDIECRAPEYE phase is a narrative chapter with the user as the hero character.
- **Multimedia is inline**: Images and audio generated during chat appear directly in the chat stream, not in a separate studio.
- **Voice-to-text fills documents**: Users can speak their answers to Socratic questions, and the system structures them into PEARL fields and quest objectives.
- **Static VRAM Budget**: LongCat owns GPU exclusively (~84GB). Pete runs on CPU (zero VRAM).
- **Distrobox**: LongCat runs in `sglang-engine` distrobox container. Ports exposed on 127.0.0.1.
