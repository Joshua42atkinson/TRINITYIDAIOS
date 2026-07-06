# TRINITY — Master Architecture & Project Map

**Date:** July 6, 2026 | **Author:** Joshua Atkinson + Cascade

> This is the master document. It points to every project, every tool, every workflow, and shows how they connect. All other docs reference this one.

---

## 1. THE PROJECT ECOSYSTEM

Trinity is not one codebase. It's a constellation of projects that share common protocols, pedagogical frameworks, and a unified vision: **AI-native spatial education**.

### Active Projects

| Project | Path | Role | Status |
|---------|------|------|--------|
| **TRINITYIDAIOS** | `/home/joshua/Workflow/TRINITYIDAIOS` | Core platform — agent, tools, API server, memory, RAG | Working, 78 commits, Mar 2026–Jul 2026 |
| **Semantic Slime** | `/home/joshua/Semantic Slime` | Reference implementation — Bevy 0.18 educational game with FACES + VAAM | Working, Bevy 0.18.1, WASM build |
| **Bertrand spatial-engine-bevy** | `/home/joshua/Workflow/Bertrand-Masterclass/apps/spatial-engine-bevy` | XR engine — Bevy 0.18 OpenXR, spatial UI, hand tracking | Working, targets XREAL Aura |
| **Bertrand companion-app** | `/home/joshua/Workflow/Bertrand-Masterclass/apps/companion-app` | Mentorship monetization model — Stripe, Supabase, 5 tiers | Working, production-ready |
| **Day_Dream** | `/home/joshua/Workflow/Day_Dream` | VAAM origin — branching somatic sandbox, Bevy 0.18 | 93 commits, Nov 2025–May 2026 |
| **phonethagoras** | `/home/joshua/Workflow/phonethagoras` | Math education research, Daydream arcana system | Research/docs |

### Archived (for reference)

| Project | Path | What It Was |
|---------|------|-------------|
| **desktop_trinity** | `ARCHIVE_VAULT/desktop_trinity` | Original Trinity (Dec 2025) — VaaM engine, Iron Road physics, 5-zone architecture |
| **trinity-ndk** | `ARCHIVE_VAULT/Phone/trinity-ndk` | Phone NDK app — Bevy, JNI, PEARL, quest system, VR_DESIGN.md |
| **TRINITY_UI** | `ARCHIVE_VAULT/TRINITY_UI` | Early UI crate experiment |
| **Daydream website** | `ARCHIVE_VAULT/Daydream-website-backup` | Marketing site backup |

### How They Connect

```
                    ┌─────────────────────────────────┐
                    │     SEMANTIC SLIME               │
                    │  (Reference Implementation)      │
                    │  Bevy 0.18 + FACES + VAAM        │
                    │  "This is what Trinity makes"    │
                    └────────────┬────────────────────┘
                                 │ proves the pattern
                                 ▼
┌──────────────┐    ┌─────────────────────────────────┐    ┌──────────────────┐
│   PHONE      │───▶│      TRINITYIDAIOS              │◀───│  BERTRARD XR     │
│   PWA        │    │  (Core Platform)                │    │  spatial-engine  │
│   chat +     │    │                                 │    │  Bevy 0.18       │
│   preview    │    │  Agent loop + Tools + Memory    │    │  OpenXR          │
│              │◀───│  RAG + SQLite + EYE             │───▶│  Hand tracking   │
└──────────────┘    │                                 │    │  Spatial UI      │
                    │  API :3000                      │    └──────────────────┘
                    └──────┬──────┬──────┬────────────┘
                           │      │      │
                    ┌──────▼──┐ ┌▼─────┐ ┌▼──────────┐
                    │LM Studio│ │vLLM  │ │ComfyUI    │
                    │(Brain)  │ │Omni  │ │(Specialist)│
                    │:1234    │ │:8000 │ │:8188      │
                    └─────────┘ └──────┘ └───────────┘
                                         │
                                  ┌──────▼──────┐
                                  │  Blender    │
                                  │  (3D refine)│
                                  │  Python API │
                                  └─────────────┘

                    ┌─────────────────────────────────┐
                    │     BERTRAND COMPANION APP       │
                    │  (Mentorship Monetization)       │
                    │  Stripe + Supabase + 5 tiers     │
                    │  "The payment model Trinity uses"│
                    └─────────────────────────────────┘

                    ┌─────────────────────────────────┐
                    │     DAY_DREAM                    │
                    │  (VAAM Origin)                   │
                    │  Somatic sandbox, PLING! gate     │
                    │  "The pedagogical philosophy"    │
                    └─────────────────────────────────┘
```

---

## 2. THE FULL WORKFLOW — MATURE STATE

This is the end-to-end pipeline when Trinity is mature:

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  1. INTENT CAPTURE                                                           │
│  User chats in PWA (phone or desktop)                                        │
│  → PWA sends POST /api/agent/chat to Trinity :3000                           │
│  → Trinity agent loop starts (ID persona, ADDIECRAPEYE framework)            │
│  → Agent asks Socratic questions (grade? duration? standards? interactivity?)│
└──────────────────────────┬───────────────────────────────────────────────────┘
                           ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  2. LESSON DESIGN (Trinity + LM Studio)                                      │
│  Trinity agent calls LM Studio :1234 for LLM inference                       │
│  → LM Studio runs the brain (tool calling, reasoning, Socratic dialogue)     │
│  → Agent builds lesson spec (objectives, assets list, assessment plan)       │
│  → Agent calls align_standards → RAG over NGSS/Common Core                   │
└──────────────────────────┬───────────────────────────────────────────────────┘
                           ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  3. ASSET GENERATION (Trinity + vLLM Omni + ComfyUI)                         │
│  Agent calls creative tools:                                                 │
│  → generate_image → vLLM Omni :8000 (fast, unified, inline in chat)         │
│  → generate_voice → vLLM Omni :8000 (TTS, narration)                        │
│  → generate_3d_model → ComfyUI :8188 (TRELLIS/TripoSR → glTF)               │
│  → generate_video → ComfyUI :8188 (HunyuanVideo, when needed)               │
│  → generate_music → ComfyUI :8188 (ACE-Step, when needed)                   │
│  → review_content_safety → LM Studio :1234 (LLM reviews for K-12)           │
└──────────────────────────┬───────────────────────────────────────────────────┘
                           ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  4. 3D REFINEMENT (Trinity + Blender)                                        │
│  If 3D models need cleanup:                                                  │
│  → Trinity calls Blender Python API (headless)                               │
│  → Blender optimizes meshes, applies materials, exports glTF                 │
│  → Returns refined asset path to Trinity                                     │
└──────────────────────────┬───────────────────────────────────────────────────┘
                           ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  5. SCENE ASSEMBLY (Trinity + Bevy OpenXR)                                   │
│  Agent calls assemble_scene:                                                 │
│  → Trinity packages assets into Bevy scene / Godot project / WebXR           │
│  → Scene spec → trinity-xr crate (Bevy 0.18 + bevy_oxr)                     │
│  → Scene pushed to VR via WS /api/xr/connect                                 │
│  → User puts on XREAL Aura → sees lesson at real scale                       │
│  → User grabs, rotates, refines in VR → voice commands back to Trinity       │
└──────────────────────────┬───────────────────────────────────────────────────┘
                           ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  6. EVALUATION (Trinity EYE)                                                 │
│  Agent calls EYE evaluation:                                                 │
│  → AI reviews the lesson for pedagogical quality                             │
│  → Checks: objective alignment, assessment validity, pacing, engagement      │
│  → Flags issues → agent fixes → re-evaluates                                 │
│  → EYE score stored in SQLite for self-improvement                           │
└──────────────────────────┬───────────────────────────────────────────────────┘
                           ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│  7. DEPLOYMENT (Trinity → LM Studio → Phone)                                 │
│  Agent calls export tools:                                                   │
│  → export_scorm → SCORM package for Google Classroom / Canvas                │
│  → export_godot → Godot 4 project for Android XR APK                         │
│  → export_webxr → WebXR build for Chromebook                                 │
│  → Lesson saved to SQLite (save_lesson)                                      │
│  → Results sent back to PWA via SSE stream                                   │
│  → PWA shows: "Your lesson is ready. Preview in VR?"                         │
│  → Optional: Push notification via LM Studio → phone                         │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. ORCHESTRATION ARCHITECTURE

### The Question: Who Orchestrates?

Three options were considered:

| Option | Orchestrator | Pros | Cons |
|--------|-------------|------|------|
| A: Trinity orchestrates | Trinity agent loop | Full control, memory, tools, RAG, persistence | Complex code, lots to maintain |
| B: LM Studio orchestrates | LM Studio MCP | Simple, native model management | No custom tools, no RAG, no persistence, no agent loop |
| C: Hybrid | Trinity brain + LM Studio models + vLLM Omni creative | Best of all worlds | Three services to manage |

### The Answer: C — Hybrid (Trinity as Brain, Services as Tools)

**Trinity remains the orchestrator.** The agent loop in `agent.rs` is irreplaceable — it has:
- Multi-turn conversation with tool calling
- VAAM (user style learning)
- FACES (emotive state)
- ADDIECRAPEYE framework (pedagogical phases)
- RAG (knowledge retrieval)
- SQLite persistence (lessons, memory, standards)
- EYE evaluation (self-assessment)
- SSE streaming (real-time feedback to PWA)

LM Studio can't do any of that. LM Studio is a **model manager and inference server** — that's what it's good at. Trinity uses it as a tool.

### Service Roles

**Current Phase (July 2026):** Hermes 4 70B in LM Studio is the brain. vLLM is off. ComfyUI handles all creative.

| Service | Port | Role | Status | What It Does |
|---------|------|------|--------|-------------|
| **Trinity** | :3000 | **Orchestrator** | Running | Agent loop, tools, memory, RAG, persistence, EYE, API server |
| **LM Studio** | :1234 | **Brain** | Running | Hermes 4 70B — LLM inference, tool calling, reasoning. Model load/unload/download |
| **vLLM Omni** | :8000 | **Creative (general)** | Off for now | Will replace ComfyUI for images/voice when activated |
| **ComfyUI** | :8188 | **Creative** | Available | All creative: images (Janus-Pro/LongCat), voice (VibeVoice), 3D (TRELLIS), video (HunyuanVideo), music (ACE-Step) |
| **Blender** | — | **3D Refinement** | Planned | Headless Python API — mesh cleanup, material application, glTF export |
| **trinity-xr** | — | **VR Client** | Planned | Bevy 0.18 + bevy_oxr, runs on XREAL Aura, WebSocket to Trinity :3000 |

### Creative Pipeline — Current and Future

**Current (vLLM off):** ComfyUI handles everything.
- Images: Janus-Pro-7B or LongCat via ComfyUI :8188
- Voice: VibeVoice-1.5B via ComfyUI :8188
- 3D: TRELLIS or TripoSR via ComfyUI :8188
- Video: HunyuanVideo via ComfyUI :8188
- Music: ACE-Step via ComfyUI :8188
- Upscale: RealESRGAN via ComfyUI :8188

**Future (when vLLM Omni is activated):** Split by task.
- Simple creative (images, voice) -> vLLM Omni (fast, inline, one model)
- Specialized creative (3D, video, music) -> ComfyUI (node pipelines, specialized models)
- vLLM Omni activation is a P1 task. Not needed for P0.

### Hermes 4 70B — Current Brain

- **Model:** Hermes 4 70B (based on Llama-3.1-70B, Nous Research)
- **Engine:** LM Studio on :1234
- **Quantization:** Q3_K_M (~43GB VRAM) or Q4_K_M (~53GB)
- **Context:** 131K — can ingest entire codebases
- **Hybrid reasoning:** thinking tags for complex planning, fast mode for simple tasks
- **Tool calling:** OpenAI-compatible function calling via LM Studio API
- **Schema adherence:** Trained for valid JSON output — perfect for structured job chains
- **Improved steerability:** Reduced refusal rates, better instruction following

### Sandbox / Cron System (Planned)

Hermes works autonomously in a sandbox. Work is evaluated before incorporating.

```
Cron Trigger → Hermes Agent → Sandbox → EYE Eval → Pass? Incorporate : Discard
```

- **Cron:** Schedule triggers (e.g., "build lesson X overnight", "evaluate lesson Y at 3am")
- **Hermes Agent:** Runs autonomously via Trinity job system (POST /api/jobs, max_turns=200)
- **Sandbox:** Work happens in isolated directory/branch. Nothing touches production until evaluated.
- **EYE:** AI evaluates the work. Pass → incorporate. Fail → discard and log.
- This lets Hermes build and test workflows autonomously while you sleep.

### What This Replaces

| Current Code | Replaced By | Lines Saved |
|-------------|-------------|-------------|
| `hotel_manager.rs` (383 lines) | LM Studio load/unload API | 383 |
| `vllm_fleet.rs` (12K lines) | LM Studio daemon + vLLM Omni single instance | ~12,000 |
| `tools.rs` + `tools/` (92K lines) | LM Studio MCP (for model mgmt) + Trinity tools (for ID-specific) | ~60,000 (keep ID tools, delete model mgmt) |
| `sidecar_monitor.rs` | LM Studio self-monitors | ~500 |
| ComfyUI image gen calls | vLLM Omni image gen (for simple images) | Simplification, not deletion |

**Keep ComfyUI for:** TRELLIS (3D), HunyuanVideo (video), ACE-Step (music), RealESRGAN (upscale)

### Current inference_router.rs Already Supports This

The `inference_router.rs` (868 lines) already has:
- `BackendKind::VllmOmni` — vLLM Omni as a backend type
- `BackendKind::LmStudio` — LM Studio as a backend type
- `PartyRole` enum — P/A/R/T/Y role routing
- `switch_to_role()` — gear shift between roles
- `auto_detect()` — probes known ports on startup
- Config-driven (`configs/runtime/default.toml`)
- Health monitoring with failover

**What needs to change:**
1. Make LM Studio the primary brain (default, already is in config)
2. Make vLLM Omni the primary creative (for images/voice)
3. Keep ComfyUI as a creative tool (called by agent, not by inference router)
4. Add Blender as a tool (Python API, headless)
5. Add trinity-xr as a client (WebSocket, not inference)

---

## 4. SHARED PROTOCOLS & FRAMEWORKS

These are the connective tissue between all projects:

### FACES Protocol (4-byte emotive signaling)
- **Crate:** `trinity-faces` (in TRINITYIDAIOS)
- **Used by:** Semantic Slime (pet emotive states), Trinity (agent emotive state)
- **Spec:** Aura (256 values) + Container (5) + Focus (6) + Action (5)
- **Status:** 105 tests passing, zero dependencies, NPU target
- **Future:** FACES-Embed (neural classification on NPU)

### VAAM (Vocabulary Acquisition Autonomous Meaning)
- **Origin:** Day_Dream (Nov 2025)
- **Used by:** Semantic Slime (word → pet stats), Trinity (user style tracking), Daydream (somatic sandbox)
- **Mapping:** Vocabulary→Action, Autonomy→Container, Acquisition→Focus, Mastery→Aura
- **Status:** Working in Semantic Slime, tracking in Trinity

### ADDIECRAPEYE (12-phase instructional design framework)
- **Used by:** Trinity (agent phases), Bertrand (curriculum design)
- **Phases:** Analysis → Design → Development → Implementation → Evaluation → Contrast → Repetition → Alignment → Proximity → Envision → Yoke → Evolve
- **Status:** Working in Trinity's `conductor_leader.rs`

### Iron Road (Resource economy)
- **Origin:** desktop_trinity (Dec 2025)
- **Used by:** Trinity (Coal/Steam/XP tracking)
- **Concept:** Mass=Intrinsic Load, Friction=Extraneous Load, Steam=Germane Load
- **Status:** Working in Trinity's `narrative.rs` / `quests.rs`

### PLING! (Somatic gate)
- **Origin:** Day_Dream (pitch matching to unlock choices)
- **Used by:** Bertrand (guitar education), Daydream (somatic sandbox)
- **Concept:** Vocalize a frequency to ground the nervous system before making a choice
- **Status:** Working in Daydream and Bertrand

### Character Sheet (Progression system)
- **Used by:** Semantic Slime (pet mastery), Trinity (user progression), Daydream (student profile)
- **Fields:** VAAM attunement scores, mastery tiers (Hero → Outlaw → EdgeLord → BestSelf)
- **Status:** Working across all three projects

---

## 5. MATURITY — WHAT "DONE" LOOKS LIKE (UPDATED)

### Level 0: Where We Are (July 6, 2026)
- ✅ Trinity agent loop with tool calling (78 commits)
- ✅ ComfyUI creative pipeline (images, voice, 3D, video, music — all via :8188)
- ✅ PWA chat interface (phone.html, 1,429 lines — works but dev-focused)
- ✅ Hermes 4 70B loaded in LM Studio on :1234 (current brain)
- ✅ Semantic Slime — working Bevy 0.18 educational game (reference implementation)
- ✅ Bertrand spatial-engine-bevy — working Bevy 0.18 OpenXR engine
- ✅ FACES protocol (105 tests, zero-dep)
- ✅ VAAM tracking
- ✅ Iron Road economy
- ✅ Bertrand mentorship monetization model (Stripe, 5 tiers, production-ready)
- ✅ Daydream — VAAM origin, somatic sandbox (93 commits)
- ⚠️ vLLM Omni OFF for now — will activate for images/voice in P1
- ❌ PWA not installable (no manifest, no service worker)
- ❌ PWA has no SME interview mode, no onboarding, no lesson display
- ❌ No ID persona in agent
- ❌ No 3D/voice/video agent tools wired to ComfyUI
- ❌ No VR integration (trinity-xr not ported)
- ❌ No standards DB
- ❌ No SCORM export
- ❌ No Blender integration
- ❌ No EYE → agent self-improvement loop
- ❌ No sandbox/cron system for autonomous Hermes work

### Level 1: MVP — Teacher Creates Lesson Through Chat
- **PWA as the Face:** manifest, service worker, SME interview mode, teacher quick actions, onboarding, lesson display (Sprint 0)
- **LM Studio as brain:** Switch inference router to LM Studio :1234, Hermes 4 70B (Sprint 1)
- **ID system prompt** in agent.rs — Socratic, ADDIECRAPEYE (Sprint 2)
- **`generate_image`** → ComfyUI Janus-Pro/LongCat (Sprint 2)
- **`generate_voice`** → ComfyUI VibeVoice (Sprint 2)
- **`generate_3d_model`** → ComfyUI TRELLIS (Sprint 2)
- **`review_content_safety`** → Hermes LLM review via LM Studio (Sprint 2)
- **Test:** Real teacher creates one lesson through the PWA. It works end-to-end.

### Level 2: Spatial Workspace — Teacher Previews in VR
- `trinity-xr` crate ported from Bertrand
- VR chat panel (WebSocket to :3000)
- VR asset viewer (3D model preview, grab, rotate)
- `assemble_scene` tool (Bevy scene / Godot project / WebXR)
- Blender integration (headless Python API for 3D refinement)
- **Test:** Teacher creates lesson on phone, puts on VR, previews it, says "make it bigger," Trinity regenerates.

### Level 3: Sellable Product — Non-Technical Teacher Deploys
- Standards alignment (NGSS/Common Core DB + `align_standards` tool)
- SCORM export (`export_scorm` tool)
- Lesson save/load/resume (`save_lesson`, `list_lessons`)
- PWA onboarding (examples, hints, progressive disclosure)
- Android XR APK build (`cargo apk build --features xr`)
- Mentorship review system (Stripe + review queue, adapted from Bertrand)
- Applied to Android XR Developer Catalyst Program
- **Test:** Non-technical teacher creates VR lesson from scratch, deploys to Google Classroom, Joshua reviews via mentorship system.

### Level 4: Spatial OS — AI Self-Improves, Community Grows
- EYE evaluation → agent self-improvement loop (lessons get better over time)
- VAAM profiles personalize agent behavior per teacher
- Community lesson library (teachers share and remix)
- Multi-engine export (Bevy, Godot, WebXR, SCORM, Android XR)
- XREAL Aura launch-title ID tool
- Mentorship revenue stream active ($6K–$145K/yr projection)
- Semantic Slime as the gold-standard template (Trinity generates games *like* Semantic Slime)
- **Test:** 10 teachers actively using Trinity, 5 paying for mentorship, 1-2 lessons per week each.

### What "Done" Does NOT Look Like
- NOT done when code compiles → done when a teacher creates a lesson without help
- NOT done when all gap items are checked → done when the loop works end-to-end
- NOT done when deployed to XREAL Aura → done when teachers prefer VR over desktop
- NOT done when revenue exists → done when teachers renew because it made them better

---

## 6. THE TODO — TOOLS AND WORKFLOWS

### P0: Must Have for Level 1 (MVP)

| # | Sprint | Tool/Workflow | What It Does | Service |
|---|--------|---------------|-------------|---------|
| 0 | Sprint 0 | **PWA as the Face** | manifest, service worker, SME interview mode, teacher quick actions, onboarding, lesson display, mode switching | PWA phone.html |
| 1 | Sprint 1 | LM Studio integration | Switch inference router to LM Studio :1234 (Hermes 4 70B). Replace hotel_manager. | LM Studio :1234 |
| 2 | Sprint 2 | ID system prompt | Makes agent behave as instructional designer (Socratic, ADDIECRAPEYE) | Trinity agent.rs |
| 3 | Sprint 2 | `generate_image` tool | ComfyUI Janus-Pro/LongCat image generation, inline in chat | ComfyUI :8188 |
| 4 | Sprint 2 | `generate_voice` tool | ComfyUI VibeVoice TTS narration | ComfyUI :8188 |
| 5 | Sprint 2 | `generate_3d_model` tool | ComfyUI TRELLIS/TripoSR → glTF | ComfyUI :8188 |
| 6 | Sprint 2 | `review_content_safety` tool | Hermes LLM reviews content for K-12 appropriateness | LM Studio :1234 |
| 7 | Sprint 2 | End-to-end test | Real teacher creates lesson through PWA | All |

### P1: Must Have for Level 2-3 (Sellable)

| # | Tool/Workflow | What It Does | Service |
|---|---------------|-------------|---------|
| 8 | Port `trinity-xr` | Bevy 0.18 OpenXR from Bertrand → Trinity workspace | trinity-xr crate |
| 9 | VR chat panel | Chat interface as floating 3D panel in VR | trinity-xr + WS |
| 10 | VR asset viewer | 3D model preview in VR (grab, rotate, scale) | trinity-xr |
| 11 | `assemble_scene` tool | Package assets → Bevy scene / Godot / WebXR | Trinity |
| 12 | Blender integration | Headless Python API — mesh cleanup, glTF export | Blender |
| 13 | Standards DB | SQLite — NGSS, Common Core | Trinity + SQLite |
| 14 | `align_standards` tool | RAG over standards DB | Trinity |
| 15 | `export_scorm` tool | SCORM 1.2/2004 package for LMS | Trinity |
| 16 | `save_lesson` / `list_lessons` | Lesson persistence in SQLite | Trinity + SQLite |
| 17 | PWA onboarding | Examples, hints, transparent agent steps | PWA frontend |
| 18 | XR WebSocket bridge | WS /api/xr/connect for VR ↔ server | Trinity |
| 19 | `generate_video` tool | ComfyUI HunyuanVideo | ComfyUI :8188 |
| 20 | `add_enrichment` tool | Vocab cards, quiz questions, annotations | Trinity |

### P2: Must Have for Level 4 (Maturity)

| # | Tool/Workflow | What It Does | Service |
|---|---------------|-------------|---------|
| 21 | VR scene builder | Visual scene assembly in VR (place assets, set triggers) | trinity-xr |
| 22 | SME interview mode | Guided SME interview workflow on phone | PWA + agent |
| 23 | Android XR APK | `cargo apk build --features xr` for XREAL Aura | trinity-xr |
| 24 | Bevy 0.18 templates | Upgrade templates to match Semantic Slime patterns | templates/ |
| 25 | EYE → agent loop | AI self-improvement from evaluation feedback | Trinity EYE |
| 26 | Community library | Teachers share and remix lessons | Trinity + SQLite |
| 27 | Mentorship review system | Stripe + review queue (adapted from Bertrand) | Trinity + Stripe |
| 28 | Semantic Slime as template | `scaffold_bevy_game` outputs Semantic Slime-style projects | Trinity tools |
| 29 | Sandbox / Cron system | Hermes works autonomously in sandbox, EYE evaluates before incorporating | Trinity + LM Studio |
| 30 | vLLM Omni activation | Replace ComfyUI for images/voice (simpler pipeline) | vLLM Omni :8000 |

### Critical Path

```
Sprint 0 (PWA) → Sprint 1 (LM Studio) → Sprint 2 (ID + Tools) → Level 1: Teacher creates lesson through PWA
      ↓
Sprint 3 (trinity-xr) → P1 items → Level 2-3: Teacher previews in VR, deploys to classroom
      ↓
P2 items → Level 4: Spatial OS, autonomous Hermes, mentorship revenue, community
```

**Focus on P0 first.** Sprint 0 (PWA) is the face — what the teacher sees. Sprint 1 (LM Studio) is the brain — what thinks. Sprint 2 (ID + Tools) is the hands — what builds. Everything else is speculation until a real teacher creates a real lesson through the PWA.

---

## 7. MEMORY BUDGET (Strix Halo 128GB)

| Component | VRAM | Notes |
|-----------|------|-------|
| LM Studio (one LLM) | 10–43GB | Primary brain. Auto-evict when idle |
| vLLM Omni (creative) | 17–42GB | Image + voice generation. Can share with LM Studio if different model |
| ComfyUI (resident) | 17GB | Janus-Pro + VibeVoice (always on for quick creative) |
| ComfyUI (on demand) | up to 53GB | TRELLIS (16GB), HunyuanVideo (13GB), ACE-Step (8GB) |
| Trinity + OS | 8GB | Rust binary, SQLite, Blender headless |
| **Peak (LM Studio + ComfyUI resident + TRELLIS)** | ~84GB | Within 128GB pool |
| **Peak (vLLM Omni + ComfyUI resident + TRELLIS)** | ~75GB | If vLLM Omni replaces LM Studio temporarily |

**Bus contention schedule (resident-but-paused):**
- Chat/reasoning: LM Studio active, vLLM Omni idle, ComfyUI idle
- Creative (image/voice): vLLM Omni active, LM Studio idle, ComfyUI idle
- Creative (3D/video): ComfyUI active, LM Studio idle, vLLM Omni idle
- VR preview: trinity-xr on headset, Trinity server routing, all inference idle

---

## 8. DOCUMENT MAP

| Document | Path | What It Contains |
|----------|------|-----------------|
| **THIS DOC** | `docs/active/MASTER_ARCHITECTURE.md` | Master architecture, project map, workflow, orchestration |
| Spatial Pivot Plan | `docs/active/SPATIAL_PIVOT_PLAN.md` | Detailed plan (887 lines, 21 sections) — XR, XREAL Aura, gap analysis, monetization |
| FACES Gap Analysis | `docs/active/FACES_GAP_ANALYSIS.md` | FACES protocol gap analysis, detection layer, evaluation harness |
| Semantic Slime GDD | `/home/joshua/Semantic Slime/GDD.md` | Game design doc — core loop, FACES mapping, psycholinguistics |
| Semantic Slime Architecture | `/home/joshua/Semantic Slime/ARCHITECTURE.md` | Bevy ECS implementation, module boundaries, data flow |
| Bertrand XR README | `Bertrand-Masterclass/apps/spatial-engine-bevy/README.md` | Voix Vive XR — Bevy OpenXR for XREAL Aura |
| VR_DESIGN.md | `ARCHIVE_VAULT/Phone/trinity-ndk/VR_DESIGN.md` | 593-line XR architecture, VR ID landscape, needs analysis |
| Daydream Bible | `Day_Dream/docs/DAYDREAM_BIBLE.md` | VAAM philosophy, somatic gate, topological choice |
| Bertrand Pricing | `Bertrand-Masterclass/apps/companion-app/src/data/pricingData.js` | Mentorship monetization model (5 tiers, Stripe) |
| Trinity Agent System | `crates/trinity/src/agent.rs` | Agent loop, AGENT_SYSTEM prompt, tool calling |
| Trinity Tools | `crates/trinity/src/tools.rs` | 34 tools, tool gauge system, tool definitions |
| Trinity Creative | `crates/trinity/src/creative.rs` | ComfyUI pipeline coordinator |
| Trinity Inference Router | `crates/trinity/src/inference_router.rs` | Multi-backend router, P-ART-Y roles, auto-detect |
