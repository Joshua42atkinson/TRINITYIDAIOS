# Trinity ID AI OS — Research Bible & Session Context
## April 5, 2026 — Ascension Architecture (Testing & Integration phase)
* **Latest Milestone:** Completed the integration of the Ascension Architecture (Ghost Train and Gemini Protocol). Re-established the core UI logic around Free Will, ensuring Pete scaffolds without forcing software lockouts, and validated these pedagogical mechanics directly into the `agent.rs` workflow.
* **Next Session Goal:** Testing and Integration — ensuring the Iron Road systems are stable and producing verification videos demonstrating how Trinity prepares students for graduation.

---

> **CRITICAL NOMENCLATURE ANCHOR & SYSTEM DIRECTIVE FOR ALL AI AGENTS:**
> The capitalization and spelling of the core pedagogical frameworks must NEVER drift. We use words as systems management:
> 1. **ADDIECRAPEYE**: *Always* fully capitalized, exactly as written. It is the 12-station instructional design lifecycle. Do not use variations.
> 2. **PEARL**: *Always* fully capitalized (Perspective, Engineering, Aesthetic, Research, Layout). It is the pedagogical focusing lens. Do not use variations.

## 1. WHAT TRINITY IS

Trinity ID AI OS is a gamified instructional design system that helps K-12 teachers build educational games. It uses AI agents orchestrated through ADDIECRAPEYE to autonomously create games, lesson plans, and educational media.

**TRINITY = ID + AI + OS:**
- **ID = ADDIE** — Instructional Design backbone
- **AI = CRAP** — Visual Design principles
- **OS = EYE** — Observer/Metacognition 

**THE TRINITY DELIVERABLES & WEB UI MAPPING (LEARNING + FUN + WORK):**
- **LEARNING (The IRON ROAD Tab)**: Structured application over rote memorization, built directly into the dev process and demonstrated in the portfolio. The page opens to `IRON ROAD`.
- **FUN (The ART Tab)**: The full edutainment "lite novel" **DAYDREAM** in the Bevy window. It represents the gamified adventure of the development process. The page opens to `ART Aesthetic Research Temp`.
- **WORK (The YARDMASTER Tab)**: The actual product you are on the Iron Road to build. The page opens to `Yard IDE`.

**THE THREE UX SYSTEMS:**
- **AUDIO** — Telephone line / Voice interfaces.
- **WEB** — React frontend (Iron Road / ART page / About).
- **BEVY** — **DAYDREAM** (The immersive 3D native Bevy engine, spawned as an OS sidecar process. Pure Rust. No JavaScript in the DAYDREAM engine).

**The P-ART-Y Framework (Who operates Trinity):**
- **P = Pete** — The ONLY AI personality (Mistral Small 4 119B)
- **A = Aesthetics** — CRAP visual design, ComfyUI assets 
- **R = Research** — QM audits, tests, CI/CD
- **T = Tempo** — Code gen, Bevy scaffolding
- **Y = You** — The Yardmaster. Executive core.

---

## 2. THE "MEANING MAKING" TRACE (Isomorphic Alignment)

1. **AI Attention**: Sacred Circuitry (15 nodes)
2. **User Preference**: VAAM Bridge (Profile + Word Weights)
3. **Methodology**: ADDIECRAPEYE (12 stations) lifecycle
4. **Identity**: Character Sheet (Moved to "About" — directs the portfolio build, the project, and the actual user profile. Acts as the permanent "agreement between the computer and the user")
5. **Academic Progress**: LDT Portfolio (12 portfolio artifacts mapped to ADDIECRAPEYE phases, QM scoring)

**Functional Flow:**
User Message → VAAM Bridge → Pete Orchestration → Quest Objective Complete → Station Advance → **Portfolio Artifact Vaulted** → QM/Competency Recalculation → **Character Sheet Updated**.

---

## 3. RUNTIME ARCHITECTURE (Agnostic HTTP Inference)

> **ARCHITECTURE CHANGE (March 30–31, 2026):** Trinity now uses an agnostic HTTP InferenceRouter
> that dispatches to any OpenAI-compatible backend. LM Studio is the primary backend.
> Embedded `llama-cpp-2` has been fully archived. The system is a lightweight Rust binary
> that connects to whichever backend the user launches on their machine.

- **LM Studio (PRIMARY, :1234)**:
  - Mistral Small 4 119B Q4_K_M.
  - 2M+ token dual context window, Q8_0 KV cache quantization.
  - System-prompt persona differentiation (no `id_slot` required).
  - Flash Attention enabled.
- **HTTP Fallback (AUTO-DETECT)**:
  - llama-server (:8080), Ollama (:11434), any OpenAI-compat (configurable).
  - Auto-detected by InferenceRouter when primary is offline.
- **Voice (Embedded ORT)**: Supertonic-2 TTS (~280MB) & Whisper Base STT (~278MB).
- **Native RAG (Embedded ORT)**: all-MiniLM-L6-v2 embeddings via `ort` crate, cosine similarity in Rust.
- **ComfyUI SDXL Turbo (:8188)**: Image generation (HTTP sidecar).
- **MCP Server**: `trinity-mcp-server` stdio bridge for IDE integration (Zed, Cursor).
- **Background Jobs**: SQLite-persisted async task queue for overnight autonomous generation.

**Boot Order:**
1. Server starts on :3000 (instant)
2. TTS/STT ONNX models load on CPU (~2s)
3. ComfyUI probed (HTTP, separate process)
4. InferenceRouter checks LM Studio → llama-server → Ollama (auto-detect)

---

## 4. CURRENT SYSTEM STATE (Purdue Presentation Ready)
- **Agnostic HTTP Inference**: LIVE. LM Studio serves Mistral Small 4 119B via HTTP; Trinity dispatches via InferenceRouter.
- **BYOP Architecture**: LIVE. Users can "Bring Your Own Pipeline" — any OpenAI-compatible backend works.
- **Startup Connection Handshake**: LIVE. `/api/config/setup` actively pings external LLMs (LM Studio/Ollama) with a 3-second timeout to prevent UI load on dead connections.
- **The Forge 3D WASM Studio**: `Archived` — replaced by DAYDREAM (native Bevy sidecar, no WASM).
- **Code Textbook Autopoiesis (Phase 8)**: LIVE.
- **UI Deliverables Triad**: LIVE. Work (Yardmaster IDE), Fun (Art Studio/DAYDREAM), Learning (Iron Road).
- **Art Studio (Conversational Media)**: LIVE. Developer telemetry stripped in favor of a purely conversational interface with a dynamic Virtual File System (VAAM) for navigating assets.
- **Yardmaster (Root Dev Zone)**: LIVE. The beastlogger has been transformed into a massive, centralized media terminal, chronologically merging ComfyUI, System, and Forge Agent traces.
- **Portfolio & Character Sheet Unification**: LIVE.
- **Edge Guard Security**: LIVE.
- **Setup Wizard Resilience**: LIVE. Automatically skips setup based on `localStorage` to avoid looping lockouts when the VRAM/backend daemon is offline.
- **Dynamic Ignition Button**: LIVE. Extracted backend port parameters directly from `localStorage`, removing hardcoded `.sh` logic.
- **Portfolio Graceful Degradation**: LIVE. The public `LDTAtkinson` Portfolio dynamically catches SSE socket drops from the Rust inference pipe and renders "🔌 [SYS_ERR] The Trinity Engine is offline" to visitors beautifully.
- **Sidecar Vaulting**: LIVE.

---

## 5. DEFERRED TASKS (Future Action Queue)

### Priority Tasks (Next Session Focus)
- [ ] **Testing and Integration**: Run end-to-end tests for the Ascension Architecture integration and prepare to record demonstration videos of Trinity's pedagogical efficacy.
- [ ] **Demonstrate Graduation**: Verify that Trinity can effectively teach the "user/player" how to graduate from the IRON ROAD phase through actual mechanics.
- [ ] **ComfyUI / ORT Permanent Integration**: Ensure offline image generation operations work effectively.
- [ ] **Audio Conversation Pipeline**: Ensure Supertonic TTS + Whisper STT are functioning for bidirectional real-time audio.

### Quick Wins (< 30 min each)
- [x] **Express Mode Button Labels**: 🚂 ⚡ 🔧 buttons now show text labels (Iron Road, Express, Workshop).
- [x] **Bible vLLM Cleanup**: Remove all vLLM references from TRINITY_FANCY_BIBLE.md.
- [x] **HOOK_BOOK vLLM Cleanup**: Remove vLLM references from HOOK_BOOK.md.
- [x] **PROFESSOR.md Cleanup**: Remove vLLM references.
- [ ] **docs/ Sweep**: Clean remaining vLLM references in docs/ subdirectories.
- [ ] **Copywriting Summary**: Create a 2-page executive overview distilled from the Bible.
- [ ] **inference.rs Comments**: Update file header comments to reflect agnostic architecture.
- [x] **LDTAtkinson vLLM References**: Clean vLLM mentions from portfolio site source.
- [x] **All 28 Game Mechanics Wired**: R1-R7 (Coal/Steam, Scope Creep, Friction, Vulnerability, Shadow, Per-Phase Objectives, Perspective) fully connected in agent.rs pipeline.

### Medium Tasks (1-2 hours)
- [ ] **AppImage Packaging**: Update build-appimage.sh to bundle trinity binary + frontend, add first-run model wizard.
- [ ] **HuggingFace Downloader**: POST /api/model/download + SSE progress stream + first-run UI.
- [ ] **Recommended Models List**: Curated list of GGUF models with hardware requirements.
- [ ] **Purdue Student IP Office Submission**: Package documentation, source, and demo for formal submission.

### Archive Reference
- vLLM scripts → `archive/vllm-scripts/`
- Old maturation maps → `archive/maturation-maps-march-2026/`
- vLLM profiling crash issue documented in archive

---

## 6. vLLM REMOVAL — COMPLETE (March 28, 2026)

All vLLM code, configuration, and service files have been removed from active source.
- Zero vLLM references in any Rust source file (verified April 3, 2026)
- All PostgreSQL-specific SQL (NOW(), SERIAL, JSONB, TIMESTAMPTZ, ::TEXT) remediated to SQLite-native syntax
- Config updated: InferenceRouter auto-detects LM Studio (:1234) → llama-server (:8080) → Ollama (:11434)
- `embedded_inference.rs` archived, `llama-cpp-2` build dependency stripped
- 13 vLLM files archived to `archive/vllm-scripts/`
- **294/294 tests passing, 0 failures** (verified April 3, 2026)

---

## 7. ELECTRON APPIMAGE LESSONS LEARNED (March 30, 2026)

**DO NOT ATTEMPT TO DAEMONIZE THE LM STUDIO APPIMAGE.**

1. **Electron vs. Tauri Backends**: LM Studio is built in Electron. Electron apps bundle Chromium and fundamentally expect graphical (X11/Wayland) environments and valid standard file descriptors. If spawned headlessly (via `tokio::process::Command` with `Stdio::null()`), Electron silently crashes. Trinity (being a Rust/Axum + React stack) is aligned with the Tauri ecosystem, which is infinitely superior at true headless background operations.
2. **AppImages & FUSE Mounts**: When the Electron app crashed, its FUSE `/tmp` mount held a PID lock file. Subsequent ignition attempts launched new AppImages that saw the orphaned lock and immediately quit with the error: "Another instance of the app is already running."
3. **The `lms CLI` is a thin client**: `lms daemon status` and `lms server start` are completely incapable of managing models internally; they are merely IPC clients that talk to the already-running LM Studio GUI desktop application.

**Architecture Policy**: 
If Trinity detects `localhost:1234` is offline, it must trigger LM Studio to launch *visibly in the foreground* on the user's desktop, allowing it to boot safely. Only after the graphical application has stabilized will Trinity orchestrate the `lms` commands to start the server and load Mistral. All internal attempts to "hide" or "silence" the AppImage boot are forbidden.

---

## 8. SESSION NOTES: IDE vs AGENTIC OS (March 31, 2026)

**CRITICAL OBSERVATION: ON vs WITH**
The distinction between an IDE-embedded AI (Antigravity/Cursor) and a Local Agentic System (Trinity) is absolute:
- **IDE AIs work ON the system**: They edit text files, run sandboxed shell commands, and read local context, but they are intrinsically separated from the internal memory and graphics pipe of the engine. They cannot run sub-agents natively inside your 3D world.
- **Trinity works WITH the system**: As a local native OS running inside the Bevy pipeline, Trinity has direct semantic access to the MCP (`trinity-mcp-server`), 3D spatial data (`trinity-daydream`), and the pedagogical schema (`ADDIECRAPEYE`).

**Today's Fixes & Upgrades:**
1. **`bevy_egui` 0.33 Panic Resolved**: Fixed a fatal crash in `trinity-daydream/src/hud.rs` where accessing `ctx.style()` prior to the frame render pass caused a core dump (`Called available_rect() before Context::run()`). Trinity is now stable and boots cleanly into the DAYDREAM frontend.
2. **`scaffold_elearning_module` Macro**: Added a dedicated Rust automation tool to `crates/trinity/src/tools.rs` that allows Trinity to bypass context-exhausting `shell` chains and autonomously scaffold a full Vite + React project natively via `tokio::process::Command`.
3. **Transparent Reasoning**: Upgraded the `AGENT_SYSTEM` prompt to strictly enforce Mistral reporting its internal scratchpad via `<thinking>...</thinking>` tags before emitting UI badges, completely resolving the "Blind Planner" UI hang.

**IDE Migration Note**: Because the user requires full structural integration with the MCP Server to maintain workflow, the transition to **Zed IDE** or **Cursor** is validated. Antigravity fundamentally lacks the auto-mounting UI for external contextual MCP pipelines, meaning the AI embedded here will always operate one layer removed from Trinity's active memory pool.
