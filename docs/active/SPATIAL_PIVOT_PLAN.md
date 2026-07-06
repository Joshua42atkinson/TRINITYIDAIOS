# TRINITY ID AI OS — Spatial Education Platform Plan

**Date:** July 6, 2026 | **Author:** Joshua Atkinson + Cascade

> **See also:** `MASTER_ARCHITECTURE.md` — the master document that ties all projects together (Trinity, Semantic Slime, Bertrand XR, Daydream, Bertrand companion-app) and defines the full workflow pipeline and orchestration architecture.

## 1. THE PIVOT

Trinity stops being a model orchestrator and becomes an **AI instructional designer for spatial computing**. The chat IS the portal. The agent IS the instructional designer. Teacher's plain-language request → complete VR lesson scene, all through conversation. LM Studio handles all LLM inference. ComfyUI handles asset generation. Bevy/Godot handle XR deployment.

### Core Principle: Chat is the Portal, Agent is the ID

No separate lesson builder UI. No separate modules with separate APIs. The phone PWA chat is the **entire UX**. The agent loop (already in `agent.rs`) **IS** the instructional designer — it has the right system prompt, the right tools, and the ADDIECRAPEYE framework. The teacher just talks. Trinity asks questions, generates assets, assembles scenes — all through chat.

Users learn by **results-based prompt engineering** — they see what Trinity produces, refine their request, get better results. No tool to learn, just communication to improve. New "vibe coding" people learn as they work — Trinity explains what it's doing, suggests better approaches, shows examples.

## 2. WHY

- **vLLM is hard.** Podman, ROCm flags, firmware crashes, no easy model download/load
- **LM Studio has native APIs** for load/unload/download/MCP — replaces hotel_manager.rs (383 lines), vllm_fleet.rs (12K lines), tools.rs (92K lines)
- **No clear product.** Trinity was "an execution engine" — execution of what? Now: **K-12 EdTech VR content creation**

## 3. WHAT LM STUDIO REPLACES

| LM Studio API | Replaces in Trinity |
|---------------|---------------------|
| `POST /api/v1/models/load` | hotel_manager.rs (383 lines) |
| `POST /api/v1/models/unload` | hotel_manager.rs kill_port |
| `POST /api/v1/models/download` | Manual HuggingFace CLI |
| `GET /api/v1/models/download/status` | Nothing (didn't exist) |
| MCP built-in | tools.rs + tools/ (92K+ lines) |
| `lms daemon up` + systemd | Podman + vLLM + start-diffusiongemma.sh |
| Auto-evict with TTL | Focus mode scripts |

## 4. WHAT TRINITY KEEPS / DELETES / BUILDS

### DELETE (~110K lines)
- `hotel_manager.rs` — LM Studio load/unload
- `vllm_fleet.rs` — LM Studio daemon
- `tools.rs` + `tools/` — LM Studio MCP
- `sidecar_monitor.rs` — LM Studio self-monitors
- Focus mode scripts — LM Studio auto-evict
- Podman/vLLM scripts — eliminated

### KEEP (core value)
- `main.rs` / `routes.rs` / `handlers/` — API server
- `inference.rs` — LLM client (already works with LM Studio :1234)
- `creative.rs` — ComfyUI pipeline coordinator
- `agent.rs` — Agent loop for autonomous execution
- `jobs.rs` — Background job system
- `persistence.rs` / `rag.rs` / `memory_store.rs` — SQLite + RAG
- `narrative.rs` / `quests.rs` — Story + LitRPG for EdTech
- `export.rs` / `eye_container.rs` — EYE evaluation
- `auth.rs` / `voice.rs` / `telephone.rs` — Security + voice + WebSocket
- `ort_embed.rs` — Embedded embeddings

### SIMPLIFY
- `inference_router.rs` → single LM Studio backend on :1234
- `handlers/inference.rs` → add LM Studio proxy endpoints, remove hotel
- `main.rs` → remove vllm_fleet, hotel auto-start
- `configs/runtime/default.toml` → strip to LM Studio + ComfyUI

### BUILD (agent tools, not separate modules)

The agent loop in `agent.rs` already does multi-turn conversations with tool calling. We don't need new modules with separate APIs — we need **better system prompts** and **better tools** the agent calls.

| What | Type | Purpose | Priority |
|------|------|---------|----------|
| ID system prompt | Prompt | Makes the agent behave as an instructional designer: Socratic questioning, ADDIECRAPEYE framework, pedagogical spec generation | P0 |
| `lm_studio_client.rs` | Rust module | Thin proxy for LM Studio v1 API (load/unload/download/status) | P0 |
| `generate_3d_model` tool | Agent tool | Calls ComfyUI TRELLIS/TripoSR → returns glTF path | P0 |
| `generate_voice` tool | Agent tool | Calls ComfyUI VibeVoice → returns audio path | P0 |
| `generate_video` tool | Agent tool | Calls ComfyUI HunyuanVideo → returns video path | P1 |
| `generate_music` tool | Agent tool | Calls ComfyUI ACE-Step → returns audio path | P1 |
| `review_content_safety` tool | Agent tool | LLM reviews generated content for K-12 appropriateness (violence, bias, accuracy) | P0 |
| `add_enrichment` tool | Agent tool | Generates vocabulary cards, quiz questions, annotations for the lesson | P1 |
| `assemble_scene` tool | Agent tool | Packages assets into Bevy scene / Godot project / WebXR bundle | P1 |
| `export_scorm` tool | Agent tool | Exports lesson as SCORM 1.2/2004 package for LMS deployment | P1 |
| `align_standards` tool | Agent tool | RAG over NGSS/Common Core database, returns matched standards | P1 |
| `save_lesson` tool | Agent tool | Saves lesson spec + assets to project workspace in SQLite | P1 |
| `list_lessons` tool | Agent tool | Lists saved lessons, allows resuming/refining previous work | P2 |
| PWA onboarding | Frontend | Examples, hints, transparent agent steps, progressive disclosure | P1 |

## 5. THE INSTRUCTIONAL DESIGN PIPELINE (CHAT-FIRST)

Everything happens in the chat. The agent drives the conversation, calls tools transparently, and shows results inline.

```
Teacher: "I want my 5th graders to learn about the water cycle"

Trinity: "Great! Before I start building, let me ask a few questions:
  • How long should the experience be? (5 min, 15 min, 30 min?)
  • Should students interact with water molecules, or observe a simulation?
  • Do you want narration, text labels, or both?
  • Any specific NGSS standards you're targeting?
  • What would prove to you that they learned it?"

Teacher: "15 minutes, interactive, both narration and labels, NGSS 5-ESS2-1,
          and they should be able to label the four stages correctly"

Trinity: "Perfect. Here's my plan:
  1. Generate a 3D landscape with a lake, sky, and mountains
  2. Create interactive water molecules students can pick up
  3. Generate narration for each stage (evaporation, condensation, precipitation, collection)
  4. Create a quiz at the end where students label the stages
  5. Align to NGSS 5-ESS2-1 and 5-ESS2-2

  Starting generation now..."

  [Agent calls generate_3d_model → landscape]
  [Agent calls generate_3d_model → water_molecule]
  [Agent calls generate_voice → narration_evaporation]
  [Agent calls review_content_safety → all assets pass]
  [Agent calls add_enrichment → quiz questions, vocab cards]
  [Agent calls assemble_scene → Bevy scene + Godot project]
  [Agent calls export_scorm → SCORM package for LMS]

Trinity: "Done! Your lesson 'The Water Cycle: A Spatial Journey' is ready.
  • 3D scene with interactive water molecules ✅
  • Narration for all 4 stages ✅
  • Quiz with 5 questions ✅
  • Aligned to NGSS 5-ESS2-1, 5-ESS2-2 ✅
  • Safety reviewed ✅

  Preview on your phone, or I can export for your VR headset.
  Want me to adjust anything?"
```

The teacher never left the chat. They never opened a separate tool. They just talked.

This maps to the arxiv paper "Multi-Agent Framework for XR Content Creation in K-12" (2604.04728): Pedagogical Agent → Execution Agent → Safeguard Agent → Tutor Agent. But in Trinity, these are **one agent with different tools**, not four separate services. The ADDIECRAPEYE framework is encoded in the system prompt, not in separate code modules.

## 6. VR AS UI — THE SPATIAL OS

Trinity is not just a phone chat app. **VR is the primary UI for building and testing.** The user puts on their XR glasses and steps into Trinity's spatial workspace — chat panels float in 3D space, generated assets appear at real scale, scenes are assembled by hand or voice.

### Target Hardware: XREAL Aura (Google Android XR)

| Spec | Value |
|------|-------|
| **Display** | Optical see-through (OST) — Sony Micro-OLED, 1920×1200 per eye |
| **FOV** | 70° (virtually borderless) |
| **Weight** | < 95g |
| **Chip** | Snapdragon Reality Elite + X1S Spatial Coprocessor |
| **Platform** | Android XR + Google Gemini AI |
| **Hand tracking** | World-facing cameras ×2 (XR_EXT_hand_tracking) |
| **Spatial anchoring** | 6DoF tracking |
| **Input** | Hands (pinch gesture), voice, touchpad on compute puck |
| **Launch** | Fall 2026 (US, UK, Japan, Canada, South Korea) |
| **Price** | ≤ $1,500 |
| **Dev program** | Android XR Developer Catalyst Program (g.co/dev/catalyst) |

### Why XREAL Aura Is THE Target

1. **Optical see-through** — You see the real world through glass, not camera passthrough. Zero latency. This means teachers see real students while digital overlays enhance the lesson.
2. **Android XR** — Google's platform. Play Store apps work. Jetpack XR SDK, OpenXR 1.1, WebXR all supported.
3. **Lightweight (< 95g)** — Wearable all day. Not a bulky headset.
4. **Hand tracking + voice** — No controllers needed. Teachers point and speak.
5. **Fall 2026 launch** — We have months to be ready as a launch-title ID tool.
6. **Developer kits available** — Apply to the Catalyst Program now.

### Trinity's Three-Device Architecture

| Device | Role | UI |
|--------|------|----|
| **Phone (PWA)** | Chat portal, SME interviews, quick previews, on-the-go lesson requests | Text + voice chat |
| **Strix Halo (server)** | AI inference, asset generation, scene assembly, job execution | Headless (:3000 API) |
| **XREAL Aura (VR)** | Spatial workspace — build, preview, test, and experience lessons in XR | 3D floating panels, hand interaction, spatial audio |

### What the VR UI Looks Like

```
┌─────────────────────────────────────────────────────────┐
│  TRINITY SPATIAL WORKSPACE (XREAL Aura)                  │
│                                                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │  CHAT    │  │  ASSET   │  │  SCENE   │              │
│  │  PANEL   │  │  VIEWER  │  │  BUILDER │              │
│  │          │  │          │  │          │              │
│  │ Teacher: │  │  [3D     │  │  [Empty  │              │
│  │ "water   │  │  model   │  │  scene   │              │
│  │  cycle"  │  │  floats  │  │  with    │              │
│  │          │  │  here]   │  │  placed  │              │
│  │ Trinity: │  │          │  │  assets] │              │
│  │ "Great!  │  │  Grab to │  │          │              │
│  │  Let me  │  │  rotate  │  │  Voice:  │              │
│  │  ask..." │  │  Pinch   │  │  "Place  │              │
│  │          │  │  to scale│  │  the sun │              │
│  └──────────┘  └──────────┘  └──────────┘              │
│                                                          │
│  ┌─────────────────────────────────────────┐            │
│  │  LESSON TIMELINE (bottom panel)          │            │
│  │  [Intro] → [Explore] → [Quiz] → [Review] │            │
│  └─────────────────────────────────────────┘            │
│                                                          │
│  Pinch + drag to move panels. Voice to command.          │
│  All panels are render-to-texture 3D quads.              │
└─────────────────────────────────────────────────────────┘
```

### Existing Code We Can Reuse

**Bertrand-Masterclass/apps/spatial-engine-bevy/** is a WORKING Bevy 0.18 OpenXR engine that already targets XREAL Aura:

| File | What It Does | Reuse Path |
|------|-------------|-------------|
| `xr_shell.rs` | XR environment: floor, lighting, tonemapping | Port directly to `trinity-xr/` |
| `spatial_ui.rs` | Render-to-texture 3D floating panels with pointer drag + inertia physics | Port directly — this IS the VR UI system |
| `holographic_ui.rs` | Holographic panels that appear in XR space | Port + adapt for Trinity chat |
| `hand_tracking.rs` | Hand tracking via OpenXR | Port directly |
| `environment_manager.rs` | Environment management | Port + extend for lesson environments |
| `system_menu.rs` | System menu (third-eye anchored) | Port + adapt for Trinity controls |
| `interaction.rs` | Interaction system | Port directly |
| `ipc.rs` | IPC — currently connects to Bertrand | **Modify: connect to Trinity :3000 API** |
| `spatial_audio.rs` | Spatial audio | Port directly |
| `widgets.rs` | UI widgets (glass panels, text styles) | Port + extend |

**ARCHIVE_VAULT/Phone/trinity-ndk/VR_DESIGN.md** (593 lines) contains:
- Full XR architecture (5-layer cake)
- Bevy 0.18 + `bevy_oxr` integration plan
- VR ID use cases (3D model manipulation, spatial AI tutorials, MR workspace)
- VR ID landscape analysis (Strivr, PIXO, VictoryXR, Mursion, Labster)
- "What Instructional Designers Actually Need" — 7 pain points
- Competitive moat analysis
- Phase plan for XR integration
- Dependency roadmap (bevy 0.18, bevy_mod_openxr 0.5, openxr 0.21)

**ARCHIVE_VAULT/Phone/trinity-ndk/crates/bevy-agentic/** has:
- `inference.rs` (22KB) — Async inference routing for Bevy
- `orchestrator.rs` (20KB) — Sub-agent orchestration
- `pipeline.rs` (14KB) — ADDIECRAPEYE stage machine
- `runner.rs` (9KB) — Async runner
- `tests.rs` (12KB) — 34 tests pass

### XR Deployment Targets

| Engine | Use Case | Status |
|--------|----------|--------|
| **Bevy 0.18 + bevy_oxr** | Primary — Trinity XR workspace on XREAL Aura | `spatial-engine-bevy` works, needs port |
| **Godot 4** | Production Android XR for classroom deployment | To install |
| **WebXR** | Chromebook-friendly classroom access (zero install) | Future |

The `assemble_scene` agent tool outputs all three formats from the same lesson spec.

## 7. API — CHAT IS THE PORTAL

No separate lesson API. The existing `/api/agent/chat` endpoint handles everything. The agent calls tools internally. New endpoints are minimal:

| Endpoint | Purpose |
|----------|---------|
| `POST /api/agent/chat` | **Already exists** — chat is the portal, agent is the ID |
| `GET /api/creative/assets/{filename}` | **Already exists** — serve generated assets |
| `GET /api/models` | List LM Studio models (proxy) |
| `POST /api/models/load` | Load model (proxy to LM Studio) |
| `POST /api/models/unload` | Unload model (proxy) |
| `POST /api/models/download` | Download new model from HuggingFace (proxy) |
| `GET /api/models/download/status` | Download progress (proxy) |
| `WS /api/xr/connect` | WebSocket for trinity-xr client (VR ↔ server bridge) |
| `POST /api/xr/scene/push` | Push assembled scene to VR headset for preview |
| `GET /api/lessons` | List saved lessons (for resume/refine in chat) |
| `GET /api/lessons/{id}` | Load lesson spec + assets (for resume in chat) |

## 8. MEMORY BUDGET

| Component | VRAM | Notes |
|-----------|------|-------|
| LM Studio (one model) | 10–100GB | Auto-evict when idle |
| ComfyUI (resident) | 17GB | Janus-Pro + VibeVoice |
| ComfyUI (on demand) | up to 53GB | TRELLIS, HunyuanVideo, etc. |
| Trinity + OS | 8GB | Rust binary, SQLite |
| **Peak** | ~112GB | Within 120GB TTM pool |

## 9. SELLABLE VALUE

Traditional VR lesson: $1,800-5,000, 5-8 days (3D artist + voice actor + ID + developer)
Trinity on Strix Halo: **$0 marginal cost, ~30 minutes, one teacher**

Solves the #1 barrier to VR in K-12: **content creation bottleneck**.

## 10. MIGRATION SPRINTS

### Sprint 1: LM Studio Integration + Code Stripping (~4hr)
1. Create `lm_studio_client.rs`
2. Add model management endpoints (load/unload/download/status)
3. Simplify `inference_router.rs` to single LM Studio backend
4. Update `default.toml`
5. Delete hotel_manager, vllm_fleet, tools, sidecar_monitor
6. Update PWA — Models tab
7. Test: chat + model load/unload via PWA

### Sprint 2: ID Persona + Agent Tools (~6hr)
1. Add "Instructional Designer" persona to `agent.rs` (Socratic questioning, ADDIECRAPEYE, pedagogical spec)
2. Extend `tools/creative.rs`: `generate_3d_model` (ComfyUI TRELLIS/TripoSR), `generate_voice` (VibeVoice), `generate_video` (HunyuanVideo)
3. Create `tools/id.rs`: `review_content_safety`, `add_enrichment`, `align_standards`
4. Create `tools/scene.rs`: `assemble_scene` (Bevy scene / Godot project / WebXR), `export_scorm`
5. Create `tools/lesson.rs`: `save_lesson`, `list_lessons`, `load_lesson`
6. Add tool definitions to `tools.rs` tool list
7. Test via chat: "water cycle 5th grade" → agent asks Socratic questions → generates assets → assembles scene

### Sprint 3: Trinity XR — Port spatial-engine-bevy (~8hr)
1. Create `crates/trinity-xr/` workspace member (Bevy 0.18 + bevy_oxr)
2. Port `xr_shell.rs`, `spatial_ui.rs`, `holographic_ui.rs`, `hand_tracking.rs` from Bertrand
3. Create `chat_panel.rs` — chat interface as floating 3D panel (connects to Trinity :3000)
4. Create `asset_viewer.rs` — 3D asset preview (grab, rotate, scale generated models)
5. Create `scene_builder.rs` — visual scene assembly in VR (place assets, set triggers)
6. Modify `ipc.rs` — connect to Trinity :3000 API instead of Bertrand
7. Add `WS /api/xr/connect` WebSocket endpoint to Trinity server
8. Add `POST /api/xr/scene/push` — push assembled scene to VR
9. Test: desktop OpenXR simulator → chat panel works → asset viewer displays generated 3D model

### Sprint 4: Standards DB + Onboarding (~4hr)
1. Create SQLite migrations for `lessons` and `standards` tables
2. Seed NGSS/Common Core standards
3. Wire `align_standards` tool to RAG over standards DB
4. Add PWA onboarding: example prompts, transparent agent steps, progressive disclosure
5. Add SME interview mode to PWA (guided interview workflow)
6. End-to-end test: teacher request → complete VR lesson → preview in VR → export SCORM

### Sprint 5: Android XR Build + XREAL Aura Prep (~4hr)
1. Configure `trinity-xr` for Android XR build (`cargo apk build --features xr`)
2. Port `AndroidManifest.xml` from Bertrand (com.trinity.xr)
3. Test OpenXR loader on Android XR
4. Apply to Android XR Developer Catalyst Program (g.co/dev/catalyst)
5. Build and test on Vive Pro Elite XR (stand-in for XREAL Aura)
6. Prepare for XREAL Aura dev kit

### Sprint 6: Docs Sync (~1hr)
1. Update all docs to reflect new architecture
2. Update MASTER_TASK_LIST.md
3. Update MASTER_PIVOT_DOCUMENT.md
4. Archive VR_DESIGN.md findings into active docs

**Total: ~27 hours to spatial OS for VR**

## 11. COMPETITIVE LANDSCAPE — REPLACING ALL ID TOOLS

### The Tools Instructional Designers Use Today

| Tool | Cost | What It Does | Trinity Replacement |
|------|------|-------------|---------------------|
| **Articulate 360** | $1,449/yr (Personal), $1,749/yr (Teams) | Industry default — Storyline 360 + Rise 360 + AI course generation | `instructional_design.rs` — Socratic intake → lesson spec → asset generation → scene assembly |
| **Adobe Captivate** | $408/yr ($34/mo) | Software simulations, VR/AR eLearning | `scene_assembler.rs` — interactive VR scenes with quizzes, branching |
| **iSpring Suite** | $970/yr | PowerPoint-to-course, quizzes, dialogue sims | Trinity's output is more capable (3D, VR, voice) |
| **Lectora** | $1,999/yr | Accessibility/compliance, VR via CenarioVR | `scene_assembler.rs` + `safeguard.rs` — accessibility + safety built in |
| **Elucidat** | $2,500/author/yr | Cloud enterprise authoring, AI Learning Accelerator | Trinity PWA — cloud via Tailscale, AI-native |
| **Camtasia** | $300 one-time | Screen recording, video editing | ComfyUI HunyuanVideo + VibeVoice — AI video + narration |
| **Vyond** | ~$300/yr | Animated explainer videos | ComfyUI image + video — AI-created, not template-based |
| **Canva** | Free-$120/yr | Graphics for non-designers | ComfyUI LongCat/FLUX — AI image generation |
| **H5P** | Free | Interactive content types (quizzes, branching) | `tutor.rs` — quizzes, vocab, triggers embedded in 3D scenes |
| **EON Reality** | $36-280/user/mo | Full XR content creation + LMS + AI agents | **Full Trinity pipeline** — content creation + scene assembly + safety |
| **VictoryXR** | $10K-91K bundles | VR labs, AI tutor, pre-made content | **Full Trinity pipeline** — custom content, not pre-made bundles |
| **ManageXR** | $7-10/device/mo | Device fleet management, kiosk mode, remote view | **Complementary** — Trinity creates content, ManageXR deploys it |

### Total Cost Comparison

**School VR setup today (commercial):**
- ManageXR: $2,520-3,600/yr (30 devices)
- EON/VictoryXR content: $10,000-91,000
- Articulate 360: $1,449/yr
- Adobe Captivate: $408/yr
- Camtasia: $300
- **Total: $14,000-96,000+/yr**

**School VR setup with Trinity:**
- Trinity on Strix Halo: $0 (open source, local)
- ManageXR: $2,520-3,600/yr (still needed for fleet management)
- Content creation: $0 (Trinity generates it)
- **Total: $2,520-3,600/yr** (just device management)

**Trinity eliminates 80-95% of the cost.**

### User-Friendliness Advantage

Commercial tools have steep learning curves:
- Articulate Storyline: medium-high, desktop app, Windows-first
- Adobe Captivate: high, complex UI, professional tool
- EON Reality: requires training, platform-specific
- VictoryXR: pre-made content only, no customization

**Trinity's UX:**
1. Teacher opens phone.html PWA
2. Types or speaks: "I want my 5th graders to explore the solar system"
3. Trinity asks Socratic questions (grade? duration? standards?)
4. Trinity generates the lesson — art, 3D models, narration, quizzes
5. Teacher previews on phone, reviews in VR, exports for classroom

**Zero technical expertise required.** No 3D modeling, no prompt engineering, no game development. The teacher talks, Trinity builds. Articulate/Captivate still require you to be an ID who knows the tool. Trinity makes anyone an instructional designer.

### What ManageXR Does That Trinity Cannot (and shouldn't try)

- Fleet device management (MDM)
- Remote view student screens
- Kiosk mode / lockdown
- Device health monitoring
- App distribution to headsets

These are **complementary**. Trinity creates content, ManageXR deploys it. A school needs both — but Trinity replaces the expensive content creation layer.

## 12. WHAT TRINITY CAN'T DO (and shouldn't try)

- Be a VR engine (Bevy/Godot — we USE Bevy, we don't BUILD Bevy)
- Be a model inference server (LM Studio)
- Be a tool execution framework (LM Studio MCP)
- Be a 3D modeling tool (ComfyUI + TRELLIS)
- Be an LMS (Google Classroom / Canvas)
- Be a device fleet manager (ManageXR)
- Be multi-user (single-teacher, single-device)

## 13. GAP ANALYSIS — WHAT WE HAVE vs WHAT WE NEED

### What We Already Have (in active codebase)

| Component | Location | Status |
|-----------|----------|--------|
| Agent loop with tool calling | `agent.rs` (2,347 lines) | ✅ Working — needs ID persona |
| 34 agent tools | `tools.rs` (2,478 lines) | ✅ Working — needs ID-specific tools |
| ComfyUI image generation | `tools/creative.rs` + `creative.rs` (1,485 lines) | ✅ Working |
| ComfyUI music generation | `tools/creative.rs` | ✅ Working |
| Lesson plan generation tool | `tools.rs` — `generate_lesson_plan` | ✅ Exists |
| Quiz/rubric/curriculum tools | `tools.rs` — `generate_quiz`, `generate_rubric`, `curriculum_map` | ✅ Exist |
| Bevy game scaffolding | `tools.rs` — `scaffold_bevy_game` | ✅ Exists |
| 3D concept spawning | `tools.rs` — `daydream_command` | ✅ Exists |
| OCR / document analysis | `tools.rs` — `analyze_document`, `analyze_image` | ✅ Exists |
| RAG + SQLite persistence | `rag.rs`, `persistence.rs` | ✅ Working |
| PWA chat interface | `frontend/` | ✅ Working |
| ADDIECRAPEYE framework | `conductor_leader.rs` | ✅ Working |
| FACES protocol | `trinity-faces` crate | ✅ 105 tests passing |

### What We Already Have (in archive — needs porting)

| Component | Location | Status |
|-----------|----------|--------|
| **Bevy 0.18 OpenXR engine** | `Bertrand-Masterclass/apps/spatial-engine-bevy/` | ✅ Working — port to `trinity-xr/` |
| Spatial UI panels (3D floating) | `spatial_ui.rs` (161 lines) | ✅ Working — render-to-texture, pointer drag |
| Holographic UI panels | `holographic_ui.rs` (185 lines) | ✅ Working |
| Hand tracking | `hand_tracking.rs` (5,673 bytes) | ✅ Working |
| XR environment shell | `xr_shell.rs` (87 lines) | ✅ Working |
| Environment manager | `environment_manager.rs` (8,735 bytes) | ✅ Working |
| System menu (3D anchored) | `system_menu.rs` (9,401 bytes) | ✅ Working |
| Bevy agentic runner | `ARCHIVE_VAULT/Phone/trinity-ndk/crates/bevy-agentic/` | ✅ 34 tests pass |
| VR design doc (593 lines) | `ARCHIVE_VAULT/Phone/trinity-ndk/VR_DESIGN.md` | ✅ Complete research |
| 6 Bevy game templates | `templates/bevy_*` | ⚠️ Bevy 0.15 — need 0.18 upgrade |

### What We DON'T Have (must build)

| # | Gap | What's Missing | Where It Goes | Priority |
|---|-----|----------------|-------------|----------|
| 1 | **ID system prompt** | No instructional designer persona | `agent.rs` — new persona | P0 |
| 2 | **3D model generation tool** | No agent tool for TRELLIS/TripoSR | `tools/creative.rs` — `generate_3d_model` | P0 |
| 3 | **Voice/narration tool** | No agent tool for VibeVoice | `tools/creative.rs` — `generate_voice` | P0 |
| 4 | **Video generation tool** | No agent tool for HunyuanVideo | `tools/creative.rs` — `generate_video` | P1 |
| 5 | **Scene assembly tool** | No packaging of assets into Bevy/Godot/WebXR | `tools/scene.rs` — `assemble_scene` | P1 |
| 6 | **Safety validation** | No K-12 content review | `tools/id.rs` — `review_content_safety` | P0 |
| 7 | **Standards alignment** | No NGSS/Common Core DB | SQLite + `tools/id.rs` — `align_standards` | P1 |
| 8 | **SCORM/xAPI export** | No LMS format export | `tools/scene.rs` — `export_scorm` | P1 |
| 9 | **Lesson management** | No save/load/resume | `tools/lesson.rs` + SQLite | P1 |
| 10 | **Enrichment tool** | No vocab cards, quiz triggers | `tools/id.rs` — `add_enrichment` | P1 |
| 11 | **VR chat panel** | No chat interface in VR | `trinity-xr/chat_panel.rs` | P1 |
| 12 | **VR asset viewer** | No 3D preview in VR | `trinity-xr/asset_viewer.rs` | P1 |
| 13 | **VR scene builder** | No visual scene assembly in VR | `trinity-xr/scene_builder.rs` | P2 |
| 14 | **XR ↔ server bridge** | No WebSocket for VR ↔ Trinity | `WS /api/xr/connect` | P1 |
| 15 | **SME interview mode** | No guided SME interview workflow | PWA mode + system prompt | P2 |
| 16 | **Onboarding / learn-as-you-work** | No examples, hints, progressive disclosure | PWA frontend | P1 |
| 17 | **Android XR build** | No APK build for XREAL Aura | `trinity-xr` Cargo.toml + AndroidManifest | P2 |
| 18 | **Bevy 0.18 templates** | Templates on 0.15, need 0.18 | `templates/` upgrade | P2 |

## 14. EMBEDDED CAPABILITIES TO CONSIDER LATER

| Capability | Use Case | Priority |
|-----------|----------|----------|
| OCR (`ocrs` crate) | Teacher photographs textbook → Trinity extracts text → VR lesson | P2 |
| PDF parsing (`lopdf`) | Teacher uploads lesson plan PDF → VR enhancement | P2 |
| Standards RAG | Auto-align lessons to NGSS/Common Core | P1 |
| Image classification (ORT + CLIP) | Teacher uploads image → lesson suggestions | P3 |

## 15. VIBE CODING — LEARN AS YOU WORK

Trinity is designed for new "vibe coding" people — users who learn by doing, not by reading manuals. The chat-first approach means:

1. **Start with examples** — PWA shows example prompts on first launch ("I want my 5th graders to explore the solar system")
2. **Transparent agent steps** — Every tool call is shown in chat: "Generating 3D model... ✓", "Reviewing content safety... ✓"
3. **Results-based prompt engineering** — User sees what Trinity produced, refines their request, gets better results. No tool to learn, just communication to improve.
4. **Progressive disclosure** — Simple at first (just type a request), complexity revealed over time (standards alignment, SCORM export, scene builder)
5. **Socratic feedback** — Trinity asks questions before building, so the user learns what makes a good lesson spec by answering questions
6. **VR preview** — User puts on glasses, sees the result at real scale, immediately understands what works and what doesn't
7. **SME interview mode** — User takes their phone to a subject matter expert, Trinity interviews the SME, captures requirements, and the big computer at home builds the lesson while they're still talking

### The Phone-as-Interview-Tool Workflow

```
Teacher (at school):  "I need a VR lesson on photosynthesis"
Trinity (phone):      "Great! Let me ask a few questions while the computer
                       at home starts working..."
                       → Socratic questions (grade, duration, standards, interactivity)
                       → Captures SME knowledge (teacher IS the SME here)
                       → Submits lesson spec to Strix Halo via Tailscale

Strix Halo (at home):  → Generates 3D plant cell model (TRELLIS)
                       → Generates narration (VibeVoice)
                       → Creates quiz questions
                       → Assembles Bevy scene
                       → Saves to SQLite

Teacher (back home):   Puts on XREAL Aura → Trinity spatial workspace
                       → Reviews lesson in VR at real scale
                       → "Make the chloroplasts bigger"
                       → Trinity regenerates → updates scene
                       → Exports SCORM for Google Classroom
```

## 16. GOOGLE AURA / ANDROID XR STRATEGY

### Why Android XR Is the Right Platform

- **Open** — OpenXR 1.1 support, not locked to one vendor
- **Google Play Store** — Existing Android apps work in XR
- **Jetpack XR SDK** — Compose for XR, SceneCore, ARCore for Jetpack XR
- **Godot support** — Official Android XR plugin for Godot 4
- **WebXR** — Chrome on Android XR supports WebXR
- **Gemini integration** — Google's AI assistant built into the platform

### Trinity's Position in the Android XR Ecosystem

Trinity is NOT an Android XR app. Trinity is a **spatial OS** that runs on a Strix Halo base station and serves content to Android XR glasses. The `trinity-xr` client is an Android XR app, but Trinity itself is the server.

```
┌─────────────────────────────────────────────┐
│  XREAL Aura (Android XR client)              │
│  ┌─────────────────────────────────────┐    │
│  │  trinity-xr (Bevy 0.18 + OpenXR)    │    │
│  │  • Chat panel (WebSocket to :3000)  │    │
│  │  • Asset viewer (3D model preview)  │    │
│  │  • Scene builder (visual assembly)  │    │
│  │  • Hand tracking + voice input      │    │
│  └─────────────────────────────────────┘    │
│           ↕ Tailscale / Wi-Fi                │
│  ┌─────────────────────────────────────┐    │
│  │  Strix Halo (Trinity server :3000)  │    │
│  │  • Agent loop (ID persona)          │    │
│  │  • LM Studio (LLM inference)        │    │
│  │  • ComfyUI (asset generation)       │    │
│  │  • SQLite (lessons, standards, RAG) │    │
│  └─────────────────────────────────────┘    │
└─────────────────────────────────────────────┘
```

### Android XR Developer Catalyst Program

- **URL:** g.co/dev/catalyst
- **What:** Early access to XREAL Aura dev kits + tools/resources
- **Action:** Apply immediately. Trinity is exactly the kind of spatial computing app Google wants to showcase.
- **Pitch:** "Trinity is the first AI-native instructional design platform for Android XR. Teachers create VR lessons by talking. No 3D modeling, no game development, no code."

### Trinity as Spatial OS for VR

Trinity's vision is to be the **spatial OS for VR** — not an app, but the platform through which all spatial content is created, managed, and deployed:

1. **Create** — Chat with Trinity (phone or VR) → AI generates 3D models, narration, quizzes, scenes
2. **Preview** — See results in VR at real scale → grab, rotate, refine
3. **Assemble** — Place assets in 3D space → set triggers, quizzes, narration points
4. **Deploy** — Export as Bevy scene, Godot project, WebXR, or SCORM package
5. **Iterate** — Chat with Trinity to refine → regenerate assets → update scene

This is the loop: **Chat → Generate → Preview in VR → Refine → Deploy**. All through conversation. All through one portal.

---

## 17. STRATEGIC REVIEW — WHAT'S GOOD, WHAT'S BAD, WHAT WINS

### What's Good in This Plan

- **Chat-first** — No separate UI to learn. The teacher already knows how to talk.
- **Agent-as-ID** — One agent with tools, not four microservices. Simpler, fewer failure points.
- **Existing working code** — Bevy 0.18 OpenXR engine works (Bertrand). Agent loop works (2,347 lines). ComfyUI pipeline works. We're not starting from zero.
- **XREAL Aura target** — Right hardware at the right time. Fall 2026 launch. Android XR. Lightweight. Hand tracking. We have months to be ready.
- **Three-device architecture** — Phone (capture intent), Server (generate), VR (preview/refine). Each device does what it's best at.
- **Sovereign ownership** — Not cloud-locked. Not vendor-locked. Schools own their content and their hardware.
- **$0 marginal cost** — Once the Strix Halo is paid for, every additional lesson is free.

### What's Bad / Risks in This Plan

- **ComfyUI pipelines are incomplete** — VibeVoice needs installation. HunyuanVideo works but is slow. TRELLIS works but output quality for K-12 content is untested. We're assuming pipelines work before verifying.
- **No actual users yet** — We're building for a hypothetical teacher. We need real teachers testing this before XREAL Aura launches.
- **Bevy 0.18 + bevy_oxr is bleeding edge** — The Bertrand engine works on desktop OpenXR, but Android XR (XREAL Aura) is untested. bevy_oxr v0.4.0 may not support all Android XR extensions.
- **XREAL Aura doesn't exist in our hands yet** — We're building for hardware we can't test on. The Catalyst Program is our path to dev kits, but timing is tight.
- **Scope is large** — 18 gap items, 6 sprints, 27 hours. That's focused work, but real life has interruptions. We need to prioritize ruthlessly.
- **The "delete 110K lines" plan is aggressive** — Some of that code (hotel_manager, vllm_fleet) may still be useful if LM Studio doesn't cover every use case. Don't delete until the replacement is proven.
- **No monetization in the plan** — This is a product, not just a project. It needs a revenue model (see Section 18).
- **No feedback loop for the AI** — The agent generates content but doesn't learn from whether it was good. EYE evaluation exists but isn't wired to the agent's self-improvement.

### What Makes This Succeed Where Other VR Programs Fail

**The #1 reason VR education fails: content creation bottleneck.**

Every VR education company hits the same wall:
- **Strivr** — Enterprise only, $50K+ custom content, needs their team to build it
- **VictoryXR** — Pre-made content only, $10K-91K bundles, can't customize
- **EON Reality** — Platform lock-in, requires their proprietary tools
- **Labster** — Science labs only, not general-purpose
- **Google Expeditions** — Killed because teachers couldn't create their own content

**Trinity solves this at the root:**
1. **Teachers create by talking** — No 3D modeling, no game dev, no code. The barrier is "can you describe what you want?" not "can you use Blender."
2. **Content is generated, not pre-made** — Every lesson is custom. Every teacher gets exactly what they need. No content library to maintain.
3. **Sovereign and open** — Schools own everything. No subscription trap. No vendor lock-in. No platform death risk.
4. **AI is the ID, not a feature** — Other tools add AI as a bolt-on. Trinity's agent IS the instructional designer. The ADDIECRAPEYE framework is baked into the system prompt, not a separate module.
5. **VR is the UI, not just the output** — Other tools: build on desktop → export to VR. Trinity: build IN VR. See it at real scale as you create it. Grab it, resize it, test it immediately.
6. **Three-device, one portal** — Phone captures intent (anywhere), server generates (powerful), VR refines (spatial). No other tool does all three.

**The moat is not the code. The moat is the loop:**

```
Chat → Generate → Preview in VR → Refine → Deploy
 ↑                                    ↓
 └────────── human feedback ──────────┘
```

This loop gets faster with use. The agent learns the teacher's style (VAAM). The teacher learns to prompt better (vibe coding). The asset library grows. Each lesson makes the next one faster. Competitors can copy individual features, but they can't copy the accumulated relationship between teacher and AI.

## 18. MONETIZATION — MENTORSHIP MODEL

### The Bertrand Masterclass Precedent

Bertrand's model (from `pricingData.js`):

| Tier | Price | What You Get |
|------|-------|-------------|
| Free | $0 | Full curriculum, offline AI, all tools — no gate |
| Community | $5/mo | Cloud AI + community + mentorship blog |
| Apprentice | $100/mo | Access to Bertrand's reviews (4/mo, AI pre-screens first) |
| Journeyman | $500/mo | 4 live 1-on-1 sessions + async reviews |
| Master | $1000/mo | 8 live sessions, direct messaging, Bertrand is your mentor |

**Key insight from Bertrand:** "AI makes content free. Human attention is the premium."

The model works because:
1. AI pre-screens every submission → flags issues, generates draft review
2. Human (Bertrand) reviews the AI analysis → adds judgment, records feedback
3. Human time drops from 12 min to ~5 min per review → scales 2.4x
4. Students pay for ACCESS to human judgment, not for content

### Trinity's Mentorship Model

Same principle, applied to VR instructional design:

| Tier | Price | What You Get |
|------|-------|-------------|
| **Free** | $0 | Full platform, local AI, all tools, all templates. Create unlimited VR lessons. No gate. |
| **Community** | $10/mo | Cloud sync, community library (share lessons), AI model upgrades, blog |
| **Apprentice** | $100/mo | Submit VR lessons for peer review (4/mo). AI pre-screens pedagogical quality, safety, standards alignment. Joshua reviews the AI analysis, adds judgment, records 3-5 min feedback video. |
| **Journeyman** | $500/mo | 4 live 1-on-1 sessions/month (Zoom or VR). Co-design lessons together. Joshua watches you build in VR, corrects approach in real time. |
| **Master** | $1000/mo | 8 live sessions/month. Direct messaging. Joshua is your instructional design mentor. Quarterly curriculum review. |

### Why This Works for Trinity

1. **The platform is free** — No barrier to adoption. Teachers try it, it works, they want more.
2. **AI does the heavy lifting** — Trinity generates the lesson. The AI pre-screens for pedagogical quality, safety, standards alignment.
3. **Joshua adds human judgment** — Is this lesson actually good? Will students learn from it? Is the pacing right? Is the assessment fair? AI can't answer these definitively. Joshua can.
4. **Joshua's time scales** — AI pre-screening means Joshua spends 5 min per review, not 30 min. He can serve 10x more clients.
5. **The funnel is built in** — Free platform → teacher creates lessons → teacher wants validation → pays for peer review → Joshua reviews → teacher improves → stays subscribed

### Revenue Projection

| Scenario | Clients | Avg Revenue/client/mo | Monthly Revenue | Annual Revenue |
|----------|---------|----------------------|-----------------|----------------|
| Conservative | 5 Apprentice | $100 | $500 | $6,000 |
| Moderate | 10 Apprentice + 2 Journeyman | ~$183 avg | $1,833 | $22,000 |
| Growth | 20 Apprentice + 5 Journeyman + 1 Master | ~$214 avg | $5,571 | $66,857 |
| Scale | 50 Apprentice + 10 Journeyman + 3 Master | ~$186 avg | $12,143 | $145,714 |

This is Joshua's income for reviewing VR lessons created on Trinity. The platform itself is free. The value is Joshua's expertise as an instructional designer and VR education pioneer.

### Comparison to Bertrand

| Aspect | Bertrand | Trinity |
|--------|----------|---------|
| **Content** | Guitar curriculum (12 chapters) | VR lesson creation platform |
| **AI role** | Pre-screens video submissions | Generates entire VR lessons |
| **Human role** | Bertrand reviews guitar technique | Joshua reviews instructional design quality |
| **Free tier** | Full curriculum + offline AI | Full platform + local AI |
| **Premium** | Access to Bertrand's eyes | Access to Joshua's eyes |
| **Scale mechanism** | AI cuts review time 12min→5min | AI cuts lesson creation 5 days→30 min |
| **Revenue to** | 100% Bertrand | 100% Joshua |

Same model. Different domain. Both work because **AI makes content free, human attention is the premium.**

### What Joshua Sells (Not Software)

Joshua is not selling Trinity. Joshua is selling **his judgment**:
- "Is this VR lesson pedagogically sound?"
- "Will 5th graders actually learn from this?"
- "Is the assessment aligned to the objective?"
- "Is the pacing right for the grade level?"
- "Does this meet NGSS standards?"
- "What's missing? What would make it better?"

Trinity is the tool that makes Joshua's expertise scale. Without Trinity, Joshua can review maybe 2-3 lessons per week. With Trinity's AI pre-screening, Joshua can review 10-20 lessons per week. That's the business.

## 19. TWO AUDIENCES — THE USER AND THE AI

Trinity has two audiences. Every line of code, every system prompt, every UI element serves both:

### Audience 1: The User (Teacher / Instructional Designer)

**What they need:**
- Simplicity — type or speak, get results
- Transparency — see what Trinity is doing, understand why
- Control — refine, redirect, override
- Trust — confidence that the output is pedagogically sound
- Speed — from idea to preview in minutes, not days
- Ownership — my lessons, my content, my hardware

**How the code serves them:**
- PWA chat interface (phone.html) — the portal
- VR spatial workspace (trinity-xr) — the preview/build environment
- Transparent tool calls in chat ("Generating 3D model... ✓")
- Example prompts and progressive disclosure (onboarding)
- SME interview mode (guided workflow)
- Export options (SCORM, Godot, WebXR — deploy anywhere)

### Audience 2: The AI (Agent / Trinity's Brain)

**What it needs:**
- Clear system prompt — who am I, what's my job, how do I think
- Well-defined tools — what can I do, what are the parameters, what do I get back
- Context — conversation history, RAG, VAAM profile, lesson state
- Feedback — did this work? was it good? what should I do differently?
- Memory — lessons created, what worked, teacher preferences
- Autonomy — ability to chain tools, make decisions, recover from errors

**How the code serves it:**
- `agent.rs` system prompt — the ID persona, ADDIECRAPEYE framework, tool descriptions
- `tools.rs` tool registry — clear names, descriptions, parameters, permissions
- `inference.rs` — LLM client with structured tool calling
- `rag.rs` + `persistence.rs` — knowledge base and memory
- `conductor_leader.rs` — phase-specific Socratic prompts
- `vaam.rs` — learns user's communication style over time
- `eye_container.rs` — EYE evaluation (the AI's self-assessment)
- `creative.rs` — ComfyUI pipeline (the AI's paintbrush)

### Where They Meet

The **agent loop** is where the two audiences intersect:

```
User says: "water cycle for 5th grade"
     ↓
AI thinks: [ADDIECRAPEYE Analysis phase — what's the learning objective?]
     ↓
AI asks: "How long? Interactive? NGSS standards?"
     ↓
User answers
     ↓
AI thinks: [Design phase — what assets do I need?]
     ↓
AI calls: generate_3d_model → generate_voice → review_content_safety
     ↓
User sees: "Generating landscape... ✓ Generating narration... ✓"
     ↓
AI thinks: [Development phase — assemble the scene]
     ↓
AI calls: assemble_scene → export_scorm
     ↓
User sees: "Your lesson is ready. Preview in VR?"
     ↓
User puts on VR glasses → sees the lesson at real scale
     ↓
User says: "Make the clouds bigger"
     ↓
AI thinks: [Refinement — regenerate cloud asset, update scene]
     ↓
Loop continues...
```

**The code is the conversation between human and AI.** Every feature should be evaluated against both audiences:
- Does this help the user express intent? → Good for Audience 1
- Does this help the AI understand and execute? → Good for Audience 2
- Does this help them collaborate? → Good for both

### The AI as Student (Emergent Technology)

Trinity is an emergent technology not just because it's new, but because **the AI itself emerges** — it gets better with use. The more lessons it creates, the more it learns about:
- What works for different grade levels (VAAM + lesson history)
- What the teacher's style is (VAAM profile)
- What assets are reusable (asset library grows)
- What pedagogical patterns are effective (EYE evaluation feedback)

This is why the two-audience view matters. If we only optimize for the user, the AI stays dumb. If we only optimize for the AI, the user gets lost. We need both:
- **User-facing features:** chat, VR preview, examples, onboarding, export
- **AI-facing features:** better system prompts, richer tools, RAG over past lessons, EYE self-evaluation, VAAM learning

## 20. MATURITY MODEL — WHAT "DONE" LOOKS LIKE

### Level 0: Where We Are Now (July 2026)
- Agent loop works with tool calling ✅
- ComfyUI image generation works ✅
- PWA chat works ✅
- Bevy 0.18 OpenXR engine exists (Bertrand) ✅
- 6 Bevy game templates exist (0.15) ✅
- No ID persona, no 3D/voice/video tools, no VR integration, no standards, no export

### Level 1: Minimum Viable Spatial ID (Sprint 1-2)
**The agent can create a lesson spec and generate basic assets through chat.**
- LM Studio integration ✅
- ID system prompt in agent.rs ✅
- `generate_3d_model` tool (TRELLIS) ✅
- `generate_voice` tool (VibeVoice) ✅
- `review_content_safety` tool ✅
- Teacher can chat: "water cycle 5th grade" → get lesson spec + 3D model + narration
- **Test:** Real teacher creates one lesson through chat. It works end-to-end.

### Level 2: Spatial Workspace (Sprint 3)
**The teacher can preview and refine the lesson in VR.**
- `trinity-xr` crate ported from Bertrand ✅
- Chat panel in VR (WebSocket to :3000) ✅
- Asset viewer in VR (3D model preview) ✅
- `assemble_scene` tool ✅
- Teacher puts on VR → sees chat + 3D model floating in space → can grab and rotate
- **Test:** Teacher creates lesson on phone, puts on VR, previews it, says "make it bigger," Trinity regenerates.

### Level 3: Sellable Product (Sprint 4-5)
**A teacher can create, preview, refine, and deploy a VR lesson without technical help.**
- Standards alignment (NGSS/Common Core) ✅
- SCORM export ✅
- Lesson save/load/resume ✅
- PWA onboarding (examples, hints, progressive disclosure) ✅
- Android XR APK build ✅
- Applied to Android XR Developer Catalyst Program ✅
- **Test:** A non-technical teacher (not Joshua) creates a VR lesson from scratch, deploys it to Google Classroom, and it works. Joshua reviews it via the mentorship system.

### Level 4: Maturity — The Spatial OS (Ongoing)
**Trinity is the platform through which all spatial content is created, managed, and deployed.**
- AI learns from EYE evaluation — lessons get better over time
- VAAM profiles personalize the agent's behavior per teacher
- Community library — teachers share and remix lessons
- Multi-engine export (Bevy, Godot, WebXR, SCORM, Android XR)
- XREAL Aura launch-title ID tool
- Mentorship revenue stream active
- **Test:** 10 teachers actively using Trinity, 5 paying for mentorship reviews, 1-2 lessons per week each.

### What "Done" Does NOT Look Like
- Trinity is NOT done when the code compiles. It's done when a teacher creates a lesson without help.
- Trinity is NOT done when we have all 18 gap items. It's done when the loop (Chat → Generate → Preview → Refine → Deploy) works end-to-end.
- Trinity is NOT done when we deploy to XREAL Aura. It's done when a teacher prefers creating in VR over creating on desktop.
- Trinity is NOT done when we have revenue. It's done when teachers renew their mentorship subscription because it made them better at their job.

## 21. THE TODO — PRIORITIZED

### P0: Must Have for Level 1 (MVP)
1. LM Studio integration (Sprint 1)
2. ID system prompt in `agent.rs`
3. `generate_3d_model` tool
4. `generate_voice` tool
5. `review_content_safety` tool
6. End-to-end test: teacher chat → lesson spec + assets

### P1: Must Have for Level 2-3 (Sellable)
7. Port `trinity-xr` from Bertrand (Sprint 3)
8. VR chat panel + asset viewer
9. `assemble_scene` tool
10. Standards DB + `align_standards` tool
11. `export_scorm` tool
12. Lesson save/load (`save_lesson`, `list_lessons`)
13. PWA onboarding (examples, hints)
14. XR↔server WebSocket bridge
15. `generate_video` tool
16. `add_enrichment` tool

### P2: Must Have for Level 4 (Maturity)
17. VR scene builder (visual assembly in VR)
18. SME interview mode
19. Android XR APK build
20. Bevy 0.18 template upgrades
21. EYE evaluation → agent self-improvement loop
22. Community lesson library
23. Mentorship review system (Stripe + review queue)

### The Critical Path

```
P0 (Sprint 1-2) → Level 1: Teacher creates lesson through chat
        ↓
P1 (Sprint 3-4) → Level 2-3: Teacher previews in VR, deploys to classroom
        ↓
P2 (Sprint 5+) → Level 4: Trinity is the spatial OS, mentorship revenue active
```

**Focus on P0 first.** Everything else is speculation until a real teacher creates a real lesson through chat.
