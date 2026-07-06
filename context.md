# TRINITY ID AI OS — Project Context

> **Last Updated:** July 6, 2026
> **Master Docs:** `AGENTS.md` (entry point) → `docs/active/MASTER_ARCHITECTURE.md` (full architecture) → `docs/active/SPATIAL_PIVOT_PLAN.md` (detailed plan)
> **Hardware:** AMD Strix Halo APU — Radeon 8060S (`gfx1151`), 128GB Unified LPDDR5X
> **Runtime:** Rust/Axum (:3000) + LM Studio (:1234) + vLLM Omni (:8000) + ComfyUI (:8188)
> **Thesis:** Trinity is an AI instructional designer for spatial computing. Teacher chats → Trinity generates VR lessons. Chat is the portal. Agent is the ID.

---

## 1. What TRINITY Is

Trinity is an **AI instructional designer for spatial computing**. A teacher chats in plain language → Trinity generates a complete VR lesson (3D models, narration, quizzes, scenes) through an agent loop with tool calling.

**Orchestration (Trinity is the brain):**
- **Trinity** (:3000) = Orchestrator (agent loop, tools, memory, RAG, persistence, EYE)
- **LM Studio** (:1234) = Brain — Hermes 4 70B loaded and running
- **vLLM Omni** (:8000) = OFF for now (will activate for images/voice in P1)
- **ComfyUI** (:8188) = All creative (images, voice, 3D, video, music)
- **Blender** = 3D refinement (planned, headless Python API)
- **trinity-xr** = VR client (planned, Bevy 0.18 + bevy_oxr, WebSocket to Trinity)

**Sandbox/Cron (planned):** Hermes works autonomously in a sandbox. EYE evaluates before incorporating. Build workflows overnight, evaluate in the morning.

**The Pipeline:**
```
Teacher chats → Trinity agent (Socratic questions) → Asset generation (vLLM Omni + ComfyUI) → Scene assembly (Bevy OpenXR) → EYE evaluation → Deploy (SCORM/Godot/WebXR)
```

---

## 2. Three-Device Architecture

| Device | Name | Role | Hardware | Connection |
|--------|------|------|----------|------------|
| **Phone** | PYTHAGORPHUS | The Director — human input, Socratic questioning, quest management | Pixel 10 Pro XL, 16GB RAM, Gemini Nano (AICore) | Tailscale `100.83.9.71` |
| **Desktop** | TRINITY | The Engine — AI inference, orchestration, content generation | Strix Halo, 128GB VRAM, 50 TOPS NPU | Tailscale `100.83.222.35` |
| **XR** | (future) | The Canvas — spatial review, EYE phase | XREAL Aura (Fall 2026) | WiFi6E |

**Authority Flow:** Phone (commands) → Desktop (compute) → XR (spatial output)

---

## 3. Studio Model Setup (Day Mode)

Two models are always resident on the desktop in Studio mode:

| Role | Model | Engine | Port | VRAM | Speed | Purpose |
|------|-------|--------|------|------|-------|---------|
| **P** | DiffusionGemma 26B AWQ-INT4 | vLLM (podman) | :8000 | ~42GB | ~84 tok/s (MoE) | Execution — story, code, tool calling, reasoning |
| **A** | ComfyUI + Janus-Pro-7B + VibeVoice-1.5B | ComfyUI | :8188 | ~17GB | varies | Creative — image gen, voice, video, 3D on demand |

**Total resident VRAM:** ~59GB of 128GB. Tier 2 models (HunyuanVideo, TRELLIS, LongCat, ACE-Step, FLUX, TripoSR) loaded on demand by ComfyUI within a 53GB budget.

**Hermes 4 70B (H slot):** Managed externally by LM Studio on :8002 when needed for planning. Not hotel-managed — submitted jobs to Trinity via POST /api/jobs.

### Computational Economy: Resident-But-Paused
Models stay loaded in VRAM permanently. Only one model computes at a time — others sit idle (zero bus contention). The 210 GB/s memory bus is the constraint, not capacity.

### Hotel Modes (API-controlled)
- **Studio:** P + A always resident (default). `POST /api/inference/hotel/studio`
- **Solo:** P only, A shut down (IDE agents welcome). `POST /api/inference/hotel/solo`
- **Closed:** All models off (night shift / maintenance). `POST /api/inference/hotel/close`

### Night Mode
Close hotel, kill podman, load large models (MiniMax-M2.7-172B, Mistral-119B) in LM Studio for overnight background jobs. Submit via POST /api/jobs with max_turns=200.

---

## 4. Phone PWA

**URL:** `http://100.83.222.35:3000/trinity/phone.html` (Tailscale) or `http://localhost:3000/trinity/phone.html` (local)
**Features:**
- Chat with streaming SSE responses (P: DiffusionGemma 26B)
- Voice input (Web Speech API — 🎤 button)
- Text-to-speech for responses (🔊 button)
- Hotel status monitoring (P/A health, Studio/Solo/Closed modes)
- Image generation (ComfyUI via agent tool calling)
- Inline image display in chat (SSE `event: image` handler)
- RAG memory (ingest + search)
- Persistent memory (SQLite-backed)
- Focus Mode controls (Creative/Code/Night)

**Native app (archived, 70% built):** `ARCHIVE_VAULT/Phone/trinity-ndk/` — Bevy/Rust Android app with Gemini Nano integration, inference router, desktop proxy. Needs Bevy 0.16→0.18 upgrade and APK rebuild.

---

## 5. Key API Endpoints

| Endpoint | Purpose |
|----------|---------|
| `GET /api/health` | Full subsystem health |
| `POST /api/chat/yardmaster` | Agent chat with tool calling (SSE streaming) |
| `POST /api/chat/stream` | Chat streaming |
| `GET /api/inference/status` | Active inference router status |
| `GET /api/inference/hotel` | Hotel status (read-only) |
| `POST /api/inference/hotel/studio` | Launch P + A (Studio mode) |
| `POST /api/inference/hotel/solo` | P only (Solo mode) |
| `POST /api/inference/hotel/close` | All models off (Closed mode) |
| `POST /api/creative/image` | Image generation via ComfyUI |
| `GET /api/creative/status` | ComfyUI health probe |
| `POST /api/rag/search` | RAG semantic search |
| `GET /api/rag/stats` | RAG statistics |
| `POST /api/jobs` | Submit background job |
| `POST /api/jobs/chain` | Submit job chain (overnight workflow) |
| `GET /api/focus` | Focus mode status |
| `POST /api/focus/creative` | Creative focus (kill IDEs, start studio) |
| `POST /api/focus/code` | Code focus (kill models, keep IDEs) |
| `POST /api/focus/night` | Night shift (kill everything) |

---

## 6. Launching Trinity

```bash
# One-command startup
bash ~/trinity-models/start-diffusiongemma.sh  # P: vLLM on :8000
~/Workflow/TRINITYIDAIOS/target/release/trinity --headless &  # Trinity on :3000
curl -X POST http://localhost:3000/api/inference/hotel/studio  # Launch A: ComfyUI on :8188

# Or use the startup script:
./scripts/launch/trinity_day.sh

# Phone: http://100.83.222.35:3000/trinity/phone.html (Tailscale)
# Local: http://localhost:3000/trinity/phone.html
```

---

## 7. What Is Working (Don't Touch)

| Component | File(s) | Why It's Done |
|-----------|---------|---------------|
| Inference routing | `agent.rs` | Routes to P (DiffusionGemma) for execution |
| Content extraction fix | `inference.rs` | Prefers `content` over `reasoning_content` |
| OpenAI-compatible inference client | `inference.rs` | Streaming, tool calling, dynamic model resolution |
| Multi-backend inference router | `inference_router.rs` | Auto-detect, failover, health probing, P-ART-Y roles |
| Hotel manager | `hotel_manager.rs` | Studio/Solo/Closed modes, P+A lifecycle, CPU pinning (will be replaced by LM Studio) |
| Conductor phase system | `conductor_leader.rs` | 12 Socratic prompts, Bloom's mapping |
| Agentic tool loop | `agent.rs` | Tool calling, SSE streaming, memory injection |
| Quest state machine | `quests.rs` | ADDIECRAPEYE phase gating, XP/Coal/Steam |
| EYE Package export | `export.rs` | HTML5 Quiz, Adventure, DOCX, ZIP |
| Auth + rate limiting | `auth.rs` | Bearer token on dangerous endpoints, sliding window |
| ORT in-process embeddings | `ort_embed.rs` | nomic-embed-text INT8 ONNX, CPU execution |
| Persistent memory | `memory_store.rs` | SQLite-backed, API routes, agent loop integration |
| RAG semantic search | `rag.rs` | ORT-first, Ollama fallback, hash fallback |
| PWA with voice/TTS | `phone.html` | Chat, voice input, read-aloud, hotel controls, image gen, focus modes |
| **Master architecture doc** | `docs/active/MASTER_ARCHITECTURE.md` | Project ecosystem, 7-stage workflow, orchestration decision, maturity model |
| **Spatial pivot plan** | `docs/active/SPATIAL_PIVOT_PLAN.md` | 21-section plan: XR UI, XREAL Aura, gap analysis, monetization, two-audience framework |
| **AGENTS.md** | `AGENTS.md` | Entry point for AI agents — drift prevention, P0 focus, rules |
| **Sprint workflows** | `.windsurf/workflows/` | Sprint 1 (LM Studio), Sprint 2 (ID tools), Sprint 3 (trinity-xr) |

---

## 8. Dead Code Removed (July 3, 2026)

2,861 lines of dead code were removed from compilation. See `docs/active/MASTER_TASK_LIST.md` for details.

FACES Protocol was moved to the Semantic Slime project (`/home/joshua/Semantic Slime/`) on July 5, 2026. The `trinity-faces` crate no longer exists in this workspace.

---

## 9. What's Next — P0 Sprint Focus

**Current level: Level 0. Goal: Level 1 — teacher creates lesson through chat.**

P0 items (see `AGENTS.md` and `docs/active/MASTER_ARCHITECTURE.md` Section 6):

0. **PWA as the Face** — manifest, service worker, SME interview mode, teacher quick actions, onboarding, lesson display (Sprint 0)
1. **LM Studio integration** — Switch to LM Studio :1234, Hermes 4 70B as brain (Sprint 1)
2. **ID system prompt** — Add instructional designer persona to agent.rs (Sprint 2)
3. **`generate_image` tool** — ComfyUI Janus-Pro/LongCat (Sprint 2)
4. **`generate_voice` tool** — ComfyUI VibeVoice (Sprint 2)
5. **`generate_3d_model` tool** — ComfyUI TRELLIS (Sprint 2)
6. **`review_content_safety` tool** — Hermes LLM review via LM Studio (Sprint 2)
7. **End-to-end test** — Real teacher creates lesson through PWA (Sprint 2)

**Do not work on P1 or P2 items until P0 is done.**

Workflows: `.windsurf/workflows/sprint0-pwa.md`, `sprint1-lm-studio.md`, `sprint2-id-tools.md`, `sprint3-trinity-xr.md`
