# TRINITY ID AI OS: Maturation Map

> **Updated: April 8, 2026 — LongCat Multimodal Iron Road Milestone**

The TRINITY system is a layered, Socratic ecosystem designed to shepherd a user from a raw idea to a fully shipped portfolio product. It relies on a three-tier agentic hierarchy (ID-AI-OS) unified under the LongCat-Next 74B MoE Omni-Brain, seamlessly operating within the Iron Road LitRPG experience.

## The Agentic Hierarchy (TRINITY)

To avoid cognitive overload, TRINITY dynamically engages logic engines (models) invisibly via natural Socratic conversation inside the **Iron Road**.

### 1. The Omni-Brain (LongCat-Next 74B MoE) — ✅ LOADED & SERVING
**Model:** LongCat-Next (74B MoE via HuggingFace `transformers` + `bitsandbytes` NF4)  
**Port:** 8010 (FastAPI sidecar inside `sglang-engine` distrobox container)  
**VRAM:** ~84 GB of 128 GB Unified Memory  
**Role:** Unified Omni-Brain — The Great Recycler (ID + Storyteller + Media)  
**Capabilities (April 8, 2026):**
- ✅ **Text generation** — `/v1/chat/completions` (OpenAI-compatible)
- ✅ **Image generation** — `/v1/images/generations` (DiNA visual tokens → FLUX VAE decode)
- ✅ **TTS / Audio** — `/tts` (CosyVoice vocoder, voice cloning capable)
- 🔍 **Audio understanding** — `/v1/audio/transcriptions` (STT for voice-to-text)
- 🔍 **Image understanding** — (via multimodal chat)

**Function:**
- The ultimate authority of the session, claiming ~84GB of the 128GB Unified Memory.
- Replaces the fractured multi-agent pipeline (Gemma text + Flux image + Kokoro audio). LongCat *is* the Instructional Designer, the Storyteller, the Illustrator, and the Narrator simultaneously via Discrete Native Autoregression (DiNA).
- Operates in **three conductor modes**: Instructional (Socratic), Narrative (LitRPG storyteller), and Hybrid (weaves instruction into narrative).
- The user IS the protagonist of the Iron Road LitRPG — every ADDIECRAPEYE phase is a narrative chapter.
- Generates inline multimedia: images appear in chat, narration plays automatically, documents are filled by voice.
- Applies the **ADDIECRAPEYE** methodology via the Conductor Protocol.
- Maintains the master game state, character sheet, and quest logic in one context window (131K tokens).

### 2. Programmer Pete (Qwen3-Coder REAP 25B A3B) — ⬚ NOT YET WIRED
**Model:** Qwen3-Coder-REAP-25B-A3B-Rust (GGUF)  
**Hardware:** CPU via `llama-server` (Vulkan/ROCm optional)  
**Port:** 8000 (configured in `inference_router.rs` as `pete-coder`, not yet serving)  
**GGUF Weights Available:**
- `Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf` (~/trinity-models/gguf/)
- `Qwen3-Coder-REAP-25B-A3B-Rust-IQ1_S.gguf` (~/trinity-models/gguf/)

**Role:** Subagent (The Deterministic Compiler — EXHALE)  
**Function:**
- Generates coding outputs sequentially alongside LongCat. Runs on CPU via `llama-server` so it does NOT compete with LongCat's GPU memory.
- Executes `cargo_check`, writes Rust `.rs` files, builds React `.jsx`.
- Receives tool instructions from the Great Recycler's orchestration layer.

**What's Needed:**
1. Install `llama-server` (llama.cpp) and verify it serves GGUF on port 8000
2. Update `default.toml` to point `pete-coder` at port 8000
3. Wire `agent.rs` to dispatch coding tasks to `pete-coder` instead of the active (LongCat) backend

### 3. Native Media Coprocessor (NPU) — ⬚ DEFERRED
**Models:** Nomic-Embed-Text (Vector Memory) & SDXL-Turbo (Draft Generation)  
**Hardware:** NPU Core Execution (via `ONNX Runtime` Rust Bindings)  
**Role:** The Ephemeral Drafter  
**Function:**
- Bypasses the GPU entirely, enabling hyper-fast context generation.
- Currently deferred — LongCat handles image generation natively, and nomic-embed can run alongside.

---

## VRAM Budget: The ID-AI-OS Stack on 128GB Strix Halo

```
┌──────────────────────────────────────────────────────────────┐
│ 128 GB UNIFIED MEMORY — AMD Strix Halo (gfx1151)           │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ██████████████████████████████████████████  84 GB           │
│  LongCat-Next 74B MoE (NF4) — THE OMNI-BRAIN               │
│  ID = Great Recycler (Socratic Instructional Designer)      │
│  AI = Storyteller (LitRPG narrator — same model, diff prompt)│
│   A = Aesthetics (DiNA image generation)                    │
│   T = Tempo (CosyVoice TTS + voice cloning)                │
│                                                              │
│  ████████████████  15 GB (CPU/mmap — NOT in VRAM)           │
│  Qwen3-Coder REAP 25B A3B (Q4_K_M GGUF via llama-server) │
│  OS = Programmer Pete (deterministic code execution)        │
│                                                              │
│  ░░░░░░░░░░░░░░  ~44 GB FREE                               │
│  KV Cache (LongCat 131K context) + OS + PyTorch buffers    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Model Inventory (~/trinity-models)

| Model | Size on Disk | Runtime VRAM | Execution | P-ART-Y Role |
|-------|-------------|-------------|-----------|-------------|
| **LongCat-Next 74B MoE** | 151 GB (bf16) | **~84 GB** (NF4) | GPU (ROCm) | **P** + **A** + **R** + **T** |
| **Qwen3-Coder REAP 25B A3B** (Q4_K_M) | 15 GB | **0 GB** (CPU mmap) | CPU (llama-server) | **Y** (coding) |
| **Qwen3-Coder REAP 25B A3B** (IQ1_S) | 5 GB | **0 GB** (CPU mmap) | CPU (llama-server) | **Y** (lightweight) |
| **Crow-9B-Opus** | 5.3 GB | **0 GB** (CPU mmap) | CPU (llama-server) | Quick captions |
| **Kokoro TTS v1.0** | 338 MB | **~0.3 GB** (ONNX) | CPU | Voice synthesis |
| **Nomic-Embed-Text** (ONNX) | 23 MB | **~0.02 GB** | CPU/NPU | RAG embeddings |
| **Whisper-Base** (ONNX) | 2.4 MB | **~0.1 GB** | CPU/NPU | Speech-to-text |

### Also Available (not loaded simultaneously)

| Model | Size | Notes |
|-------|------|-------|
| Gemma-4 31B Dense AWQ | ~18 GB | Legacy primary brain (before LongCat) |
| Gemma-4 26B MoE AWQ | ~14 GB | Alternative lighter model |
| Gemma-4 E2B / E4B | ~3-5 GB | Ears (audio understanding) |
| ACE-Step 3.5B | ~7 GB | Music generation |
| CogVideoX-2b | ~5 GB | Video generation |
| TripoSR | ~2 GB | Image-to-3D mesh |
| FLUX.1-schnell (GGUF) | 6.4 GB | Alternative image gen |

> [!NOTE]
> **Key insight:** Because Qwen REAP runs on CPU via `llama-server` mmap, it costs **zero VRAM**. 
> LongCat owns the GPU exclusively. The CPU has 16 Zen 5c cores available — plenty for GGUF inference 
> at ~10-15 tok/s for code generation tasks. Pete doesn't need to be fast; he needs to be correct.

---

## System Readiness Matrix

### What Works RIGHT NOW ✅

| System | Port | Status | Notes |
|--------|------|--------|-------|
| **Trinity Rust Backend** | 3000 | ✅ Compiles & Serves | 73+ API routes, all Chariot viewers |
| **React Frontend** | 3001 | ✅ Vite Dev | Iron Road, Yardmaster, Art Studio, Handbook |
| **LongCat-Next Omni** | 8010 | ✅ Model Loaded | Text + Image gen confirmed, TTS + STT ready |
| **SQLite Persistence** | — | ✅ Working | Sessions, jobs, chat history, character sheet |
| **ADDIECRAPEYE Quest Engine** | — | ✅ Working | 12 phases, objectives, game mechanics |
| **Player Handbook ELearning** | — | ✅ Working | Book viewer with audiobook narration |
| **Field Manual Viewer** | — | ✅ Working | 6 generated illustrations |
| **Character Sheet** | — | ✅ Working | Full CRUD via /api/character |
| **VAAM Bridge** | — | ✅ Working | Vocabulary mining, Coal tracking |
| **Background Jobs** | — | ✅ Wired | POST /api/jobs spawns agent loop |
| **Scope Creep Detection** | — | ✅ Working | PEARL-aware semantic checking |
| **Conductor Protocol** | — | ✅ Working | Phase-specific Socratic prompts |
| **Voice Cloning** | — | ✅ joshua.wav recorded | Zero-shot voice clone for Recycler narrator |

### What's BROKEN / STALE ⚠️

| System | Issue | Fix Effort |
|--------|-------|-----------|
| **`default.toml`** | ✅ FIXED — now points to LongCat :8010 | Done |
| **`CONTEXT.md`** | ✅ FIXED — rewritten for LongCat architecture | Done |
| **Conductor prompts** | Standard Socratic only — needs Narrative/Hybrid modes | 2 hours |
| **Chat multimedia** | No inline image/audio rendering in chat stream | 3 hours |
| **Voice pipeline** | TTS priority still Kokoro-first, needs LongCat CosyVoice | 1 hour |
| **LitRPG prompts** | Users not addressed as protagonist in narrative | 2 hours |
| **Frontend multimedia** | No inline image/audio rendering in chat UI | 3 hours |
| **Programmer Pete** | No `llama-server` binary installed (deferred) | 30 min |
| **Audiobook art** | Only 6/24 splash images exist | 30 min — run generation script |

### What's NEEDED for Overnight Autonomous Work 🌙

| # | Requirement | Blocks | Fix |
|---|-------------|--------|-----|
| 1 | **Fix `default.toml`** — route to LongCat :8010 | Everything | Update 3 port numbers |
| 2 | **Install `llama-server`** for Qwen REAP GGUF | Pete coding agent | `apt` or build from source |
| 3 | **Wire dual-dispatch in `agent.rs`** | Autonomous coding | Route code tasks to Pete, Socratic to LongCat |
| 4 | **Verify Background Jobs** | Overnight work | Submit a test job via `/api/jobs` |
| 5 | **Work Log persistence** | Morning review | Already wired — verify reports/ directory |

---

## User Journey (Path of Maturation)

### Phase 1: The Yard (Novice)
Users land in the Socratic CLI. They interact purely with the ID (Great Recycler on LongCat). They answer foundational scope questions.
- **Active Subsystems:** ID (100%) via LongCat port 8010.

### Phase 2: The Iron Road (Apprentice)
Users begin writing the Hook Book. As they establish narrative and visuals, the ID silently delegates tasks. Users notice characters appearing visually and audio narrative generating dynamically.
- **Active Subsystems:** ID (60%) via LongCat, AI Media (40%) via LongCat DiNA.

### Phase 3: The Daydream Forge (Journeyman)
The User crosses from theory to product engineering. The Great Recycler awakens Programmer Pete to translate the ADDIECRAPEYE scaffolding into raw Rust/React logic. The User now plays a game of QA, reviewing Pete's builds against the ID's original theories.
- **Active Subsystems:** ID (40%) via LongCat, OS (60%) via Qwen REAP on llama-server.

### Phase 4: Autopoiesis (Master)
The User fully commands the TRINITY loop on a single, isolated Strix Halo node. The User feeds complex constraints (PEARL schemas) into the Recycler.
1. The **NPU** passively embeds all textbook constraints and rapidly blasts out UI scene drafts.
2. The **CPU** (Qwen REAP Pete) sequentially executes the required `.rs` files and handles CLI operations.
3. The **GPU** (LongCat) strictly orchestrates the flow, applies precise narrative text, generates high-fidelity multimodal assets, and interacts structurally with the user.

---

## Boot Order (Current — April 8, 2026)

```bash
# 1. Start the Omni-Brain (inside sglang-engine distrobox)
distrobox enter sglang-engine -- bash ./longcat_omni_sidecar/launch_engine.sh
# Loads on port 8010, takes ~2.5 minutes, uses ~84GB unified memory
# Serves: text, images (DiNA), TTS (CosyVoice), STT, voice cloning

# 2. Start Programmer Pete (when ready — NOT YET WIRED)
# llama-server \
#   --model ~/trinity-models/gguf/Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf \
#   --port 8000 --ctx-size 32768 --n-gpu-layers 0 --threads 16

# 3. Start Trinity server
cargo run --release -p trinity  # port 3000

# 4. Start frontend dev server (for development)
cd LDTAtkinson/client && npm run dev -- --port 3001
```

---

## Appendix A: Purdue Demo Flow (Updated — LongCat Native)

The system now generates images natively via LongCat DiNA tokens — no ComfyUI sidecar needed.

**Proven flow for the demo:**

1. **Launch the Core:**
   - Show `distrobox enter sglang-engine` loading LongCat-Next (2.5 min boot, 84GB VRAM)
   - Show `cargo run --release --bin trinity` starting the Axum server
2. **The Portfolio Hub:**
   - Open browser to `http://localhost:3000/trinity`
   - Show the React interface with Iron Road, Art Studio, Handbook views
3. **The Four Chariots (The Crown Jewel):**
   - Open the Player's Handbook ELearning viewer
   - Show the double-page book UI with audiobook narration
   - Show inline chapter art generated by LongCat
4. **The Iron Road Integration:**
   - Click "Ask Pete" to enter the Socratic workspace
   - Demonstrate PEARL creation, VAAM vocabulary mining, Coal/Steam tracking
   - Show the Conductor Protocol adapting Pete's personality to the current ADDIECRAPEYE phase
5. **Media Generation:**
   - Use the Art Studio to generate an image via LongCat DiNA
   - Show the image appearing inline in the chat stream
