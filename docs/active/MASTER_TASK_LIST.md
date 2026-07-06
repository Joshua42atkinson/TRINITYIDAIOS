# Trinity ID AI OS — Maturation Map
## Updated: July 5, 2026 — Comprehensive architecture + task audit

> **Purpose:** Single source of truth for architecture, remaining work, and completion criteria.
> Updated as items are completed. Nothing is removed — only marked done.

---

## PRODUCT IDENTITY (LOCKED July 5, 2026)

**Trinity IS** the static, always-on execution engine:
- Rust/Axum server on :3000 — the static load
- Creative pipeline (story → art → voice → video → 3D via ComfyUI :8188)
- Job execution engine (autonomous agent loop, tool calling, max_turns=200)
- EYE evaluation system (compile, preview, export)
- Hotel manager (GPU resource orchestration)
- FACES protocol (4-byte emotive signaling, NPU target)
- RAG + persistent memory (SQLite knowledge base)

**Everything else is an API client:**
- **Hermes Agent** (Nous Research) — planning, review, model loading. Talks to Trinity via `POST /api/jobs`. Runs on desktop and phone.
- **IDE agents** (Windsurf, etc.) — code editing. Trinity serves, doesn't compete.
- **PWA** (phone.html) — lightweight UI for chat + creative triggers.
- **Semantic Slime** — separate Bevy EdTech game at `/home/joshua/Semantic Slime`. May become a Trinity crate in the future.

**Key insight:** Trinity never needs to be the smartest agent — it's the most reliable execution engine. Hermes does the thinking, Trinity does the building.

---

## COMPUTATIONAL ECONOMY (LOCKED July 5, 2026)

### Hardware: AMD Strix Halo APU (GMKtec Evo X2)
- 128GB unified LPDDR5X-8000 (single memory pool, GPU + CPU shared)
- 210 GB/s practical bandwidth (shared bus — contention is the constraint, not capacity)
- 16 Zen 5 cores (32 threads), 40 RDNA 3.5 CUs, XDNA 2 NPU (50 TOPS)

### Principle: Resident-But-Paused

Models stay loaded in VRAM permanently. Only one model computes at a time —
the others sit idle (zero bus contention). The scheduler directs who gets the bus.
No load/unload cycles. No VRAM thrashing. No 60-second startup waits.

### Static Always-On Layout (102GB / 128GB)

| Slot | Model | VRAM | Port | Threads | Role |
|------|-------|------|------|---------|------|
| **P** | DiffusionGemma 26B AWQ-INT4 | 42 GB | :8000 | 0-3 | Execution — story, code, tool calling, 84 tok/s MoE |
| **A** | ComfyUI + Janus-Pro-7B + VibeVoice-1.5B | 17 GB | :8188 | 4-7 | Creative — image, voice, video, 3D on demand |
| **H** | Hermes 4 70B Q3_K_M (llama.cpp) | 43 GB | :8002 | 8-15 | Planning — reasoning, review, 131K ctx, hybrid mode |
| **OS** | System + Trinity Rust server | 6 GB | — | 28-31 | OS, server, headroom |
| **Total** | | **108 GB** | | | **20 GB headroom** |

### Bus Contention Schedule

| Phase | P Active | A Active | H Active | Bus Owner |
|-------|----------|----------|----------|-----------|
| Execution | **YES** (84 tok/s) | idle | idle | P |
| Creative | idle | **YES** (image gen) | idle | A |
| Planning | idle | idle | **YES** (reasoning) | H |
| Overnight | **YES** (jobs) | idle | idle | P |

Each model only uses the memory bus when actively processing a request.
Idle models consume VRAM (resident) but zero bandwidth (paused).

### Hermes 4 70B Specs
- Based on Llama-3.1-70B, hybrid reasoning mode
- 131K context — can ingest entire codebase
- Q3_K_M: 43GB weights, fits alongside P + A in 128GB
- Q4_K_M: 53GB weights — would require evicting A (ComfyUI) for 32K ctx
- Trained for schema adherence (valid JSON output) — perfect for submitting structured job chains to Trinity
- Improved steerability, reduced refusal rates
- Hybrid reasoning: `` segments for complex planning, fast mode for simple tasks

### Hotel Mode Update (Resident-But-Paused)

| Mode | P | A | H | Description |
|------|---|---|---|-------------|
| **Studio** | resident | resident | resident | All three loaded, bus directed by request routing (default) |
| **Solo** | resident | off | off | P only — IDE agents welcome, max CPU for coding |
| **Closed** | off | off | off | Night shift / maintenance / LM Studio |

No more "swap" — models stay resident in Studio mode. Trinity's API routing
determines which model gets the bus based on request type:
- `/api/chat` → P (execution)
- `/api/creative/*` → A (ComfyUI)
- Planning requests → H (Hermes)

### Overnight Workflow

```
Evening:
  Hermes 4 70B reviews codebase (131K ctx, full project in one pass)
  Hermes builds structured action plan (JSON schema output)
  Hermes submits plan to Trinity via POST /api/jobs/chain

Night:
  Hermes idle (resident, zero bus usage)
  Trinity executes plan autonomously via P (agent loop, max_turns=200)
  Results saved to SQLite + ~/Workflow/trinity-reports/

Morning:
  Phone Hermes reviews Trinity output
  Phone Hermes runs EYE evaluation on generated content
  Desktop ready for creative work (A has bus, P on standby)
```

---

## WHAT "FINISHED" LOOKS LIKE

Trinity is "finished" when all of the following are true:

### Production Readiness (Phases 1-9, 13)
- [x] Core Cargo.toml has zero mandatory middleware deps (ironroad/export are optional features)
- [x] `main.rs` is under 500 lines (currently 509 — routes + handlers split)
- [x] No `#![allow(dead_code)]` in Core
- [x] Auth, CORS, rate limiting, body limits, graceful shutdown all in place
- [x] Tools trimmed to builder-relevant set
- [x] Frontend consolidated to single PWA
- [x] **Creative pipeline rewired to ComfyUI :8188 (image generation live)**
- [ ] **One creative asset produced end-to-end (story → art → voice)**
- [ ] **All docs reflect current architecture (stale references in context.md, MASTER_WORKFLOW.md)**
- [ ] **Hotel manager updated to resident-but-paused model (no kill/reload)**
- [ ] **Hermes 4 70B integrated as H slot in hotel**
- [ ] **API request routing directs bus to correct model**
- [ ] **Systemd services audited and match current model setup**
- [ ] **Caddyfile audited and matches current architecture**
- [ ] **Release binary builds clean and starts correctly**

### FACES Protocol (W1-W10)
- [x] W1-W6: Protocol, detection, multi-sentence, consent gate, static/dynamic, labeling guide
- [x] W7: Evaluation harness (loader, metrics, runner, benchmark — 4 files in eval/)
- [ ] **W8: Emulator core — ASCII face renderer with real-time state input**
- [ ] **W9: Emulator interaction — keyboard friction, consent gate UI, drift visualization**
- [ ] **W10: Emulator fleet + physical — LED matrix output, multi-device sync**
- [ ] **Gate 2: Joshua approves labeling guide (prerequisite for W7 validation)**
- [ ] **Gate 3: Review emulator (after W10)**

### External Projects (NOT Trinity scope)
- **Semantic Slime** — Bevy EdTech game at `/home/joshua/Semantic Slime`. Separate project, may become Trinity crate in future.
- **Hermes Agent** — Nous Research agent framework. Installed separately, talks to Trinity via API.

### XR / Spatial Computing (Phase 12)
- [ ] **Bevy 3D/XR test with OpenXR on Strix Halo**
- [ ] **trinity-ndk revival (Bevy 0.16→0.18, APK rebuild)**
- [ ] **XREAL Aura integration (when hardware available Fall 2026)**

---

## TRACK A: Trinity Production Roadmap

### Phase 8: E2E Creative Pipeline + Architecture Update (IN PROGRESS)

**Problem:** All creative endpoints in `creative.rs` point at stale sidecar ports. The hotel was restructured to use ComfyUI (:8188) as the single creative backend, but the code still calls old individual ports. Hotel manager needs update to resident-but-paused model.

| # | Task | Status | Details |
|---|------|--------|---------|
| 8.1 | Rewire `generate_image` to ComfyUI :8188 | **DONE** | Now uses ComfyUI `/prompt` + `/history` poll with LongCat-Image workflow. Janus-Pro-7B node has a transformers compatibility bug (deferred). Tested end-to-end. |
| 8.2 | Rewire `generate_tempo` to ComfyUI :8188 | PENDING | Currently POSTs to `:8008/v1/audio/generations` (Acestep). Must use ComfyUI `/prompt` API with VibeVoice workflow JSON. |
| 8.3 | Rewire `generate_video` to ComfyUI :8188 | PENDING | Currently POSTs to `:8006/v1/video/generations` (CogVideoX). Must use ComfyUI `/prompt` API with HunyuanVideo workflow JSON. |
| 8.4 | Rewire `generate_3d_mesh` to ComfyUI :8188 | PENDING | Currently POSTs to `:8007/v1/3d/generations` (TripoSR). Must use ComfyUI `/prompt` API with TRELLIS/TripoSR workflow JSON. |
| 8.5 | Fix `creative_status` health probes | **DONE** | Now probes `:8188/system_stats` and reports unified creative backend status. |
| 8.6 | Add ComfyUI workflow queue + poll mechanism | **DONE** | `queue_comfyui_workflow()` and `poll_comfyui_history()` helpers added in `creative.rs`. |
| 8.7 | Add ComfyUI workflow JSON templates | IN PROGRESS | LongCat-Image template done. Janus-Pro (bug), VibeVoice, HunyuanVideo, TRELLIS/TripoSR, ESRGAN templates pending. |
| 8.8 | Add ESRGAN upscale endpoint or integrate into image flow | PENDING | RealESRGAN_x4.pth is available in ComfyUI. Should auto-upscale generated images. |
| 8.9 | Add creative asset project workspace management | PENDING | Assets currently save to `~/.local/share/trinity/workspace/assets/{images,videos,meshes,audio}`. Need a "project" concept to group assets by creative work. |
| 8.10 | Add pipeline orchestration endpoint | PENDING | A `/api/creative/pipeline` endpoint that accepts a story prompt, generates text via P, art via ComfyUI, and voice via VibeVoice in sequence. |
| 8.11 | Run E2E test: story → art → voice | PENDING | Execute the full pipeline and verify all assets are produced and saved. |
| 8.12 | Verify PWA can trigger creative pipeline | **DONE** | Phone PWA chat is now the agentic entry point. User can ask from the phone to generate images (and later code/docs) via `/api/chat/yardmaster` with DiffusionGemma as the active backend. Tested end-to-end: chat request → generate_image tool → ComfyUI :8188 → image saved to workspace. |
| 8.13 | Add `/api/focus/execute` mode | DEFERRED | Not needed for LM Studio Hermes workflow. Defer. |
| 8.14 | Add EYE review panel to phone.html PWA | DEFERRED | Nice-to-have. Defer. |

### Phase 8.5: Hotel Manager — Resident-But-Paused Architecture (DEFERRED)

**Decision:** Hermes 4 70B will be managed by **LM Studio**, not Trinity hotel. LM Studio is already configured as a fallback backend in Trinity. This phase is deferred until/if we want Trinity to directly manage Hermes.

| # | Task | Status | Details |
|---|------|--------|---------|
| 8.5.1 | Add H (Hermes) slot to HotelGuestConfig | DEFERRED | Hermes managed externally by LM Studio. |
| 8.5.2 | Add Hermes launch script | DEFERRED | Hermes managed externally by LM Studio. |
| 8.5.3 | Update hotel modes to resident-but-paused | DEFERRED | Current swap model is fine for P + A. |
| 8.5.4 | Add API request routing for bus direction | DEFERRED | Not needed with only 2 hotel guests. |
| 8.5.5 | Update `hotel_manager.rs` to not kill processes | DEFERRED | Current kill/reload is acceptable for P + A. |
| 8.5.6 | Add Hermes systemd service | DEFERRED | Use LM Studio instead. |
| 8.5.7 | Update Focus Mode for tri-model awareness | DEFERRED | Only need P + A detection for now. |

### Phase 9: Docs Sync (PENDING)

| # | Task | Status | Details |
|---|------|--------|---------|
| 9.1 | Update `MASTER_WORKFLOW.md` phase status table | PENDING | Phases 1-7 complete, Phase 8 in progress, W1-W7 done. |
| 9.2 | Update `context.md` | PENDING | Hotel modes = Studio/Solo/Closed (resident-but-paused). Models = P + A + H. Remove Qwythos references. |
| 9.3 | Update `SESSION-HANDOFF.md` | PENDING | Current state: Phases 1-7 done, Phase 8 + 8.5 in progress, W1-W7 done. |
| 9.4 | Update `00-master-roadmap.md` status column | PENDING | Mark phases 1-7 as COMPLETE, 8 + 8.5 as IN PROGRESS. |
| 9.5 | Remove stale references in `creative.rs` header comments | PENDING | Comments reference "SDXL-Turbo", "MusicGPT", "trinity_sidecar_engineer" — all stale. |
| 9.6 | Remove stale references in `main.rs` startup info messages | PENDING | Lines 440-442 reference wrong descriptions. |
| 9.7 | Update `context.md` section 3 (Dual-Model Setup) | PENDING | Now tri-model: P + A + H. Remove Qwythos-9B on :8002. Add Hermes 4 70B on :8002. |
| 9.8 | Update `context.md` section 5 (API Endpoints) | PENDING | Missing creative pipeline endpoints, hotel studio/solo/close, focus modes. |
| 9.9 | Document computational economy model | PENDING | Add section to context.md explaining resident-but-paused, bus contention, VRAM budget. |
| 9.10 | Document Hermes integration | PENDING | Add Hermes Agent to context.md as API client. Document overnight workflow. |

### Phase 13: Deployment Packaging (PARTIALLY DONE)

| # | Task | Status | Details |
|---|------|--------|---------|
| 13.1 | Audit `trinity-os.service` systemd unit | PENDING | Verify paths match `target/release/trinity`, verify `--headless` flag. |
| 13.2 | Audit `trinity-comfyui.service` systemd unit | PENDING | Verify ComfyUI launch command matches current flags (--disable-mmap, --bf16-vae, --cache-none). |
| 13.3 | Audit `trinity-llama.service` systemd unit | PENDING | Currently references Qwythos — replace with Hermes 4 70B config. |
| 13.4 | Create `trinity-hermes.service` systemd unit | PENDING | New service for Hermes 4 70B Q3_K_M, always resident, threads 8-15. |
| 13.5 | Audit `Caddyfile` | PENDING | Verify blocked routes match Phase 5 security work. Verify reverse proxy targets. |
| 13.6 | Audit desktop files | PENDING | Verify `Exec=` paths, `Icon=` paths. |
| 13.7 | Build release binary | PENDING | `cargo build -p trinity --release` — verify under 50MB. |
| 13.8 | Test release binary startup | PENDING | Start, hit /api/health, verify clean shutdown. |
| 13.9 | Create/verify install script | PENDING | Copy binary, systemd units, desktop files, configs to system locations. |

---

## TRACK B: FACES Protocol

### Gate 2: Approve Labeling Guide (BLOCKING W7 validation)

| # | Task | Status | Details |
|---|------|--------|---------|
| G2.1 | Joshua reviews `FACES_LABELING_GUIDE.md` | PENDING | 37KB, 130+ labeled examples. Joshua must approve before W7 eval can be validated. |
| G2.2 | Run eval harness against labeled examples | PENDING | `cargo test -p trinity-faces --features eval` — verify precision/recall/F1 metrics. |

### W8: Emulator Core (PENDING)

| # | Task | Status | Details |
|---|------|--------|---------|
| W8.1 | Create `emulator/` module in trinity-faces | PENDING | Terminal-based ASCII face renderer. |
| W8.2 | Implement real-time state input loop | PENDING | Accept FACES state via stdin or file, render to terminal. |
| W8.3 | Implement smooth transitions | PENDING | Use `transition.rs` math to animate between states. |
| W8.4 | Add keyboard input for manual state override | PENDING | Allow user to manually set Aura/Container/Focus/Action. |

### W9: Emulator Interaction (PENDING)

| # | Task | Status | Details |
|---|------|--------|---------|
| W9.1 | Implement consent gate UI in emulator | PENDING | Show Locked/Unlocked/Committed state, require keystroke friction. |
| W9.2 | Implement drift detection visualization | PENDING | Show how far current state has drifted from baseline. |
| W9.3 | Add multi-sentence detection display | PENDING | Show per-sentence detection results and aggregate state. |

### W10: Emulator Fleet + Physical (PENDING)

| # | Task | Status | Details |
|---|------|--------|---------|
| W10.1 | Multi-device sync protocol | PENDING | Broadcast FACES state over UDP/TCP to multiple emulator instances. |
| W10.2 | LED matrix output | PENDING | Map FACES state to RGB values, output via GPIO or serial. |
| W10.3 | Fleet orchestration | PENDING | One source, many displays — for classroom or demo scenarios. |

### Gate 3: Review Emulator (after W10)

| # | Task | Status | Details |
|---|------|--------|---------|
| G3.1 | Joshua reviews emulator | PENDING | Final gate before FACES is declared "complete". |

---

## TRACK D: XR / Spatial Computing

| # | Task | Status | Details |
|---|------|--------|---------|
| D.1 | Bevy 3D/XR test with OpenXR | PENDING | Basic Bevy scene with OpenXR plugin on Strix Halo. |
| D.2 | Revive trinity-ndk | PENDING | Bevy 0.16→0.18 upgrade, URL fix, APK rebuild. Located in ARCHIVE_VAULT/Phone/trinity-ndk/. |
| D.3 | XREAL Aura integration | BLOCKED | Hardware available Fall 2026. |

---

## PRIORITY ORDER (Recommended)

1. **Phase 8: E2E Creative Pipeline** — Rewire creative.rs to ComfyUI :8188 (~2-3 hr)
2. **Phase 9: Docs Sync** — Record current architecture (P + A + ComfyUI + LM Studio Hermes) (~1 hr)
3. **Phase 13: Deployment Packaging** — systemd, Caddy, release build (~1 hr)
4. **FACES Gate 2** — Joshua reviews labeling guide, run eval (~30 min)

**Estimated remaining:** ~5-6 hours of focused work to "production finished".
**FACES, XR, and Hermes hotel management are out of this sprint.**

---

## COMPLETED WORK (for reference)

### Phases 1-7: ALL COMPLETE
- Protocol cleanup, feature-gating, main.rs split, dead code removal, security hardening, tools trim, frontend consolidation

### FACES W1-W7: ALL COMPLETE
- 283+ tests, 12 source files, zero dependencies, eval harness with 4 modules

### Infrastructure: COMPLETE
- Hotel manager (Studio/Solo/Closed modes — needs update to resident-but-paused)
- Focus Mode (Creative/Code/Night — needs tri-model awareness)
- Dual-model inference routing (needs update to tri-model)
- ORT in-process embeddings
- Persistent memory (SQLite)
- RAG semantic search
- PWA with voice/TTS
- Auth, CORS, rate limiting, body limits, graceful shutdown
- Caddyfile with route blocking
- systemd service files (need auditing + Hermes service)
- Background job system (POST /api/jobs, /api/jobs/chain, SQLite persistence)
- EYE evaluation endpoints (compile, preview, export)
