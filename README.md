# Trinity ID AI OS

**AI can compute. Trinity is for the imagination.**

Trinity is a prompting system that prompts both the human and the AI. It uses Socratic questioning to draw out the human's vision before any work begins, then executes through a 12-phase instructional design methodology (ADDIECRAPEYE). The result: better output, because the AI understands what you actually want before it starts building.

> Runs on a 128 GB AMD Strix Halo (Zen 5 + RDNA 3.5 + XDNA 2 NPU) with local LLMs and ComfyUI creative pipeline — no cloud APIs, no per-token costs, everything on-device.

---

## The Triple Reflection

| Layer | What it does | How |
|-------|-------------|-----|
| **Socratic** (Depth) | AI treats user as SME, questions before working | P (DiffusionGemma) or H (Hermes) |
| **FACES** (Image) | Emotional context via 4-byte protocol | NPU-accelerated (moved to Semantic Slime project) |
| **LitRPG** (Narrative) | Lightweight quest scaffolding for workflows | ADDIECRAPEYE state machine |

**The pipeline:** Human Imagination → Socratic questioning → FACES (emotional context) → LitRPG (quest framing) → ADDIECRAPEYE (12-phase execution) → Strix Halo (GPU: LLM, NPU: FACES) → Android XR (imagination visible in space)

---

## Three-Device Architecture

| Device | Role | Hardware |
|--------|------|----------|
| **Phone** (Director) | Human input, Socratic questioning, quest management | Pixel 10 Pro XL, Gemini Nano |
| **Desktop** (Engine) | AI inference, orchestration, content generation | Strix Halo, 128GB VRAM |
| **XR** (Canvas) | Spatial review, EYE phase | XREAL Aura (Fall 2026) |

**Authority flow:** Phone (commands) → Desktop (compute) → XR (spatial output)

---

## Quick Start

### Prerequisites
- Linux system with 16 GB+ RAM (developed on 128 GB Strix Halo)
- **vLLM** on port :8000 (P: DiffusionGemma 26B — execution engine)
- **ComfyUI** on port :8188 (A: creative pipeline — image, voice, video, 3D)
- Rust 1.80+

### Launch

```bash
# 1. Start P (DiffusionGemma 26B) — always-on via podman/vLLM on :8000
bash ~/trinity-models/start-diffusiongemma.sh

# 2. Start Trinity server
cargo run -p trinity -- --headless

# 3. Launch A (ComfyUI) via hotel manager
curl -X POST http://localhost:3000/api/inference/hotel/studio

# 4. Open on phone or desktop:
#    http://100.83.222.35:3000/trinity/phone.html  (Tailscale)
#    http://localhost:3000/trinity/phone.html       (local)
```

### Studio Model Setup

| Role | Model | Engine | Port | VRAM | Purpose |
|------|-------|--------|------|------|---------|
| **P** | DiffusionGemma 26B AWQ-INT4 | vLLM (podman) | :8000 | ~42GB | Execution — story, code, tool calling, reasoning |
| **A** | ComfyUI + Janus-Pro-7B + VibeVoice-1.5B | ComfyUI | :8188 | ~17GB | Creative — image gen, voice, video, 3D on demand |

Both are always resident in Studio mode (~59GB of 128GB). Tier 2 models (HunyuanVideo, TRELLIS, LongCat, FLUX, TripoSR) are loaded on demand by ComfyUI.

**Hermes 4 70B (H):** Managed externally by LM Studio on :8002 when needed for planning. Submits jobs to Trinity via POST /api/jobs.

---

## Capabilities

| Feature | Description |
|---------|-------------|
| **Chat Mode** | DiffusionGemma 26B executes tasks with tool calling, Socratic questioning |
| **Phone PWA** | Chat, voice input, TTS, hotel controls, image gen, focus modes, RAG, persistent memory |
| **Hotel Manager** | 3 modes: Studio (P+A), Solo (P only), Closed (all off) |
| **RAG** | Semantic search via in-process ONNX Runtime embeddings (nomic-embed-text INT8) |
| **Persistent Memory** | SQLite-backed cross-session memory, injected into agent system prompt |
| **ADDIECRAPEYE** | 12-phase instructional design state machine with Bloom's Taxonomy mapping |
| **FACES Protocol** | 4-byte emotional context protocol (moved to Semantic Slime project July 2026) |
| **EYE Package Export** | HTML5 quiz games, text adventures, DOCX documents |
| **Security** | Bearer token auth on dangerous endpoints, rate limiting, input validation |

---

## Project Structure

```
TRINITYIDAIOS/
├── crates/
│   ├── trinity/              # Main Axum server, agent loop, tools, inference
│   ├── trinity-protocol/     # Shared types (CharacterSheet, QuestState, etc.)
│   ├── trinity-quest/        # Quest engine (ADDIECRAPEYE state machine)
│   ├── trinity-iron-road/    # Book writing, VAAM vocabulary
│   ├── trinity-voice/        # Voice pipeline
│   ├── trinity-daydream/     # Bevy 3D/XR (future)
│   └── trinity-mcp-server/   # MCP server for extensibility
├── docs/active/              # Current docs (MASTER_PIVOT, MASTER_TASK_LIST, SECURITY, etc.)
├── docs/archive/             # Historical docs (preserved, not active)
├── scripts/launch/           # trinity_day.sh, trinity-start.sh — startup scripts
├── scripts/focus/            # creative-focus.sh, code-focus.sh, night-shift.sh
├── context.md                # System context (kept current)
└── README.md                 # This file
```

---

## Documentation

| Doc | Purpose |
|-----|---------|
| `docs/active/MASTER_PIVOT_DOCUMENT.md` | Core vision, three-device architecture, honest assessment |
| `docs/active/MASTER_TASK_LIST.md` | Architecture audit, task tracking, completion criteria |
| `docs/active/ADDIECRAPEYE_CANONICAL.md` | 12-phase instructional design reference |
| `docs/active/SECURITY.md` | Security model for public deployment |
| `docs/TRINITY_IDENTITY.md` | Authoritative 3-tier architecture: Core, Middleware, Products |
| `context.md` | Current system state, port map, what's working |

---

## FACES Protocol

FACES is a 4-byte emotional context protocol designed to be the TCP/IP of emotive AI. It encodes Focus, Action, Container, Energy, and State into a single u32.

**Status:** Moved to the [Semantic Slime](https://github.com/Joshua42atkinson/SemanticSlime) project on July 5, 2026. W1-W6 complete (283 tests + 15 doc-tests). The `trinity-faces` crate no longer exists in this workspace. Trinity's Triple Reflection philosophy still references FACES as the "Image" layer, but the implementation is an external product dependency.

---

## License

Private project. Public deployment at LDTAtkinson.com for educational demonstration.
