# AGENTS.md — Trinity ID AI OS

> **Read this first.** This file is the entry point for any AI agent working on Trinity.

## What Trinity Is

Trinity is an **AI instructional designer for spatial computing**. A teacher chats in plain language → Trinity generates a complete VR lesson (3D models, narration, quizzes, scenes) through an agent loop with tool calling. The chat IS the portal. The agent IS the instructional designer.

**Target hardware:** XREAL Aura (Android XR, Fall 2026)
**Server:** AMD Strix Halo, 128GB unified memory
**Stack:** Rust/Axum + Bevy 0.18 + OpenXR + ComfyUI + LM Studio + vLLM Omni

## Read These Docs (In Order)

1. **`docs/active/MASTER_ARCHITECTURE.md`** — Master doc. Project ecosystem, full 7-stage workflow, orchestration decision, shared protocols, maturity model, prioritized todo. **Start here.**
2. **`docs/active/SPATIAL_PIVOT_PLAN.md`** — Detailed plan (21 sections). XR UI, XREAL Aura specs, gap analysis, monetization, two-audience framework, maturity levels.
3. **`context.md`** — Current system state, what's working, port map, launch commands.
4. **`docs/active/MASTER_TASK_LIST.md`** — Architecture audit, task tracking, completion criteria.
5. **`docs/active/ADDIECRAPEYE_CANONICAL.md`** — 12-phase instructional design framework reference.

## Do Not Read (Unless Specifically Needed)

- `docs/active/MASTER_PIVOT_DOCUMENT.md` — 108KB, mostly historical. Superseded by MASTER_ARCHITECTURE.md.
- `docs/active/MASTER_WORKFLOW.md` — Pipeline status from July 5. Superseded by MASTER_ARCHITECTURE.md Section 2.
- `docs/active/SESSION-HANDOFF.md` — Session state from prior work sessions. May be stale.
- `docs/archive/` — Historical docs. Do not reference for current work.

## The Current Focus: P0 Sprint

**We are at Level 0. The goal is Level 1: a teacher creates a lesson through the PWA.**

The P0 sprint sequence (from MASTER_ARCHITECTURE.md Section 6):

- **Sprint 0: PWA as the Face** — manifest, service worker, SME interview mode, teacher quick actions, onboarding, lesson display, mode switching (phone/VR)
- **Sprint 1: LM Studio integration** — Switch inference router to LM Studio :1234 (Hermes 4 70B). Replace hotel_manager.
- **Sprint 2: ID persona + tools** — ID system prompt, generate_image (ComfyUI), generate_voice (ComfyUI), generate_3d_model (ComfyUI TRELLIS), review_content_safety (Hermes)
- **End-to-end test** — Real teacher creates lesson through PWA

**Current brain:** Hermes 4 70B in LM Studio on :1234. vLLM is OFF. ComfyUI handles all creative.

**Do not work on P1 or P2 items until P0 is done.** Everything else is speculation until a real teacher creates a real lesson through the PWA.

## Architecture Rules

### Orchestration: Trinity is the Brain

- **Trinity** (:3000) = Orchestrator (agent loop, tools, memory, RAG, persistence, EYE)
- **LM Studio** (:1234) = Brain (LLM inference, model management)
- **vLLM Omni** (:8000) = Creative general (images, voice — inline in chat)
- **ComfyUI** (:8188) = Creative specialist (3D, video, music)
- **Blender** = 3D refinement (headless Python API)
- **trinity-xr** = VR client (Bevy 0.18 + bevy_oxr, WebSocket to Trinity)

Trinity orchestrates. Services are tools the agent calls. Do not make LM Studio or ComfyUI the orchestrator.

### Code Conventions

- **Rust** — Axum server, agent loop, tools. Follow existing patterns in `agent.rs`, `tools.rs`.
- **Agent tools** — Defined in `tools.rs` as `ToolInfo` structs. Tool execution in `tools/` directory. Each tool is an async function returning `Result<String, String>`.
- **System prompts** — Constants at top of `agent.rs`. The `AGENT_SYSTEM` const is the Yardmaster prompt. ID persona will be a new const or mode.
- **Inference** — All LLM calls go through `inference_router.rs`. Use `router.active_url()`, never hardcode URLs.
- **Creative pipeline** — `creative.rs` coordinates ComfyUI calls. New tools should follow the `tool_generate_image` pattern.
- **Persistence** — SQLite via `sqlx`. Migrations in `migrations/`. Follow existing patterns in `persistence.rs`.
- **Frontend** — PWA is `phone.html` served by Trinity. React app in `frontend/` is secondary.

### What Not to Touch

- `agent.rs` agent loop mechanics (tool calling, SSE streaming, memory injection) — **works, don't break it**
- `inference.rs` content extraction fix — **works, don't break it**
- `inference_router.rs` multi-backend routing — **works, extend don't rewrite**
- `rag.rs` + `ort_embed.rs` — **works, don't break it**
- `persistence.rs` + `memory_store.rs` — **works, don't break it**
- `auth.rs` — **works, don't break it**
- `quests.rs` + `conductor_leader.rs` — **works, don't break it**

### What to Delete (After P0 Proven)

- `hotel_manager.rs` (383 lines) — replaced by LM Studio API
- `vllm_fleet.rs` (12K lines) — replaced by LM Studio daemon + vLLM Omni single instance
- `sidecar_monitor.rs` — LM Studio self-monitors

**Do not delete until the replacement is proven working.**

## Drift Prevention

### Before Starting Work

1. Read `context.md` Section 7 ("What Is Working") — know what not to break
2. Check `docs/active/MASTER_ARCHITECTURE.md` Section 6 — know which P0 item you're on
3. Read the specific file you're modifying before editing

### During Work

1. **One P0 item at a time.** Don't jump between items.
2. **Test after each change.** `cargo check -p trinity` after every code edit.
3. **No new files unless needed.** Extend existing files. New tools go in `tools/` directory.
4. **No new docs unless needed.** Update existing docs. The master docs are MASTER_ARCHITECTURE.md and SPATIAL_PIVOT_PLAN.md.
5. **No scope creep.** If you find a P1 or P2 issue, note it in a comment but don't fix it now.

### After Work

1. Run `cargo check -p trinity` — must pass
2. Update `context.md` Section 7 if something new is working
3. Update `docs/active/MASTER_TASK_LIST.md` if a task is completed
4. Commit with a clear message referencing the P0 item number

## Key File Map

| File | Lines | Purpose |
|------|-------|---------|
| `crates/trinity/src/agent.rs` | 2,347 | Agent loop, system prompts, tool calling |
| `crates/trinity/src/tools.rs` | 2,478 | 34 tools, tool gauge, tool definitions |
| `crates/trinity/src/inference.rs` | — | LLM client, tool-aware chat completion |
| `crates/trinity/src/inference_router.rs` | 868 | Multi-backend router, P-ART-Y roles |
| `crates/trinity/src/creative.rs` | 1,485 | ComfyUI pipeline coordinator |
| `crates/trinity/src/main.rs` | 5,254 | API server, routes, handlers |
| `crates/trinity/src/persistence.rs` | — | SQLite persistence |
| `crates/trinity/src/rag.rs` | — | RAG semantic search |
| `crates/trinity/src/conductor_leader.rs` | — | 12-phase Socratic prompts |
| `crates/trinity/src/handlers/inference.rs` | 581 | Inference API handlers |
| `crates/trinity/frontend/` | — | PWA + React frontend |

## Related Projects

| Project | Path | Relationship |
|---------|------|-------------|
| Semantic Slime | `/home/joshua/Semantic Slime` | Reference implementation — Bevy 0.18 + FACES + VAAM |
| Bertrand XR | `/home/joshua/Workflow/Bertrand-Masterclass/apps/spatial-engine-bevy` | XR engine to port as `trinity-xr` |
| Bertrand companion-app | `/home/joshua/Workflow/Bertrand-Masterclass/apps/companion-app` | Mentorship monetization model (Stripe, 5 tiers) |
| Day_Dream | `/home/joshua/Workflow/Day_Dream` | VAAM origin, somatic sandbox |
