# ⛪ THE TRINITY BIBLE — Definitive Edition

![The Five Ring Architecture — Soul at the center](/images/five_ring_architecture.png)

> *"The Bible is the train. Trinity's pedagogy is the payload. The destination is the student."*

**System**: TRINITY ID AI OS — Instructional Design Artificial Intelligence Operating System
**Author**: Joshua Atkinson
**Institution**: Purdue University — Learning Design & Technology
**Hardware**: AMD Ryzen AI Max+ 395 (Strix Halo) — 128 GB unified memory
**License**: Apache 2.0
**Genesis Date**: March 2026
**Version**: v1.0
**Scale**: 40,000+ Lines of Code (Rust/React)

---

## Feature Status Overview

| Feature | Status | Evidence |
|---------|--------|----------|
| **Inference Router (Agnostic HTTP)** | `Verified` | `inference_router.rs` — LM Studio / Ollama / llama-server auto-detect |
| **Quality Scorecard** | `Verified` | `quality_scorecard.rs`, unit tests pass |
| **Socratic Protocol & Agent Tools** | `Verified` | `conductor_leader.rs`, `tools.rs`, 30 available tools |
| **LDT Portfolio HUD** | `Verified` | `CharacterSheet.jsx`, `character_api.rs` |
| **App Modes (Iron Road, Express, Yardmaster, Demo)** | `Verified` | `AppMode` enum in `main.rs`, React UI |
| **Creative Pipeline (Images, Music, Video, 3D)** | `Verified` | `creative.rs`, `useCreative.js` — ComfyUI/MusicGPT/Hunyuan |
| **Voice Pipeline (Supertonic-2 TTS)** | `Verified` | `supertonic.rs`, native ONNX — browser fallback auto-detect |
| **DAYDREAM (Native Bevy Sidecar)** | `Verified` | `trinity-daydream` crate — Pure Rust Bevy 0.18.1 3D LitRPG, no JS |
| **ADDIECRAPEYE Phase Navigation** | `Verified` | Vertical 12-tab sidebar, phase-aware input & badge |
| **EYE Export** | `Verified` | `/api/eye/export` → download button |
| **Safety Badges (CowCatcher/EdgeGuard)** | `Verified` | `GameHUD.jsx` — visible safety indicators, `edge_guard.rs` |
| **Phase-Aware Messaging** | `Verified` | `activePhase` sent with every `/api/chat/zen` call |
| **RLHF Feedback (Thumbs Up/Down)** | `Verified` | 👍/👎 buttons on narrator messages → `/api/rlhf/resonance` |
| **Scout Sniper + RLHF Economy** | `Verified` | Hope/Nope → coal/steam/XP payout via `scope_creep_decision` |
| **Model Switcher** | `Verified` | `Yardmaster.jsx` — lists `/api/models`, switches via `/api/models/switch` |
| **Native RAG (ONNX Embeddings)** | `Verified` | `rag.rs` — pure Rust `ort` + `all-MiniLM-L6-v2`, no Python |
| **Journal & Reflections** | `Verified` | `JournalViewer.jsx` — timeline, weekly reflections, bookmarks, export |
| **Book Narrative** | `Verified` | `GameHUD.jsx` — chapters from `/api/book`, generate via `/api/narrative/generate` |
| **Setup Wizard (BYOM)** | `Verified` | `SetupWizard.jsx` — API health gate, dynamic backend selection |
| **Tauri v2 Desktop App** | `Verified` | Dual-headed binary: Tauri main thread + Axum background thread |
| **Background Job Runner** | `Verified` | `jobs.rs` — SQLite-persisted task queue, headless multi-turn agent |
| **MCP Server** | `Verified` | `trinity-mcp-server` crate — Model Context Protocol for agentic extensibility |
| **Shadow Process** | `Verified` | `CharacterSheet.jsx` — Ghost Train stop button → `/api/character/shadow/process` |
| **TCG HookDeck Spells** | `Verified` | `character_sheet.rs`, `CharacterSheet.jsx` — physical TCG spell cards to tame creatures |
| **Multi-user Sessions** | `Roadmap` | Planned vLLM PagedAttention deployment |

---

## 🚃 Car 0: Table of Contents

### ADDIE — Extract the Wisdom *(Cars 1–5: "What is Trinity?")*

- [Car 1: ANALYZE — System Overview & Philosophy](#-car-1-analyze--system-overview--philosophy)
  - [1.1 What Is TRINITY ID AI OS?](#11-what-is-trinity-id-ai-os) · [1.2 Three Stakeholders](#12-the-three-stakeholders) · [1.3 P-ART-Y Framework](#13-the-p-art-y-framework) · [1.4 Operating Modes](#14-the-three-operating-modes) · [1.5 Architecture](#15-the-three-layer-architecture) · [1.6 App State](#16-the-application-state) · [1.7 API Surface](#17-the-api-surface) · [1.8 Module Map](#18-the-server-module-map) · [1.9 Hardware](#19-the-hardware-platform) · [1.10 Frontend](#110-the-frontend) · [1.11 Field Manual](#111-field-manual-cross-reference)
- [Car 2: DESIGN — Architecture & Crate Registry](#-car-2-design--architecture--crate-registry)
  - [2.1 Protocol Crate](#21-the-protocol-crate--trinitys-language) · [2.2 Workspace Layout](#22-the-workspace-layout) · [2.3 Dependencies](#23-dependency-philosophy) · [2.4 Module Inventory](#24-the-module-inventory)
- [Car 3: DEVELOP — The ADDIECRAPEYE Framework](#-car-3-develop--the-addiecrapeye-framework)
  - [3.1 12 Stations](#31-the-12-stations) · [3.2 Bloom's Integration](#32-blooms-taxonomy-integration) · [3.3 Sacred Circuitry](#33-sacred-circuitry--the-15-word-cognitive-scaffolding) · [3.4 Circuit↔Station Map](#34-the-circuit--station-isomorphism) · [3.5 Hotel Pattern](#35-the-hotel-management-pattern) · [3.6 Socratic Prompts](#36-phase-implementation-socratic-prompts)
- [Car 4: IMPLEMENT — Iron Road Game Mechanics](#-car-4-implement--the-iron-road-game-mechanics)
  - [4.1 Core Loop](#41-the-iron-road-core-loop) · [4.2 VAAM](#42-vaam--vocabulary-as-a-mechanism) · [4.3 SemanticCreep](#43-semanticcreep--vocabulary-creatures) · [4.4 Bestiary](#44-the-bestiary) · [4.5 MadLibs](#45-lesson-madlibs) · [4.6 Events](#46-the-game-loop-events) · [4.7 Book](#47-the-book-of-the-bible) · [4.8 Tests](#48-test-coverage) · [4.9 Pythagorean PPPPP](#49-the-pythagorean-ppppp) · [4.10 TCG HookDeck](#410-the-tcg-hookdeck)
- [Car 5: EVALUATE — Quality Systems & Security Rings](#-car-5-evaluate--quality-systems--security-rings)
  - [5.1 Ring System](#51-the-ring-system) · [5.2 Tool Permissions](#52-ring-1-tool-permissions) · [5.3 Persona Gates](#53-ring-2-persona-based-access-control) · [5.4 Sandboxing](#54-ring-5-command-sandboxing) · [5.5 Perspective Engine](#55-ring-6-the-perspective-engine) · [5.6 QM Rubric](#56-quality-matters-rubric--automated-evaluation) · [5.7 Scorecard](#57-quality-scorecard--document-evaluation) · [5.8 Cow Catcher](#58-the-cow-catcher--error-classification)

### CRAP — Place the Wisdom *(Cars 6–9: "How does Trinity work?")*

- [Car 6: CONTRAST — What Makes Trinity Different](#-car-6-contrast--what-makes-trinity-different)
  - [6.1 PEARL](#61-the-pearl--perspective-engineering-aesthetic-research-layout) · [6.2 PEARL Fields](#62-the-three-pearl-fields) · [6.3 PEARL Lifecycle](#63-pearl-phase-lifecycle) · [6.4 PEARL Evaluation](#64-pearl-evaluation--weighted-alignment) · [6.5 CharacterSheet](#65-the-charactersheet--persistent-identity) · [6.6 User Classes](#66-the-four-user-classes) · [6.7 Intent Engineering](#67-intent-engineering--the-digital-quarry) · [6.8 Comparison](#68-trinity-vs-other-tools)
- [Car 7: REPETITION — The Isomorphic Patterns](#-car-7-repetition--the-isomorphic-patterns)
  - [7.1 Isomorphism](#71-the-isomorphism-principle) · [7.2 Three-to-Twelve](#72-the-three-to-twelve-pattern) · [7.3 Cognitive Load](#73-cognitive-load-theory-in-code) · [7.4 Golem Metaphor](#74-the-golem-metaphor) · [7.5 HW-to-Game Map](#75-hardware-to-game-mapping) · [7.6 Locomotive Profiles](#76-locomotive-profiles)
- [Car 8: ALIGNMENT — Pete's Socratic Protocol](#-car-8-alignment--petes-socratic-protocol)
  - [8.1 Socratic Core](#81-the-socratic-core) · [8.2 System Prompt](#82-the-yardmaster-system-prompt) · [8.3 Dual Persona](#83-dual-persona-architecture) · [8.4 VAAM in Chat](#84-vaam-integration-in-chat) · [8.5 Agent Loop](#85-the-multi-turn-agent-loop)
- [Car 9: PROXIMITY — User Interface & Experience](#-car-9-proximity--user-interface--experience)
  - [9.1 Components](#91-the-16-react-components) · [9.2 Glassmorphism](#92-glassmorphism-design-system) · [9.3 CharacterSheet UI](#93-the-charactersheet-ui) · [9.4 Four Chariots](#94-the-four-chariots--in-app-help-menu) · [9.5 Three Modes](#95-the-three-modes)

### EYE — Refine the Wisdom *(Cars 10–12: "Where does Trinity go?")*

- [Car 10: ENVISION — Purdue Integration & LDT Portfolio](#-car-10-envision--purdue-integration--ldt-portfolio)
  - [10.1 LDT Portfolio](#101-the-ldt-portfolio--the-graduation-track) · [10.2 Standards](#102-standards-alignment) · [10.3 Graduation](#103-graduation-requirements) · [10.4 Heavilon Events](#104-heavilon-events--memorial-steps) · [10.5 Artifact Vault](#105-the-artifact-vault)
- [Car 11: YOKE — ART Pipeline & Creative Tools](#-car-11-yoke--art-pipeline--creative-tools)
  - [11.1 ART Pipeline](#111-the-art-creative-pipeline) · [11.2 ComfyUI](#112-comfyui-workflow--sdxl-turbo) · [11.3 Visual Style](#113-visual-style-system) · [11.4 Voice](#114-voice-pipeline) · [11.5 Models](#115-model-inventory)
- [Car 12: EVOLVE — Deployment, Hardware, and the Lexicon](#-car-12-evolve--deployment-hardware-and-the-lexicon)
  - [12.1 Strix Halo](#121-the-amd-strix-halo-platform) · [12.2 KV Cache](#122-the-dual-kv-cache-architecture) · [12.3 Server](#123-server-architecture) · [12.4 Lexicon](#124-the-trinity-lexicon) · [12.5 What's Next](#125-whats-next)

---

## How to Read This Bible

This document is structured as **12 train cars** following Trinity's own ADDIECRAPEYE framework — the same framework that governs the system it describes. Each car is self-contained and code-referenced.

**Code references** use the format `📍 file.rs:L##` pointing to exact lines in the source. Every architectural claim can be verified against running code.

| Group | Cars | Purpose | Question Answered |
|-------|------|---------|-------------------|
| **ADDIE** (1–5) | Analyze → Design → Develop → Implement → Evaluate | Extract the Wisdom | *What is Trinity?* |
| **CRAP** (6–9) | Contrast → Repetition → Alignment → Proximity | Place the Wisdom | *How does Trinity work?* |
| **EYE** (10–12) | Envision → Yoke → Evolve | Refine the Wisdom | *Where does Trinity go?* |

### The Crate Map

All code lives under `crates/` in the workspace. Eight active crates:

| Crate | Role | Key Modules |
|-------|------|-------------|
| `trinity` | Headless HTTP server + Tauri host (Layer 1) | 37+ modules, `main.rs` entry point |
| `trinity-protocol` | Shared types & language | 26 public modules, `lib.rs` |
| `trinity-quest` | Quest engine & game state | Hero stages, XP/Coal/Steam |
| `trinity-iron-road` | Book writing & VAAM | Narrative, vocabulary, SemanticCreep |
| `trinity-voice` | Voice pipeline | Supertonic-2 TTS (ONNX native), 10 voices |
| `trinity-daydream` | Bevy 3D DAYDREAM (Layer 3) | Pure Rust sidecar — PEARL-driven 3D LitRPG world, Avian3D physics |
| `trinity-mcp-server` | Model Context Protocol | Standardized agentic extensibility protocol |
| (archive) | Legacy crates | `trinity-sidecar`, `trinity-bevy-graphics` — preserved, excluded from workspace |

> 📍 `Cargo.toml:L1-27` — Workspace members with inline role comments

---

# ADDIE — Extract the Wisdom

> *Cars 1–5 answer: "What is Trinity?"*

---

## 🚃 Car 1: ANALYZE — System Overview & Philosophy

> **Bloom's Level**: Remember / Understand
> **Sacred Circuit**: The Lens (circuit #1)
> **Body Metaphor**: The Eyes — seeing the system for the first time

### 1.1 What Is TRINITY ID AI OS?

TRINITY ID AI OS is a **locally-hosted, AI-powered instructional design operating system** that transforms a single educator's expertise into complete learning experiences — courses, games, assessments, and interactive content — without sending a single byte to the cloud.

The name breaks down as:

- **TRINITY** — Three stakeholders (Learner × Instructor × Institution), three AI agents (Pete × ART × Yardmaster), three UX systems (AUDIO / WEB / BEVY), three deliverables (FUN / WORK / LEARNING)
- **ID** — Instructional Design, the academic discipline
- **AI** — Artificial Intelligence, running locally on consumer hardware
- **OS** — Operating System, not just a chatbot but a complete design environment

**The core thesis**: An instructional designer working alone with Trinity on a laptop produces output that previously required a team, a budget, and months of calendar time — because the AI handles the mechanical work while the human retains creative control.

### 1.2 The Three Stakeholders

Every design decision in Trinity serves exactly three audiences:

| Stakeholder | Needs | Trinity Serves Via |
|-------------|-------|-------------------|
| **Learner** | Engagement, challenge calibration, skill tracking | VAAM vocabulary tiers, SemanticCreep gamification, adaptive difficulty |
| **Instructor** | Curriculum tools, quality assurance, standards alignment | PEARL focusing, Quality Scorecard, QM Rubric, EYE Container export |
| **Institution** | Compliance, portfolio evidence, scalable deployment | LDT Portfolio with IBSTPI/AECT mapping, FERPA-safe local execution |

### 1.3 The P-ART-Y Framework

Trinity's AI is not one monolithic model. It is three specialized agents, each with a distinct personality, responsibility, and hardware assignment:

| Agent | Full Name | Role | Model |
|-------|-----------|------|-------|
| **P** (Pete) | Programmer Pete | Socratic mentor, curriculum guide | Mistral Small 4 119B MoE (68 GB, dual KV cache 256K×2 = 500K+ context) |
| **ART** | Aesthetic Research Technician | Creative pipeline — images, video, music, 3D | SDXL Turbo (ComfyUI), HunyuanVideo, Hunyuan3D-2.1, trinity-tempo-ai |
| **Y** (Yardmaster) | OS/Dev Agent | Code generation, tool execution, system admin | Ming-flash-omni-2.0 (future), currently uses Pete's model |

> 📍 `main.rs:L220-268` — `installed_model_inventory()` lists all 10 deployed models with paths and sizes

The **PARTY** mnemonic: **P**ete + **A**RT + the use**R** + **T**rinity + **Y**ardmaster. The user is always at the center.

### 1.4 The Three Operating Modes

Trinity adapts its entire UI and workflow to three distinct modes:

```rust
pub enum AppMode {
    IronRoad,    // Full LitRPG gamification — the learning game
    Express,     // Skip game mechanics — guided wizard → export
    Yardmaster,  // IDE/Agent mode — code generation and system admin
}
```

> 📍 `main.rs:L84-92` — `AppMode` enum with three variants

| Mode | User Profile | What Changes |
|------|-------------|--------------|
| **Iron Road** | Students, self-learners | Full game loop: quests, vocabulary battles, narrative chapters, XP/Coal/Steam economy |
| **Express** | Working instructional designers | Streamlined wizard: PEARL → objectives → export. No game overhead. |
| **Yardmaster** | Developers, system administrators | Multi-turn agentic chat with tool execution, file operations, code generation |

The mode is stored in `AppState` and switchable at runtime via `/api/mode`:

> 📍 `main.rs:L133` — `pub app_mode: Arc<RwLock<AppMode>>`
> 📍 `main.rs:L823` — `.route("/api/mode", get(get_app_mode).post(set_app_mode))`

### 1.5 The Three-Layer Architecture

Trinity is not a chatbot wrapper. It is a three-layer operating system:

```
┌─────────────────────────────────────────────────────────┐
│  Layer 3: DAYDREAM (Spatial Sandbox)                     │
│  Bevy 0.18.1 — pure Rust 3D LitRPG sidecar process      │
│  📍 crates/trinity-daydream/                            │
├─────────────────────────────────────────────────────────┤
│  Layer 2: Protocol (The Language)                        │
│  Shared types, ADDIECRAPEYE enums, CharacterSheet        │
│  📍 crates/trinity-protocol/ (26 public modules)        │
├─────────────────────────────────────────────────────────┤
│  Layer 1: Headless Server + Tauri Host                   │
│  Axum HTTP API on port 3000 — the "engine room"         │
│  Tauri v2 wraps Layer 1 for native desktop delivery      │
│  📍 crates/trinity/src/main.rs (startup)                │
└─────────────────────────────────────────────────────────┘
```

**Layer 1** (Headless Server) is the foundation. It runs without a screen. The Axum HTTP server spawns on a background `tokio::spawn` thread, while `tauri::Builder` owns the main thread for native UI rendering (`npx tauri dev`). Passing `--headless` or `TRINITY_HEADLESS=1` bypasses Tauri entirely, allowing the same binary to serve as a headless daemon on `LDTAtkinson.com` via Cloudflared EdgeGuard on port 3000.

**Layer 3** (DAYDREAM) runs as a **separate native process** — not embedded in the Tauri WebView. Due to Linux/Wayland `winit` constraints (both Tauri and Bevy demand the main thread), the Bevy `daydream` binary is spawned as an OS child process, cleanly isolating the 3D render pipeline from the web UI.

> 📍 `main.rs:L415-417` — Startup banner: `"TRINITY HEADLESS SERVER - Layer 1"`
> 📍 `main.rs` — `let addr = "0.0.0.0:3000"` — binds to all interfaces

### 1.6 The Application State

Every route in Trinity shares a single `AppState` struct, now organized into **three semantic layers** — System, Player, and Project:

```rust
pub struct AppState {
    // ── System Layer (hardware, AI, database) ──
    pub inference_router: Arc<RwLock<InferenceRouter>>,          // Multi-backend LLM routing
    pub db_pool:          sqlx::SqlitePool,                      // SQLite + In-Memory V-RAG
    pub cow_catcher:      Arc<RwLock<CowCatcher>>,               // Error handling
    pub vaam_bridge:      Arc<VaamBridge>,                       // Vocabulary mining
    pub tts_engine:       Option<Arc<Mutex<SupertonicEngine>>>,  // Native ONNX TTS
    pub stt_engine:       Option<Arc<Mutex<WhisperEngine>>>,     // Native ONNX STT
    pub ignition_status:  Arc<RwLock<String>>,                   // LM Studio boot state machine
    pub job_queue:        JobQueue,                              // Background task runner

    // ── Player Context (identity — persists across projects) ──
    pub player: PlayerContext, // { character_sheet, bestiary, app_mode }

    // ── Project Context (the active PEARL — one per game) ──
    pub project: ProjectContext, // { game_state, conversation_history, book, book_updates, session_id }
}
```

> 📍 `main.rs:L147-199` — `AppState` with `PlayerContext` + `ProjectContext` + System Layer

The **Identity Split** (Tier 3.5 Maturation) separates *who the educator is* (PlayerContext) from *what they're building* (ProjectContext). This enables future multi-project support — a single educator can switch between PEARLs while preserving their identity, bestiary, and vocabulary profile.

### 1.7 The API Surface

Trinity exposes **~85 HTTP endpoints** organized into 15 functional groups:

| API Group | Route Prefix | Purpose | Lines |
|-----------|-------------|---------|-------|
| Health | `/api/health` | Subsystem status checks | L774 |
| Chat | `/api/chat/*` | Conversational AI (streaming + batch) | L778-780 |
| Quest | `/api/quest/*` | Game state, objectives, phase advancement | L792-798 |
| PEARL | `/api/pearl/*` | Per-project focusing agent | L800-804 |
| Character | `/api/character/*` | User identity and portfolio | L806-811 |
| Inference | `/api/inference/*` | Multi-backend LLM management | L819-821 |
| Iron Road | `/api/bestiary`, `/api/book/*` | Vocabulary creatures, narrative book | L816-833 |
| EYE Export | `/api/eye/*` | Compile → Preview → Export learning artifacts | L835-837 |
| Creative | `/api/creative/*` | ComfyUI images, video, music, 3D mesh | L839-850 |
| Voice | `/api/voice/*` | TTS/STT conversation (Supertonic-2, PersonaPlex) | L852-855 |
| Persistence | `/api/sessions`, `/api/projects` | Conversation history, DAYDREAM archive | L857-861 |
| RAG | `/api/rag/*` | Semantic search via SQLite in-memory embeddings | L863-864 |
| Quality | `/api/yard/score` | Pedagogical document evaluation | L866 |
| Journal | `/api/journal/*` | Chapter milestones, weekly reflections | L868-870 |
| Tools | `/api/tools/*` | Agentic tool listing and execution | L790-791 |

> 📍 `main.rs:L773-877` — Complete route table with section comments

### 1.8 The Server Module Map

The main server binary declares **37+ internal modules**:

```
crates/trinity/src/
├── main.rs              — Entry point, routes, state, Tauri host
├── agent.rs             — Multi-turn agentic chat loop
├── character_api.rs     — Portfolio artifact vaulting
├── character_sheet.rs   — CharacterSheet persistence (~/.trinity/)
├── conductor_leader.rs  — ADDIECRAPEYE phase orchestrator
├── cow_catcher.rs       — Error handling, obstacle classification
├── creative.rs          — ComfyUI/MusicGPT/Hunyuan3D client
├── edge_guard.rs        — Route-level security middleware
├── export.rs            — EYE Container → HTML5 export
├── eye_container.rs     — Bundle quest data into exportable artifact
├── gpu_guard.rs         — Hardware-safe GPU resource guard
├── health.rs            — Real health endpoint (all subsystems)
├── http.rs              — Shared HTTP clients (3 timeout profiles)
├── inference.rs         — OpenAI-compatible LLM client
├── inference_router.rs  — Multi-backend auto-detect & failover
├── jobs.rs              — Background job runner (SQLite-persisted task queue)
├── journal.rs           — Chapter milestones, weekly reflections
├── music_streamer.rs    — Background music from CharacterSheet genre
├── narrative.rs         — Great Recycler LitRPG prose generation
├── persistence.rs       — SQLite sessions, messages, projects
├── perspective.rs       — Ring 6: multi-perspective AI evaluation
├── quality_scorecard.rs — Pedagogical document scoring (5 dimensions)
├── quests.rs            — HTTP API for quest engine
├── rag.rs               — Native ONNX semantic + full-text search (ort + all-MiniLM-L6-v2)
├── rlhf_api.rs          — RLHF resonance feedback endpoint
├── scope_creep.rs       — Scope creep creature generation
├── sidecar_monitor.rs   — External service health monitoring
├── skills.rs            — Skill system integration
├── stt.rs               — Whisper STT engine (native ONNX)
├── supertonic.rs        — Supertonic-2 TTS engine (native ONNX)
├── telephone.rs         — Real-time audio-to-audio voice pipeline
├── tools.rs             — Agentic tools with permission gates
├── trinity_api.rs       — V1 Trinity chat endpoint
├── vaam.rs              — Vocabulary scanning
├── vaam_bridge.rs       — VAAM + Sacred Circuitry integration
├── voice.rs             — Voice conversation endpoints
└── voice_loop.rs        — Hands-free voice loop
```

> 📍 `main.rs:L50-101` — Module declarations

### 1.9 The Hardware Platform

Trinity is designed for a specific class of hardware: **AMD Strix Halo** with unified CPU-GPU-NPU memory architecture.

| Component | Specification | Trinity Uses For |
|-----------|--------------|-----------------|
| **CPU** | Ryzen AI Max+ 395 (16C/32T Zen 5) | Server, I/O, orchestration |
| **GPU** | Radeon 8060S (40 CUs RDNA 3.5) | LM Studio inference backend (Vulkan) |
| **NPU** | XDNA 2 (50 TOPS) | Planned: speculative decoding, embeddings, voice |
| **Memory** | 128 GB unified LPDDR5x-8000 | Shared across CPU+GPU+NPU — no copy overhead |

**Why this matters**: The 128 GB unified memory means a 68 GB model (Mistral Small 4 119B MoE) can load without any GPU VRAM carving. The CPU, GPU, and NPU all see the same memory space. This eliminates the #1 bottleneck in local AI inference.

**Agnostic Inference Architecture**: Trinity no longer embeds llama.cpp directly. Instead, it acts as a lightweight HTTP dispatcher via the `InferenceRouter`, connecting to whichever OpenAI-compatible backend the user prefers — LM Studio (port 1234), Ollama (port 11434), llama-server (port 8080), or any custom endpoint. This "Bring Your Own Pipeline" (BYOP) approach dropped build times from minutes to seconds and decoupled Trinity from any single inference engine.

**Dual KV Cache**: Mistral Small 4 runs with two KV caches at up to 1M tokens each via LM Studio's `--parallel 2`, providing **2M+ total context** — enough to hold entire curricula, textbooks, and project histories simultaneously. This happens entirely locally, with zero cloud dependency.

> 📍 `main.rs:L287-335` — `installed_model_inventory()`: all models with sizes and paths
> 📍 `inference_router.rs` — Multi-backend auto-detect and failover logic

### 1.10 The Frontend

Trinity's web UI is built with **16 React components** served from the Axum backend:

| Component | Purpose |
|-----------|---------|
| `ArtStudio.jsx` | Creative pipeline — image/music/video/3D generation via `useCreative` hook |
| `ChapterRail.jsx` | ADDIECRAPEYE phase progress rail |
| `CharacterSheet.jsx` | User identity and skill dashboard |
| `CreepCard.jsx` | Vocabulary creature display card |
| `ExpressWizard.jsx` | Streamlined Express mode wizard |
| `GameHUD.jsx` | Iron Road HUD (XP, Coal, Steam) + Safety Badges (CowCatcher/EdgeGuard/Demo) |
| `JournalViewer.jsx` | Chapter milestones and reflection viewer |
| `NavBar.jsx` | Mode-aware navigation bar |
| `OnboardingTour.jsx` | First-run guided tour |
| `PearlCard.jsx` | PEARL project focus display |
| `PerspectiveSidebar.jsx` | Ring 6 multi-perspective annotations |
| `PhaseWorkspace.jsx` | Current ADDIECRAPEYE phase workspace |
| `QualityScorecard.jsx` | Document quality evaluation display |
| `ScopeCard.jsx` | Scope creep creature encounter card |
| `SetupWizard.jsx` | Bring Your Own Mind — dynamic backend selection wizard |
| `TrainStatus.jsx` | Iron Road train progress animation |
| `Yardmaster.jsx` | IDE/Agent mode — agentic chat with DAYDREAM terminal via `useYardmaster` hook |
| `DAYDREAM` | The native Bevy sidecar — Pure Rust 3D LitRPG world. No JavaScript. Spawned as OS child process. |


> 📍 `crates/trinity/frontend/src/components/` — 18 files
> 📍 `main.rs` — Static file serving: React `frontend/dist/` with SPA fallback

### 1.11 Field Manual Cross-Reference

> *See: [Ask Pete Field Manual](ASK_PETE_FIELD_MANUAL.md) §1 Genesis Logic, §2 The Machine*

The Field Manual provides the **psychological grounding** for the architectural decisions in this car:

- **§1 Genesis Logic** — Why Trinity exists: "The Boilermaker Special doesn't ask your major. It asks you to hold on." Trinity's architecture (headless + protocol + sandbox) mirrors Purdue's approach: provide the locomotive, let the student choose the destination.
- **§2 The Machine** — The hardware section answers: "What is the machine?" The Strix Halo is not just a computer — it is a *cognitive amplifier*. The unified memory architecture means no artificial boundaries between what the CPU thinks, what the GPU creates, and what the NPU accelerates.

---

## 🚃 Car 2: DESIGN — Architecture & Crate Registry

> **Bloom's Level**: Understand / Apply
> **Sacred Circuit**: The Loom (circuit #2)
> **Body Metaphor**: The Brain — understanding the structure

### 2.1 The Protocol Crate — Trinity's Language

Everything in Trinity starts with `trinity-protocol`. It defines the **shared language** — every type, enum, and struct that crosses crate boundaries:

```
crates/trinity-protocol/src/
├── lib.rs                — 26 public modules, re-exports (146 lines)
├── agents.rs             — Agent types, memory config, metrics
├── artifact.rs           — Artifact graph: nodes, edges, plan tasks
├── asset_generation.rs   — Yardmaster content templates & generation
├── brain.rs              — Brain service client trait
├── bridge.rs             — MemoryBridge context management
├── character_sheet.rs    — CharacterSheet, LDT Portfolio, IBSTPI domains
├── diffusion.rs          — Image generation types
├── id_contract.rs        — Learning objectives, quest milestones
├── memory.rs             — Memory service client trait
├── ontology.rs           — Material integrity ontology
├── pearl.rs              — PEARL focusing agent (612 lines)
├── production.rs         — Production types
├── profile.rs            — User profile, project profile, bestiary
├── qm_rubric.rs          — Quality Matters rubric evaluator
├── sacred_circuitry.rs   — 15-concept cognitive scaffolding (796 lines)
├── semantic_creep.rs     — SemanticCreep creatures, battle system
├── sidecars.rs           — Sidecar service definitions
├── state.rs              — State types
├── stream.rs             — SSE stream events, orchestrator config
├── task.rs               — Task types
├── trinity_mcp_server.rs — MCP safe modification engine
├── tutorial_events.rs    — Tutorial event types
├── types.rs              — Core shared types
├── vaam_profile.rs       — VAAM communication style profile
├── vocabulary.rs         — Vocabulary database, tiers, genres
└── yardmaster_generator.rs — Content generation parameters
```

> 📍 `trinity-protocol/src/lib.rs:L1-17` — ADDIECRAPEYE "Sign on the Cave Wall" header
> 📍 `trinity-protocol/src/lib.rs:L55-65` — Philosophy block: "Language is the substrate of thought"
> 📍 `trinity-protocol/src/lib.rs:L67-92` — 26 module declarations
> 📍 `trinity-protocol/src/lib.rs:L94-145` — Type re-exports (CharacterSheet, Pearl, Circuit, SemanticCreep, etc.)

The protocol's design philosophy is encoded in its own doc comment:

> *"Language is the substrate of thought. The Protocol defines the potential limits of communication between the Mind (Brain) and the Body. It must be expressive, type-safe, and future-proof."*

Three rules govern protocol changes:
1. **Type Safety**: Use Rust enums to make illegal states unrepresentable
2. **Compatibility**: Append, don't break
3. **Clarity**: Type names should be self-documenting

### 2.2 The Workspace Layout

```toml
[workspace]
members = [
    "crates/trinity",                  # Main entry, HTTP server, UI shell
    "crates/trinity-protocol",         # Shared types, ADDIE enums, HeroStage
    "crates/trinity-quest",            # Quest board, XP/Coal/Steam
    "crates/trinity-iron-road",        # Book writing, narrative
    "crates/trinity-voice",            # PersonaPlex, Ask Pete
    "crates/trinity-bevy-graphics",    # Bevy 3D Yard
]
```

> 📍 `Cargo.toml:L1-26` — Workspace definition with role annotations

### 2.3 Dependency Philosophy

Trinity's `[workspace.dependencies]` reveals its architectural priorities:

| Dependency | Version | Why |
|-----------|---------|-----|
| `tokio` | 1.0 (full) | Async runtime — everything is non-blocking |
| `axum` | (latest) | HTTP framework — thin, composable, tower-compatible |
| `serde` + `serde_json` | 1.0 | Every type is serializable — the Protocol is JSON-native |
| `sqlx` | 0.7 | Async SQLite — zero-config local file database |
| `bevy` | 0.18.1 | 3D game engine — the Spatial Sandbox |
| `reqwest` | 0.11 | HTTP client for sidecar communication |
| `tracing` | 0.1 | Structured logging — every operation is observable |
| `uuid` + `chrono` | latest | Identity and time — nothing is anonymous or timeless |

> 📍 `Cargo.toml:L44-97` — Workspace dependencies with category comments

### 2.4 The Module Inventory

The main server crate (`crates/trinity/src/`) has **31 internal modules** registered at the top of `main.rs`:

> 📍 `main.rs:L43-73` — 31 `mod` declarations

Grouped by responsibility:

**AI & Inference** (5 modules):
- `agent.rs` — Multi-turn agentic chat with tool-calling loop
- `inference.rs` — OpenAI-compatible HTTP client for any LLM
- `inference_router.rs` — Multi-backend auto-detect and failover (LM Studio → llama-server → Ollama)
- `rag.rs` — Native ONNX semantic + full-text search (ort + all-MiniLM-L6-v2)
- `perspective.rs` — Ring 6 multi-perspective evaluation

**Game Mechanics** (5 modules):
- `quests.rs` — Quest HTTP API (state, objectives, phases)
- `scope_creep.rs` — Scope creep creature generation
- `vaam.rs` — Vocabulary scanning
- `vaam_bridge.rs` — VAAM + Sacred Circuitry integration
- `narrative.rs` — Great Recycler LitRPG prose generation

**Content & Export** (3 modules):
- `creative.rs` — ComfyUI/MusicGPT/Hunyuan3D client (1156 lines)
- `export.rs` — EYE Container → HTML5 export
- `eye_container.rs` — Bundle quest data into exportable artifact

**User Identity** (3 modules):
- `character_sheet.rs` — Persistence to `~/.trinity/`
- `character_api.rs` — Portfolio artifact vaulting endpoint
- `skills.rs` — Skill system integration

**Infrastructure** (8 modules):
- `persistence.rs` — SQLite sessions, messages, projects
- `rag.rs` — SQLite in-memory semantic + full-text search
- `health.rs` — Real health endpoint (all subsystems)
- `http.rs` — Shared HTTP clients (3 timeout profiles)
- `gpu_guard.rs` — Hardware-safe GPU resource guard
- `sidecar_monitor.rs` — External service health monitoring
- `cow_catcher.rs` — Error handling, obstacle classification
- `journal.rs` — Chapter milestones, weekly reflections

**Voice** (2 modules):
- `voice.rs` — Voice conversation endpoints
- `voice_loop.rs` — Hands-free voice loop

**System** (5 modules):
- `conductor_leader.rs` — ADDIECRAPEYE phase orchestrator
- `tools.rs` — Agentic tools with permission gates
- `quality_scorecard.rs` — Pedagogical document scoring
- `trinity_api.rs` — V1 Trinity chat endpoint
- `music_streamer.rs` — Background music from CharacterSheet

**Feedback** (1 module):
- `rlhf_api.rs` — RLHF resonance feedback endpoint (includes `ResonanceRating` types)

---

*Cars 3–12 continue in the ADDIECRAPEYE sequence. The next group (Car 3: DEVELOP) documents the framework that governs everything above.*

---

## 🚃 Car 3: DEVELOP — The ADDIECRAPEYE Framework

![The 12 ADDIECRAPEYE stations — a circular journey through instructional design](/images/addiecrapeye_phases.png)
> **Bloom's Level**: Create
> **Sacred Circuit**: Prepare (circuit #4)
> **Body Metaphor**: The Skeleton — building the frame that holds everything

### 3.1 The 12 Stations

ADDIECRAPEYE is a 12-station instructional design framework that fuses three methodologies:

- **ADDIE** (5 stations): Analyze → Design → Develop → Implement → Evaluate — the gold standard of instructional systems design, originally developed by the Center for Educational Technology at Florida State University (c. 1975) for the U.S. Army (Molenda, 2003).
- **CRAP** (4 stations): Contrast → Repetition → Alignment → Proximity — foundational visual design principles coined by Robin Williams in *The Non-Designer's Design Book* (Peachpit Press, 1994).
- **EYE** (3 stations): Envision → Yoke → Evolve — **original contribution by Joshua Atkinson** (2026) as part of the Trinity ID AI OS project. EYE adds a reflective vision-iteration loop that completes the 12-station cycle. It is not derived from any existing instructional design framework.

```rust
pub enum AddiecrapeyePhase {
    Analysis,      // 1.  The Golem's Eyes
    Design,        // 2.  The Golem's Brain
    Development,   // 3.  The Golem's Skeleton
    Implementation,// 4.  The Golem's Muscles
    Evaluation,    // 5.  The Golem's Voice
    Contrast,      // 6.  The Golem's Skin
    Repetition,    // 7.  The Golem's Heart
    Alignment,     // 8.  The Golem's Spine
    Proximity,     // 9.  The Golem's Hands
    Envision,      // 10. The Golem's Third Eye
    Yoke,          // 11. Connective Tissue
    Evolve,        // 12. The Golem's Lungs
}
```

> 📍 `conductor_leader.rs:L36-49` — `AddiecrapeyePhase` enum (12 variants)
> 📍 `conductor_leader.rs:L70-88` — `next()` method: cyclic state machine (Evolve → Analysis)
> 📍 `trinity-protocol/src/lib.rs:L1-17` — "Sign on the Cave Wall" showing all 12 stations

### 3.2 Bloom's Taxonomy Integration

Each ADDIECRAPEYE station maps to a specific Bloom's cognitive level. This is the **pedagogical backbone** — it tells Pete what type of thinking the learner should be doing at each station:

| Station | Bloom's Level | Cognitive Activity |
|---------|--------------|-------------------|
| Analysis | Remember/Understand | Extract intent, identify audience, recall prior knowledge |
| Design | Apply | Map objectives to mechanics, apply Bloom's verbs to goals |
| Development | Create | Build assets, write code, produce tangible artifacts |
| Implementation | Apply | Deploy, integrate, verify setup matches design |
| Evaluation | Evaluate | QM rubric, quality review, measure against criteria |
| Contrast | Analyze | Visual hierarchy analysis, emphasis ranking, boundary design |
| Repetition | Apply | Pattern reinforcement, consistency audit, core loop solidity |
| Alignment | Evaluate | Structure compliance, scope pruning, Extraneous Load → zero |
| Proximity | Analyze | UX grouping, Miller's Law (7±2), human-computer interaction |
| Envision | Evaluate | Meta-cognitive reflection, compare against original goals |
| Yoke | Create | System integration, couple frontend to backend, final assembly |
| Evolve | Create | Ship, publish, the Golem takes its first breath |

> 📍 `conductor_leader.rs:L109-154` — `bloom_level()` method with full cognitive descriptions

### 3.3 Sacred Circuitry — The 15-Word Cognitive Scaffolding

The Sacred Circuitry is a **15-concept attention scaffolding system** that structures *how* the AI focuses during any ADDIECRAPEYE workflow. Where ADDIECRAPEYE says *what* to do, the Circuitry says *how to attend*.

```
Scope:  Center → Expand → Balance → Prepare  (define the problem)
Build:  Express → Extend → Unlock → Flow      (produce the work)
Listen: Receive → Relate → Realize             (process feedback)
Ship:   Act → Transform → Connect → Manifest   (deliver output)
```

> 📍 `sacred_circuitry.rs:L56-76` — `Circuit` enum with 15 variants
> 📍 `sacred_circuitry.rs:L80-96` — `Circuit::ALL` array (canonical order)
> 📍 `sacred_circuitry.rs:L124-138` — `quadrant()` method mapping circuits to 4 attention phases

Each circuit has:
- A **name** (one word: Center, Expand, Balance, etc.)
- A **quadrant** (Scope, Build, Listen, Ship)
- A **description** explaining what the attention pattern does
- An **ADDIECRAPEYE station** mapping
- An **auto-reply** — structured phrases the AI selects to signal which pattern is active

Example circuit descriptions:

| Circuit | What It Does |
|---------|-------------|
| Center | Lock attention onto the core problem. Filter noise. Identify the single most important thing. |
| Expand | Survey the solution space. What approaches exist? What adjacent knowledge applies? |
| Balance | Weigh tradeoffs. Hold competing constraints without premature commitment. |
| Flow | Enter sustained productive execution. Trust the process. Maintain momentum. |
| Realize | The insight moment. Understand the root cause. See what the right approach actually is. |
| Manifest | Deliver and release. The work is done. Let it go into the world. |

> 📍 `sacred_circuitry.rs:L172-201` — `description()` method with plain-English attention definitions

### 3.4 The Circuit ↔ Station Isomorphism

Each Sacred Circuit maps to an ADDIECRAPEYE station, creating a dual-layer scaffolding:

| Circuit | Quadrant | → Station |
|---------|----------|-----------|
| Center | Scope | Analyze |
| Expand | Scope | Design |
| Balance | Scope | Design |
| Prepare | Scope | Develop |
| Express | Build | Implement |
| Extend | Build | Envision |
| Unlock | Build | Contrast |
| Flow | Build | Repetition |
| Receive | Listen | Evaluate |
| Relate | Listen | Alignment |
| Realize | Listen | Proximity |
| Act | Ship | Proximity |
| Transform | Ship | Yoke |
| Connect | Ship | Yoke |
| Manifest | Ship | Evolve |

> 📍 `sacred_circuitry.rs:L140-168` — `addiecrapeye_station()` method with full mapping

### 3.5 The Hotel Management Pattern

The Conductor orchestrates which AI model runs during each phase. In multi-model configurations, different phases require different capabilities. The phases are grouped into **4 P-ART gears** to prevent thrashing:

| Gear | Model Role | Phases |
|------|-----------|--------|
| **P** (Pete) | Socratic Mirror | Analysis, Envision, Yoke, Evolve |
| **A** (Aesthetics) | Visual Design | Design, Contrast, Proximity |
| **R** (Research) | Quality/Testing | Evaluation, Alignment |
| **T** (Tempo) | Code Generation | Development, Implementation, Repetition |

> 📍 `conductor_leader.rs:L466-503` — `manage_hotel_sidecars()` with P-ART gear routing
> 📍 `conductor_leader.rs:L496-500` — Lone Wolf Mode: single Mistral handles all phases (current)

In the current **Lone Wolf** configuration, Mistral Small 4 119B handles all gears — but the routing infrastructure is ready for multi-model deployment.

### 3.6 Phase Implementation: Socratic Prompts

Each ADDIECRAPEYE phase has a distinct system prompt that teaches Pete its role. Every prompt follows the **Socratic Protocol**: ask, don't tell. Present choices, don't dictate.

Example from the Analysis phase:

```
SOCRATIC PROTOCOL: Do not tell — ask. Lead with questions:
- 'Who will use what you're building, and what do they struggle with?'
- 'What does success look like — not for you, but for the learner?'
- 'What words must someone master to understand your subject?'
```

> 📍 `conductor_leader.rs:L596-633` — Analysis phase: Socratic extraction, "Golem's Eyes"
> 📍 `conductor_leader.rs:L635-672` — Design phase: "Golem's Brain", Bloom's discovery
> 📍 `conductor_leader.rs:L674-714` — Development phase: "Golem's Skeleton", user-led build
> 📍 `conductor_leader.rs:L716-745` — Implementation phase: "Golem's Muscles", scope creep flags
> 📍 `conductor_leader.rs:L747-800+` — Evaluation phase: QM Rubric integration, "the Mirror"

### 3.7 Field Manual Cross-Reference

> *See: [Ask Pete Field Manual](ASK_PETE_FIELD_MANUAL.md) §3 The Process, §4 The Promise*

- **§3 The Process** — ADDIECRAPEYE is the process. The Field Manual calls it "the railroad that never stops." Each station is a stop, and the Conductor (Pete) keeps the train moving.
- **§4 The Promise** — "To a student who shows up: you are the Subject Matter Expert. Pete does not replace you. He scaffolds you."

---

## 🚃 Car 4: IMPLEMENT — The Iron Road Game Mechanics

> **Bloom's Level**: Apply
> **Sacred Circuit**: Express (circuit #5)
> **Body Metaphor**: The Muscles — putting the framework into motion

### 4.1 The Iron Road Core Loop

The Iron Road is Trinity's game layer — it transforms dry instructional design into an engaging experience. The core loop has 5 phases:

```
COLLECT   → scan user text for vocabulary, create Wild Creeps
CONSTRUCT → tame Creeps via multi-dimensional TamingProgress
QUEST     → fill Lesson MadLib slots with Tamed Creeps
BATTLE    → Creep vs Creep for contested MadLib slots
REWARDS   → Context Points, VaamProfile update, Book chapter generation
```

> 📍 `trinity-iron-road/src/game_loop.rs:L1-23` — Architecture header defining the 5-phase loop
> 📍 `trinity-iron-road/src/lib.rs:L1-38` — Crate structure: book, great_recycler, narrative, game_loop, vaam, pete_core

### 4.2 VAAM — Vocabulary Acquisition Autonomy Mastery

**VAAM** is the core insight that makes Trinity unique among AI tools: *words are what LLMs and people have in common*. Instead of treating vocabulary as static content, Trinity makes it the **game currency**.

Every message between user and AI is scanned for vocabulary. Words with 4+ characters are tracked. The system monitors:

1. **Passive discovery** — the word appears in conversation
2. **Multi-phase usage** — the word appears across different ADDIECRAPEYE phases
3. **Deliberate usage** — the user intentionally selects the word (not just passive appearance)
4. **Intent alignment** — how well the word usage matches the user's stated intent

> 📍 `game_loop.rs:L61-109` — `scan_text()`: word discovery across phases and quadrants
> 📍 `game_loop.rs:L72` — `filter(|w| w.len() >= 4)` — minimum 4 characters to be interesting

### 4.2.1 VAAM as Intentional Intelligence — Value to AI Systems

> *"The AI industry measures intelligence in tokens. Trinity measures it in words understood."*

What makes VAAM structurally unique in the AI landscape is not **what** it tracks, but **how** it transforms tracking into control:

**1. Bidirectional Vocabulary Alignment**

VAAM scans **both** user input and AI output through the same circuit detection system. The VaamBridge (`vaam_bridge.rs`) processes user messages for vocabulary mastery AND processes AI responses for Sacred Circuitry alignment. No other system treats the AI's own vocabulary usage as a measurable, adjustable signal.

> 📍 `vaam_bridge.rs:L90-165` — `process_user_input()`: full pipeline (vocabulary + circuit + profile)
> 📍 `vaam_bridge.rs:L167-185` — `process_ai_output()`: circuit detection on AI responses
> 📍 `sacred_circuitry.rs:scan_ai_alignment()` — AI Coal engine: on-circuit = focused, off-circuit = drifting

**2. Deliberateness Scoring**

`WordWeight.affinity = frequency × deliberateness`. VAAM doesn't just count how often a word appears — it tracks whether the user **chose** it when alternatives existed. A word used 10 times passively scores lower than a word used 3 times deliberately. This distinguishes understanding from parroting.

> 📍 `vaam_profile.rs:L298-308` — `WordWeight.recalculate()`: affinity = frequency × deliberate ratio

**3. Negotiated Attention Contracts**

`VaamProfile.agreements` are explicit contracts between user and AI about what matters. Built through conversation, referenced in every system prompt. This is not RLHF (thumbs up/down on outputs) — it is **co-created structural preference** that persists across sessions and shapes AI behavior by contract, not by statistical averaging.

> 📍 `vaam_profile.rs:L354-369` — `Agreement`: topic, circuit, weight, timestamps
> 📍 `vaam_bridge.rs:L204-270` — `prompt_context()`: agreements injected into system prompt

**4. Cognitive Load Theory as Runtime Telemetry**

The Coal/Steam/Friction economy is CLT (Sweller, 1988) implemented as game mechanics:

| CLT Concept | Trinity Mechanic | Measurement | Effect |
|-------------|-----------------|-------------|--------|
| **Intrinsic Load** | Coal | Words on-circuit vs. off-circuit | AI self-regulation via system prompt |
| **Germane Load** | Steam | Productive tool calls | Quest advancement gates |
| **Extraneous Load** | Friction | RLHF negative signals | Pete's tone adjustment |

No other AI framework applies cognitive load theory to the AI's own attention management. Agent frameworks (LangChain, CrewAI, AutoGen) provide memory and tools but no cognitive scaffolding. LMS systems (Canvas, Moodle) manage courses but not cognitive load. Intelligent Tutoring Systems (ANDES, Carnegie Mellon Cognitive Tutors) tracked student knowledge states but used pre-authored rule systems, not live LLMs.

**The structural claim:** Trinity is the first system that applies Cognitive Load Theory to both the human AND the AI simultaneously, using vocabulary as the shared measurement unit, structured by a game engine that makes pedagogical rigor feel like play.

### 4.3 SemanticCreep — Vocabulary Creatures

Every vocabulary word becomes a **SemanticCreep** — a creature with stats derived from its linguistic properties:

- **Element** — determined by word characteristics (Fire, Water, Earth, Air, Lightning, Crystal)
- **Role** — Guardian, Scout, Healer, Tank, Striker, Controller
- **Stats** — derived from word length, syllable count, Bloom's affinity
- **State** — Wild → Tameable → Tamed → Evolved

The taming process is **multi-dimensional** — a word isn't tamed by repetition alone. It requires:
1. Usage across **different ADDIECRAPEYE phases** (Arithmos — counting dimensions)
2. Usage across **different Sacred Circuitry quadrants** (Harmonia — balanced contexts)
3. **Deliberate** selection by the user, not just passive appearance (Logos — intentional meaning)
4. A **minimum taming score** threshold before the Scope Hope/Nope decision

> 📍 `trinity-protocol/src/semantic_creep.rs` — `SemanticCreep` struct with element, role, stats, TamingProgress
> 📍 `game_loop.rs:L111-120` — `scope_hope_creep()`: user tames the word
> 📍 `game_loop.rs:L122-127` — `scope_nope_creep()`: user rejects the word (stays wild)

### 4.4 The Bestiary

The `CreepBestiary` is the player's vocabulary collection — their Pokédex of words:

```rust
pub struct CreepBestiary {
    pub creeps: Vec<SemanticCreep>,     // All encountered Creeps
    pub words_scanned: u64,            // Total words processed
    pub creeps_tamed: u32,             // Successfully tamed words
    pub slots_filled: u32,             // MadLib slots completed
    pub battles_won: u32,              // PvP vocabulary battles
}
```

> 📍 `game_loop.rs:L32-44` — `CreepBestiary` struct
> 📍 `game_loop.rs:L130-132` — `usable_creeps()`: only Tamed or Evolved words are usable
> 📍 `game_loop.rs:L150-163` — `summary()` method for UI display

### 4.9 The Pythagorean PPPPP

To ensure absolute adherence to the **Doctrine of Systems Isomorphism**, every active Rust data structure in Trinity maps to the **Pythagorean PPPPP** (The 5Ps). This guarantees the LitRPG game layer isn't a superficial skin; it is mathematically identical to the Instructional Design engine.

- **Psychology (The Human Element)**: Code managing cognitive friction limiters.
  - `ShadowStatus`: Prevents complete systemic collapse by invoking the Heavilon Protocol when user resilience (imposter syndrome) drops.
  - `vulnerability`: Tracks structured willingness to fail directly measuring pedagogical courage.
- **Philosophy (The Intent)**: Code driving behavioral depth and metrics.
  - `IntentPosture`: Efficiency (speed over retention) vs Mastery (productive struggle scaffolding).
  - `ResonanceLevel`: The lifetime metric of domain mastery, stored in the `CharacterSheet`.
- **Pedagogy (The Method)**: Code providing the scaffolding.
  - `HookDeck` & `HookCard`: 37 Instructional Design spells cast to combat structural anomalies.
  - `LdtPortfolio` & `VaamBridge`: Vaulting competencies inside academic standards.
- **Programming (The Infrastructure)**: Code managing literal, physical hardware.
  - `mana_pool_vram` & `stamina_ram`: GPU metrics dictating maximum LLM context batching limits.
- **Production (The Execution)**: Code managing the actual workflow events.
  - `Coal` (attention) and `Steam` (momentum): Explicit currency modifiers affecting AI generation depth.
  - `TrackFriction`: Context-dependent penalties mapped directly to Extraneous Cognitive Load.

> 📍 `context.md` — Detailed breakdown and implementation tracking.
> 📍 `TRINITY_MECHANICS_MAP.md` — Mermaid flowchart visualization of the isomorphic architecture.

### 4.10 The TCG HookDeck

The **Hook Book** has been fully industrialized into a Trading Card Game (TCG) mechanic, where "Hooks" are physical cards used to interact with the LLM API and combat Semantic Creeps.

When a word is detected as out-of-bounds (a Scope Creep), a `ScopeCreepModal` invokes the `POST /api/bestiary/tame` API. Instead of simply rejecting the request, the user selects a *Hook Card* from their active `hook_deck` (e.g., 'Socratic Interview', 'Quality Scorecard') to tame the specific anomaly. 

Each `HookCard` tracks its own specific:
- `level` (Current competency inside the system)
- `xp` (Earned from repeated, successful tames)
- `creeps_tamed` (Total number of anomalies corrected by this specific design mechanism)

The starter deck contains four Level 1 cards representing the 4 Schools of Magic (Pedagogy, Creation, Systems, Identity). 

> 📍 `trinity-protocol/src/character_sheet.rs` — Definition of `HookCard`.
> 📍 `HOOK_BOOK.md` — Detailed expansion of the 37 unreleased cards and schools.

### 4.4.1 Scout Sniper — Dual-Mode Scope Management

> *"The Scout sees hope. The Sniper sees nope. Both serve the Iron Road."*

When a player's ideas generate **SemanticCreeps** (out-of-scope vocabulary or feature requests), Programmer Pete activates the **Scout Sniper** — a dual-mode decision tool:

| Mode | Call | Action | Iron Road Metaphor |
|------|------|--------|-------------------|
| 🔭 **Scout** ("Scope Hope") | `POST /api/bestiary/tame {decision: "hope"}` | Tames the creep → create MOC → fill with code → test → deliver | Lay tracks → Load cargo → Ship cargo → Get paid |
| 🎯 **Sniper** ("Scope Nope") | `POST /api/bestiary/tame {decision: "nope"}` | Bags & Tags → marks as not achievable, redundant, or counter-productive | Bestiary record — product maturity = knowing what you ARE and what you ARE NOT |

**The flow:**
```
Player speaks in Story Mode
  → Great Recycler judges scope alignment
    → IN-SCOPE: stays in Design Doc (YOUR PRODUCT panel)
    → OUT-OF-SCOPE: SemanticCreep spawns in Bestiary
      → Pete activates Scout Sniper
        → SCOUT (hope): tame it, build it, ship it
        → SNIPER (nope): bag it, tag it, record it
```

> 📍 `scope_creep.rs` — `detect_scope_creep()` trigger detection
> 📍 `main.rs:scope_creep_decision()` — Hope/Nope handler at `/api/bestiary/tame`
> 📍 `PhaseWorkspace.jsx` — Frontend tame/nope UI
> 📍 `CreepCard.jsx` / `ScopeCard.jsx` — Creature display cards

**RLHF Economy — "Pay Pete":**

The Scout Sniper earns rewards through the RLHF system. Every decision costs Coal (compute budget) and generates Steam (momentum) + XP (experience). Trinity grows with the user based on this data:

| Decision | Coal Cost | Steam Reward | XP | Friction | Effect |
|----------|-----------|-------------|-----|----------|--------|
| 🔭 Scout (Hope) | -5 | +8 | +10 | -3% | Tame → build → ship → maturity ↑ |
| 🎯 Sniper (Nope) | -2 | +3 | +3 | -1% | Tag → record → scope clarity ↑ |

> *"The player creates the monsters. Pete processes them. Coal is spent, steam is earned. The maturity map grows."*

### 4.5 Lesson MadLibs

Once Creeps are tamed, they fill slots in **Lesson MadLibs** — structured lesson plan templates with typed gaps:

```
"The {noun_topic} is {adjective_quality}."
→ Fill noun_topic with your tamed "geology" Creep
→ Fill adjective_quality with your tamed "creative" Creep
→ Result: "The geology is creative."
```

When all slots are filled, the lesson is complete and generates a `LessonCompleted` event that flows into the Book system.

> 📍 `trinity-iron-road/src/vaam/madlibs.rs` — `LessonMadlib` struct with typed slots
> 📍 `game_loop.rs:L274-302` — `complete_lesson()`: fill slots → update VaamProfile → emit events

### 4.6 The Game Loop Events

Every action in the Iron Road produces typed events that drive the narrative:

| Event | Trigger | Effect |
|-------|---------|--------|
| `CreepDiscovered` | New word found in text | UI shows wild creature card |
| `CreepTameable` | Multi-dimensional score threshold reached | Scope Hope/Nope decision prompt |
| `SlotFilled` | Tamed Creep placed in MadLib slot | Context Points earned |
| `BattleResolved` | Two Creeps compete for same slot | Winner keeps the slot |
| `LessonCompleted` | All MadLib slots filled | Book chapter generated |

> 📍 `game_loop.rs:L172-212` — `GameLoopEvent` enum with 5 variants
> 📍 `game_loop.rs:L214-272` — `to_recycler_event()`: bridges game events to Book chapters

### 4.7 The Book of the Bible

Completed lessons flow into the **Book of the Bible** — an append-only narrative ledger that records the user's learning journey as LitRPG prose. The Great Recycler (narrative AI) transforms mechanical game events into story chapters.

> 📍 `trinity-iron-road/src/book.rs` — `BookOfTheBible` struct: append-only narrative ledger
> 📍 `trinity-iron-road/src/great_recycler.rs` — `RecyclerEvent` to narrative chapter conversion
> 📍 `main.rs:L125-128` — Book in AppState with Pythagorean documentation

### 4.8 Test Coverage

The game loop has **16 unit tests** validating the core mechanics:

| Test | What It Validates |
|------|-------------------|
| `test_scan_discovers_creeps` | Words 4+ chars become CreepDiscovered events |
| `test_scan_repetition_does_not_auto_tame` | Same-phase repetition ≠ taming (prevents gaming) |
| `test_scan_multi_phase_makes_tameable` | Cross-phase + deliberate usage → tameable |
| `test_usable_creeps_filters` | Only Tamed/Evolved Creeps are "usable" |
| `test_lesson_completion_events` | Filled MadLib → LessonCompleted event |
| `test_event_to_recycler_event` | Game events bridge to Book chapters correctly |
| `test_save_and_load_state` | Bestiary JSON persistence round-trips |
| `test_bestiary_summary` | UI summary string formats correctly |

> 📍 `game_loop.rs:L328-510` — 8 `#[test]` functions with assertion coverage

### 4.9 Field Manual Cross-Reference

> *See: [Ask Pete Field Manual](ASK_PETE_FIELD_MANUAL.md) §5 The Iron Network, §6 The Heavilon Algorithm*

- **§5 The Iron Network** — "The user is the locomotive. Trinity is the track." The Iron Road game mechanics are the track — they guide without constraining.
- **§6 The Heavilon Algorithm** — "Failure is data, not death." When a Creep loses a battle or a lesson fails QM review, the system recycles the event as learning data, not punishment.

---

## 🚃 Car 5: EVALUATE — Quality Systems & Security Rings

> **Bloom's Level**: Evaluate
> **Sacred Circuit**: Receive (circuit #9)
> **Body Metaphor**: The Nervous System — sensing quality and security

### 5.1 The Ring System

Trinity's security and quality model is organized as **concentric rings**, each providing a different type of protection:

| Ring | Name | Purpose | Implementation |
|------|------|---------|----------------|
| **Ring 1** | Tool Permissions | Three-tier tool classification | `tools.rs:L58-106` |
| **Ring 2** | Persona Gates | Persona-specific system prompts | `agent.rs:L88-126` |
| **Ring 3** | Rolling Context | Summary-based context management | `agent.rs` context window |
| **Ring 4** | Session Isolation | Per-user character persistence | `character_sheet.rs:L84` |
| **Ring 5** | Sandboxing | Command blocking + path validation | `tools.rs:L431-478` |
| **Ring 6** | Perspective Engine | Multi-perspective AI evaluation | `perspective.rs` |

### 5.2 Ring 1: Tool Permissions

Every tool in Trinity has a permission level that determines how it's handled:

```rust
pub enum ToolPermission {
    Safe,           // Read-only / informational — always execute
    NeedsApproval,  // Modifies state within workspace — log and proceed
    Destructive,    // System-level or destructive — require confirmation
}
```

> 📍 `tools.rs:L58-67` — `ToolPermission` enum (3 variants)
> 📍 `tools.rs:L70-106` — `tool_permission()`: maps 30 tool names to permission levels

**Permission distribution:**
- **Safe** (11 tools): `read_file`, `list_dir`, `list_files`, `search_files`, `quest_status`, `cowcatcher_log`, `sidecar_status`, `process_list`, `system_info`, `load_session_context`, `zombie_check`
- **NeedsApproval** (13 tools): `write_file`, `cargo_check`, `quest_advance`, `work_log`, `task_queue`, `save_session_summary`, `generate_lesson_plan`, `generate_rubric`, `generate_quiz`, `curriculum_map`, `scout_sniper`, `analyze_document`, `analyze_image`
- **Destructive** (7 tools): `shell`, `python_exec`, `sidecar_start`, `scaffold_bevy_game`, `project_archive`, `avatar_pipeline`, `generate_image`

Unknown tools default to **Destructive** — the most restrictive level.

> 📍 `tools.rs:L104` — `_ => ToolPermission::Destructive` — unknown = most restrictive

### 5.3 Ring 2: Persona-Based Access Control

Trinity uses **dual persona preambles** — the same AI brain operates in two distinct modes, each with different cognitive patterns:

| Persona | Mode | Thinking Style |
|---------|------|---------------|
| **Great Recycler** 🔮 | Strategic | Expansive, connective, asks WHY before HOW |
| **Programmer Pete** ⚙️ | Execution | Focused, pragmatic, ACT FIRST |

Persona selection is achieved via **system prompt differentiation** — the active persona's preamble is prepended to each inference request. The historical `id_slot` / `persona_slot()` KV cache routing mechanism has been archived; the system now relies on LM Studio's `--parallel 2` slots at the infrastructure level, with persona selection handled purely at the prompt layer.

> 📍 `agent.rs` — `GREAT_RECYCLER_PREAMBLE`: "chronicler of ideas, architect of systems"
> 📍 `agent.rs` — `PROGRAMMER_PETE_PREAMBLE`: "the builder, the debugger, the one who ships"

### 5.4 Ring 5: Command Sandboxing

The shell tool blocks **44 dangerous command patterns** across 6 categories:

| Category | Examples | Count |
|----------|---------|-------|
| Filesystem destruction | `rm -rf /`, `mkfs`, `dd if=`, fork bomb | 7 |
| System control | `shutdown`, `reboot`, `systemctl disable` | 3 |
| Privilege escalation | `sudo`, `su -`, `passwd`, `chmod -R 777 /` | 5 |
| Process killing | `pkill -9`, `kill -9 1`, `killall` | 3 |
| Network exfiltration | `curl | bash`, `nc -e`, `/dev/tcp/` | 8 |
| Data exfiltration | `scp`, `rsync`, `sftp`, `| python` | 7 |

> 📍 `tools.rs:L431-478` — Blocked command patterns with category comments
> 📍 `tools.rs:L426-429` — `dry_run` parameter: preview commands without executing

**Path validation** provides additional sandboxing:

- **Read access**: workspace + entire home directory
- **Write access**: workspace + `~/.local/share/trinity/` + `~/Workflow/` + `/tmp/`
- **Auto-backup**: existing files are backed up with timestamp before overwrite

> 📍 `tools.rs:L271-318` — `validate_path_with_mode()`: read vs. write path sandboxing
> 📍 `tools.rs:L362-373` — Auto-backup on write: `file.bak.YYYYMMDD_HHMMSS`

### 5.5 Ring 6: The Perspective Engine

Ring 6 evaluates Pete's responses through **multiple lenses** before the user sees them. Each lens is a short, focused LLM call that **annotates** — never modifies — Pete's output:

| Lens | What It Checks |
|------|---------------|
| **Bloom's Check** | Does Pete's response match the current phase's Bloom's verb? |
| **Practitioner** | Would an experienced teacher in this subject agree? |
| **Devil's Advocate** | What assumption is Pete making that could be wrong? |

> 📍 `perspective.rs:L1-29` — Architecture comment: three default lenses, parallel execution, 100-token budget each
> 📍 `perspective.rs` — Lenses fire via `tokio::join!` and results sent as SSE "perspective" events

### 5.6 Quality Matters Rubric — Automated Evaluation

Trinity includes a **complete QM Rubric evaluator** that scores instructional design contracts across 4 criteria:

| Criterion | QM Standard | What It Measures |
|-----------|-------------|-----------------|
| Learning Objectives | QM 2.1-2.4 | Measurable verbs, conditions, criteria, content depth |
| Action Mapping | QM 3.1-3.3 | Goal specificity, observable behaviors |
| Assessment Alignment | QM 4.1-4.3 | Milestone clarity, reasonable cognitive cost |
| Cognitive Load | QM 5.1-5.3 | Bloom's appropriateness, total coal cost, chunk size (3-7) |

```rust
pub struct QmEvaluation {
    pub overall_score: f32,        // 0-100 aggregate
    pub criteria: Vec<QmCriterion>, // 4 individual scores
    pub meets_standards: bool,     // overall ≥ 70 AND all criteria met
    pub feedback: Vec<String>,     // Human-readable feedback
}
```

> 📍 `qm_rubric.rs:L19-34` — `QmEvaluation` struct
> 📍 `qm_rubric.rs:L52-124` — `QmRubricEvaluator::evaluate()`: runs all 4 criteria
> 📍 `qm_rubric.rs:L96` — Passing threshold: `overall >= 70.0 && criteria.iter().all(|c| c.met)`
> 📍 `qm_rubric.rs:L357-389` — 26 measurable verbs aligned with Bloom's Taxonomy

The evaluator checks for 26 measurable verbs (identify, list, define, describe, explain, summarize, compare, contrast, analyze, evaluate, create, apply, demonstrate, implement, solve, calculate, measure, classify, arrange, construct, design, formulate, judge, critique, assess, recommend, justify).

### 5.7 Quality Scorecard — Document Evaluation

Beyond the QM Rubric (which evaluates instructional *contracts*), the **Quality Scorecard** evaluates uploaded *documents* across 5 pedagogical dimensions:

| Dimension | What It Measures |
|-----------|-----------------|
| Bloom's Coverage | Are all 6 cognitive levels represented? |
| ADDIE Alignment | Does the document follow instructional design phases? |
| Accessibility | Readability, structure, alt text |
| Student Engagement | Hooks, interactivity, variety |
| Assessment Clarity | Rubrics, measurable objectives |

> 📍 `quality_scorecard.rs:L1-25` — Architecture: "NotebookLM summarizes your syllabus. Trinity tells you what's missing."

This is Trinity's **competitive differentiator** against tools like NotebookLM: Trinity doesn't just summarize content — it evaluates its pedagogical quality.

### 5.8 The Cow Catcher — Error Classification

The Cow Catcher system classifies runtime errors into typed obstacles for systematic debugging:

```rust
pub enum ObstacleType {
    LLMTimeout,         // AI model didn't respond in time
    CompilationError,   // cargo build/check failed
    TestFailure,        // Tests didn't pass
    ModelLoadFailure,   // Couldn't load a GGUF/ONNX model
}
```

> 📍 `cow_catcher.rs:L14-30` — `Obstacle` struct with severity, location, context
> 📍 `cow_catcher.rs:L26-31` — `ObstacleType` enum (4 categories)

### 5.9 Field Manual Cross-Reference

> *See: [Ask Pete Field Manual](ASK_PETE_FIELD_MANUAL.md) §7 The Safety Net, §8 The Calibration*

- **§7 The Safety Net** — "No one dies on the Iron Road." The ring system ensures that even destructive actions are sandboxed. The Cow Catcher catches what the rings miss.
- **§8 The Calibration** — Quality evaluation isn't punishment — it's calibration. The QM Rubric tells the user where they stand, not where they failed.

---

# CRAP — Place the Wisdom

> *Cars 6–9 answer: "How does Trinity work?"*

---

## 🚃 Car 6: CONTRAST — What Makes Trinity Different

> **Bloom's Level**: Analyze
> **Sacred Circuit**: Unlock (circuit #7)
> **Body Metaphor**: The Skin — what the world sees, what separates inside from outside

### 6.1 The PEARL — Perspective Engineering Aesthetic Research Layout

The PEARL is Trinity's **focusing agent** — the per-project alignment document that captures *what* the user is building, *how* it should be delivered, and *why* it matters.

```rust
pub struct Pearl {
    pub subject: String,       // What the SME knows (the pearl of wisdom)
    pub medium: PearlMedium,   // How it should be delivered
    pub vision: String,        // What the user expects to FEEL at the end
    pub phase: PearlPhase,     // Current lifecycle phase
    pub evaluation: PearlEvaluation,  // Alignment scores
    pub refined_count: u32,    // How many times the user has refined their vision
}
```

> 📍 `pearl.rs:L254-281` — `Pearl` struct: subject, medium, vision, phase, evaluation
> 📍 `pearl.rs:L1-22` — Architecture comment: PEARL sits between CharacterSheet (WHO) and QuestState (WHERE)

The PEARL answers: **"What are we building, and does it still match what we intended?"**

### 6.2 The Three PEARL Fields

| Field | Question It Answers | Example |
|-------|-------------------|---------|
| **Subject** | What pearl of wisdom does the SME carry? | "Newtonian physics" |
| **Medium** | How should it be delivered? | Game, Storyboard, Simulation, Lesson Plan, Assessment, Book |
| **Vision** | What should the learner FEEL? | "Students feel like they discovered Newton's laws themselves" |

> 📍 `pearl.rs:L34-50` — `PearlMedium` enum: 7 delivery formats (Game, Storyboard, Simulation, LessonPlan, Assessment, Book, Other)
> 📍 `pearl.rs:L79-89` — `suggested_tools()`: maps each medium to recommended ART tools

### 6.3 PEARL Phase Lifecycle

The PEARL has four phases that map directly to the three ADDIECRAPEYE groups:

| Phase | ADDIECRAPEYE Stations | Cognitive Activity | Icon |
|-------|----------------------|-------------------|------|
| **Extracting** 🦪 | ADDIE (1-5) | Pull wisdom out of the SME | 🦪 |
| **Placing** 💎 | CRAP (6-9) | Design the experience around it | 💎 |
| **Refining** ✨ | EYE (10-12) | Reflect, iterate, ship | ✨ |
| **Polished** 🌟 | Complete | All 12 stations passed alignment check | 🌟 |

> 📍 `pearl.rs:L96-156` — `PearlPhase` enum + `from_station()` mapping stations 1-12 to phases
> 📍 `pearl.rs:L314-324` — `refine()`: user consciously updates vision or medium (autopoiesis counter increments)

### 6.4 PEARL Evaluation — Weighted Alignment

Every PEARL is evaluated with weighted alignment scores:

| Group | Weight | What It Measures |
|-------|--------|-----------------|
| ADDIE | **40%** | Did we extract the right wisdom? (foundation) |
| CRAP | **35%** | Is the design faithful to the wisdom? (form) |
| EYE | **25%** | Does the output match the vision? (polish) |

- **Advancement threshold**: 0.6 (60%) — you don't need perfection, but you need alignment
- **Grading**: A+ (≥90%), A (≥80%), B+ (≥70%), B (≥60%), C (≥50%), D (≥30%), F (<30%)

> 📍 `pearl.rs:L162-239` — `PearlEvaluation`: weighted scoring, `is_aligned()`, letter `grade()`
> 📍 `pearl.rs:L198-201` — `overall_alignment()`: `(addie * 0.40 + crap * 0.35 + eye * 0.25).clamp(0.0, 1.0)`

### 6.5 The CharacterSheet — Persistent Identity

While the PEARL tracks per-project alignment, the **CharacterSheet** tracks the user's persistent identity across all projects. It has **30+ fields** organized across 7 domains:

| Domain | Key Fields | Code Reference |
|--------|-----------|---------------|
| **Identity** | `alias`, `user_class`, `resonance_level`, `total_xp` | `character_sheet.rs:L99-109` |
| **Hardware** | `mana_pool_vram`, `stamina_ram`, `agility_compute`, `concurrency_mode` | `character_sheet.rs:L113-121` |
| **Project** | `genre`, `party_config`, `creative_config`, `audio_preferences` | `character_sheet.rs:L123-141` |
| **Skills** | `skills` (HashMap), `completed_contracts` | `character_sheet.rs:L143-146` |
| **Intent** | `intent_posture`, `session_intent`, `vulnerability`, `grounding_complete`, `shadow_status` | `character_sheet.rs:L153-184` |
| **Cognitive** | `current_coal`, `current_steam`, `track_friction`, `cargo_slots`, `locomotive_profile` | `character_sheet.rs:L186-207` |
| **Portfolio** | `ldt_portfolio` (LDT graduation tracker) | `character_sheet.rs:L209-214` |

> 📍 `character_sheet.rs:L98-230` — Full `CharacterSheet` struct definition (30+ fields)
> 📍 `character_sheet.rs:L84` — Persistence path: `~/.local/share/trinity/character_sheet.json`

### 6.6 The Four User Classes

During **The Awakening** (character creation), users select a class that determines how Trinity supports them:

| Class | Tagline | AI Fills the Gaps |
|-------|---------|-------------------|
| **Subject Matter Expert** 🧑‍🏫 | "I know what needs to be taught" | Content selection, accuracy verification |
| **Instructional Designer** 🎓 | "I know how to scaffold the learning" | ADDIE structure, Bloom's mapping |
| **Stakeholder** 📊 | "I know what success looks like" | Evaluation criteria, outcomes tracking |
| **Player** 🎮 | "I experience what gets built" | Learner perspective, engagement testing |

> 📍 `character_sheet.rs:L388-429` — `UserClass` enum with taglines and emojis
> 📍 `character_sheet.rs:L232-280` — `CharacterSheet::new()` with all defaults

### 6.7 Intent Engineering — The Digital Quarry

The CharacterSheet captures *intent*, not just identity:

- **IntentPosture** — Mastery (learn through struggle, 2× XP, 1.5× coal cost) vs. Efficiency (ship it, 1× XP, 0.75× coal)
- **Vulnerability** — 0.0 (wants certainty) to 1.0 (open to discovery)
- **Grounding Ritual** — "I Am Here. I Am Enough. I Choose." — completed before any quest interaction
- **Shadow Status** — Ghost Train tracker (Clear → Stirring → Active → Processed)

The `ShadowStatus` mechanic is grounded in Phil Stutz's therapeutic framework (*The Tools*, Stutz & Michels, 2012) and Jungian Shadow Integration. The Shadow is not a failure state — it is unprocessed telemetry. The system does not attempt to "defeat" the Shadow; it follows the Jungian model of *Active Imagination* where the user engages in structured dialogue (via Pete's Socratic scaffolding) to *integrate* the Shadow.

**Game Mechanic Mapping (Stutz → Trinity):**

| Stutz Tool | Trinity State | Trigger |
|------------|--------------|---------|
| Part X (Inner Critic) | `Stirring` | RLHF negative feedback × 1 → Pete adjusts scaffolding |
| Reversal of Desire | `Active → Processed` | RLHF negative × 3 → Journal required → Memorial Step |
| String of Pearls | `PEARL.refined_count` | Each PEARL refinement = a pearl on Stutz's string |

**Wiring Status:** ✅ **WIRED** — `ShadowStatus` transitions are fully implemented in `rlhf_api.rs`. Negative RLHF feedback (👎) escalates Shadow (Clear → Stirring → Active), raises track friction, and recalculates vulnerability. Shadow processing via journal reflection (`POST /api/character/shadow/process`) transitions Active → Processed, reducing vulnerability and friction. Pete's system prompt reads vulnerability at `character_api.rs:L58`: `vulnerability > 0.7` triggers gentle Socratic mode. See `MATURATION_MAP.md` Soft Spot §5.

> 📍 `character_sheet.rs:L20-33` — Intent Engineering philosophy block (Brené Brown, Pythagoras)
> 📍 `character_sheet.rs:L39-77` — `IntentPosture` with `coal_multiplier()` and `xp_multiplier()`
> 📍 `character_sheet.rs:L950-968` — `ShadowStatus` enum: Clear, Stirring, Active, Processed
> 📍 `character_sheet.rs:L321-366` — `intent_summary()` for conductor prompt injection
> 📍 `character_api.rs:L58-60` — `vulnerability > 0.7` triggers gentle Socratic mode (Pete reads Shadow through vulnerability)
> 📍 Field Manual §3.4 — Pete's narrative explanation of the Ghost Train

### 6.8 Trinity vs. Other Tools

| Feature | Trinity | NotebookLM | ChatGPT Edu |
|---------|---------|-----------|-------------|
| Evaluates pedagogical quality | ✅ QM Rubric + Scorecard | ❌ Summarizes only | ❌ No evaluation |
| Game layer for engagement | ✅ SemanticCreep + VAAM | ❌ | ❌ |
| Runs locally/offline | ✅ 100% local | ❌ Cloud-only | ❌ Cloud-only |
| Multi-modal creative pipeline | ✅ Image + Music + Video + 3D | ❌ | ❌ |
| Persistent user identity | ✅ CharacterSheet | ❌ | ❌ Session-only |
| Standards alignment (IBSTPI/QM) | ✅ Automated scoring | ❌ | ❌ |

---

## 🚃 Car 7: REPETITION — The Isomorphic Patterns

> **Bloom's Level**: Apply
> **Sacred Circuit**: Flow (circuit #8)
> **Body Metaphor**: The Heart — the repeating patterns that keep the system alive

### 7.1 The Isomorphism Principle

Trinity's design is built on a single principle: **everything maps to everything**. The same 12-station pattern (ADDIECRAPEYE) appears at every layer of the system:

```
ADDIECRAPEYE stations   →  Bloom's Taxonomy levels
Sacred Circuitry words  →  ADDIECRAPEYE stations
PEARL phases            →  ADDIECRAPEYE groups
Hotel gears (P-A-R-T)  →  ADDIECRAPEYE phases
Game loop events        →  Book chapters
User intent             →  AI behavior
```

### 7.2 The Three-to-Twelve Pattern

- **3 groups** (ADDIE, CRAP, EYE) → **12 stations** → **15 circuits** → **6 Bloom's levels**
- **3 agents** (Pete, ART, Yardmaster) → **4 P-ART gears** → **6 party roles**
- **3 stakeholders** (Learner, Instructor, Institution) → **4 user classes**
- **3 modes** (IronRoad, Express, Yardmaster) → **3 layers** (Server, Protocol, Sandbox)

This isn't accidental. Each "three" fractures into detail while maintaining structural coherence.

### 7.3 Cognitive Load Theory in Code

Trinity implements Sweller's Cognitive Load Theory as game mechanics:

| CLT Concept | Game Mechanic | Implementation |
|-------------|--------------|----------------|
| **Intrinsic Load** | Coal (attention reserve) | `character_sheet.rs:L111` — `current_coal: f32` |
| **Germane Load** | Steam (productive momentum) | `character_sheet.rs:L190-191` — `current_steam: f32` |
| **Extraneous Load** | Track Friction (waste) | `character_sheet.rs:L196-197` — `track_friction: f32` |
| **Working Memory** | Cargo Slots (7 ± 2) | `character_sheet.rs:L200-202` — `cargo_slots: u8` (default: 7) |

> 📍 `character_sheet.rs:L946-947` — `default_cargo_slots() -> u8 { 7 }` — Miller's Law
> 📍 `character_sheet.rs:L942-943` — `default_steam() -> f32 { 0.0 }` — "Steam is earned, not given"

### 7.4 The Golem Metaphor

Each ADDIECRAPEYE station maps to a body part of the Golem — the creature being assembled:

| Station | Body Part | Narrative Beat |
|---------|----------|---------------|
| Analysis | Eyes 👁️ | The Call to Adventure |
| Design | Brain 🧠 | Crossing the Threshold |
| Development | Skeleton 🦴 | Building the Frame |
| Implementation | Muscles 💪 | Road of Trials |
| Evaluation | Voice 🗣️ | The Mirror |
| Contrast | Skin 🎨 | Visual Identity |
| Repetition | Heart ❤️ | The Heartbeat |
| Alignment | Spine 🦷 | Structural Integrity |
| Proximity | Hands 🤲 | Touch and Interaction |
| Envision | Third Eye 🔮 | Meta-awareness |
| Yoke | Connective Tissue 🧬 | System Integration |
| Evolve | Lungs 🫁 | First Breath |

> 📍 `conductor_leader.rs:L597-604` — Analysis prompt: "This is the Golem's Eyes — the Call to Adventure"
> 📍 `conductor_leader.rs:L636-643` — Design prompt: "This is the Golem's Brain — Crossing the Threshold"

### 7.5 Hardware-to-Game Mapping

The user's physical hardware maps to game stats:

| Hardware | Game Stat | Field |
|----------|----------|-------|
| GPU VRAM | Mana Pool | `mana_pool_vram: u32` |
| System RAM | Stamina | `stamina_ram: u32` |
| NPU/Compute | Agility | `agility_compute: u32` |

These determine the `ConcurrencyMode`:

| VRAM | Mode | Party Size |
|------|------|-----------|
| < 24GB | LoneWolf | 1 model at a time |
| 32-64GB | SmallSquad | Conductor + 1 specialist |
| 128GB+ | Guild | Full party (all roles) |

> 📍 `character_sheet.rs:L376-386` — `ConcurrencyMode` enum (LoneWolf, SmallSquad, Guild)
> 📍 `character_sheet.rs:L509-540` — `PartyConfig::auto_configure()`: VRAM→party size algorithm

### 7.6 Locomotive Profiles

Users have cognitive processing archetypes named after railroad locomotive types:

| Profile | Thinking Style | Pete's Adjustment |
|---------|---------------|------------------|
| 🚄 **Interceptor Express** | Fast, impatient | Shorter prompts, more autonomy |
| 🔬 **Analyzer Class** (default) | Methodical, analytical | Deeper explanations, more Socratic |
| 🔀 **All-Terrain Switcher** | Versatile, adaptive | Balanced, reads the room |
| 🛡️ **Armored Supply Train** | Cautious, safety-first | More encouragement, gentle pacing |

> 📍 `character_sheet.rs:L970-1008` — `LocomotiveProfile` enum with cognitive descriptions

---

## 🚃 Car 8: ALIGNMENT — Pete's Socratic Protocol

> **Bloom's Level**: Evaluate
> **Sacred Circuit**: Relate (circuit #10)
> **Body Metaphor**: The Spine — structural integrity, holding everything in line

### 8.1 The Socratic Core

The Great Recycler's fundamental behavior is encoded in the persona preamble:

> "You NEVER produce deliverables directly. You ask questions that reveal what the user already knows. Your job is done when the user has clarity, not when they have a product."

Programmer Pete's fundamental behavior is the complement:

> "When asked to create, you CREATE. Lesson plans, rubrics, code, artifacts — you produce them. Your job is done when the user has a product, not just a plan."

This is not a suggestion — it's enforced through the dual-persona architecture. The **Great Recycler** (Slot 0) handles reflection; **Pete** (Slot 1) handles execution. Every ADDIECRAPEYE phase prompt in `conductor_leader.rs` begins with **SOCRATIC PROTOCOL** followed by 3 guiding questions.

### 8.2 The Yardmaster System Prompt

The Yardmaster (dev agent) operates under a strict behavioral contract:

```
ABSOLUTE RULE: When the user asks you to do something concrete, USE A TOOL IMMEDIATELY.
NEVER SAY THESE PHRASES:
- "Want me to go ahead?"
- "Shall I proceed?"
- "Would you like me to..."
- "Before I proceed, can you confirm..."
If you catch yourself about to say any of these, STOP and use a tool instead.
```

> 📍 `agent.rs:L148-200` — Full Yardmaster system prompt (workspace map, tool format, banned phrases)
> 📍 `agent.rs:L150-159` — Anti-permission-asking rules (5 banned phrases)

### 8.3 Dual Persona Architecture

Trinity operates two distinct personas, each with its own KV cache slot for instant switching:

| Persona | Slot | Breath | Cognitive Style |
|---------|------|--------|----------------|
| **Great Recycler** 🔮 | 0 | Inhale | "You NEVER produce deliverables. You ask questions. Make them THINK." |
| **Programmer Pete** ⚙️ | 1 | Exhale | "ACT FIRST. When asked to build, BUILD. Make them THINGS." |

The **inhale/exhale** metaphor is literal: strategic thinking (slot 0) and execution (slot 1) alternate like breathing. The KV cache slots enable switching without re-tokenizing the 8K+ token system prompt.

> 📍 `agent.rs:L92-108` — Great Recycler preamble: "the Socratic mentor, the one who asks questions"
> 📍 `agent.rs:L110-130` — Programmer Pete preamble: "the builder, the executor, the one who ships"
> 📍 `agent.rs:L132-145` — `persona_slot()`: maps "recycler" → 0, "programmer" → 1

### 8.4 VAAM Integration in Chat

Every message in the agent chat loop is scanned for vocabulary (VAAM integration). When a word matches the user's active vocabulary set, Coal is awarded:

> 📍 `agent.rs` — VAAM vocabulary scanning on every user message
> 📍 `vaam.rs:L1-30` — Architecture: VAAM scans messages for vocabulary, awards Coal for correct contextual usage

### 8.5 The Multi-Turn Agent Loop

The agent loop implements a full tool-calling cycle:

```
User message → System prompt + history → LLM generates
→ Parse for <tool>...</tool> tags → Execute tools → Feed results back
→ Repeat until final answer or max iterations (default: 16)
```

> 📍 `agent.rs:L54-75` — `AgentRequest`: message, history, mode (dev/ironroad), max_turns (default 16)
> 📍 `agent.rs:L84-86` — `default_max_turns() -> u32 { 16 }`

### 8.6 Field Manual Cross-Reference

> *See: [Ask Pete Field Manual](ASK_PETE_FIELD_MANUAL.md) §1 The Identity, §2 The Promise*

- **§1 The Identity** — "Pete is not a chatbot. Pete is a mentor who happens to run on silicon."
- **§2 The Promise** — "I will ask, not tell. I will scaffold, not solve. I will remember what you've shown me."

---

## 🚃 Car 9: PROXIMITY — User Interface & Experience

> **Bloom's Level**: Analyze
> **Sacred Circuit**: Realize (circuit #11)
> **Body Metaphor**: The Hands — the user's physical contact with the system

### 9.1 The 16 React Components

Trinity's frontend is built as a modular React application with 16 components, each mapped to a specific system concern:

| Component | Role | Backend Source |
|-----------|------|---------------|
| **NavBar** | Tab navigation across modes | Routes to different views |
| **CharacterSheet** | LitRPG-styled user identity HUD | `GET /api/character` |
| **PhaseWorkspace** | ADDIECRAPEYE station workspace | Phase-specific tool display |
| **ChapterRail** | Timeline of completed chapters | Book of the Bible entries |
| **PearlCard** | PEARL alignment visualization | Subject/Medium/Vision display |
| **CreepCard** | SemanticCreep creature card | Word stats, element, taming progress |
| **ScopeCard** | Scope Hope/Nope decision prompt | Taming confirmation dialog |
| **GameHUD** | Iron Road gameplay overlay | Coal, Steam, XP bars |
| **TrainStatus** | Server + sidecar health monitor | Health endpoint polling |
| **Yardmaster** | Agent chat (dev console) | `POST /api/agent/chat` (SSE) |
| **ArtStudio** | Creative tools (image/music/3D) | ComfyUI + MusicGPT APIs |
| **ExpressWizard** | Guided lesson builder wizard | Step-by-step ADDIE flow |
| **JournalViewer** | Reflection journal reader | Shadow processing + Book entries |
| **PerspectiveSidebar** | Bloom's/Practitioner/Devil's lenses | SSE "perspective" events |
| **QualityScorecard** | Document evaluation display | QM rubric results |
| **OnboardingTour** | First-time user experience | The Awakening flow |

> 📍 `frontend/src/components/` — 16 `.jsx` files

### 9.2 Glassmorphism Design System

All components share a unified visual language defined in `tokens.css` (309 lines):

| Token | Value | Semantic |
|-------|-------|----------|
| `--gold` | `#CFB991` | Identity, branding, headings |
| `--bg` | `#131210` | Warm black background |
| `--bg-card` | `rgba(24, 22, 18, 0.88)` | Glass card surface |
| `--border-glass` | `rgba(207, 185, 145, 0.15)` | Gold-tinted borders |
| `--font-display` | `'Cinzel', serif` | Headers, Purdue heritage |
| `--font-ui` | `'Inter', sans-serif` | Body text, interface |
| `--font-mono` | `'JetBrains Mono', monospace` | Data, code, metrics |
| `--font-body` | `'Crimson Text', serif` | Prose, reflections |
| `--r-md` | `10px` | Card border radius |

Semantic data colors are preserved for CRAP Contrast:
- **Emerald `#10B981`** — Success, portfolio progress, AECT cleared
- **Cyan `#22d3ee`** — Data scores, QM values
- **Purple `#A78BFA`** — Metacognition (EYE group), ATD
- **Amber `#F59E0B`** — Warnings, Heavilon events, cargo slots
- **Red `#EF4444`** — Errors, friction, Ghost Train Active

> 📍 `tokens.css:L5-105` — Full design token definitions
> 📍 `CharacterSheet.jsx:L313-580` — Styles object aligned to tokens.css

### 9.3 The CharacterSheet UI

The `CharacterSheet.jsx` component renders the **ADDIECRAPEYE Agreement** — the contract between the user and the Iron Road. Structured **finish-line-first**, it answers the user's core question: *"What am I building, and how mature is it?"*

**Section 1 — YOUR PEARL (The Contract You Wrote):**
- 🔮 The user's own subject, medium, and vision statement
- ADDIE / CRAP / EYE alignment bars + overall alignment %
- *"This is YOUR definition of success. Pete's Socratic interview shaped it."*

**Section 2 — 🏁 WHAT YOU'RE BUILDING (The Finish Line):**
- 5 deliverables: Game Design Document, HTML5 Interactive Game, Lesson Plans & Rubrics, Visual Design System, LDT Portfolio Artifact
- Each lights up when its ADDIECRAPEYE group is complete

**Section 3 — 📈 MATURATION MAP:**
- 6 auto-scored dimensions: Content Readiness, Production Quality, Pedagogical Rigor, Visual Design, Metacognitive Depth, Portfolio Completion
- Overall maturation % — computed from completed phases

**Section 4 — 📜 THE ADDIECRAPEYE AGREEMENT (12-Station Grid):**
- 12 station cards grouped by ADDIE / CRAP / EYE
- Each shows: icon, name, Bloom's level, deliverable produced, Hero's Journey chapter
- 🔮 PEARL lens badge (e.g., `A·P` = Aesthetic + Perspective) — *"everything that is CRAP needs a PEARL"*
- Status: ✅ COMPLETE / 🔶 ACTIVE / 🔒 LOCKED
- Active phase shows real quest objectives from `/api/quest`

**Section 5 — Bottom Panels:**
- ⚙️ Cognitive Logistics: Coal, Steam, Friction bars + Cargo Slots
- 🏆 Competency Scores: QM, IBSTPI, ATD, AECT, Heavilon, Memorial
- 📖 Artifact Vault: collapsible, shows vaulted artifacts with QM scores

**Design principle:** Maturation, not complication. Every element has direction and use.

> 📍 `CharacterSheet.jsx` — Full component with PEARL fetch, maturation scoring, and ADDIECRAPEYE agreement grid

### 9.4 The Four Chariots — In-App Help Menu

Trinity's **Help Menu** (accessible via ❓ icon in NavBar) provides direct access to the project's four root documents — the "Four Chariots" that drive Trinity's self-identity:

| Chariot | File | Reader | Purpose |
|---------|------|--------|---------|
| 📖 **The Bible** | `TRINITY_FANCY_BIBLE.md` | Developers | Technical spec, every line of code explained |
| 🤝 **Field Manual** | `ASK_PETE_FIELD_MANUAL.md` | Educators | Pete's operating philosophy & cognitive logistics |
| 🎓 **Professor Programming** | `PROFESSOR.md` | Stakeholders | Standards alignment, privacy, institutional evaluation |
| 🎮 **Player's Handbook** | `PLAYERS_HANDBOOK.md` | Players | Philosophy, identity & conscious learning |

When a user encounters unfamiliar terminology (e.g., "Subconscious Inventory"), Pete can direct them to the relevant Chariot. When a stakeholder questions the "gamification" approach, the Professor Programming document provides the pedagogical justification with IBSTPI/AECT/QM standards alignment.

> 📍 `NavBar.jsx:L12-43` — Help menu dropdown with Four Chariots links
> 📍 `main.rs:L813-824` — Static file routes serving root `.md` documents via `/docs/`

### 9.5 The Three Modes

The UI adapts to three operating modes:

| Mode | Target User | UI Configuration |
|------|------------|-----------------|
| **Iron Road** 🚂 | Learners | Full LitRPG: CreepCards, GameHUD, ChapterRail, narrative |
| **Express** ⚡ | Teachers (quick path) | ExpressWizard: guided ADDIE, minimal game chrome |
| **Yardmaster** 🔧 | Developers | Agent console, process list, sidecar management |

> 📍 `main.rs:L84-92` — `AppMode` enum: IronRoad, Express, Yardmaster

---

# EYE — Refine the Wisdom

> *Cars 10–12 answer: "Where is Trinity going?"*

---

## 🚃 Car 10: ENVISION — Purdue Integration & LDT Portfolio

> **Bloom's Level**: Evaluate
> **Sacred Circuit**: Transform (circuit #13)
> **Body Metaphor**: The Third Eye — meta-awareness and institutional reflection

### 10.1 The LDT Portfolio — The Graduation Track

The LDT Portfolio is the **isomorphic bridge** between Trinity's game mechanics and Purdue University's Learning Design and Technology program requirements:

```
12 completed artifacts = graduation.
The game IS the portfolio.
```

> 📍 `character_sheet.rs:L1010-1025` — Architecture comment: "Isomorphic mapping: academic rubrics → game physics"
> 📍 `character_sheet.rs:L1029-1056` — `LdtPortfolio` struct with 9 fields

### 10.2 Standards Alignment

The portfolio tracks alignment across **four professional standards**:

| Standard | Organization | What It Measures | Field |
|----------|-------------|-----------------|-------|
| **IBSTPI** | Intl Board of Standards | Instructional design competencies | `ibstpi_score: f32` |
| **ATD** | Assoc for Talent Dev | Capability model | `atd_score: f32` |
| **AECT** | Assoc for Ed Comm & Tech | Ethics | `aect_score: f32` |
| **QM** | Quality Matters | Course design quality | `qm_alignment_score: f32` |

> 📍 `character_sheet.rs:L1039-1048` — Individual standard scores (0.0-100.0 each)

### 10.3 Graduation Requirements

```rust
pub fn is_graduation_ready(&self) -> bool {
    self.completed_challenges >= 12
        && self.artifact_vault.len() >= 12
        && self.qm_alignment_score >= 85.0
}
```

> 📍 `character_sheet.rs:L1076-1080` — Graduation check: 12 artifacts + QM ≥ 85.0
> 📍 `character_sheet.rs:L1083-1095` — `recalculate()`: auto-upgrades gate review status

### 10.4 Heavilon Events & Memorial Steps

Two Purdue-specific metrics track resilience:

- **Heavilon Events Survived** — Count of catastrophic failures rebuilt "one brick higher" (named after Heavilon Hall, which burned and was rebuilt one brick higher)
- **Memorial Steps Climbed** — Deep reflection journals after burnout (maps to Purdue Memorial Union's 17 steps)

> 📍 `character_sheet.rs:L1051-1055` — `heavilon_events_survived: u32`, `memorial_steps_climbed: u32`

### 10.5 The Artifact Vault

Each completed artifact is stored in the **Subconscious Inventory**:

> 📍 `character_sheet.rs:L1098-1100+` — `PortfolioArtifact` struct: title, type, QM score, AECT ethics cleared, reflection journal
> 📍 `CharacterSheet.jsx:L170-198` — Vault UI: artifact cards with QM scores, AECT badges, reflection quotes

---

## 🚃 Car 11: YOKE — ART Pipeline & Creative Tools

> **Bloom's Level**: Create
> **Sacred Circuit**: Connect (circuit #14)
> **Body Metaphor**: The Connective Tissue — coupling all subsystems

### 11.1 DAYDREAM — The Native Gaming System

**DAYDREAM** is Trinity's native 3D gaming system, built entirely in **pure Rust** with Bevy 0.18.1. There is **no JavaScript** in the DAYDREAM engine — it is a standalone binary (`daydream`) spawned as an OS child process, communicating with the main Trinity server via HTTP.

This architectural decision was made for three reasons:
1. **Process isolation** — Bevy and Tauri both demand the main thread on Linux/Wayland. Running them as separate processes eliminates `winit` conflicts and hot-reload panics.
2. **Performance** — Native Rust ECS with Avian3D physics runs at full speed without WebView overhead.
3. **Philosophical alignment** — DAYDREAM is the user's blank canvas. It should not be constrained by browser limitations. If a user wants to extend DAYDREAM, they do so in Rust (or in Python for ML inference pipelines via sidecar integration).

> 📍 `crates/trinity-daydream/Cargo.toml` — `[[bin]] name = "daydream"` with `desktop` feature gate
> 📍 `crates/trinity-daydream/src/daydream.rs` — Main DAYDREAM module: 3D world, ADDIECRAPEYE integration

### 11.2 The ART Creative Pipeline

**ART** = Aesthetics, Research, Tempo — Trinity's creative subsystem that generates multi-modal content:

| Modality | Technology | Endpoint | Port |
|----------|-----------|----------|------|
| **Image** | SDXL Turbo via ComfyUI | `POST /api/creative/image` | :8188 |
| **Music** | trinity-tempo-ai (procedural) | `POST /api/creative/tempo` | CLI sidecar |
| **Video** | HunyuanVideo via CLI | `POST /api/creative/video` | CLI sidecar |
| **3D Mesh** | Hunyuan3D-2.1 via Gradio | `POST /api/creative/mesh3d` | :7860 |

> 📍 `creative.rs:L1-24` — Architecture: "CRAP design system: Contrast, Repetition, Alignment, Proximity"
> 📍 `creative.rs:L44-62` — `ImageRequest`: prompt, negative_prompt, style, width (default 1024), height
> 📍 `creative.rs:L86-97` — `MusicRequest`: style, duration_secs (default 60), mood
> 📍 `creative.rs:L114-128` — `VideoRequest`: prompt, duration_secs (default 4), fps (24), height (720)
> 📍 `creative.rs:L152-163` — `Mesh3DRequest`: prompt, image_base64, format (default "glb")

### 11.2 ComfyUI Workflow — SDXL Turbo

Image generation uses a 7-node ComfyUI workflow:

```
CheckpointLoader → CLIPTextEncode(+) → CLIPTextEncode(-) →
EmptyLatentImage → KSampler(4 steps, euler, cfg 1.0) →
VAEDecode → SaveImage
```

> 📍 `creative.rs:L336-373` — Full ComfyUI SDXL Turbo workflow JSON
> 📍 `creative.rs:L360-361` — KSampler: 4 steps, euler sampler, cfg 1.0 (Turbo defaults)

### 11.3 Visual Style System

Visual styles are determined by the character's genre selection:

| Genre | Visual Style | Music Style |
|-------|-------------|-------------|
| Steampunk | Brass, gears, amber lighting | Orchestral |
| Cyberpunk | Neon, chrome, holograms | Electronic |
| Solarpunk | Clean, modern, simple | Ambient |
| Dark Fantasy | Magic, ethereal, medieval | Orchestral |

> 📍 `character_sheet.rs:L760-780` — `CreativeConfig::from_genre()`: genre → visual + music mapping
> 📍 `character_sheet.rs:L782-828` — `VisualStyle` enum (6 styles) with ComfyUI prompt suffixes
> 📍 `character_sheet.rs:L830-881` — `MusicStyle` enum (6 styles) with MusicGPT prompts

### 11.4 Voice Pipeline

Trinity implements a **dual voice pipeline**:

| Pipeline | Technology | Latency | GPU Impact |
|----------|-----------|---------|-----------|
| **Walkie-Talkie** (NOW) | Whisper STT + Supertonic-2 TTS (ONNX, CPU/GPU) | ~2-4s round trip | Low (ONNX inference) |
| **Telephone** (FUTURE) | PersonaPlex/Moshi audio-to-audio | ~0ms perceived | GPU contention |

> 📍 `voice.rs:L1-27` — Architecture: dual pipeline with fallback
> 📍 `voice.rs:L9-14` — "STT/TTS run on NPU, leaving 100% GPU for Mistral Small 4"
> 📍 `voice.rs:L115-188` — `voice_conversation()`: tries PersonaPlex first, falls back to sidecar
> 📍 `voice.rs:L299-319` — `check_npu_availability()`: checks `/dev/xdna` and `lspci` for NPU

### 11.5 Model Inventory

Trinity's AI party is composed of pre-configured model assignments:

| Role | Model | Size | Active Params |
|------|-------|------|--------------|
| **P** (Conductor) | Mistral Small 4 119B MoE | 68GB | ~6.5B |
| **Y** (Yardmaster) | Ming-flash-omni-2.0 | 195GB | MoE (256 experts, 8 active) |
| **R** (Research) | REAP 25B MoE | 15GB | 3B |
| **R** (Research) | Crow 9B | 5GB | 9B |
| **T** (Tempo) | OmniCoder 9B | 5GB | 9B |
| **Evaluator** | Qwen3.5-27B Opus | 21GB | 27B |
| **Visionary** | Qwen3.5-35B-A3B | 20GB | 3B |

> 📍 `character_sheet.rs:L620-712` — `ModelAssignment` factory methods with full specs
> 📍 `character_sheet.rs:L625-634` — Mistral Small 4: "256k context with Q4 KV cache quantization, vision capable"

---

## 🚃 Car 12: EVOLVE — Deployment, Hardware, and the Lexicon

> **Bloom's Level**: Create
> **Sacred Circuit**: Manifest (circuit #15)
> **Body Metaphor**: The Lungs — the Golem takes its first breath

### 12.1 The AMD Strix Halo Platform

Trinity runs on AMD's Strix Halo (Ryzen AI Max+ 395):

| Spec | Value | Trinity Usage |
|------|-------|--------------|
| CPU | Zen 5, 16 cores / 32 threads | Tokio async runtime, Bevy ECS |
| GPU | RDNA 3.5, 40 CUs | Mistral inference via llama-server |
| NPU | XDNA 2, 50 TOPS | Voice STT/TTS (Whisper/Piper) |
| RAM | 128 GB unified (LPDDR5X) | All models loaded concurrently |
| Context | 256K × 2 KV cache = 500K+ tokens | Dual persona slots |

The **unified memory** architecture is the key enabler — GPU, CPU, and NPU share the same 128GB pool, meaning Mistral's 68GB model doesn't compete with system RAM.

> 📍 `main.rs:L220-268` — `installed_model_inventory()`: all models with sizes and paths
> 📍 `conductor_leader.rs:L240-252` — `ConductorConfig::default()`: model path, context 32768 (conservative start)

### 12.2 The Dual KV Cache Architecture

Trinity's inference architecture is **agnostic** — it dispatches to whichever OpenAI-compatible backend the user prefers via the `InferenceRouter`. The recommended configuration uses LM Studio with Mistral Small 4 119B MoE:

- **Dual slots** via LM Studio's `--parallel 2` — two simultaneous conversations
- **Up to 1M tokens per slot** — enabled by MLA (Multi-head Latent Attention)
- **Combined**: 2M+ tokens of persistent context across personas

Persona differentiation (Great Recycler vs. Programmer Pete) is handled at the system prompt layer, not via hardcoded KV cache slot routing. This makes the system backend-agnostic — any OpenAI-compatible API works.

> [!NOTE] 
> **ADDENDUM: Author's Preferred AI LLM: MS4 (Mistral Small 4 119B)**
>
> We have conducted extensive real-world memory profiling on the Trinity architecture running on AMD Strix Halo (128GB unified memory).
> 
> **1. The MLA Advantage**
> Mistral Small 4 explicitly uses **Multi-head Latent Attention (MLA)**. Its `config.json` sets `kv_lora_rank: 256`. Instead of storing full `32 heads × 128 dim` KV matrices per token, it stores a compressed 256-dimensional latent vector.
> * Standard dense FP16 KV cache: ~576 KB per token
> * MS4 MLA with Q4 KV Quantization: **~9 KB per token** (64x smaller!)
>
> **2. Real-World Memory Footprint (1M Context)**
> Because of MLA, we can safely push MS4 far beyond its native 256k window. Our active production configuration in LM Studio:
> * **Model**: `mistral-small-4-119b-2603`
> * **Context Length**: `1,048,576` (1 Million tokens)
> * **Parallel**: `2`
> * **KV Cache Quantization**: `Q4` (Must be enabled in LM Studio settings)
> * **Total Actual Memory**: ~85 GB total (67GB model weights + 18GB for the dual 1M token KV caches). 
> This comfortably fits within 128GB with headroom for the OS and creative sidecars. LM Studio's built-in memory estimator does *not* account for MLA and will falsely warn that 1M context requires >100GB of KV cache. Ignore the warning.
>
> **3. Evaluation Batch Size (n_batch)**
> Setting evaluation batch size to **512** (or 1024) is correct. This dictates how many tokens are processed simultaneously during prompt ingestion (the "prefill" phase). 512 provides a stable balance between prompt parsing speed and VRAM spike stability on unified memory.

### 12.3 Server Architecture

```
Layer 0: Tauri v2 Desktop Shell (optional)
  ├── Native window management
  ├── React WebView (Iron Road / ART / Yardmaster)
  └── --headless mode bypasses for daemon deployment (LDTAtkinson.com)

Layer 1: Headless Server (trinity crate)
  ├── Axum HTTP/SSE server on :3000
  ├── 85+ API endpoints across 15 groups
  ├── SQLx database (SQLite)
  ├── InferenceRouter (agnostic HTTP → LM Studio / Ollama / llama-server)
  ├── Background Job Runner (SQLite-persisted task queue)
  ├── Native ONNX (Supertonic-2 TTS + Whisper STT + all-MiniLM-L6-v2 RAG)
  └── Tool dispatch (30 tools, 3 permission tiers)

Layer 2: Protocol (trinity-protocol crate)
  ├── 26 public modules
  ├── Shared types: CharacterSheet, Pearl, Circuit, Quest
  └── Zero dependencies on Layer 1

Layer 3: DAYDREAM (trinity-daydream crate)
  ├── Bevy 0.18.1 ECS (pure Rust, no JS)
  ├── 3D Yard environment with Avian3D physics
  ├── Spawned as OS child process (sidecar)
  └── Connects to Layer 1 via HTTP

Layer 4: MCP Server (trinity-mcp-server crate)
  ├── Model Context Protocol for external agentic tools
  ├── SQLite memory access via stdio transport
  └── Enables IDE integration (Zed, Cursor)
```

### 12.4 The Trinity Lexicon

Key terms defined in code, collected for reference:

| Term | Definition | Code Source |
|------|-----------|------------|
| **ADDIECRAPEYE** | 12-station instructional design framework | `conductor_leader.rs:L36-49` |
| **Coal** | Attention reserve (intrinsic cognitive load) | `character_sheet.rs:L111` |
| **Steam** | Productive momentum (germane load) | `character_sheet.rs:L190-191` |
| **Track Friction** | Extraneous cognitive load penalty | `character_sheet.rs:L196-197` |
| **Cargo Slots** | Working memory capacity (Miller's 7±2) | `character_sheet.rs` |
| **SemanticCreep** | Vocabulary creature with elemental stats | `semantic_creep.rs` |
| **PEARL** | Per-project alignment (subject/medium/vision) — the user's contract | `pearl.rs` |
| **PEARL Contract** | The PEARL as the quest board contract — user-defined success criteria shaped by Pete's Socratic interview | `CharacterSheet.jsx` |
| **Maturation Map** | 6-dimension auto-scored progress visualization (Content, Production, Pedagogy, Design, Reflection, Portfolio) | `CharacterSheet.jsx` |
| **Finish Line** | The 5 deliverables the user walks away with (GDD, HTML5, Lesson Plans, Design System, Portfolio Artifact) | `CharacterSheet.jsx` |
| **DAYDREAM** | Pure Rust Bevy 3D sidecar — spawned as OS child process, NO JavaScript. Full lit-novel edutainment with music, pictures, and emotionally specific voice. | `trinity-daydream` crate |
| **Hook Book** | Catalog of 30 system capabilities organized by framework layer (7 Foundations, 13 Experience, 10 Infrastructure) | `HOOK_BOOK.md` |
| **Scope Hope** | User tames a word (accepts into vocabulary) | `game_loop.rs` |
| **Scope Nope** | User rejects a word (leaves wild) | `game_loop.rs` |
| **Heavilon Event** | Catastrophic failure rebuilt stronger | `character_sheet.rs` |
| **Sacred Circuitry** | 15-word attention scaffolding system | `sacred_circuitry.rs` |
| **VAAM** | Vocabulary Acquisition Autonomy Mastery | `trinity-iron-road/src/vaam/` |
| **The Awakening** | Character creation (class + hardware scan) | `character_sheet.rs` |
| **Lone Wolf** | Single-model mode (< 24GB VRAM) | `character_sheet.rs` |
| **Hotel Management** | Model hot-swap protocol | `conductor_leader.rs` |
| **Book of the Bible** | Append-only narrative ledger | `trinity-iron-road/src/book.rs` |
| **Great Recycler** | Narrative AI that writes Book chapters | `trinity-iron-road/src/great_recycler.rs` |
| **Cow Catcher** | Runtime error classification system | `cow_catcher.rs` |
| **MCP Server** | Model Context Protocol — standardized agentic extensibility for IDE integration | `trinity-mcp-server` crate |
| **Background Jobs** | SQLite-persisted task queue for autonomous multi-turn agent execution | `jobs.rs` |
| **Setup Wizard (BYOM)** | "Bring Your Own Mind" — first-run wizard to select inference backend | `SetupWizard.jsx` |
| **EdgeGuard** | Route-level security — Demo mode restriction for public Cloudflared access | `edge_guard.rs` |
| **Ignition** | LM Studio boot state machine: idle → launching → daemon_up → ready | `main.rs` |
| **InferenceRouter** | Agnostic HTTP dispatcher — auto-detects LM Studio / Ollama / llama-server | `inference_router.rs` |

### 12.5 What's Next

Trinity is in **late prototype** stage. The Golem has its skeleton, muscles, and voice.

**Completed (March 2026):**
- ✅ **LDTAtkinson.com** — Portfolio website live, hosted from the same Strix Halo (Caddy reverse proxy + auto-HTTPS)
- ✅ **Agnostic Inference** — HTTP router supports LM Studio, Ollama, llama-server, or any OpenAI-compatible API
- ✅ **Background Job Runner** — SQLite-persisted task queue for autonomous agent execution
- ✅ **MCP Server** — Model Context Protocol for IDE integration (Zed, Cursor)
- ✅ **Setup Wizard (BYOM)** — "Bring Your Own Mind" first-run backend selection
- ✅ **EdgeGuard** — Route-level security for public Cloudflared access (Demo mode)
- ✅ **Native ONNX Voice** — Supertonic-2 TTS + Whisper STT without Python sidecar
- ✅ **DAYDREAM Sidecar** — Pure Rust Bevy 3D engine as OS child process

**Roadmap:**
- **NPU Integration** — Move Whisper/Piper to XDNA 2 for zero-GPU voice
- **Multi-Model Party** — Enable simultaneous REAP + Crow + Pete via Guild mode
- **Native RAG (ONNX)** — NPU-accelerated `all-MiniLM-L6-v2` embeddings
- **Gate Review API** — Formal instructor review workflow for LDT Portfolio
- **Purdue Pilot** — First classroom deployment with LDT students
- **VAAM as Edge Intelligence** — Vocabulary-based attention management is MORE valuable on constrained devices (7B-32B models, 4-8K context). VAAM's 500-char prompt budget and Sacred Circuitry's Coal scanner reduce prompt engineering burden and manage drift where brute-force context is unavailable. No other system provides cognitive load management for edge AI.
- **Mobile Trinity** — The VAAM profile (< 2KB JSON) + Sacred Circuitry (15 words) + Coal economy can run as a PWA or React Native shell on a phone, pointing to any local model or remote llama.cpp server. The cognitive scaffolding layer is device-independent.
- **Car-Based Crate Chunking** — Organize the Cargo workspace into deployment-target "train cars": Phone Car (VAAM profile + circuitry scanner), Edge Car (+ quest engine), Desktop Car (+ creative pipeline + voice), Server Car (+ multi-user llama.cpp). A Yardmaster couples the cars for each deployment target.

> *The Golem breathes. The iron has been laid. The next train is loading.*
>
> *The portfolio is at [LDTAtkinson.com](https://LDTAtkinson.com) — 23 artifacts, 4 competency domains, powered by the same `#CFB991` gold that lights this Bible.*

---

*End of the TRINITY FANCY BIBLE — 12 ADDIECRAPEYE Train Cars*

*Generated with validated `📍 file:line` code references. Every claim points to running code.*

*"I have told you everything. The rest is up to you." — Pete*
