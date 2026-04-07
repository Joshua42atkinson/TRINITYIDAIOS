# Trinity ID AI OS — Research Bible & Session Context
## April 5, 2026 — One Big Brain Architecture (Committed)
* **Architecture Decision:** Committed to a single-LLM design. Gemma-4 31B Dense AWQ is the sole reasoning engine. Pete, the Great Recycler, and Tempo are **system prompt personas**, not separate models. This is simpler, higher quality (31B always-active params vs 4B in MoE), and uses only ~18GB of 128GB unified memory.
* **Voice Pipeline:** Switched from Voxtral (CC BY-NC) to **Kokoro TTS** (Apache 2.0) — stress-free licensing for everyone. 6 voices, low-latency, port 8200.
* **ASR Pipeline (Phase 2):** Gemma-4 E2B will serve as the audio listener for hands-free operation — the "ears" for accessibility.
* **All 166 tests passing, 0 failures.** Code is demo-ready.

---

> **CRITICAL NOMENCLATURE ANCHOR & SYSTEM DIRECTIVE FOR ALL AI AGENTS:**
> The capitalization and spelling of the core pedagogical frameworks must NEVER drift. We use words as systems management:
> 1. **ADDIECRAPEYE**: *Always* fully capitalized, exactly as written. It is the 12-station instructional design lifecycle. Do not use variations.
> 2. **PEARL**: *Always* fully capitalized (Perspective, Engineering, Aesthetic, Research, Layout). It is the pedagogical focusing lens. Do not use variations.
> 
> **CRITICAL PROGRESSIVE MIGRATION POLICY:**
> **DO NOT MASS-DELETE THE REACT (JS/JSX) FRONTEND JUMPING TO PURE RUST.** We are moving away from JS toward Python/PyO3 and Rust, but this must be done SLOWLY and INTENTIONALLY through sidecars to isolate the UI, preserving the pure Rust backend over time. Replace individual components systematically and only with user consent. "Pure Rust pivots" that delete `LDTAtkinson/client` or the functional Web UI are strictly forbidden.

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
- **AUDIO** — Kokoro TTS (speech output) + Gemma-4 E2B (speech input / ASR).
- **WEB** — React frontend (Iron Road / ART page / About).
- **BEVY** — **DAYDREAM** (The immersive 3D native Bevy engine, spawned as an OS sidecar process. Pure Rust. No JavaScript in the DAYDREAM engine).

**The P-ART-Y Framework (Who operates Trinity):**
- **P = Pete** — The ONLY AI personality (Gemma-4 31B Dense, system-prompt persona)
- **A = Aesthetics** — CRAP visual design, image generation
- **R = Research** — QM audits, tests, CI/CD
- **T = Tempo** — Narrative pacing, audio pipeline conductor
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

## 3. RUNTIME ARCHITECTURE (One Big Brain + Kokoro Voice)

> **ARCHITECTURE DECISION (April 5, 2026):** Trinity uses ONE LLM (Gemma-4 31B Dense AWQ)
> for all reasoning. Pete and the Great Recycler are **system prompt personas**, not separate models.
> The InferenceRouter dispatches to whichever backend is healthy (auto-detect).
> Voice output via Kokoro TTS (Apache 2.0). Voice input via Gemma-4 E2B (Phase 2).

### The Committed Model Stack

| Service | Port | Model | Memory | License | Role |
|---------|------|-------|--------|---------|------|
| **Great Recycler** | 8001 | Gemma-4 31B Dense AWQ | ~18GB | Apache 2.0 | THE brain — reasoning, Pete, Recycler, Tempo via system prompt |
| **Kokoro TTS** | 8200 | Kokoro (kokoro_sidecar.py) | ~1GB | Apache 2.0 | THE voice — 6 presets, emotion-aware narration |
| **nomic-embed** | 8005 | nomic-embed-text-v1.5 | ~1GB | Apache 2.0 | Embeddings for RAG |
| **E2B Listener** | 8003 | Gemma-4 E2B (Phase 2) | ~3GB | Apache 2.0 | THE ears — speech-to-text for hands-free |

**Total Phase 1: ~20GB of 128GB unified memory. 108GB left for you.**

### How Pete & Recycler Share One Brain (KV Cache)

The 31B model runs as a single vLLM process. Each chat request gets its own **KV cache allocation** within the shared pool. Pete and the Great Recycler never interfere because:

1. **Separate requests = separate KV entries.** When Pete answers a Socratic question, that request has its own KV state. When the Recycler narrates, that's a different request with its own KV state. They don't share conversation memory unless the code explicitly merges their histories.

2. **System prompts are the personality switch.** The `conductor_leader.rs` module selects the right system prompt based on the active ADDIECRAPEYE phase:
   - **Analysis/Design phases** → Pete's warm Socratic prompt
   - **Evaluation/Envision phases** → Recycler's authoritative DM prompt
   - **DM break-character** → `[DM]` tag triggers `NarratorMode::OutOfCharacter`

3. **vLLM prefix caching** helps here — if Pete's system prompt prefix is reused across multiple requests, vLLM caches those KV entries automatically, making repeat calls faster.

4. **32K context per request** is plenty for either persona. Pete rarely needs more than a few thousand tokens of conversation. The Recycler's narration chunks are even shorter.

### Voice Persona Mapping (Kokoro)

| Trinity Persona | Kokoro Voice | Emotion System |
|----------------|-------------|----------------|
| Pete (Conductor) | am_adam | Warm, contemplative, celebratory |
| Great Recycler (DM) | am_fenrir | Authoritative, sarcastic, urgent |
| NPC | am_echo | Neutral |
| Student/Youser | af_heart | Encouraging |

### Boot Order (Phase 1)
```bash
# 1. Start the brain (in vllm distrobox)
distrobox enter vllm -- vllm serve ~/trinity-models/vllm/gemma-4-31B-it-AWQ-4bit \
  --port 8001 --gpu-memory-utilization 0.35 --max-model-len 32768 \
  --dtype half --served-model-name "Great_Recycler"

# 2. Start the voice (Kokoro TTS, Apache 2.0)
source ~/trinity-vllm-env/bin/activate
python3 scripts/launch/kokoro_sidecar.py  # port 8200

# 3. Start Trinity server
cargo run --release --bin trinity  # port 3000

# 4. Start React frontend
cd LDTAtkinson/client && npm run dev  # port 5173
```

### Boot Order (Phase 2 — Hands-Free)
```bash
# Add the ears (Gemma-4 E2B for ASR)
distrobox enter vllm -- vllm serve ~/trinity-models/vllm/gemma-4-E2B-it \
  --port 8003 --gpu-memory-utilization 0.05 --max-model-len 4096 \
  --dtype half --served-model-name "Tempo_Listener"
```
**Flow:** 🎤 Mic → E2B (ASR, port 8003) → 31B (Think, port 8001) → Kokoro (Speak, port 8200) → 🔊

---

## 4. CURRENT SYSTEM STATE (April 5, 2026)
- **One Big Brain Architecture**: COMMITTED. Gemma-4 31B Dense AWQ as sole reasoning engine.
- **Kokoro TTS (Apache 2.0)**: WIRED. voice.rs fully integrated, 6 voices, emotion detection, DM break-character.
- **InferenceRouter**: LIVE. Auto-detects ports 8001 (Recycler), 8002 (Pete), 8003 (Tempo), 8000 (proxy), 8080 (llama-server), 1234 (LM Studio), 11434 (Ollama).
- **166 Tests Passing**: 0 failures (verified April 5, 2026).
- **detect_emotion() Restored**: Powers emotional narrator pacing for hands-free voice loop.
- **BYOP Architecture**: LIVE. Users can "Bring Your Own Pipeline" — any OpenAI-compatible backend works.
- **UI Deliverables Triad**: LIVE. Work (Yardmaster IDE), Fun (Art Studio/DAYDREAM), Learning (Iron Road).
- **All 28 Game Mechanics Wired**: Coal/Steam, Scope Creep, Friction, Vulnerability, Shadow, Per-Phase Objectives, Perspective.
- **37 Agent Tools**: Full tool suite including Scout Sniper, Dynamic Vibe Orchestrator, session continuity.

---

## 5. DEFERRED TASKS (Future Action Queue)

### Priority Tasks (Next Session Focus)
- [ ] **Testing and Integration**: Run end-to-end tests for the Ascension Architecture integration and prepare to record demonstration videos of Trinity's pedagogical efficacy.
- [ ] **Demonstrate Graduation**: Verify that Trinity can effectively teach the "user/player" how to graduate from the IRON ROAD phase through actual mechanics.
- [ ] **vLLM Omni Permanent Integration**: Ensure offline image generation operations via Omni work effectively.
- [ ] **Audio Conversation Pipeline**: Ensure Kokoro TTS + Gemma-4 E2B STT are functioning for bidirectional real-time audio.

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
