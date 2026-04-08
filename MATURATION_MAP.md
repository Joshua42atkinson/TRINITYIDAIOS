# Trinity AI OS — Maturation Map

> **Last Updated**: April 7, 2026
> **Purpose**: Single source of truth. What works, what doesn't, what "finished" means.
> **Use this document** to see where Trinity is _right now_ and how far each system is from done.
>
> **Architecture**: **Dual Brain Sidecar Model (128GB Strix Halo APU)**. \n> 1. **LongCat-Next (SGLang)** runs on Port 8010. It is the core Omni-Brain. It handles **Pete (Exhale)**, the **Great Recycler (Inhale)**, and **Tempo (Acestep 1.5)** for all chat, LitRPG world-building, inline images, and TTS natively.\n> 2. **vLLM (+ Hotel)** runs on Port 8000. It acts as the mechanical engine, handling **Yardmaster (RUST REAP)** coding, Nomic Embeddings (Acestep 1.5 persistence), and hotloaded P.A.R.T.Y Hotel **Aesthetics** (Flux Schnell, CogVideoX, Tripo).

---

## How to Read This

Each subsystem has a **maturity level** (L0–L5) and a **"Finished" definition**:

| Level | Meaning |
|-------|---------|
| **L0** | Not started — design exists in Bible only |
| **L1** | Stubbed — structs/routes exist, no real logic |
| **L2** | Wired — code works in isolation (unit tests pass) |
| **L3** | Integrated — works end-to-end when you run the server |
| **L4** | Polished — error handling, persistence, restart-safe |
| **L5** | Shippable — documented, tested, demo-ready |

---

## 🟢 What You Can Use RIGHT NOW

These are L3+ and work today when you `cargo run` + launch vLLM:

| System | Level | What Works | Launch |
|--------|-------|-----------|--------|
| **Web Chat UI** | L3 | Send messages, get streaming AI responses at `localhost:3000` | `cargo run -p trinity` |
| **Agent Loop (Yardmaster)** | L3 | 65-turn multi-tool agentic chat with file ops, shell, cargo_check, scouts | POST `/api/chat/yardmaster` |
| **38 Agentic Tools** | L3 | File read/write, shell exec, cargo check, image gen, RAG search, task queue | via agent loop |
| **Tool Gauge System** | L3 | Narrow/Standard/Broad filtering controls which tools are exposed | automatic per mode |
| **Inference Router** | L4 | Auto-detect vLLM, health probe, failover, hot-switch between backends | automatic |
| **Health Dashboard** | L4 | Honest subsystem checks — LLM, DB, Voice, CowCatcher, uptime | GET `/api/health` |
| **SQLite Persistence** | L3 | Sessions, messages, tool calls, projects saved to `.trinity/trinity_memory.db` | automatic |
| **RAG Search** | L3 | vLLM Nomic-embed (768-dim) semantic search + full-text fallback | automatic in agent loop |
| **VAAM Vocabulary** | L3 | Every message scanned, Coal awarded for domain terms, vocab profile tracks growth | automatic |
| **Scope Creep Detection** | L3 | PEARL-aligned semantic check fires in agent loop, emits SSE events | automatic |
| **Character Sheet** | L3 | Loads/saves player profile to `~/.local/share/trinity/character_sheet.json` | automatic |
| **CowCatcher** | L3 | Runtime error classification, hardware monitoring, auto-restart threshold | automatic |
| **Daydream Native UI** | L2 | Bevy holographic book renders, chat input works, but no media or SSE event display | `cargo run --bin daydream` |

---

## 🟡 What's Built But Dormant

| System | Level | What's Missing | "Finished" |
|--------|-------|---------------|-----------|
| **Kokoro TTS Sidecar** | Deprecated | Overridden by LongCat | Legacy python logic targeting port 8200 preserved in archive, now handled via Pete's **Acestep 1.5** pipeline on LongCat `longcat_audiogen_start`. |
| **Image Gen (Aesthetics)** | L2 | Wired to Port 8000 | The python proxy intercepts and maps Aesthetic image generation requests to the vLLM Hotel. |
| **Legacy Fragmented Agents** | Deprecated | Replaced by Dual Brain | Single LongCat Omni-Brain dynamically handles Pete (Exhale) and Recycler (Inhale) in unified shared cache. Aesthetics bound to vLLM. |

---

## 🟠 What's Half-Built (code exists, not connected)

| System | Level | The Gap | "Finished" |
|--------|-------|---------|-----------
| **Conductor Orchestration** | L3 | `phase_system_prompt()` is now called by `agent.rs` on every Iron Road turn. The agent loop appends the phase's Socratic coaching to the system prompt. `orchestrate_quest` route also wired for `/api/quest/execute`. | ✅ Wired — agent responses now vary by ADDIECRAPEYE phase. |
| **BookUpdate Channel** | L1 | Broadcast channel created in `main.rs`. Conductor's `run()` listens for updates. **Nothing ever sends them.** | After each agent response, a `BookUpdate` is published. Conductor tracks progression. Book of the Bible grows automatically. |
| **Media in Chat** | L3 | Markdown image tags from tool results now render as `<img>` in all three chat UIs (Yardmaster, PhaseWorkspace, ArtStudio). Image tokens extracted before HTML-escaping to preserve URLs. | ✅ Wired — `![alt](url)` from `tool_generate_image` renders inline in all chat views. |
| **Hotel Management** | L1 | `manage_hotel_sidecars()` logs "Lone Wolf: no swap". | When Conductor says "Gear A: Aesthetics", the router switches to the appropriate model/persona. |
| **Background Job Runner** | L2 | `jobs.rs` (541 lines) — SQLite-persisted task queue exists. No scheduler runs them. | Headless agent enqueues tasks, a background worker picks them up and executes autonomously. |
| **Daydream SSE Rendering** | L1 | Backend emits `vaam`, `cognitive_load`, `creep_tameable`, `shadow_status` events. Daydream has no panels for them. | Native Bevy UI shows VAAM progress, scope creep modals, cognitive load meter, shadow status in the HUD. |
| **Creative Pipeline** | L3 | `creative.rs` routes to vLLM Omni `:8000/v1/images/generations`. `tool_generate_image` in `tools.rs` returns `![img](url)` markdown for inline rendering. | ✅ Wired — `generate_image` tool uses vLLM Omni natively. ComfyUI references removed from agent prompts. |

---

## 🔴 What's Designed But Not Started

| System | Level | Reference | "Finished" |
|--------|-------|----------|-----------|
| **Multi-Player Profiles** | L0 | Bible Feature Table | Each player profile gets its own directory (`~/.local/share/trinity/players/{id}/`). Character sheet, bestiary, VAAM profile, game state all isolated. `POST /api/player/switch` hot-swaps identity without restart. |
| **Per-Project Sidecars** | L0 | `context.md:117` | Each Iron Road project gets its own SQLite database, narrative ledger, and workspace directory. Switching projects preserves full conversation history and quest state. |
| **EYE Container Export** | L1 | `eye_container.rs` (240 lines) + `export.rs` (597 lines) | User completes the 12-station Iron Road. Clicks "Export". Gets a standalone HTML5 learning package with SCORM/xAPI metadata. Playable in any LMS. |
| **PyO3 Python Bridge** | L0 | Bible | Bevy Daydream embeds Python via PyO3. Students write game logic in Python, Bevy renders it live. Safe sandbox. |
| **MCP Server Integration** | L1 | `trinity-mcp-server` crate | Trinity exposes tools via MCP protocol. External editors (VS Code, Cursor) can use Trinity as an AI backend. |

---

## Priority Stack (What to Do Next)

### Tier 1: Dual Brain Architecture (Current Focus)
1. ✅ **Proxy Scaffold Complete** — Python API proxy built `scripts/launch/start_longcat_server.py`.
2. ✅ **SGLang Launcher** — Built `start_sglang_omni.sh` with `bitsandbytes` APU compression.
3. 🔄 **Dual Brain Integration** — Separating Omni tasks to SGLang (:8010) and Embeddings/REAP/Hotel (Flux/CogVideoX) to vLLM (:8000).
4. 🔄 **Acestep 1.5 Persistence** — Accordion architecture. Per-project/player sidecar SQLite databases driven by vLLM embeddings.

### Tier 2: Multi-player persistence
5. ✅ **EYE Container Export** — DONE. `export.rs` generates real HTML5 quizzes and bundles.
6. ✅ **Background Job Runner** — DONE. `jobs.rs` fully wired.
7. ☐ **VAAM vocabulary sources** — words in the database mostly lack definitions.
8. ☐ **Profile switcher API** — `POST /api/player/switch` — per-player profiles not yet isolated

### Tier 3: Omni-Modal UI/UX Refinement
9. ☐ **Native Voice Chat** — Enable full-duplex longcat voice pipeline in React UI.
10. ☐ **Audio-to-Video Workflow** — Test generation of interactive media using LongCat.

---

## Full Capability Inventory

### API Routes (94 registered in main.rs)

**Wired** = handler has real logic. **Stub** = returns placeholder. **Dormant** = code complete but needs sidecar.

#### Chat & Inference (6 routes — all wired ✅)
| Route | Method | Notes |
|-------|--------|-------|
| `/api/chat` | POST | Simple non-streaming chat |
| `/api/chat/stream` | POST | SSE streaming chat (Iron Road) |
| `/api/chat/yardmaster` | POST | 65-turn agent loop with tools |
| `/api/chat/zen` | POST | Narrative mode (routes to :8081 if available) |
| `/api/chat/portfolio` | POST | Portfolio assistant |
| `/api/v1/trinity` | POST | Unified Trinity API endpoint |

#### Quest & Game State (10 routes — all wired ✅)
| Route | Method | Notes |
|-------|--------|-------|
| `/api/quest` | GET | Current game state |
| `/api/quest/complete` | POST | Complete an objective |
| `/api/quest/advance` | POST | Advance ADDIECRAPEYE phase |
| `/api/quest/party` | POST | Toggle party member |
| `/api/quest/subject` | POST | Set quest subject |
| `/api/quest/tame_creep` | POST | Tame a scope creep |
| `/api/quest/cast_spell` | POST | Cast Hook Card spell |
| `/api/quest/economy` | POST | Update Coal/Steam/XP |
| `/api/quest/execute` | POST | Execute orchestration |
| `/api/quest/circuitry` | GET | Sacred Circuitry state |

#### Identity, Persistence & System (20+ routes — all wired ✅)
Sessions, character sheet, projects, health, hardware, tools, telemetry, journal, RLHF, analytics — all functional.

#### Voice & TTS (6 routes — ✅ Transitioning to Acestep 1.5 on :8010)
`/api/voice/*`, `/api/tts`, `/api/telephone` — Transitioning from Kokoro legacy to native LongCat audio decoding. PhaseWorkspace voice toggle wires to Pete's native audio settings.

#### Creative (8 routes — ✅ rerouted to vLLM Omni)
`/api/creative/*` — `creative.rs` routes to vLLM `:8000/v1/images/generations`. ComfyUI dependency purged from agent prompts and tool descriptions.

#### EYE Export (3 routes — ✅ fully wired)
`/api/eye/compile`, `/api/eye/preview`, `/api/eye/export` — generate real HTML5 quiz, adventure, DOCX portfolio, and ZIP bundle. VAAM vocabulary included in glossary.

---

### Agentic Tools (38 registered in tools.rs)

| # | Tool | Category | Status | What It Does |
|---|------|----------|--------|-------------|
| 1 | `read_file` | File Ops | ✅ | Read file contents |
| 2 | `write_file` | File Ops | ✅ | Write/overwrite with auto-backup |
| 3 | `list_dir` | File Ops | ✅ | List directory contents |
| 4 | `search_files` | File Ops | ✅ | Grep search across codebase |
| 5 | `shell` | Execution | ✅ | Shell command (Broad gauge) |
| 6 | `python_exec` | Execution | ✅ | Python code execution |
| 7 | `cargo_check` | Build | ✅ | Rust compilation verification |
| 8 | `quest_status` | Game | ✅ | ADDIECRAPEYE phase/XP/Coal |
| 9 | `quest_advance` | Game | ✅ | Advance/retreat quest phase |
| 10 | `work_log` | Session | ✅ | Write session work report |
| 11 | `task_queue` | Session | ✅ | Read/add/complete/next tasks |
| 12 | `save_session_summary` | Session | ✅ | Cross-session continuity |
| 13 | `load_session_context` | Session | ✅ | Bootstrap from last session |
| 14 | `cowcatcher_log` | System | ✅ | View error/obstacle logs |
| 15 | `sidecar_status` | System | ✅ | Check AI sidecar health |
| 16 | `process_list` | System | ✅ | Running processes (ps aux) |
| 17 | `system_info` | System | ✅ | Memory, disk, GPU, services |
| 18 | `zombie_check` | System | ✅ | Find/kill zombie processes |
| 19 | `scout_sniper` | Planning | ✅ | Generate ADDIECRAPEYE quest chain |
| 20 | `update_vibe` | Creative | ✅ | Set visual/music/narrator mood |
| 21 | `generate_lesson_plan` | Pedagogy | ✅ | Lesson plan from topic+grade |
| 22 | `generate_rubric` | Pedagogy | ✅ | Grading rubric generator |
| 23 | `generate_quiz` | Pedagogy | ✅ | Quiz/assessment generator |
| 24 | `curriculum_map` | Pedagogy | ✅ | Multi-week curriculum mapper |
| 25 | `analyze_document` | Vision | 🟡 | OCR via vision sub-agent (:8081) |
| 26 | `analyze_image` | Vision | ✅ | Image analysis via primary LLM |
| 27 | `analyze_screen_obs` | Vision | ✅ | Live desktop screenshot + AI analysis |
| 28 | `generate_image` | Creative | ✅ | Routes to vLLM Omni `:8000/v1/images/generations`. Returns `![img](url)` markdown for inline chat rendering. |
| 29 | `generate_music` | Creative | 🟡 | Future vLLM audio model |
| 30 | `generate_video` | Creative | 🟡 | Future vLLM video model |
| 31 | `generate_mesh3d` | Creative | 🟡 | Hunyuan3D-2.1 (needs :7860) |
| 32 | `blender_render` | Creative | 🟡 | Blender CLI render |
| 33 | `avatar_pipeline` | Creative | 🟡 | NPC avatar creation pipeline |
| 34 | `scaffold_bevy_game` | Scaffold | ✅ | Create Bevy game project |
| 35 | `scaffold_elearning_module` | Scaffold | ✅ | Create Vite+React e-learning project |
| 36 | `sidecar_start` | System | ✅ | Start model sidecar |
| 37 | `daydream_command` | Daydream | ✅ | Send command to Bevy engine |
| 38 | `project_archive` | File Ops | ✅ | Archive project to DAYDREAM |

**Summary**: 31/38 tools fully wired. 7 need future media models/sidecars (video, audio, mesh, blender, avatar, analyze_document).

---

### Rust Modules (39 files, 24,032 lines in trinity-server)

The big six (85% of code):

| Module | Lines | Purpose | Status |
|--------|-------|---------|--------|
| `main.rs` | 4,504 | HTTP server, 94 routes, AppState, SSE | ✅ |
| `tools.rs` | 3,106 | 38 tool implementations + gauge system | ✅ |
| `agent.rs` | 2,284 | Multi-turn agentic chat loop (65 turns) | ✅ |
| `creative.rs` | 1,296 | Image/audio/video generation | ✅ Rerouted to vLLM Omni |
| `conductor_leader.rs` | 1,204 | 12-phase ADDIECRAPEYE orchestrator + QM rubric | ✅ Wired via `phase_system_prompt()` |
| `quests.rs` | 915 | Quest state machine + Iron Road game | ✅ |

Supporting modules:

| Module | Lines | Purpose | Status |
|--------|-------|---------|--------|
| `voice.rs` | 822 | Audio Generation + voice pipelines | ✅ Acestep 1.5 live on :8010 |
| `inference_router.rs` | 721 | Multi-backend auto-detect + failover | ✅ |
| `persistence.rs` | 704 | SQLite sessions/messages/projects | ✅ |
| `export.rs` | 597 | EYE export container | ✅ HTML5 quiz/adventure/DOCX/ZIP |
| `quality_scorecard.rs` | 582 | QM rubric evaluation | ✅ |
| `perspective.rs` | 526 | Perspective API | ✅ |
| `jobs.rs` | 541 | Background job queue | ✅ Fully wired, tokio::spawn per job |
| `journal.rs` | 475 | Session journal persistence | ✅ |
| `inference.rs` | 468 | OpenAI-compatible HTTP client | ✅ |
| `vaam.rs` | 465 | VAAM core vocabulary analysis | ✅ |
| `edge_guard.rs` | 455 | Security/safety filtering | ✅ |
| `rag.rs` | 444 | vLLM semantic search + text fallback | ✅ |
| `vaam_bridge.rs` | 438 | VAAM integration bridge | ✅ |
| `telephone.rs` | 432 | WebSocket audio-to-audio | 🟡 Needs sidecar |
| `narrative.rs` | 398 | Story/chapter generation | ✅ |
| `cow_catcher.rs` | 345 | Error classification + auto-repair | ✅ |
| `skills.rs` | 297 | Skill tree system | ✅ |
| `trinity_api.rs` | 254 | Unified API endpoint | ✅ |
| `eye_container.rs` | 241 | EYE package structure | ✅ VAAM vocab wired in |
| `character_sheet.rs` | 208 | Character persistence | ✅ |
| `health.rs` | 201 | Honest subsystem health check | ✅ |
| `authenticity_scorecard.rs` | 172 | Yardmaster scoring | ✅ |
| `rlhf_api.rs` | 150 | RLHF feedback API | ✅ |
| `music_streamer.rs` | 123 | Audio streaming | 🟡 |
| `character_api.rs` | 121 | Character REST API | ✅ |
| `http.rs` | 115 | Shared HTTP clients (QUICK, LONG) | ✅ |
| `sidecar_monitor.rs` | 104 | Sidecar health polling | ✅ |
| `scope_creep.rs` | 100 | PEARL-aligned scope check | ✅ |
| `vllm_fleet.rs` | 93 | vLLM multi-instance management | ✅ |
| `beast_logger.rs` | 63 | Creep bestiary logging | ✅ |
| `rlhf_ui.rs` | 30 | RLHF UI helpers | ✅ |
| `voice_loop.rs` | 27 | Voice loop delegation | 🟡 |
| `lib.rs` | 13 | Crate root | ✅ |

---

### Wiring Gap Summary

| Category | Total | ✅ Wired | 🟡 Dormant | 🟠 Disconnected |
|----------|-------|---------|-----------|-----------------|
| API Routes | 94 | 90 | 4 | 0 |
| Agentic Tools | 38 | 31 | 7 | 0 |
| Rust Modules | 39 | 36 | 3 | 0 |
| **Overall** | **171** | **157 (92%)** | **14 (8%)** | **0 (0%)** |

**Biggest remaining leverage points**:
1. `telephone.rs` + `voice_loop.rs` — Need refactoring away from ASR sidecar to send/receive audio strings transparently via the LongCat Acestep 1.5 pipeline.
2. Creative media tools — `generate_music`, `generate_video`, `generate_mesh3d` need future vLLM audio/video models
3. VAAM word definitions — enrich vocabulary packs so EYE exports include a real glossary

**ComfyUI is deprecated.** vLLM Omni handles all media generation.

---

## 📚 The Living Code Textbook Standard
Every core Rust module in `crates/trinity/src/` features a standardized pedagogical header comprising:
- Purpose and Architectural function
- Hook Book connection for instructional design
- Cow Catcher telemetry status
- Formal Maturity Level mapping

## 🏆 Definition of Maturity ("L5 Shippable")
With the recovery and successful linking of our `trinity-mcp-server` integration, the 3 Core Deliverables (Book, Product, and HTML EYE Portfolio), and the complete standard header implementation, the core engine has achieved **L5 Shippable**. All new and existing modules track maturity via this matrix:

| Level | Name | Criteria |
|---|---|---|
| **L1** | Concept / Stubs | Idea exists in documentation or code stub. No implementation. |
| **L2** | Scaffolding | Basic HTTP routes and structs exist, mock data returned. |
| **L3** | Isolated Execution | Capable of executing tasks but disconnected from the gamified ADDIECRAPEYE lifecycle or persistence layer. |
| **L4** | Integrated Loop | Component runs, feeds data back to the gamified ecosystem, and appropriately impacts Steam/Coal/Resonance tracking. |
| **L5** | Shippable Textbook | Highly performant, fault-tolerant (Cow Catcher), fully integrated, and features the Top 30-Line Pedagogical Header. |

---

## Document Hierarchy

| Document | Purpose | When to Read |
|----------|---------|-------------|
| **MATURATION_MAP.md** (this file) | Where we ARE right now | Every session start |
| **TRINITY_FANCY_BIBLE.md** | Where we're GOING (vision) | When designing new features |
| **context.md** | AI session handoff notes | Start of each AI conversation |
| **docs/VLLM_LESSONS_LEARNED.md** | Hardware-specific gotchas | When touching vLLM config |
| **docs/HOOK_BOOK.md** | Capability catalogue for users | When building tutorials |

---

## Architecture Principles (non-negotiable)

1. **Dual Brain Engines.** SGLang (Port 8010) and vLLM (Port 8000) coexist. No ComfyUI, no raw llama-server for core tasks, though embedded llama.cpp is allowed for CPU fallback.
2. **Strix Halo is an APU (128GB).** GPU and NPU share compute. We maximize RAM by allocating specific model operations between the two sidecars.
3. **Don't delete code.** Move to archive if deprecated. Update labels and references.
4. **P.A.R.T.Y System via Dual Brain.** SGLang (Port 8010) runs **P**ete, the Great **R**ecycler, and **T**empo (Acestep 1.5). vLLM (Port 8000) runs **A**esthetics (Hotel models) and **Y**ardmaster (Qwen REAP).
5. **Ports:** Port 3000 is the web UI. Port 8010 is LongCat Omni. Port 8000 is vLLM (Embeddings, REAP, Hotel).
