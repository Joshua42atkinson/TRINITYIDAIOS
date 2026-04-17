# TRINITY ID AI OS — Project Context

> **Last Updated:** April 17, 2026
> **Hardware:** AMD Strix Halo APU — Radeon 8060S (`gfx1151`), 128GB Unified LPDDR5x
> **Runtime:** Rust/Axum (:3000) + LM Studio (:1234) + ORT embedded models + Bevy XR
> **Purpose:** LDT course + textbook as a LitRPG game — the user is the "player," winning = graduating with three deliverables (The EYE Package)

---

## 1. What TRINITY Is

TRINITY is a **prompting system that prompts both the USER and the AI systematically** for development structuring. It is an LDT (Learning Design & Technology) course and textbook all in its own, where the user is the "player" in a LitRPG game called **The Iron Road**.

**Core Pedagogy:**
- **ADDIECRAPEYE** — 12-station state machine (Analysis → Design → Development → Implementation → Evaluation → Contrast → Repetition → Alignment → Proximity → Envision → Yoke → Evolve) that prevents AI hallucination by anchoring output to rigorous checkpoints
- **VAAM** (Vocabulary As A Mechanism) — abstract concepts mapped to 3D physical models via `PEARL` alignment
- **Cognitive Thermodynamics** — Coal/Steam/Traction/Friction track user engagement and prevent overwhelm

**Three Deliverables (The EYE Package):**
1. HTML5 Interactive Quiz
2. HTML5 Adventure Game
3. DOCX Professional Document

---

## 2. System Architecture (Agnostic P-ART-Y)

### Design Principle: Two-Tier Agnostic Architecture

TRINITY uses an agnostic architecture with **embedded ORT models** (runs anywhere) and an **optional external inference engine** (user-managed). No server administration required.

### P-ART-Y Role Map

| Role | Name | What It Does | Backend |
|------|------|-------------|---------|
| **P** | Pete — LM Studio | The Great Recycler. DM of the Iron Road. Socratic mentor, LitRPG narrator. Does majority of Trinity's work. | **LM Studio** (port 1234) — user loads any model. Parallel=2 for inhale/exhale. |
| **A** | Aesthetics | Vision + image generation. CRAP evaluation of UI layouts. | Janus Pro 7B ONNX (embedded ORT) + ComfyUI (MCP tool) |
| **R** | Research | RAG embeddings + semantic search. Grounds Pete's responses in user content. | all-MiniLM-L6-v2 ONNX (embedded ORT) |
| **T** | Tempo | Voice narration + TTS. Audio counterpart to Aesthetics. | Kokoro TTS ONNX (embedded ORT) |
| **Y** | The User | **The Subject Matter Expert.** Pete scaffolds — the user creates. | Human at the keyboard |

### Two Operating Tiers

| Tier | What Runs | Experience |
|------|-----------|------------|
| **Standalone** | Trinity binary only (ORT embedded) | Story mode, Socratic chat, VAAM, voice — no setup |
| **Enhanced** | Trinity + LM Studio | Full reasoning, tool calling, overnight sessions |

### Port Map

| Port | Service | Status |
|------|---------|--------|
| **1234** | LM Studio (Pete's Brain) | ✅ Primary inference |
| **3000** | Trinity Axum Server | ✅ Always on |
| embedded | Kokoro TTS (ORT) | ✅ Embedded |
| embedded | all-MiniLM-L6-v2 (ORT) | ✅ Embedded |
| embedded | Whisper STT (ORT) | ✅ Embedded |

> **vLLM Server Tier:** For Purdue multi-user deployment, vLLM backends on ports 8000-8002 are available via `configs/runtime/default.toml`. See `docs/archive/vllm_server_tier/` for setup.

### ART Model Fine-Tuning Pipeline

Embedded ORT models are specialized using: **Unsloth → ONNX → AMD Quark INT4**

Each ART model is fine-tuned from a larger teacher (Opus distillation) for its specific Trinity role, then quantized for minimal memory footprint.

---

## 3. Launching Trinity

```bash
# Start Trinity server (connects to LM Studio on :1234)
cargo run -p trinity

# LM Studio: start headless with parallel=2
lms server start --port 1234 --parallel 2

# Or with a specific model
lms load gemma-4-e4b --gpu max
```

### Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `LLM_URL` | Override inference endpoint | `http://127.0.0.1:1234` |
| `PETE_ENGINE_URL` | Override Pete's engine | `http://127.0.0.1:1234` |

---

## 4. Key API Endpoints

| Endpoint | Purpose |
|----------|---------|
| `GET /api/health` | Full subsystem health (LLM, DB, creative, voice, CowCatcher) |
| `GET /api/inference/status` | Active inference router status |
| `POST /api/chat` | Iron Road SSE chat (Socratic + Narration) |
| `POST /api/agent/chat` | Yardmaster agentic tool-calling loop |
| `POST /api/models/switch` | Switch active model |
| `GET /api/quest/state` | Current ADDIECRAPEYE phase + objectives |

---

## 5. Model Storage

### Active (LM Studio)
Models live in `~/.lmstudio/models/` — managed by LM Studio GUI.

### Active (ORT Embedded)
| Model | Location | Size |
|-------|----------|------|
| all-MiniLM-L6-v2 | `~/.trinity/models/` | ~90 MB |
| Kokoro TTS | `~/.trinity/models/` | ~338 MB |
| Whisper STT | `~/.trinity/models/` | ~150 MB |

### Legacy (Archived)
| Path | Status |
|------|--------|
| `~/trinity-models/vllm/` | AWQ models for vLLM server tier — archive or delete |
| `~/trinity-models/omni/LongCat-*` | **Delete** — deprecated, ~383 GB |

> **⚠️ Model Consolidation Needed:** Scan system for scattered AI models and consolidate into `~/.lmstudio/models/` for GGUF models and `~/.trinity/models/` for ONNX models.

---

## 6. What Is Working (L5 — Don't Touch)

These subsystems are mature. Refactoring them gains nothing:

| Component | File(s) | Why It's Done |
|-----------|---------|---------------|
| OpenAI-compatible inference client | `inference.rs` | Dynamic model resolution, streaming, tool calling |
| Multi-backend inference router | `inference_router.rs` | PartyRole enum, auto-detect, failover, LM Studio primary |
| Conductor phase system | `conductor_leader.rs` | 12 Socratic prompts, Bloom's mapping, RLHF injection |
| Agentic tool loop | `agent.rs` | 38 tools, parallel execution, SSE streaming |
| Quest state machine | `quests.rs` | ADDIECRAPEYE phase gating, XP/Coal/Steam, PEARL alignment |
| EYE Package export | `export.rs` | HTML5 Quiz, Adventure, DOCX, ZIP — all working |
| CowCatcher telemetry | `cow_catcher.rs` | Obstacle tracking, severity, auto-restart logic |
| VAAM vocabulary engine | `vaam.rs`, `vaam_bridge.rs` | Word scanning, Coal rewards, cognitive load |
| Scope Creep interceptor | `scope_creep.rs` | Auto-detect in both chat paths, PEARL semantic check |
| Character Sheet | `character_sheet.rs` | Shadow status, friction/traction, Gemini Protocol |
| Config system | `configs/runtime/default.toml` | LM Studio primary, ORT embedded, vLLM optional |

---

## 7. Work Queue (Prioritized)

### 🔴 P0 — Blocking Demo

| # | Task | Effort |
|---|------|--------|
| 1 | **Model consolidation scan**: find all AI models on system, move GGUF → `~/.lmstudio/models/`, ONNX → `~/.trinity/models/` | 1 hour |
| 2 | **End-to-end test**: Trinity ↔ LM Studio — full Socratic conversation, quest advance, phase change | 2 hours |
| 3 | **Onboarding flow**: Pete's first playable — user types name, gets PEARL, starts Analysis, completes one objective | 4 hours |
| 4 | **Story Engine (ORT)**: `story_engine.rs` — small ONNX model for standalone mode fallback | 4 hours |

### 🟡 P1 — Important for Polish

| # | Task | Effort |
|---|------|--------|
| 5 | **ComfyUI MCP integration**: connect as Hook Book tool, Trinity teaches user to use it | 3 hours |
| 6 | **LM Studio headless overnight script**: `trinity-overnight.sh` for autonomous worldbuilding | 2 hours |
| 7 | **Quest objective content**: `/fill-objectives` for all 12 ADDIECRAPEYE phases | 2 hours |
| 8 | **Fix React components**: update any referencing old vLLM fleet APIs | 2 hours |

### 🟢 P2 — Future

| # | Task | Effort |
|---|------|--------|
| 9 | **Unsloth fine-tuning**: Story Model (~3B) with Opus distillation | 8 hours |
| 10 | **Janus Pro ONNX export**: vision critique for CRAP evaluation | 4 hours |
| 11 | **Graphify integration**: codebase graph for 70x token reduction | 3 hours |
| 12 | **90-second demo recording** | 1 hour |

---

## 8. Workflow Reference

| Workflow | Phase | Purpose |
|----------|-------|---------|
| `/session-start` | Pre-Analysis | Check services, build Trinity, open browser |
| `/build-and-test` | Pre-Analysis | Build, test, verify server health |
| `/first-playable` | Analysis → Development | Pete's onboarding + gameplay loop |
| `/wire-pete-socratic` | Design → Development | Validate AI responses respect current phase |
| `/fill-objectives` | All phases | Populate quest objectives for all 12 phases |
| `/fix-frontend-component` | CRAP phases | Update a single React component, build-verify |
| `/fix-rust-backend` | Development → Yoke | Fix backend logic, compile-verify |
| `/research-implementation` | Envision → Yoke | Implement architectural data sourcing |
| `/commit-wrap` | Evolve | Run tests, commit, update docs |

---

## 9. File Map (Where Things Live)

```
trinity-genesis/
├── crates/trinity/src/              # Rust backend (THE code)
│   ├── main.rs                      # Routes, AppState, all API endpoints
│   ├── agent.rs                     # Yardmaster agentic loop + 38 tools
│   ├── inference_router.rs          # Multi-backend router (LM Studio primary)
│   ├── inference.rs                 # OpenAI-compatible streaming client
│   ├── conductor_leader.rs          # ADDIECRAPEYE state machine + Socratic prompts
│   ├── health.rs                    # /api/health endpoint
│   ├── cow_catcher.rs               # Obstacle telemetry
│   ├── tools.rs                     # Tool execution engine
│   ├── creative.rs                  # Image/video/3D generation dispatch
│   ├── voice.rs                     # Kokoro TTS integration
│   ├── rag.rs                       # RAG search + embeddings (ORT)
│   ├── export.rs                    # EYE Package export
│   ├── vaam.rs / vaam_bridge.rs     # Vocabulary mining
│   └── scope_creep.rs               # Scope creep interception
├── crates/trinity-daydream/         # Bevy 0.18 3D LitRPG / XR Studio
├── crates/trinity-protocol/         # Shared types (26 modules)
├── crates/trinity-quest/            # Quest engine
├── crates/trinity-iron-road/        # Book writing + VAAM
├── crates/trinity-voice/            # Voice pipeline
├── crates/trinity-mcp-server/       # MCP server for agentic extensibility
├── configs/runtime/default.toml     # Runtime config (LM Studio primary)
├── docs/core_bibles/                # TRINITY_FANCY_BIBLE.md + field manuals
├── docs/archive/vllm_server_tier/   # vLLM config for Purdue (preserved)
├── scripts/launch/                  # Launch scripts
└── context.md                       # ← YOU ARE HERE
```

---

## 10. DAYDREAM XR Architecture

The `trinity-daydream` crate dual-targets **desktop** and **XR headsets** from a single codebase using Cargo feature flags.

### Build Targets

| Target | Feature | Command |
|--------|---------|---------|
| Desktop (windowed) | `desktop` | `cargo run --features desktop -p trinity-daydream` |
| XR (OpenXR headset) | `xr` | `cargo run --features xr -p trinity-daydream` |
| Android (Quest 3S) | `xr` | `./scripts/build_xr.sh` |

### Device Triangle

```
         GMKtek Strix Halo (128GB)
         ┌─────────────────────┐
         │ Trinity Axum :3000   │
         │ LM Studio :1234      │
         │ ORT Embedded Models  │
         └──────┬────────┬──────┘
                │WiFi 7  │WiFi 7
         ┌──────▼──┐  ┌──▼──────────┐
         │ Pixel 10│  │ Quest 3S    │
         │ Mini    │  │ DAYDREAM XR │
         │ Trinity │  │ (future)    │
         └─────────┘  └─────────────┘
```

All devices connect to the same Axum API. The XR headset uses `TRINITY_SERVER_URL` env var to find the GMKtek on the LAN.
