# Trinity ID AI OS

**A gamified instructional design system that helps K&#8209;12 teachers build educational games using AI.**

Trinity combines instructional design methodology (ADDIE), visual design principles (CRAP), and metacognitive reflection (EYE) into a single AI-powered platform. Teachers describe what they want to teach — Trinity helps them build it.

> Built on a 128 GB AMD Strix Halo (Zen 5 + RDNA 3.5 + XDNA 2 NPU) running a single static 119B MoE language model. No cloud APIs. Everything runs locally.

> 🌐 **Live Demo**: [https://LDTAtkinson.com](https://LDTAtkinson.com) · [Source Archive](https://LDTAtkinson.com/downloads/TRINITY_ID_AI_OS_v1.0_source.tar.gz)

---

## What It Does

| Capability | Description |
|------------|-------------|
| **Guided Game Design** | Walk through 12 instructional design phases (ADDIECRAPEYE) with Pete, an AI mentor |
| **Socratic Teaching** | Pete never gives direct answers — he asks questions that lead to discovery |
| **Lesson Plans** | Generate Bloom's-aligned lesson plans, rubrics, quizzes, and curriculum maps |
| **Quality Scorecard** | Evaluate documents across 5 pedagogical dimensions (Bloom's, ADDIE, Accessibility, Engagement, Assessment) |
| **Image Generation** | Create visual assets via ComfyUI (SDXL Turbo) directly in conversation |
| **Document Intelligence** | Analyze documents, charts, and images with Qianfan-OCR Researcher sub-agent |
| **LDT Portfolio** | Track academic progress through 12 portfolio artifacts mapped to ADDIECRAPEYE phases — QM alignment, IBSTPI/ATD/AECT competency scoring, Gate Review graduation gate |
| **HTML5 Export** | Export self-contained quiz games and text adventures that run in any browser |
| **Bevy Game Scaffold** | Generate starter Rust/Bevy game projects pre-loaded with vocabulary and objectives |
| **Voice Conversation** | Talk to Pete using speech (Whisper STT + Kokoro TTS pipeline) |

## Quick Start

### Prerequisites
- Linux system with 128 GB+ unified RAM (developed on AMD Strix Halo)
- [llama.cpp](https://github.com/ggml-org/llama.cpp) built with Vulkan support
- [Mistral Small 4 119B](https://huggingface.co/mistralai/Mistral-Small-4-119B-2503) GGUF (Q4_K_M, ~68 GB)
- PostgreSQL 15+ with pgvector extension
- Node.js 18+ and Rust 1.80+

### Run

```bash
# 1. Start the LLM (or let Trinity auto-detect and launch it)
llama-server -m ~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
  --host 127.0.0.1 --port 8080 -ngl 99 --ctx-size 262144 --flash-attn on --jinja

# 2. Build the React frontend
cd crates/trinity/frontend && npm install && npm run build && cd ../../..

# 3. Build and start Trinity
cargo build --release
cargo run --release

# 4. Open in browser
xdg-open http://localhost:3000
```

Trinity auto-detects running inference backends (llama-server, vLLM, Ollama, LM Studio, SGLang) and will auto-launch llama-server via the **GPU Guard** if none are found. The Guard checks: (1) port already in use, (2) process already running, (3) available memory budget — preventing double loads that crash the GPU driver.

### Optional Sidecars

```bash
# Image generation (ComfyUI)
cd ~/ComfyUI && python main.py --port 8188

# Document intelligence (Qianfan-OCR Researcher)
llama-server -m ~/trinity-models/gguf/Qianfan-OCR-Q4_K_M.gguf --port 8081 --ctx-size 32768

# Voice pipeline (Whisper + Kokoro TTS)
python scripts/voice_sidecar.py  # Port 7777
```

## Architecture

```
One brain, many modes.

Pete (Mistral Small 4 119B MoE) ─── The AI personality (~68 GB Q4_K_M)
 ├── Aesthetics mode ─── CRAP visual design prompts
 ├── Research mode ───── Qianfan-OCR 4B document intelligence (:8081)
 └── Tempo mode ──────── Code generation, game scaffolding

Rust Axum Server (:3000) ─── ADDIECRAPEYE orchestration, 29 agentic tools
React Frontend ──────────── Book-view UI, 6 tabs (Iron Road / ART Studio / Character / Yardmaster / Scorecard / Voice)
PostgreSQL + pgvector ───── Sessions, RAG knowledge base
ComfyUI (:8188) ─────────── SDXL Turbo image generation
Voice (:7777) ───────────── Whisper STT + Kokoro TTS
GPU Guard ───────────────── Hotel protocol (prevents double LLM loads)
Sidecar Monitor ─────────── Checks real sidecar health, reports only true crashes
```

## Codebase

**6 workspace crates · 179 tests · 0 failures · 0 compile errors**

| Crate | Tests | Description |
|-------|:-----:|-------------|
| `trinity` | 83 | Axum server, 29 agentic tools, inference router, creative pipeline, EYE export, GPU Guard, Quality Scorecard |
| `trinity-protocol` | 67 | Shared types, ADDIE lifecycle, PEARL focus agent, VAAM alignment |
| `trinity-quest` | 18 | Quest board, XP/Coal/Steam economy, 432+ phase objectives |
| `trinity-iron-road` | 16 | Iron Road narrative, Pete core, bestiary, MadLibs |
| `trinity-voice` | 10 | SSML injection, VAAM vocal emphasis |
| `trinity-bevy-graphics` | — | 3D Yard vision processing (parked: winit 0.30.13 + Rust 1.94 issue) |

### Frontend (Vite + React)

16 components, 7 custom hooks. Key views:

- **Iron Road** — 3-column book layout: chapter rail / prose + chat / game HUD
- **ART Studio** — Image, music, video, 3D mesh generation with asset gallery
- **Character** — LDT Portfolio HUD: cognitive logistics (Coal/Steam/Friction), academic progress (12-artifact graduation track), portfolio artifact vault, intent engineering
- **Yardmaster** — IDE-grade agentic terminal with reasoning panel and tool forge
- **Scorecard** — Pedagogical quality evaluation with dimension bars and recommendations
- **Express** — 3-step wizard for quick game generation

### Key API Endpoints (`http://localhost:3000`)

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/api/health` | Subsystem health (LLM, DB, ComfyUI, Voice) |
| POST | `/api/chat/stream` | SSE streaming chat with Pete |
| POST | `/api/chat/yardmaster` | Agentic chat with tool-calling |
| POST | `/api/quest/compile` | Compile Game Design Document from quest state |
| POST | `/api/yard/score` | Quality Scorecard — 5-dimension pedagogical evaluation |
| POST | `/api/character/portfolio/artifact` | Vault LDT portfolio artifact, recalculate metrics |
| POST | `/api/pearl` | Create/replace PEARL (subject + medium + vision) |
| GET | `/api/eye/export` | Export HTML5 quiz/adventure/JSON |
| GET | `/api/inference/status` | Multi-backend router status |

See [CONTEXT.md](CONTEXT.md) for the full 40+ endpoint reference.

## The Name

**TRINITY = ID + AI + OS**
- **ID** = Instructional Design (ADDIE methodology)
- **AI** = Artificial Intelligence (CRAP visual design, automated by AI agents)
- **OS** = Operating System (EYE metacognitive layer — Envision, Yoke, Evolve)

## The Pedagogy

Trinity implements the **ADDIECRAPEYE** framework — a 12-station instructional design lifecycle:

| Stations | Framework | Focus |
|----------|-----------|-------|
| **A**nalyze → **D**esign → **D**evelop → **I**mplement → **E**valuate | ADDIE | Instructional Design |
| **C**ontrast → **R**epetition → **A**lignment → **P**roximity | CRAP | Visual Design |
| **E**nvision → **Y**oke → **E**volve | EYE | Metacognition |

Each station maps to a Bloom's Taxonomy level and a Hero's Journey chapter. The system tracks vocabulary mastery, awards XP for objective completion, and compiles everything into a portable Game Design Document.

## License

Apache 2.0 — Users own all content created with Trinity. See [LICENSE](LICENSE).

## Research Context

This is a graduate research project at Purdue University exploring AI-augmented instructional design. For detailed architecture documentation, see:

- [CONTEXT.md](CONTEXT.md) — Full research bible and session context
- [TRINITY_FANCY_BIBLE.md](TRINITY_FANCY_BIBLE.md) — Iron Road design bible (lore + mechanics + pedagogy)
- [INSTALL.md](INSTALL.md) — Step-by-step build & run guide for evaluators

## Links

- **Live Demo**: [LDTAtkinson.com](https://LDTAtkinson.com)
- **Trinity App**: [LDTAtkinson.com/trinity/](https://LDTAtkinson.com/trinity/)
- **Source Archive**: [Download v1.0](https://LDTAtkinson.com/downloads/TRINITY_ID_AI_OS_v1.0_source.tar.gz)
- **GitHub**: [github.com/Joshua42atkinson/trinity-genesis](https://github.com/Joshua42atkinson/trinity-genesis)
- [ConsciousFramework.com](https://consciousframework.com)
- [GreatRecycler.com](https://greatrecycler.com)
