# Trinity ID AI OS — Research Bible & Session Context
## March 22, 2026 — Production Prototype v13.1.0 (Phase 6: Graduation)

---

## 1. WHAT TRINITY IS

Trinity ID AI OS is a gamified instructional design system that helps K-12 teachers build educational games. It uses AI agents orchestrated through ADDIECRAPEYE (a 12-station instructional design lifecycle) to autonomously create games, lesson plans, and educational media.

**TRINITY = ID + AI + OS:**
- **ID = ADDIE** — Instructional Design backbone (Analyze, Design, Develop, Implement, Evaluate)
- **AI = CRAP** — Visual Design principles (Contrast, Repetition, Alignment, Proximity)
- **OS = EYE** — Observer/Metacognition (Envision, Yoke, Evolve)

**The P-ART-Y Framework (Who operates Trinity):**
- **P = Pete** — The ONLY AI personality (Mistral Small 4 119B, single static RAM load, ~68GB)
- **A = Aesthetics** — Functional mode: CRAP visual design, ComfyUI assets (same brain, different prompt)
- **R = Research** — Functional mode: QM audits, tests, CI/CD, the Cow Catcher (same brain, different prompt)
- **T = Tempo** — Functional mode: Code gen, Bevy scaffolding, 30-second loop momentum (same brain, different prompt)
- **Y = You** — The Yardmaster. You manage the yard. Executive core. Delegates to P-ART.

**Architecture:** One brain, one static RAM load. ART modes are just different system prompts routed through Pete. Only Python services (ComfyUI, Voice) run as sidecars for crash isolation.

**The Iron Road is the hook. Productivity is the product.** Trinity needs to actually build games, not just look like it can.

---

## 2. THE "MEANING MAKING" TRACE (Isomorphic Alignment)

Trinity ensures functional alignment across every level of the system:

1. **AI Attention (HOW to attend)**: **Sacred Circuitry** (15 nodes) — internal scaffolding, de-emphasized in UI.
2. **User Preference (WHAT user prefers)**: **VAAM Bridge** (Profile + Word Weights) — 49 tests passing.
3. **Methodology (WHAT to do)**: **ADDIECRAPEYE** (12 stations) lifecycle — 18 tests passing.
4. **Identity (WHO the user is)**: **Character Sheet** (Skill Boosts per station completion).

**Functional Flow:**
User Message → VAAM Bridge (Style/Vocab detect) → Pete Orchestration (VAAM aligned) → Quest Objective Complete → Station Advance → **Character Skill Boost** (Curriculum, Gamification, Narrative, or Assessment Design).

---

## 3. ARCHITECTURE

### Physical Setup
- **GMKtek EVO X2 128GB** = Headless server (AMD Strix Halo).
- **Strix Halo Hardware:** 16 Zen 5 cores, 40 RDNA 3.5 CUs, XDNA 2 NPU (50 TOPS).
- **Access:** `http://evo-x2:3000/` (Unified UI) or `:7777` (Voice UI).

### Runtime Architecture
```
128GB Unified RAM (125GB usable)
├── Mistral Small 4 119B MoE (~68GB via llama-server on :8080)
│   Q4_K_M GGUF (2 shards), 256K context (--ctx-size 262144), OpenAI-compatible API.
│   Vulkan backend, flash-attn on, -np 2 dual slots (Duality KV Cache).
│   KV cache: ~11.5GB @ 256K×2 slots (Recycler slot 0, Pete slot 1).
│
├── Qianfan-OCR 4B (~2.5GB via llama-server on :8081)
│   Researcher sub-agent. Document parsing, OCR, chart understanding, KIE.
│   Q4_K_M GGUF, 32K context, vision-enabled.
│
├── ComfyUI (SDXL Turbo on :8188) — ~2GB
│   Image generation wired end-to-end via Rust creative.rs
│
├── Voice Pipeline (~2GB via Python on :7777)
│   openwakeword + faster-whisper + Kokoro 82M.
│
├── Rust Axum Server (:3000)
│   ADDIECRAPEYE orchestration, Quest persistence, Unified UI.
│   VAAM Alignment: User style/prefs injected into all AI system prompts.
│   Real-time hardware telemetry (CPU/RAM/GPU/NPU via sysfs).
│   PostgreSQL: sessions, messages, projects, pgvector RAG.
│   Auto-ingest: 7 Trinity docs loaded into RAG on startup.
│   Bevy game scaffolding: templates/ → scaffold_bevy_game tool.
│
└── System (~10GB)
    OS, PostgreSQL (trinity:trinity@127.0.0.1:5432/trinity).
```

---

## 4. HARDWARE (VERIFIED)

- **Machine:** GMKtek EVO X2 128GB
- **CPU/GPU:** AMD Ryzen AI Max+ 395 (Strix Halo, gfx1151, 40 CUs RDNA 3.5)
- **Memory:** 128GB LPDDR5X-8000 unified, 256 GB/s bandwidth
- **NPU:** XDNA 2 (50 TOPS)
- **Kernel:** 6.19.4 with `iommu=pt amdgpu.gttsize=126976 ttm.pages_limit=33554432`
- **ROCm:** 7.2.0 at `/opt/rocm-7.2.0/`
- **Strix Halo env:** `HSA_OVERRIDE_GFX_VERSION=11.0.0 ROCBLAS_USE_HIPBLASLT=1`

---

## 5. MODEL STATUS

### Mistral Small 4 119B (GGUF — PRIMARY BRAIN)
- **Path:** `~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-0000{1,2}-of-00002.gguf`
- **Launch:** `LD_LIBRARY_PATH=llama.cpp/build-vulkan/bin llama-server -m .../Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf --host 127.0.0.1 --port 8080 -ngl 99 --ctx-size 262144 --flash-attn on`
- **Auto-Launch:** Trinity auto-detects and launches llama-server at startup if no LLM found
- **Specs:** 118.97B params, 128 experts, MoE, reasoning_effort=high, Vulkan backend

### Voice Pipeline Models (all CPU, ~2GB total)
- **openwakeword:** alexa, hey_jarvis, hey_mycroft (~10MB)
- **faster-whisper base.en:** ASR on CPU (~140MB)
- **Kokoro 82M:** TTS on CPU, 54 voices (~300MB)

### Qianfan-OCR (GGUF — RESEARCHER SUB-AGENT)
- **Path:** `~/trinity-models/gguf/Qianfan-OCR-Q4_K_M.gguf`
- **Launch:** `llama-server -m ~/trinity-models/gguf/Qianfan-OCR-Q4_K_M.gguf --port 8081 --ctx-size 32768`
- **Specs:** 4B params, 192 languages, Layout-as-Thought, #1 on OmniDocBench v1.5 (93.12)
- **Role:** Researcher (R in P-ART-Y) — document intelligence, OCR, chart QA, KIE

### Other Models on Disk (GGUF, for hot-swapping)
- Crow 9B (5.3GB), REAP 25B MoE (15GB), OmniCoder 9B (5.4GB)
- Qwen3.5-35B-A3B (20GB), Step-3.5-Flash-121B (83GB)

---

## 6. CODEBASE STATE (as of March 22, 2026 — Phase 6 Graduation)

**6 workspace crates, 0 compile errors, 0 warnings, 175 tests pass.**
**32,000+ active LOC | 150K+ archive LOC | 638 template LOC | React frontend (14 components, 7 hooks)**

### Workspace Crates
| Crate | LOC | Tests | Description |
|-------|:---:|:-----:|-------------|
| trinity | 17,000+ | 51 | Axum server :3000, agent (29 tools, structured function calling, **dual persona**: Great Recycler/Programmer Pete), inference (reasoning_effort + tool_calls), **inference_router** (multi-backend auto-detect + failover), persistence (tool call logging), creative, RAG, auto-launch LLM, **EYE export** (HTML5 quiz/adventure/JSON), **eye_container**, **http** (shared clients + unified health checks), **Ring 2 permission enforcement**, **Ring 3 rolling context summary**, **Ring 5 rate limiting + sandboxing** |
| trinity-protocol | 10,400+ | 67 | Shared types, ADDIE+PEARL+VAAM+vocabulary+sacred circuitry+TamingProgress |
| trinity-iron-road | 1,000+ | 16 | Iron Road narrative, game loop, bestiary, Pete core, MadLibs |
| trinity-quest | 1,550 | 18 | Quest board, XP/Coal/Steam, objectives, save/load, circuitry state |
| trinity-voice | 576 | 10 | SSML injection, VAAM vocal emphasis |
| trinity-bevy-graphics | 908 | 0 | 3D Yard — vision processing, mesh generation, materials (restored from archive) |
| *(trinity-sidecar)* | *3,815* | *0* | *Model loading, sidecar workflow — exists in crates/ but NOT in workspace members* |

### Feature Status (HONEST)
| Feature | Status | What It Does |
|---------|--------|--------------|
| Chat/Iron Road | ✅ WIRED | User→VAAM→Agent→LLM→Response. Needs llama-server running |
| Yardmaster Chat | ✅ FIXED | SSE streaming with JSON-wrapped responses. Needs llama-server running |
| VAAM/Bestiary | ✅ WORKING | Vocab detection, semantic creeps, coal economy. 49 tests |
| Quest/ADDIECRAPEYE | ✅ WORKING | 12-phase lifecycle, objectives, party, save/load. 18+15 tests |
| Character Sheet | ✅ FIXED | Flexible JSON handler — accepts UI's full payload. Persists to disk |
| RLHF Feedback | ✅ FIXED | Accepts UI's {message_id, score, phase} payload. Logs for future use |
| Persistence | ✅ LIVE | PostgreSQL sessions, messages, projects. DAYDREAM archive |
| pgvector RAG | ✅ LIVE | Semantic search (HNSW), auto-ingest, tiered search |
| Bevy Templates | ✅ READY | Purdue campus game scaffold, GDD-injected vocab |
| Scaffold Tool | ✅ READY | scaffold_bevy_game creates Bevy projects from template |
| Archive Tool | ✅ READY | project_archive moves to DAYDREAM with metadata |
| Image Generation | ✅ WIRED | ComfyUI SDXL Turbo. `creative.rs` health check auto-launches sidecar. |
| Music Generation | ✅ WIRED | MusicGPT API client. Needs MusicGPT installed |
| Health/Hardware | ✅ LIVE | Real CPU/RAM/GPU/NPU telemetry, subsystem health checks |
| Unified API | ✅ WIRED | `POST /api/v1/trinity` — single endpoint, routes by mode |
| GDD Compilation | ✅ WIRED | Compiles 12-phase chat into structured game design document |
| **App Modes** | ✅ PHASE 5A | 3 modes: `IronRoad` (gamified), `Express` (wizard), `Yardmaster` (IDE). Auto-starts sidecars. |
| **Express Mode** | ✅ BUILT | 3-step wizard in frontend (`ExpressWizard.jsx`) → Subject, Goal, Format → quick game generation. |
| **React Frontend** | ✅ BUILT | Vite+React 3-column layout. **PhaseWorkspace**, **TrainStatus**, **ChapterRail**, **ArtStudio**, **Yardmaster**, **ExpressWizard**. Mode toggle 🚂⚡🔧. |
| **Book-View UI** | ✅ LIVE | Chat bubbles → flowing serif prose. Pete's messages as book text, user words as italic quoted journal entries. Narrator (Great Recycler) golden centered prose. System messages as mono gold-bordered margin notes. |
| **Station Navigation** | ✅ LIVE | All 12 ADDIECRAPEYE phases clickable as book chapters. Station overview pages show Hero's Journey title, Bloom's level, fill badge (COMPLETE/ACTIVE/LOCKED), blurb, 3 quest objectives. Return button restores narrative. |
| **Live LLM (256K)** | ✅ VERIFIED | Mistral Small 4 119B running on Vulkan with 256K context. Pete responds with DM-style narrative, references quest objectives, drives Socratic protocol. Tested live with Physics subject. |
| **PEARL** | ✅ LIVE | Focusing agent: subject, medium, vision, ADDIE/CRAP/EYE alignment scores, phase sync. 13 unit tests. API: GET/POST /api/pearl, PUT /api/pearl/refine. |
| **Game Loop** | ✅ WIRED | Click objectives → POST /api/quest/complete → advance phases → POST /api/quest/advance. Coal burns, steam rises, XP earned. |
| **Lexicon Appendix** | ✅ DONE | 13 concepts defined in TRINITY_FANCY_BIBLE.md Appendix A. Every acronym: pedagogy + architecture + status. |
| Video Generation | ✅ WIRED | HunyuanVideo + ComfyUI endpoint `/api/creative/video`. Needs ComfyUI + HunyuanVideo running |
| 3D Mesh | ✅ WIRED | Hunyuan3D-2.1 endpoint `/api/creative/mesh3d`. Needs Hunyuan3D running |
| **1D-2D-3D Architecture** | ✅ ACTIVE | Audio-1D (Pete narrates), Book-2D (React LitRPG), Yard-3D (Bevy WASM sandbox). 2D playable, 3D Yard next. |
| **Narrative API** | ✅ WIRED | `GET /api/narrative/generate` — Great Recycler prose from live game state. Station description, success prose, failure narratives. |
| **VAAM Persistence** | ✅ WIRED | Vocabulary mastery + detection audit trail saved to PostgreSQL after every chat via `tokio::spawn`. |
| **NPU Detection** | ✅ WIRED | `VoiceStatus.npu_available` reports XDNA hardware availability to HUD. |
| **ART Studio Tab** | ✅ GALLERY LIVE | React component: 4 generation cards (Image/Music/Video/3D Mesh), sidecar status badges, Beast Logger feed, **Asset Gallery** (auto-refreshes after generation, thumbnail cards, click-to-preview modal, audio inline player). `useCreative` hook polls status (30s) and logs (5s), manages `assets` state. `GET /api/creative/assets` lists workspace files, `GET /api/creative/assets/:filename` serves with MIME types. |
| **Yardmaster Tab** | ✅ IDE UPGRADE | 3-column layout: Quest Sidebar (phase/objectives/stats) \| Chat+Forge (center) \| System+Tools (right). Model Info Bar (live turn counter), collapsible Thinking Panel (<thinking> reasoning), continuation badge. `useYardmaster` hook: 256K context, 4K response tokens, 8 turns, SSE events (thinking/resources/skill/narrative), quest polling, model info. `cleanToolCalls()` strips raw JSON from display. |
| **Agent Prompt** | ✅ AUTONOMOUS | ACT FIRST for tasks, TALK for casual chat. [CONTINUE] multi-response chaining (3x). AUTONOMOUS WORK protocol: task_queue → work → complete → log → continue. 29 tools including work_log, task_queue, analyze_document, analyze_image, **scout_sniper**. **Dual persona**: 🔮 Great Recycler (visionary) / ⚙️ Programmer Pete (executor). |
| **Auto-Launch LLM** | ✅ BUILT | Trinity probes llama-server (8080) → LM Studio (1234) → vLLM (8000) → Ollama (11434) → auto-launches llama-server with Mistral GGUF if none found. Managed by `InferenceRouter`. |
| **Inference Router** | ✅ BUILT | Phase 3: `inference_router.rs` — auto-detects 6 backends (llama-server, vLLM, Ollama, LM Studio, SGLang, **Researcher**), health monitoring, failover, TOML config, 9 unit tests. |
| **EYE Export** | ✅ BUILT | `eye_container.rs` bundles quest data → `export.rs` generates self-contained HTML5 quiz, adventure, or raw JSON. Download buttons in UI. |
| **Onboarding Tour** | ✅ BUILT | 3-step tooltip overlay for first-time users. Highlights chat, objectives, export. localStorage-gated (shows once). |
| **In-Chat Images** | ✅ BUILT | `generate_image` tool → base64 SSE `event: image` → inline rendering in PhaseWorkspace and Yardmaster. |
| **Researcher (Qianfan-OCR)** | ✅ BUILT | `analyze_document` tool calls 4B Qianfan-OCR sub-agent on :8081. Document parsing, OCR, chart QA, KIE. 192 languages. |
| **Vision Analysis** | ✅ BUILT | `analyze_image` tool sends images to primary LLM vision. General-purpose image understanding. |
| **work_log Tool** | ✅ BUILT | Agent writes timestamped markdown reports to `~/Workflow/trinity-reports/` for next-day EYE review. |
| **task_queue Tool** | ✅ BUILT | File-based task queue (`TASK_QUEUE.md`): read/add/complete/next actions for autonomous overnight work. |
| **python_exec Tool** | ✅ BUILT | Sandboxed Python execution: temp file, pip install, 60s timeout, output capture. Teachers use Python. |
| **Structured Tool Calling** | ✅ BUILT | Phase 2: OpenAI-compatible `tools` array in inference requests, `tool_calls` in responses. Agent tries structured first, falls back to regex. `--jinja` flag on all launch points. |
| **Educational Tools** | ✅ BUILT | 4 classroom-ready generators: `generate_lesson_plan` (Bloom's), `generate_rubric` (multi-criteria), `generate_quiz` (MC/TF/short), `curriculum_map` (weekly progression). |
| **Zombie Guard** | ✅ BUILT | `zombie_check` tool + automatic pre-build guard in `cargo_check`. Kills orphan rustc/cc processes before each build. Prevents stuck-build deadlocks. |
| **Tool Call Persistence** | ✅ BUILT | `trinity_tool_calls` PostgreSQL table logs every agent tool invocation with params, result, latency for analytics. |
| **Reasoning Mode** | ✅ BUILT | `reasoning_effort=high` on all Yardmaster turns. `<thinking>` tag extraction → streamed as SSE event. Pete chat passes None (stays fast). 300s timeout for reasoning-heavy tasks. |
| **Duality KV Cache** | ✅ BUILT | `id_slot` field on all inference requests. Persona→slot routing: Great Recycler=slot 0 (strategic), Programmer Pete=slot 1 (execution). llama-server runs with `--parallel 2`. Instant persona switching, 500K total context. |
| Forge Terminal | ✅ BUILT | Shows tool execution logs in Yardmaster tab (colored line types: command/success/error) |
| **3D Yard (Bevy)** | 🔶 PARKED | The blank canvas — user builds here. `trinity-bevy-graphics` archived (30K+ LOC). Blocked by winit 0.30.13 + Rust 1.94 type inference failure. Post-graduation. |
| Knowledge Tracing | ⏳ PARKED | Static SVG curve, hardcoded BKT values. Post-graduation research project. |
| RLHF Training | ⏳ PARKED | Feedback logged, not yet wired to retraining. Post-graduation. |

### Key API Endpoints (Rust server :3000)
| Method | Path | Purpose |
|--------|------|---------|
| GET | `/api/health` | Real subsystem health (LLM, DB, ComfyUI, MusicGPT, Voice) |
| POST | `/api/v1/trinity` | Unified chat — "talk to Trinity like a person" |
| POST | `/api/chat` | Direct LLM chat |
| POST | `/api/chat/stream` | SSE streaming chat |
| POST | `/api/chat/yardmaster` | Agentic chat with tool-calling |
| POST | `/api/orchestrate` | ADDIECRAPEYE quest progression |
| GET | `/api/quest` | Full game state + ADDIECRAPEYE phases |
| POST | `/api/quest/advance` | Advance to next phase |
| POST | `/api/quest/compile` | Compile GDD from quest progress |
| GET | `/api/character` | Character sheet |
| **POST** | **`/api/ground`** | **I AM grounding ritual — Intent Engineering** |
| **POST** | **`/api/intent`** | **Set session posture (Mastery/Efficiency) + purpose** |
| **POST** | **`/api/bestiary/tame`** | **Scope Hope / Scope Nope decisions** |
| **POST** | **`/api/pearl`** | **Create/replace PEARL (subject + medium + vision)** |
| **PUT** | **`/api/pearl/refine`** | **Update vision, medium, or alignment scores** |
| **GET** | **`/api/pearl`** | **Retrieve current PEARL data** |
| **POST** | **`/api/quest/complete`** | **Complete a quest objective (burns coal, generates steam)** |
| GET | `/api/bestiary` | Full Creep collection with taming scores |
| POST | `/api/creative/image` | ComfyUI image generation |
| POST | `/api/creative/music` | MusicGPT audio generation |
| GET | `/api/creative/status` | Sidecar health (ComfyUI + MusicGPT) |
| **GET** | **`/api/narrative/generate`** | **Great Recycler prose from live game state** |
| **GET** | **`/api/voice/status`** | **Voice pipeline + NPU hardware availability** |
| POST | `/api/tools/execute` | Agentic tool execution (shell, files, scaffold, archive) |
| GET | `/api/hardware` | CPU/RAM/GPU/NPU telemetry + model inventory |
| GET | `/api/sessions` | List conversation sessions |
| GET | `/api/sessions/history` | Load session chat history |
| GET | `/api/projects` | List game projects |
| POST | `/api/projects/archive` | DAYDREAM archive |
| GET | `/api/rag/stats` | RAG knowledge base statistics |
| POST | `/api/rag/search` | Semantic search |
| **GET** | **`/api/inference/status`** | **Multi-backend router status — all backends, health, capabilities** |
| **POST** | **`/api/inference/switch`** | **Switch active inference backend by name** |
| **POST** | **`/api/inference/refresh`** | **Re-probe all backends, update health flags** |
| **POST** | **`/api/eye/compile`** | **Compile EYE container from quest data** |
| **GET** | **`/api/eye/preview`** | **Preview EYE container JSON** |
| **GET** | **`/api/eye/export`** | **Export HTML5 quiz/adventure/JSON (?format=)** |

### Key API Endpoints (Voice server :7777)
- `POST /api/say` — Text-to-voice
- `POST /api/start` / `POST /api/stop` — Voice listening control
- `WS /ws` — Real-time status + conversation updates

---

## 7. KEY DECISIONS

- **llama-server** serves Mistral Small 4 GGUF on `:8080` — the working brain (Vulkan, flash-attn on)
- **Auto-launch** — Trinity spawns llama-server automatically if none found on startup
- **Python voice pipeline** on `:7777` — wake word, ASR, TTS, agentic tools
- **OpenAI-compatible API** — both Rust server and voice pipeline talk to `:8080`
- **LLM_URL env var** — Trinity backend reads `LLM_URL=http://127.0.0.1:8080` for inference
- **Models evolve, Trinity adapts** — architecture is model-agnostic
- **PostgreSQL optional** — server starts without DB (uses `connect_lazy`), quest save disabled
- **No embedded inference for shipping** — HTTP to llama-server is the production path
- **Apache 2.0** — Permissive open source. Users own content built with Trinity.

### Bevy / winit Lessons (March 21, 2026)
- **Workspace Bevy version:** `bevy = "0.18.1"`, `bevy_egui = "0.39.1"` in root `Cargo.toml`
- **winit 0.30.13 + Rust 1.94.0 = 63 E0282 errors.** The `maybe_queue_on_main` / `maybe_wait_on_main` closures in winit lack type annotations that Rust 1.94's stricter inference needs. No fix released as of March 21, 2026 (winit 0.30.14 does not exist, 0.31.0-beta.2 exists but Bevy requires `^0.30`).
- **Fix:** `trinity-bevy-graphics` overrides workspace Bevy dep: `bevy = { version = "0.18.1", default-features = false, features = ["bevy_render", "bevy_asset", "bevy_color", "bevy_mesh"] }`. Excluding `bevy_winit` avoids the winit compile failure entirely. This works because the crate is a library (vision + mesh generation), not a window launcher.
- **When windowing is needed** (Bevy WASM `<canvas>` or desktop preview): either wait for winit 0.30.14+, or build WASM targets where winit behavior may differ, or pin an older Rust.
- **`trinity-sidecar`** exists in `crates/` (3,815 LOC) but is NOT in workspace members — intentionally excluded, may need cleanup or re-inclusion.

---

## 8. PROJECT GOALS

- **The Finish Line**: Close the gap on a full production prototype for Purdue.
- **Distribution**: ConsciousFramework.com, GreatRecycler.com.
- **Nonprofit Foundation**: Planned for maintenance, grants, and community.
- **CRITICAL**: Trinity must produce real work product (game designs, lesson plans, assets). The Iron Road is the UX hook — productivity is the deliverable.

---

## 9. WHAT NEEDS TO HAPPEN NEXT

> **Completed: March 22, 2026 (Trinity Industrialization Day):**
> - **Phase 5B** — In-chat creative generation (SSE image events, inline rendering)
> - **Phase 5C** — EYE export system (`eye_container.rs`, `export.rs`, HTML5 quiz/adventure/JSON)
> - **Phase 5D** — Onboarding tour (3-step tooltip, localStorage-gated)
> - **ART Fix** — `base64_encode()` stub fixed, `[creative]` config section added
> - **Researcher** — Qianfan-OCR 4B integrated as sub-agent (`analyze_document` tool on :8081)
> - **Vision** — `analyze_image` tool for primary LLM vision
> - **Config** — `default.toml` updated with researcher backend, creative endpoints, model paths
> - Dead code pruning (44→34 annotations, all with intent comments), compiler warning cleanup, 175 tests passing
> - Shared HTTP client (`http.rs`) — 20 client duplications → 3 shared lazy-static clients
> - Dual persona system: 🔮 Great Recycler / ⚙️ Programmer Pete + Scout Sniper tool
> - **Duality KV Cache**: `id_slot` in inference, persona→slot routing, auto-launch with `--parallel 2`
> - Competitive evaluation: `docs/research/MATURATION_EVALUATION.md`
>
> **Phase 6 — Graduation (IN PROGRESS):**
> - 6A: Documentation honesty (CONTEXT.md, FUTURE_IDEAS.md refresh)
> - 6B: End-to-end Gym Coach smoke test
> - 6C: Config & model cleanup (consolidate GGUF paths)
> - 6D: Tidy loose ends (audit stubs, park explicitly)
> - 6E: Professor README (one-page institutional doc)
>
> **Ring Security System (March 22, 2026 PM):**
> - **Ring 2** ✅ — Destructive tool gate: `execute_tool_internal` blocks Destructive tools unless persona has dev/programmer clearance. Recycler/IronRoad blocked from shell/python_exec/etc.
> - **Ring 3** ✅ — Rolling context summary: old messages compressed into deterministic digest (topics, directives, tools, files) instead of hard-truncated. 10 recent messages kept verbatim.
> - **Ring 5** ✅ — Rate limiting (60 calls/min global, 5 destructive/min) + enhanced shell sandboxing (40+ blocked patterns: network exfiltration, pipe-to-exec, privilege escalation).
> - **Ring 6** — Perspective Engine (design work — deferred)

### THE PLAN: 1D-2D-3D Layered UI

The Iron Road (2D) scaffolds instructional design. The Yard (3D Bevy) is the blank canvas
where users build their game/experience. The AI automates Yard work but the user is in
the loop as EYE (Envision → Yoke → Evolve). Everything in the archive once compiled and
ran locally — we're bringing it forward to Bevy 0.18.1.

```
1D(Audio) feeds → 2D(Book) narrates → 3D(Yard) creates
ADDIE + CRAP    → Iron Road LitRPG  → Bevy studio sandbox
Pete speaks     → Player reads/plays → Yardmaster builds
```

### Priority 1: Build & Verify
1. `/build-and-test` — verify 5-crate workspace compiles, 175+ tests pass (includes zombie cleanup step)
2. Verify Pete's Socratic Protocol — `/wire-pete-socratic`

### Priority 2: The Yard (Document Management)
3. Build `YardWorkspace.jsx` — drag-and-drop document upload, inventory display
4. Create `/api/yard/upload` — multipart file upload → RAG ingest pipeline
5. Wire Yard tab in App.jsx — separate workspace for user-provided content
6. Pete catalogs and digitizes user documents (PDF, text, images)

### Priority 3: Bring Back the Yard (3D)
7. Restore `trinity-bevy-graphics` to workspace — update deps from archive version to `bevy = "0.18.1"`
8. Update `templates/first-game/` from bevy 0.14 → 0.18.1
9. Compile and test Bevy crate independently (`cargo check -p trinity-bevy-graphics`)
10. Wire Bevy WASM `<canvas>` into React frontend (the 3D Yard tab)

### Priority 4: End-to-End
11. End-to-end test — llama-server → Pete chat → all 12 phases → GDD compile
12. Scaffold test — scaffold_bevy_game → Bevy project compiles
13. Professor README — one-page institutional review doc

### Deferred (post-Purdue)
- Video Generation (HunyuanVideo), Knowledge Tracing (BKT), RLHF fine-tuning

---

## 10. FILE MAP

```
CONTEXT.md                              ← YOU ARE HERE (research bible)
TRINITY_FANCY_BIBLE.md                  ← The Iron Road design bible (lore + mechanics + pedagogy + lexicon)
crates/trinity/src/main.rs              ← Axum server (2,000+ lines)
crates/trinity/src/agent.rs             ← Agent chat with tool-calling + [CONTINUE] + AUTONOMOUS WORK (900+ lines)
crates/trinity/src/tools.rs             ← 29 agentic tools incl. analyze_document, analyze_image, scout_sniper (2,200+ lines)
crates/trinity/src/http.rs              ← Shared HTTP clients (QUICK/STANDARD/LONG) + unified check_health()
crates/trinity/src/persistence.rs       ← PostgreSQL sessions/messages/projects + tool_calls
crates/trinity/src/rag.rs               ← pgvector semantic search + RAG
crates/trinity/src/inference.rs         ← LLM client (OpenAI API to :8080) + structured tool calling
crates/trinity/src/inference_router.rs   ← Multi-backend auto-detect + failover (6 backends incl. Researcher)
crates/trinity/src/creative.rs          ← ComfyUI + MusicGPT + HunyuanVideo + Hunyuan3D (1,132 lines)
crates/trinity/src/eye_container.rs     ← EYE container: bundle quest data into exportable artifact
crates/trinity/src/export.rs            ← HTML5 quiz/adventure/JSON export engine
crates/trinity/src/conductor_leader.rs  ← ADDIECRAPEYE orchestration (Lone Wolf)
crates/trinity/src/health.rs            ← Real subsystem health checks
crates/trinity/src/trinity_api.rs       ← Unified /api/v1/trinity endpoint
crates/trinity/src/quests.rs            ← ADDIECRAPEYE quest handlers + PEARL API
crates/trinity/src/vaam.rs              ← VAAM alignment (message scanning)
crates/trinity/src/vaam_bridge.rs       ← VAAM → system prompt injection
crates/trinity/frontend/src/App.jsx     ← Main app (SubjectPicker + PEARL creation + OnboardingTour)
crates/trinity/frontend/src/components/PhaseWorkspace.jsx ← Center panel: objectives + chat + export buttons
crates/trinity/frontend/src/components/OnboardingTour.jsx ← 3-step tooltip onboarding
crates/trinity/frontend/src/components/TrainStatus.jsx    ← Coal/Steam/Velocity locomotive meters
crates/trinity/frontend/src/components/PearlCard.jsx      ← PEARL alignment display
crates/trinity/frontend/src/components/GameHUD.jsx        ← Right sidebar: PEARL + Train + Party + Bestiary
crates/trinity/frontend/src/components/ChapterRail.jsx    ← Left rail: 12-phase navigation
crates/trinity/frontend/src/components/ArtStudio.jsx      ← Creative tools + asset gallery
crates/trinity/frontend/src/components/Yardmaster.jsx     ← Agentic IDE with SSE + image rendering
crates/trinity/frontend/src/hooks/useQuest.js             ← Quest state polling (5s interval)
crates/trinity/frontend/src/hooks/usePearl.js             ← PEARL fetch + refine
crates/trinity/frontend/src/hooks/useSSE.js               ← Server-sent events listener
crates/trinity/frontend/src/hooks/useBestiary.js          ← Bestiary fetch
crates/trinity/frontend/src/hooks/useCreative.js          ← Creative sidecar status + generation
crates/trinity/frontend/src/hooks/useYardmaster.js        ← SSE streaming + image events
configs/runtime/default.toml            ← Inference backends, model paths, creative endpoints
templates/bevy_game/                    ← Bevy 0.15 game template (Purdue campus)
templates/first-game/                   ← Bevy WASM template (wasm-bindgen + web-sys)
archive/iron-road-physics/              ← Cognitive Load physics engine (Train, Node, coal/steam/velocity)
crates/archive/trinity-body/            ← 33K lines archived Bevy egui UI (for porting)
scripts/launch/demo_quick_start.sh      ← Trinity launcher script
_agent/workflows/                       ← Antigravity IDE workflows (9 workflows)
~/trinity-models/gguf/                  ← All GGUF models (Mistral, Qianfan-OCR, etc.)
llama.cpp/build-vulkan/bin/llama-server ← Vulkan-built inference server (working, has libmtmd.so)
~/Workflow/trinity-reports/              ← Yardmaster work logs for next-day EYE review
TASK_QUEUE.md                           ← File-based task queue for autonomous work
archive/                                ← 150K+ LOC of previous iterations
```
