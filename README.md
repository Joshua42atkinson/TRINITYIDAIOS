# Trinity ID AI OS

**A gamified instructional design system that helps K&#8209;12 teachers build educational games using AI.**

Trinity combines instructional design methodology (ADDIE), visual design principles (CRAP), and metacognitive reflection (EYE) into a single AI-powered platform. Teachers describe what they want to teach — Trinity helps them build it.

> Built on a 128 GB AMD Strix Halo (Zen 5 + RDNA 3.5 + XDNA 2 NPU) running a single static 119B MoE language model. No cloud APIs. Everything runs locally.

> 🌐 **Live Demo & Presentation Focus**: [https://LDTAtkinson.com](https://LDTAtkinson.com) · [Source Archive](https://LDTAtkinson.com/downloads/TRINITY_ID_AI_OS_v1.0_source.tar.gz)
>
> 📖 **The Living Code Textbook**: The Rust codebase itself is heavily commented for student inspection. It is not a black box—it is designed to be read, studied, and modified as part of the curriculum.

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
- Linux system with 16 GB+ RAM (developed on 128 GB AMD Strix Halo)
- An OpenAI-compatible LLM backend:
  - [LM Studio](https://lmstudio.ai) (recommended, port 1234)
  - [Ollama](https://ollama.com) (port 11434)
  - [llama-server](https://github.com/ggml-org/llama.cpp) (port 8080)
- Node.js 18+ and Rust 1.80+

### Run

```bash
# 1. Start an LLM backend (pick one):
#    - LM Studio: load a model and start server on port 1234
#    - Ollama: ollama serve && ollama run mistral-small
#    - llama-server: llama-server -m YOUR_MODEL.gguf --port 8080

# 2. Build the React frontend
cd crates/trinity/frontend && npm install && npm run build && cd ../../..

# 3. Build and start Trinity
TRINITY_HEADLESS=1 cargo run -p trinity --release

# 4. Open in browser
xdg-open http://localhost:3000
```

Trinity auto-detects running inference backends (llama-server, Ollama, LM Studio, SGLang) and will auto-launch llama-server via the **GPU Guard** if none are found. The Guard checks: (1) port already in use, (2) process already running, (3) available memory budget — preventing double loads that crash the GPU driver.

### Optional Sidecars

```bash
# Image generation (ComfyUI)
cd ~/ComfyUI && python main.py --port 8188

# Document intelligence (Qianfan-OCR Researcher)
llama-server -m ~/trinity-models/gguf/Qianfan-OCR-Q4_K_M.gguf --port 8081 --ctx-size 32768

# Voice pipeline (Whisper + Kokoro TTS)
python scripts/voice_sidecar.py  # Port 7777
```

---

## 🚀 The Deployment Roadmap

### Current Iteration: Web-Hosted Demo
For the current Purdue University presentation, Trinity is web-hosted at [LDTAtkinson.com](https://LDTAtkinson.com). This provides immediate, low-friction access for educators and peers to experience the ADDIECRAPEYE lifecycle and VAAM scaffolding without requiring specialized local hardware.

### Next Week: The Single-Download AppImage
Our immediate next objective is **Mini Trinity**: a zero-configuration, single-download AppImage. The download will contain the *entire system*—including the Voice engine (TTS/STT) and RAG architecture—minus the heavy LLM.
- **Zero Python. Zero CLI.** No compiling, no messy dependencies.
- Students simply download the AppImage, run it, and connect it to a single main LLM (hosted locally via LM Studio or Ollama).
- **LLM Requirements**: The connected LLM must support exactly three things: **Dual KV Cache** (to support both Pete and the Great Recycler simultaneously), **Multimodal Vision**, and **Tool Calling**. As long as the LLM can do it, so can the student.
- We also maintain specialized datasets to train smaller models for students operating on constrained hardware.

## Architecture

```
Three-layer process isolation — one brain, many modes.

┌─────────────────────────────────────────────────────────────┐
│  Layer 1: Trinity Server (Axum, port 3000)                  │
│  InferenceRouter: auto-detects LM Studio / Ollama / llama   │
│  30 agentic tools, ADDIECRAPEYE orchestration               │
│  SQLite persistence + native ONNX RAG (all-MiniLM-L6-v2)   │
│  React frontend: Iron Road / ART Studio / Yardmaster        │
├─────────────────────────────────────────────────────────────┤
│  Layer 2: Protocol (trinity-protocol crate)                  │
│  Shared types, CharacterSheet, PEARL, Sacred Circuitry       │
├─────────────────────────────────────────────────────────────┤
│  Layer 3: DAYDREAM (trinity-daydream crate)                  │
│  Native Bevy 0.18.1 sidecar — 3D LitRPG sandbox             │
├─────────────────────────────────────────────────────────────┤
│  Optional Sidecars                                           │
│  ComfyUI (:8188) · Supertonic TTS (native ONNX) · MusicGPT  │
└─────────────────────────────────────────────────────────────┘
```

## Codebase

**8 workspace crates · 0 compile errors**

| Crate | Description |
|-------|-------------|
| `trinity` | Axum server, 30 agentic tools, inference router, creative pipeline, EYE export, Quality Scorecard |
| `trinity-protocol` | Shared types, ADDIE lifecycle, PEARL focus agent, VAAM alignment, CharacterSheet |
| `trinity-quest` | Quest board, XP/Coal/Steam economy, 432 bespoke phase objectives |
| `trinity-iron-road` | Iron Road narrative, Pete core, bestiary, MadLibs |
| `trinity-voice` | SSML injection, VAAM vocal emphasis, Supertonic TTS |
| `trinity-daydream` | Native Bevy 0.18.1 3D LitRPG sidecar (pure Rust, no JS) |
| `trinity-mcp-server` | Model Context Protocol for IDE integration (Zed, Cursor, Antigravity) |

### Frontend (Vite + React)

40 components across both frontends (Trinity app + LDT Portfolio). Key views:

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

## 🐎 The Four Horses of Awareness (System Documentation)
Trinity is the **Chariot**—an autonomous engine designed to bring about the death of the false "self". To pull it forward, we rely on four core documents—our sci-fi Four Horses of the Apocalypse. They exist to strip away ego and enforce the core tenet: *Be the student you want to teach, remove the ego, and become the teacher.*

1. **[PLAYERS_HANDBOOK.md](PLAYERS_HANDBOOK.md)** *(Conquest / Who You Are)*: Focuses on the user—philosophy, conscious learning, and identity. It conquers the ego.
2. **[ASK_PETE_FIELD_MANUAL.md](ASK_PETE_FIELD_MANUAL.md)** *(War / Who Pete Is)*: AI Pedagogy, the Socratic battle against cognitive shortcuts, and system philosophy. It wages war on lazy thinking.
3. **[TRINITY_FANCY_BIBLE.md](TRINITY_FANCY_BIBLE.md)** *(Famine / What The System Is)*: Deep architectural specs, memory pipelines, and technical integration. It starves the system of bloat and enforces the raw dual-brain reality.
4. **[TRINITY_SYLLABUS.md](TRINITY_SYLLABUS.md)** *(Death / What Trinity Can Do)*: Your compass for navigating Trinity's usefulness and the 12-Station Iron Road. The death of your old workflows, making way for the rebirth of the Master Architect.

*(Note: Technical session tracking for AI agents happens in `context.md`, and `MATURATION_MAP.md` is the component maturity tracker.)*

## Links

- **Live Demo**: [LDTAtkinson.com](https://LDTAtkinson.com)
- **Trinity App**: [LDTAtkinson.com/trinity/](https://LDTAtkinson.com/trinity/)
- **Source Archive**: [Download v1.0](https://LDTAtkinson.com/downloads/TRINITY_ID_AI_OS_v1.0_source.tar.gz)
- **GitHub**: [github.com/Joshua42atkinson/trinity-genesis](https://github.com/Joshua42atkinson/trinity-genesis)
- [ConsciousFramework.com](https://consciousframework.com)
- [GreatRecycler.com](https://greatrecycler.com)
