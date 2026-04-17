# TRINITY ID AI OS — Project Context

> **Last Updated:** April 16, 2026
> **Hardware:** AMD Strix Halo APU — Radeon 8060S (`gfx1151`), 128GB Unified LPDDR5x
> **Runtime:** Rust/Axum (:3000) + React frontend + vLLM distrobox sidecars + Bevy XR (:daydream_xr)
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

## 2. System Architecture (P-ART-Y Model Matrix)

### Design Principle: Size-to-Task Harmony

TRINITY uses three sizes of **one model family** (Google Gemma 4) plus specialized tools. Same tokenizer, same tool calling, same license (Apache 2.0), same quantization (AWQ INT4). Zero prompt fragmentation.

### Port Map

| Port | P-ART-Y Role | Model | VRAM | Loading | Status |
|------|-------------|-------|------|---------|--------|
| **8001** | **T — Tempo** | Gemma 4 E4B AWQ | ~6 GB | **Always On** | ✅ Downloaded |
| **8000** | **P — Programming** | Gemma 4 26B A4B AWQ (MoE) | ~16 GB | **Hotel Swap** | ✅ Downloaded |
| **8002** | **R — Reasoning** | Gemma 4 31B Dense AWQ | ~18 GB | **Hotel Swap** | ✅ Downloaded |
| **8003** | **A — Aesthetics** | Janus Pro 7B | ~4 GB | **Hotel Swap** | ⚠️ Not downloaded |
| embedded | **A — Image Gen** | FLUX.1-schnell GGUF | ~7 GB | **Always On** | ✅ Downloaded |
| embedded | **Voice** | Kokoro TTS ONNX | ~1 GB | **Always On** | ✅ Embedded |
| embedded | **Embeddings** | nomic-embed-text-v1.5 ONNX | ~1 GB | **Always On** | ✅ Embedded |
| **3000** | **Y — Yardmaster** | Trinity Rust/Bevy Client | ~20 GB | **Always On** | ✅ Running |

**Always-On Baseline:** ~35 GB | **Peak (with 31B):** ~53 GB | **Safety Buffer:** 75 GB free

### The Hotel Swap Protocol

Only ONE heavyweight model occupies the swap zone at any time. The Conductor (`conductor_leader.rs → manage_hotel_sidecars()`) calls `hotel_manager.rs` to:
1. Check if target model is already loaded (skip if so)
2. Kill existing process on target port
3. Launch the target model via its `--bg` launch script
4. Wait for `/health` to pass (timeout: 90s)
5. If swap fails, Tempo handles everything (Lone Wolf fallback)

Model swaps happen behind the Iron Road's phase transition animation (~8-12 seconds, invisible to the user).

### Phase-to-Gear Mapping

| Phases | Gear | Model Loaded |
|--------|------|-------------|
| Analysis, Implementation, Repetition, Evolve | **T only** | *None — Tempo handles it* |
| Design, Development, Yoke | **P** | Gemma 4 26B A4B |
| Evaluation, Alignment, Envision | **R** | Gemma 4 31B Dense |
| Contrast, Proximity | **A₂** | Janus Pro 7B |

---

## 3. Launching Trinity

```bash
# Full system (UI + Tempo backend)
./scripts/launch/trinity_ignition.sh

# Individual P-ART-Y models (inside vllm distrobox automatically)
./scripts/launch/launch_tempo_e4b.sh          # T — always-on (:8001)
./scripts/launch/launch_pete_coder.sh         # P — coding brain (:8000)
./scripts/launch/launch_pete_coder.sh --bg    # P — background for Hotel swap
./scripts/launch/launch_recycler_dense.sh     # R — reasoning (:8002)
./scripts/launch/launch_recycler_dense.sh --bg # R — background for Hotel swap

# Legacy (still works, same as launch_tempo_e4b.sh)
./scripts/launch/launch_pete.sh
```

### Distrobox Context
All vLLM models run inside the `vllm` distrobox (`kyuz0/vllm-therock-gfx1151:latest`). Launch scripts automatically inject the correct AMD environment:
```bash
HSA_OVERRIDE_GFX_VERSION=11.5.1
HSA_ENABLE_SDMA=0
PYTORCH_ROCM_ARCH="gfx1151"
PYTORCH_HIP_ALLOC_CONF=expandable_segments:True
VLLM_SKIP_WARMUP=true
```

---

## 4. Key API Endpoints

| Endpoint | Purpose |
|----------|---------|
| `GET /api/health` | Full subsystem health (LLM, DB, creative, voice, CowCatcher) |
| `GET /api/inference/status` | Active inference router status |
| `GET /api/inference/fleet` | All 4 P-ART-Y sidecar statuses |
| `GET /api/inference/hotel` | Hotel swap zone occupancy |
| `POST /api/inference/hotel/swap` | Trigger model swap `{"role": "P"}` / `"R"` / `"A"` / `"checkout"` |
| `GET /api/inference/resources` | System RAM + model profiles for UI |
| `POST /api/inference/start` | Launch Tempo from UI |
| `POST /api/inference/stop` | Stop Tempo from UI |
| `POST /api/chat` | Iron Road SSE chat (Socratic + Narration) |
| `POST /api/agent/chat` | Yardmaster agentic tool-calling loop |

---

## 5. Model Storage

| Path | Model | Status |
|------|-------|--------|
| `~/trinity-models/vllm/gemma-4-E4B-it-AWQ-4bit` | T — Tempo | ✅ Ready |
| `~/trinity-models/vllm/gemma-4-26B-A4B-it-AWQ-4bit` | P — Programming | ✅ Ready |
| `~/trinity-models/vllm/gemma-4-31B-it-AWQ-4bit` | R — Reasoning | ✅ Ready |
| `~/trinity-models/vllm/flux.1-schnell-nf4-with-transformer` | A — Image Gen | ✅ Ready |
| `~/trinity-models/vllm/nomic-embed-text-v1.5-AWQ` | Embeddings | ✅ Ready |
| `~/trinity-models/vllm/Qwen2.5-14B-Instruct-AWQ` | Legacy (unused) | Can archive |
| `~/trinity-models/vllm/Qwen2.5-7B-Instruct-AWQ` | Legacy (unused) | Can archive |
| `archive/longcat/` | Deprecated LongCat sidecars | Safely ignore |
| `~/trinity-models/omni/LongCat-*` | Failed LongCat weights (~383GB) | **Delete to reclaim space** |

---

## 6. What Is Working (L5 — Don't Touch)

These subsystems are mature. Refactoring them gains nothing:

| Component | File(s) | Why It's Done |
|-----------|---------|---------------|
| OpenAI-compatible inference client | `inference.rs` | Dynamic model resolution, streaming, tool calling |
| Multi-backend inference router | `inference_router.rs` | PartyRole enum, auto-detect, failover, role-based routing |
| Hotel Manager | `hotel_manager.rs` | Full swap lifecycle: check/kill/launch/health/fallback |
| Conductor phase system | `conductor_leader.rs` | 12 Socratic prompts, Bloom's mapping, RLHF injection, Hotel wiring |
| Agentic tool loop | `agent.rs` | 38 tools, parallel execution, SSE streaming |
| Quest state machine | `quests.rs` | ADDIECRAPEYE phase gating, XP/Coal/Steam, PEARL alignment |
| EYE Package export | `export.rs` | HTML5 Quiz, Adventure, DOCX, ZIP — all working |
| Fleet manager | `vllm_fleet.rs` | Tracks all 4 ports, hotel occupant detection |
| CowCatcher telemetry | `cow_catcher.rs` | Obstacle tracking, severity, auto-restart logic |
| VAAM vocabulary engine | `vaam.rs`, `vaam_bridge.rs` | Word scanning, Coal rewards, cognitive load |
| Scope Creep interceptor | `scope_creep.rs` | Auto-detect in both chat paths, PEARL semantic check |
| Character Sheet | `character_sheet.rs` | Shadow status, friction/traction, Gemini Protocol |
| Frontend (25 components) | `frontend/src/components/` | Iron Road UI, ART Studio, Yardmaster, Inference Manager |
| Config system | `configs/runtime/default.toml` | PartyRole annotations, always_resident flags |
| Sidecar health monitor | `sidecar_monitor.rs` | Monitors all 4 P-ART-Y targets, CowCatcher integration |

---

## 7. Work Queue (Prioritized)

### 🔴 P0 — Blocking August Demo

| # | Task | Why | Effort |
|---|------|-----|--------|
| 1 | **Download Janus Pro 7B** and validate `janus_sidecar.py` works | Aesthetics role has no model. CRAP phases (Contrast, Proximity) need vision critique. | 1 hour |
| 2 | **End-to-end Hotel swap test**: trigger phase transition Analysis → Design → Evaluation → Contrast in Iron Road and verify the Conductor actually loads/unloads models | All the wiring is done but untested against real vLLM processes | 2 hours |
| 3 | **Onboarding flow**: Pete's first playable — user types name, gets PEARL, starts Analysis phase, completes one objective, and sees phase advance | Currently fragmented across components. Needs a guided walkthrough. | 4 hours |
| 4 | **Fix any React components that reference stale APIs** (e.g. `pete_online` vs `tempo_online` in InferenceManager.jsx, TrainStatus.jsx) | Backend JSON shape changed — frontend may break on health/fleet endpoints | 2 hours |

### 🟡 P1 — Important for Polish

| # | Task | Why | Effort |
|---|------|-----|--------|
| 5 | **`trinity_ignition.sh` update**: should launch `launch_tempo_e4b.sh` instead of `launch_pete.sh`, skip legacy A.R.T.Y. Hub, and just start Tempo + Trinity server | Ignition script still references the old architecture | 30 min |
| 6 | **Janus Pro CRAP evaluation integration test**: take a screenshot of a React component, send to Janus on :8003, get a structured CRAP critique back and display in QualityScorecard.jsx | Validates the full Aesthetics pipeline | 3 hours |
| 7 | **Hotel swap latency measurement**: instrument `hotel_manager.rs` to report timing to the CowCatcher and expose via `/api/inference/hotel` | Need to know if swaps fit within the phase transition animation budget | 1 hour |
| 8 | **Quest objective content**: run `/fill-objectives` to populate all 12 ADDIECRAPEYE phases with real pedagogical objectives. Currently many phases have placeholder text. | Players need real objectives to complete. | 2 hours |

### 🟢 P2 — Nice to Have / Future

| # | Task | Why | Effort |
|---|------|-----|--------|
| 9 | **LM Studio / Ollama fallback path**: verify the inference router's fallback backends work on Tier 1/2 student hardware (8GB-32GB) | Distribution to LDT students who don't have a Strix Halo | 2 hours |
| 10 | **ACE-Step 1.5 ambient music sidecar** on :8008 | Tempo role for music gen during phase transitions (mood-setting) | 3 hours |
| 11 | **Delete LongCat weights** (~383GB in `~/trinity-models/omni/LongCat-*`) | Reclaim disk space | 5 min |
| 12 | **Archive Qwen models** (`Qwen2.5-14B`, `Qwen2.5-7B` in `~/trinity-models/vllm/`) | These are unused since the Gemma 4 unification | 5 min |
| 13 | **Automated integration tests**: cargo test that spins up Tempo, sends a chat, verifies response, and checks quest state mutation | CI confidence for the August demo | 4 hours |
| 14 | ~~**Bevy XR viewport**: wire the 3D engine to display VAAM spatial representations~~ | **✅ COMPLETE** — tri-daydream now dual-targets desktop + XR. Spatial UI, P-ART-Y bridge, feature flags, build script all done. Needs Quest 3S for hardware testing. | Done |

---

## 8. Workflow Reference

The 9 agent workflows map to the ADDIECRAPEYE lifecycle:

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
│   ├── main.rs                      # Routes, AppState, all API endpoints (~5200 lines)
│   ├── agent.rs                     # Yardmaster agentic loop + 38 tools
│   ├── inference_router.rs          # Multi-backend router with PartyRole
│   ├── inference.rs                 # OpenAI-compatible streaming client
│   ├── hotel_manager.rs             # Hotel Swap Protocol (process lifecycle)
│   ├── conductor_leader.rs          # ADDIECRAPEYE state machine + Socratic prompts
│   ├── vllm_fleet.rs                # Fleet health monitoring (4 ports)
│   ├── health.rs                    # /api/health endpoint
│   ├── sidecar_monitor.rs           # Background health watchdog
│   ├── cow_catcher.rs               # Obstacle telemetry
│   ├── tools.rs                     # Tool execution engine
│   ├── creative.rs                  # Image/video/3D generation dispatch
│   ├── voice.rs                     # Kokoro TTS integration
│   ├── rag.rs                       # RAG search + embeddings
│   ├── export.rs                    # EYE Package export
│   ├── vaam.rs / vaam_bridge.rs     # Vocabulary mining
│   └── scope_creep.rs               # Scope creep interception
├── crates/trinity/frontend/src/     # React frontend
│   ├── App.jsx                      # Main app shell + routing
│   └── components/                  # 25 React components
├── crates/trinity-daydream/         # Bevy 0.18 3D LitRPG / XR Studio
│   ├── src/daydream.rs              # DaydreamPlugin (shared core, feature-gated shell)
│   ├── src/xr_shell.rs              # XR world setup (room-scale, no window)
│   ├── src/spatial_ui.rs            # Native Bevy UI dashboard (no egui)
│   ├── src/party_bridge.rs          # HTTP bridge to P-ART-Y AI fleet
│   ├── src/bin/daydream.rs          # Desktop binary (--features desktop)
│   ├── src/bin/daydream_xr.rs       # XR binary (--features xr)
│   └── Cargo.toml                   # Features: desktop, xr, save
├── configs/runtime/default.toml     # Single source of truth for backend config
├── scripts/launch/                  # Model launch scripts
├── scripts/build_xr.sh              # XR build + deploy script
├── quests/                          # ADDIECRAPEYE quest definitions
├── docs/                            # Architecture docs
│   ├── PARTY_FRAMEWORK.md           # P-ART-Y model philosophy
│   ├── MATURATION_MAP_APRIL_13.md   # Historical maturation record
│   └── HOOK_BOOK.md                 # Tool/capability catalogue
└── context.md                       # ← YOU ARE HERE
```

---

## 10. DAYDREAM XR Architecture

The `trinity-daydream` crate now dual-targets **desktop** and **XR headsets** from a single codebase using Cargo feature flags.

### Build Targets

| Target | Feature | Binary | Command |
|--------|---------|--------|---------|
| Desktop (windowed) | `desktop` | `daydream` | `cargo run --features desktop -p trinity-daydream` |
| XR (OpenXR headset) | `xr` | `daydream_xr` | `cargo run --features xr -p trinity-daydream` |
| Android (Quest 3S) | `xr` | via cargo-ndk | `./scripts/build_xr.sh` |

### Feature-Flag Matrix

| Module | Desktop | XR | Purpose |
|--------|:-------:|:--:|---------|
| `daydream.rs` | ✓ | ✓ | Core plugin (physics, commands, animations) |
| `spatial_ui.rs` | ✓ | ✓ | Native Bevy HUD: PEARL Compass, Quest, Gauges, Fleet |
| `party_bridge.rs` | ✓ | ✓ | HTTP polling to Axum server for state + blueprints |
| `xr_shell.rs` | | ✓ | Room-scale world (no window, no orbit camera) |
| `art_panels.rs` | ✓ | | egui control rail for AI art |
| `python_bridge.rs` | ✓ | | PyO3 scripting (requires CPython) |
| `train_car.rs` | ✓ | | Isomorphic train UI |
| `hud.rs`, `sao_menu.rs` | ✓ | | egui developer tools |

### Device Triangle

```
         GMKtek Strix Halo (128GB)
         ┌─────────────────────┐
         │ Trinity Axum :3000   │
         │ P-ART-Y Fleet        │
         │ (Tempo, P, R, A)     │
         └──────┬────────┬──────┘
                │WiFi 7  │WiFi 7
         ┌──────▼──┐  ┌──▼──────────┐
         │ Pixel 10│  │ Quest 3S    │
         │ Mini    │  │ DAYDREAM XR │
         │ Trinity │  │ (future)    │
         └─────────┘  └─────────────┘
```

All devices connect to the same Axum API. The XR headset uses `TRINITY_SERVER_URL` env var to find the GMKtek on the LAN.

### Key Dependencies

| Crate | Version | Purpose |
|-------|---------|--------|
| `bevy` | 0.18.1 | ECS game engine |
| `avian3d` | 0.6 | Physics (XPBD) |
| `bevy_mod_openxr` | 0.5 | OpenXR session lifecycle |
| `bevy_mod_xr` | 0.5 | XR framework abstractions |
| `moonshine-save` | 0.4 | Selective entity save/load |
| `bevy_panorbit_camera` | 0.34 | Desktop orbit controls |
