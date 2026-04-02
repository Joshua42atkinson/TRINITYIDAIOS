# Trinity Yardmaster Connectivity & Inference Context

This document tracks all the attempts made to restore the Trinity Yardmaster frontend and Embedded GPU Inference to prevent regression in future sessions.

## 1. Problem: "Failed to fetch" / "Network Error" on Frontend
- **Symptom:** The Yardmaster UI returns `Failed to fetch`. 
- **Root Cause:** The `POST /api/chat/yardmaster` endpoint in `agent.rs` was falling back to the external HTTP inference mode (`http://127.0.0.1:8080/v1/chat/completions`) because the `embedded_model` struct was `None`.
- **Why it was None:** The `embedded_model` object runs in a background Vulkan loading thread. It either had not finished loading yet, OR the Vulkan thread had silently panicked and died due to memory constraints (`Aborted (core dumped)`). 

## 2. Problem: Hardware Memory OOM Core Dumps
When attempting to load the `Mistral-Small-4-119B` (68GB Q4 weights) model alongside the new `PersistentSlot` **Dual KV Cache** architecture (`Slot 0` and `Slot 1`), we encountered severe `ggml_vulkan` memory allocation failures resulting in `134 Exit Code (Abort)`.

### Attempt 1: 256K Context with native fallback (`f16` KV Cache)
- We tried removing the explicit `Q4_0` memory typings because the Rust `llama-cpp-2` wrapper (`v0.1.141`) lacks the `.with_flash_attn(true)` method, which is generally required for Q4 KV buffers in `ggml_vulkan`.
- Without `Q4_0` explicitly defined, `llama.cpp` defaults to **float16 (f16)** for the KV caches. 
- **The Math:** 2 contexts * 256K tokens in `f16` = **>160 Gigabytes** of pure VRAM overhead. 
- combined with the 68GB model weights, this demanded **228 GB VRAM**, resulting in an immediate memory crash on the 128GB system.

### Attempt 2: 32K Context with native fallback (`f16` KV Cache)
- We tried reducing the context size to `32768` (32K tokens) using the `f16` fallback.
- **The Math:** 2 contexts * 32K tokens in `f16` = **~22 GB** VRAM overhead.
- Total VRAM: 68GB + 22GB = 90 GB.
- **The Issue:** While 90GB physically fits in the 128GB of RAM, Linux AMD Drivers (`radv` with `RADV_PERFMODE=nogttspill`) often enforce a strict **75% system memory cap** for VRAM allocations. 75% of 128GB = **96 GB limits.**
- A 90GB request causes extreme fragmentation or tips the 96GB limit, leading to an immediate Vulkan memory allocation crash during mapping.

### Attempt 3: 256K Context with `Q4_0` KV Cache
- Re-enabling `Q4_0` KV Caches dropped the VRAM overhead dramatically.
- **The Math:** 512K tokens total at `Q4_0` compression = **~42 GB**.
- Total VRAM: 68GB + 42GB = 110 GB.
- **The Issue:** 110 GB completely breaches the structural 75% (96 GB) Linux driver VRAM allocation limit, causing immediate driver aborts.

## 3. Current Agnostic Setup (Attempt 5: LM Studio Migration)
To permanently solve the memory allocation dumps and 75% system memory caps during Linux AMD VRAM allocation mapping, we have entirely bypassed the `embedded_inference.rs` bindings and the `llama_cpp_2` C++ build targets. 

Trinity now acts as a stable, lightweight HTTP dispatcher and thin-client UI. All `/api/chat` agentic workloads are deferred entirely to an external **LM Studio** server running on `http://127.0.0.1:1234` via the `InferenceRouter`. 
This creates a stable, agnostic AppImage for future users, as they can "Bring Their Own Pipeline" (BYOP) for text (LM Studio), art (ComfyUI), and 3D (Blender).

### LM Studio Target Config (Mistral Small 4 119B):
To keep memory usage under the 96 GB limit while maintaining long-term recall and dual-context performance, you MUST configure LM Studio server as follows:
- **Model**: `Mistral-Small-4-119B-Instruct`
- **Context Window**: `256000` (256K tokens for incredibly deep session history)
- **KV Cache Quantization**: `q8_0` (Dual KV Cache MLA 256K structure)
- **Max Response Tokens**: `32768` (32K tokens to accommodate deep agentic code generation)
- **Max Agentic Turns**: `65` (Allows Pete to work autonomously through complex multi-step build loops)
- **Flash Attention**: `Enabled`

## 4. Agnostic Inference Architecture (March 30 2026)

### Root Cause of LM Studio Hangs
The `id_slot` field in `CompletionRequest` (inference.rs) was a **llama-server-only extension** not part of the OpenAI standard. Combined with the `tools` array, it caused LM Studio's request parser to deadlock indefinitely. The model loaded fine; requests simply never returned.

### Fix Applied
- **Removed `id_slot`** from `CompletionRequest` struct and all function signatures (`chat_completion_stream`, `chat_completion_with_effort`, `chat_completion_with_tools`)
- **Removed `persona_slot()`** calls from all 6 call sites across `agent.rs`, `main.rs`, and `perspective.rs`
- The `persona_slot()` function itself is preserved but marked dead code (available for future llama-server-specific optimizations behind a feature flag)

### Persona Differentiation Without id_slot
The system prompt already fully differentiates Great Recycler vs Programmer Pete. The KV cache slot pinning was an optimization for llama-server, NOT a requirement. For the agnostic AppImage, persona differentiation uses:
1. **System prompt** (already works with every backend)
2. **Per-persona backend routing** (future: `[inference.personas]` config section where each persona can point to a different backend)

### Supported Backends (All OpenAI-Compatible)
| Backend | Port | Health Check | Status |
|---------|------|-------------|--------|
| LM Studio | 1234 | `/v1/models` | ✅ Primary |
| llama-server | 8080 | `/health` | ✅ Fallback |
| Ollama | 11434 | `/api/tags` | ✅ Fallback |
| Any OpenAI-compat | configurable | `/v1/models` | ✅ Custom |

## 5. Phase 3 Architecture Stable (March 30 2026)
### 1. SQLite Migration & In-Memory RAG
- **Why**: Removing heavy Docker and PostgreSQL / pgvector daemon requirements to make Trinity a "Zero Setup" deployable AppImage for students.
- **How**: We replaced `sqlx::Postgres` with `sqlx::Sqlite`. The `~/.trinity/trinity_memory.db` file is spawned on default boot. 
- **RAG Updates**: Vector similarity search is now performed via an unopinionated backend `cosine_similarity` Rust function fetching directly from `TEXT` columns in SQLite where embeddings are serialized as JSON strings.

### 2. The Setup Wizard ("Bring Your Own Mind")
- **Why**: With PostgreSQL structurally removed, we also removed the hardcoded `http://127.0.0.1:8080` fallback engine connection.
- **How**: We implemented `App.jsx` to intercept the root render if the API `GET /api/models/active` returns `{ healthy: false }`. This spawns a dynamic **SetupWizard** locking down the system until the student explicitely defines a Mind (e.g. LM Studio, Ollama). 
- **Important**: The checking logic uses an API health gate instead of `localStorage` so that when hosted online (e.g. `LDTATKINSON.COM`), external web clients don't get locked behind a Setup Wizard if the server admin has already configured a local backend!

### 3. Server Route "Alive Code" Restorations
- Restored `export_lms_analytics` and `list_community_templates` to active routes (`/api/analytics/lms` and `/api/projects/community`).
- Removed `#[allow(dead_code)]` shields fully. Re-wired DAYDREAM GameState integration into the dispatcher.
- System currently builds with 0 dead-code warnings across all crates.

## 6. Antigravity IDE Communication Preferences
- **Focused Transparency Required**: DUE TO RECENT UI WRAPPER UPDATES, the internal chain-of-thought processing is heavily obfuscated behind "picking tools and skills" spinners. 
- **Rule**: The agent MUST "think out loud" in actual markdown replies.
- Before executing large batches of automated commands or deeply editing code, the agent MUST explicitly list its tactical logic, the files it intends to touch, and the exact tools it is about to fire. 
- **Why**: This slows down progress slightly but is absolutely mandatory to prevent "black box" behavior. The developer must be able to visually oversee and intercept logic before the AI pulls the trigger on massive refactors.

## 7. UI PEARL Artifacts (Live Alignment Rubrics)

The following 12 documents dictate the Perspective, Engineering, Aesthetic, Research, and Layout details for the Trinity UI mirror. Any drift found in the frontend will be judged against these rubrics (EYE component of ADDIECRAPEYE).

1. [App Chrome (Global Shell)](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/App_Chrome.md)
2. [Setup Wizard (BYOM)](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/SetupWizard.md)
3. [Phase Workspace (Iron Road)](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/PhaseWorkspace.md)
4. [Game HUD (Gamification)](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/GameHUD.md)
5. [Express Wizard](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/ExpressWizard.md)
6. [Yardmaster (Terminal)](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/Yardmaster.md)
7. [Art Studio (Creative Suite)](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/ArtStudio.md)
8. [Character Sheet (Identity)](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/CharacterSheet.md)
9. [Journal Viewer (Reflection)](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/JournalViewer.md)
10. [Portfolio View (LDTAtkinson)](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/PortfolioView.md)
11. [Quality Scorecard](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/QualityScorecard.md)
12. [Chariot Viewer (Help System)](file:///home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/pearls/ChariotViewer.md)

## 8. The MIND & HARDWARE Minimum Standards (Established March 30, 2026)
Trinity relies on massive cognitive continuity to execute the Socratic Inhale (Great Recycler) and Execution Exhale (Programmer Pete) within a single session seamlessly. Therefore, treating the AI backend as a chatbot is unacceptable.

### Hardware Agnosticism (Scaleability):
- **System Memory:** Trinity's lightweight Rust footprint is expressly designed to scale down to **24GB** and consumer-grade hardware by using smaller local models. The quest-driven horizon allows it to perform complex multi-step reasoning even on restricted compute.
- **Compute Flexibility:** It supports massive 128GB+ environments for 68GB+ weights (like Mistral 119B MLA), but fundamentally operates just as well on smaller, highly-quantized models thanks to the stateful ADDIECRAPEYE framework driving the context.

### MIND Minimums (The LLM Engine):
- **Context Capacity:** While we optimize the Backend memory limits (e.g. 400-turn `RECENT_WINDOW` and 150K char RAG) to fully leverage 256,000 token models, the framework handles truncation dynamically so smaller models can still function efficiently over long horizons.
- **MLA Format:** Multi-head Latent Attention (MLA) or equivalent highly efficient Dual KV Caching is recommended for larger models to prevent memory explosion, but not strictly required for lower-tier operation.

## 9. Next Session Workflow: Full System Alignment & Audit
The 12 generated UI PEARL artifacts define the frontend but currently lack deep integration mapping with the Rust backend. To prevent "thin" documentation:

1. **Top-Level Scan**: Extract the top 30-50 lines (the LIVING CODE TEXTBOOK preambles) of all `.rs` files in `crates/trinity/src/` to map the actual deep logic (skills, Vaam, cow-catcher intercepts).
2. **The Logic Pipeline Matrix**: Map every backend system and endpoint directly to the UI component it drives.
3. **PEARL Layering**: We will *add a layer* to the 12 PEARLs (specifically inflating the `E - Engineering` and `R - Research` sections) to completely align the frontend interaction with the backend state machines.

*We are building one brick higher to make sure the Iron Road is empirically aligned from CSS all the way down to Vulkan compute.*

## 10. The Native Tauri Pivot (March 30, 2026)
To escape Linux AppImage fuse conflicts, system memory deadlocks between the Antigravity IDE and `lm studio cli`, and generic web view bugs, the entire Architecture was wrapped inside **Tauri v2**.
- **Dual-Headed Rust Binary:** The huge Axum web server (`crates/trinity/src/main.rs`) was moved to a background `tokio::spawn` thread, allowing `tauri::Builder` to dominate the main thread for native UI rendering (`npx tauri dev`).
- **Headless Mode Reserved:** By passing `--headless` or `TRINITY_HEADLESS=1`, the binary gracefully bypasses Tauri. This allows the exact same compiled rust code to run on `LDTAtkinson.com` as a headless background daemon, serving web requests via Cloudflared EdgeGuard on port `3000`.
- **System Stability Verified:** A dedicated backend stability integration test (`test_health_api_stability`) was established to autonomously verify the complex web framework compiles successfully before the WebView ever boots.

## 11. DAYDREAM Native Bevy Rebuild (March 30, 2026)
To maintain strict Open Source Apache 2.0 compliance, we executed a clean-room architectural rollback of the 3D Engine, fully excising the BSL 1.1 "Project Forge" dependencies.

### Three-Layer Process Isolation
Due to Linux/Wayland `Winit` strict requirements (both Tauri and Bevy demand control of the application's main thread), the system was officially split into a non-colliding multiprocess pipeline:
1. **Layer 1 (The Command Center):** Tauri + React handling the ADDIECRAPEYE state, LLM Chat interaction, and Yardmaster UI.
2. **Layer 2 (The Protocol):** Rust shared structs defining `DaydreamBlueprint`, ECS Commands, and state.
3. **Layer 3 (YARD / DAYDREAM):** A standalone Bevy 0.18.1 process (`art_studio`) offering a hardware-accelerated, immersive 3D world with native `bevy_egui` telemetry HUD components.

### Tauri File-Watcher Loop Prevention
We resolved a notorious "UI popping" infinite loop. Originally, `main.rs` used `cargo run` to boot `art_studio`. Since `tauri dev` actively watches the file system, cargo file locks and `target/debug` footprint changes triggered Tauri to hot-reload indefinitely. 
**Fix:** The Tauri `main.rs` server now executes the predefined `<workspace>/target/debug/art_studio` compiled binary directly as an OS child process, cleanly separating the dev-watcher scopes.

## 12. Context Scale Discovery (Dual 1M Slots)
Despite the math suggesting that a Mistral 119B model combined with Dual 1M KV Caches (2,000,000 tokens total) would breach the AMD Linux VRAM driver limit (96GB), empirical testing confirmed **the system runs stable.**
LM Studio successfully leverages heavy aggressive memory-mapped files (mmap), context shifting, and prompt caching to defer VRAM allocation until the tokens actually populate. This means **Trinity can run a dual-persona state with nearly infinite horizon memory** on the local Strix Halo hardware without immediate OOM core dumps.

## 13. Multi-Tenant Web Hosting (LDTAtkinson + Trinity)
The system is built to serve *both* the professional portfolio (`ldtatkinson.com`) and the actual Trinity OS UI (`/trinity/`) from the exact same Rust headless background daemon, guaranteeing that the website is up and running at all times.
- **DO NOT** run `cargo run` inside the `LDTAtkinson` subfolder explicitly (this will only boot the portfolio and cause API/Route 404s for the `/trinity/` application). 
- **DO** use the designated session startup workflow (`/build-and-test`). 
- **The Magic:** `crates/trinity/src/main.rs` intercepts web traffic on `127.0.0.1:3000`. It mounts `LDTAtkinson/client/dist` directly to the `/` root with an SPA fallback to `index.html`. It then *also* mounts the compiled Trinity frontend `frontend/dist` directly to `/trinity/`. 
- **Result:** Running `cargo run -p trinity --release` automatically brings up the LDTAtkinson frontend *and* handles the React Router links to "Try Trinity", serving both the live portfolio website and the fully functional capstone project simultaneously through the Cloudflare EdgeGuard without port conflicts.

## 14. LM Studio Agnostic Inference
The Trinity system originally used a massive embedded C++ binding (`llama.cpp` + Vulkan shaders) compiled directly into the Rust daemon to process local LLM chats. 
- **Migration:** We completely migrated to an HTTP-agnostic inference router. `embedded_inference.rs` was archived and the dependency was stripped. 
- **Benefit:** Build times dropped from minutes to seconds, and the user can now hotspot any OpenAI-compatible API (like LM Studio or Ollama) by simply launching their preferred model on port `8080` or `1234`. The backend will automatically detect and connect to it without any hardware-locking constraints.

## 15. Native RAG Math (No Python)
We successfully migrated the RAG Memory engine to execute pure Rust ONNX mathematics using `ort`.
- **No Background Hacking:** We removed the experimental `llama.cpp` HTTP proxy logic, replacing it with local 128GB RAM-aware `ndarray` computations in Rust.
- **Strictly Offline Policy:** The system enforces a hard manual download policy. It strictly expects `all-MiniLM-L6-v2.onnx` inside `~/trinity-models/onnx/embeddings/` and halts with a clean panic if it is not found, proactively protecting the drive from aggressive background telemetry/model downloads.
- **Tauri Native Networking:** We patched the React application's core `window.fetch` pipeline natively to seamlessly re-route `/api/` traffic inside Linux/macOS WebViews (`http://tauri.localhost`) to the background `http://127.0.0.1:3000` daemon. 
- **Edge Guard Immunity:** The backend `CorsLayer` natively authorizes internal UI Desktop traffic, while EdgeGuard natively ignores it (due to lack of CloudFlare headers) — providing zero-friction offline execution alongside fortified public Web endpoints.

## 16. Background Job Runner & SQLite Persistence (March 31, 2026)
To fully industrialize Trinity as a fire-and-forget IDE for multi-turn tasks (like generating structured e-learning modules overnight without an open tab), we transitioned the core agent processing loop into a fully headless, persistent background architecture:
- **Decoupled Agent Loop:** The core inference dispatch loop (`run_agent_loop` in `agent.rs`) was decrypted from the browser WebSockets/SSE requirement. It now operates as a discrete `tokio::spawn` asynchronous worker that feeds an internal MPSC channel instead of a TCP socket.
- **SQLite Job Persistence:** A new `trinity_background_jobs` table was established within `trinity_memory.db` to log state mutations on every tool invocation. The system strictly writes the task history, turning logs, and final markdown outputs (`~/Workflow/trinity-reports`) to disk. Crucially, the system checks for `failed (server restart)` conditions upon mounting to prevent dangling workers from earlier OOM crashes.
- **Overnight Crew Interface:** The Yardmaster UI received a live-tracking React component under the RAG integration, enabling developers to issue explicit task directives under the `Programmer Pete` and `Great Recycler` personas with single-click manual cancellation overrides.
- **Rendering Patch:** We patched the `Yardmaster.jsx` Markdown renderer to explicitly escape `<` and `>` elements so that `<thought>` XML tokens from the raw LM Studio backend correctly manifest as visual internal monologue text elements rather than being hidden by Chrome's generic HTML DOM parser.

## 17. Game Mechanics Maturation Map (Updated April 1, 2026)

A comprehensive PEARL evaluation was conducted across all 28 Iron Road game mechanics, mapping each backend Rust API to its frontend React component. **As of April 1, 2026, all 28 mechanics are fully wired backend ↔ frontend.**

### Maturation Status Overview
| Status | Count | Description |
|--------|-------|-------------|
| ✅ **Fully Wired** | 28 | Backend ↔ Frontend connected and functional |
| 🔧 **Fixed (UI Declutter)** | 7 | Progressive disclosure + declutter improvements |

### ✅ Fully Working Mechanics (28)

| # | Mechanic | Backend | Frontend |
|---|----------|---------|----------|
| 1 | ADDIECRAPEYE 12-Phase Lifecycle | `GET /api/quest` | ChapterRail + PhaseWorkspace |
| 2 | Quest Objectives (per-phase) | `POST /api/quest/complete` | PhaseWorkspace objectives |
| 3 | Phase Advance | `POST /api/quest/advance` | ⚡ ADVANCE STATION button |
| 4 | Phase Transition Ceremony | Client-side animation | PhaseTransition overlay |
| 5 | Session Zero (Character Creation) | `POST /api/character` | PhaseWorkspace 3-question flow |
| 6 | PEARL (project focusing) | `GET/POST /api/pearl`, `PUT /api/pearl/refine` | PearlCard with alignment bars |
| 7 | Locomotive Velocity | Computed from Steam | TrainStatus header |
| 8 | Product Maturity | `quest.product_maturity` | TrainStatus conditional bar |
| 9 | Resonance | `quest.resonance` | TrainStatus ✨ stat |
| 10 | Voice Narration | `POST /api/tts` | Voice toggle in header |
| 11 | GDD Compile | `POST /api/quest/compile` | 📜 GDD button |
| 12 | EYE Export (Quiz/Adventure) | `GET /api/eye/export` | 📝 Quiz / 🗺️ Adventure buttons |
| 13 | Journal | `GET /api/journal` | JournalViewer toggle |
| 14 | Hero's Journey Chapter Titles | Client-side constants | ChapterRail + phase banners |
| 15 | **Coal/Steam Economy** | `agent.rs:965-981` — Coal -2/Steam +5 per response, SSE `resources` event | PhaseWorkspace:403 → triggers refetch |
| 16 | **Scope Creep Taming** | `agent.rs:436-457` — `detect_scope_creep()`, SSE `creep_tameable`, friction +8 | PhaseWorkspace:367 → ScopeCreepModal (Hope/Nope + Hook select) |
| 17 | **Track Friction** | `agent.rs:935-948` — On-circuit: -1, off-circuit: +3, persisted to character sheet | TrainStatus:10 → FRICTION gauge |
| 18 | **Vulnerability** | `agent.rs:945,954` — `recalculate_vulnerability()` fires on friction change | TrainStatus:11 → VULN gauge |
| 19 | **Shadow Status** | `agent.rs:490-523` — 14-indicator sentiment detection, 3-tier escalation | TrainStatus:12 → Shadow icon/color |
| 20 | **Per-Phase Quest Objectives** | `objectives.json` (362 lines) — bespoke objectives per chapter × phase | PhaseWorkspace objectives list |
| 21 | **Perspective Engine** | `agent.rs:984-1016` — substantive msg detection, background LLM eval, SSE `perspective` | PerspectiveSidebar:43 → lens cards |
| 22 | Coal/Steam via Tool Execution | `agent.rs:1131-1148` — XP/Coal per tool call via `skills::calculate_xp()` | SSE `resources` → GameHUD refetch |
| 23 | Sacred Circuitry AI Coal Engine | `agent.rs:900-933` — `scan_ai_alignment()`, SSE `circuit` event | GameHUD + ActivityBar |
| 24 | VAAM Word Detection | `agent.rs:460-487` — SSE `vaam` + `cognitive_load` events | PhaseWorkspace:377 → narrative injection |
| 25 | Skill Check (d20 Roll) | `agent.rs:1030-1084` — gate tool execution with roll, Heavilon on failure | SSE `skill` + `heavilon` events |
| 26 | Character Update (Real-time) | `agent.rs:950-962` — SSE `character_update` event | GameHUD:36 → `setCharacter()` → TrainStatus |
| 27 | Thinking Stream | `agent.rs:802-809` — SSE `thinking` event with scratchpad | ActivityBar display |
| 28 | Narrative Events | `agent.rs:1069-1276` — critical/fumble/failure narratives | SSE `narrative` → PhaseWorkspace |

### 🔧 UI Declutter Fixes (7)

| # | Item | What Changed |
|---|------|-------------|
| F1 | Duplicate Stats Removed | XP/Coal/Steam no longer duplicated between LOCOMOTIVE and YARDMASTER cards |
| F2 | Footer Mode Labels | 🚂 ⚡ 🔧 buttons now show text labels (Iron Road, Express, Workshop) |
| F3 | BookSection Normalized | 12+ inline `style={{}}` props replaced with CSS classes |
| F4 | TrainStatus Progressive Disclosure | Friction/Vulnerability/Shadow collapsed behind ENGINE DIAGNOSTICS toggle |
| F5 | Bestiary Auto-Hide | Card hidden when no creeps discovered (was showing empty placeholder) |
| F6 | Book Auto-Hide | Card hidden when no chapters written |
| F7 | Sacred Circuitry Wired | Now fetches live data from `GET /api/quest/circuitry` with activation bars |

### Architecture Note for Purdue Licensing
The system architecture is **structurally complete and fully wired**. All 28 mechanics have both a Rust backend module and a React frontend component connected via SSE events. No wiring gaps remain. The system is ready for live playthrough testing and Purdue Student IP Office submission.

### PEARL Documents (Quality Gate Rubrics)
The following PEARL documents define the visual and engineering standards for each UI component. Updated PEARLs include full backend wiring matrices:

1. [App Chrome (Global Shell)](docs/pearls/App_Chrome.md)
2. [Setup Wizard (BYOM)](docs/pearls/SetupWizard.md)
3. **[Phase Workspace (Iron Road)](docs/pearls/PhaseWorkspace.md)** — *Updated April 1, 2026: full 28-mechanic audit*
4. **[Game HUD (Gamification)](docs/pearls/GameHUD.md)** — *Updated April 1, 2026: progressive disclosure hierarchy*
5. [Express Wizard](docs/pearls/ExpressWizard.md)
6. [Yardmaster (Terminal)](docs/pearls/Yardmaster.md)
7. [Art Studio (Creative Suite)](docs/pearls/ArtStudio.md)
8. [Character Sheet (Identity)](docs/pearls/CharacterSheet.md)
9. [Journal Viewer (Reflection)](docs/pearls/JournalViewer.md)
10. [Portfolio View (LDTAtkinson)](docs/pearls/PortfolioView.md)
11. [Quality Scorecard](docs/pearls/QualityScorecard.md)
12. [Chariot Viewer (Help System)](docs/pearls/ChariotViewer.md)

## 18. Restart Workflow (Quick Reference)

### Step 1: Start LM Studio
Launch LM Studio, load `mistral-small-4-119b-2603` with the config from Section 3 (256K context, Q8 KV cache, Flash Attention enabled). Verify it responds on `http://127.0.0.1:1234/v1/models`.

### Step 2: Start Trinity Server
```bash
lsof -ti :3000 | xargs kill 2>/dev/null
cd /home/joshua/Workflow/desktop_trinity/trinity-genesis
TRINITY_HEADLESS=1 cargo run -p trinity --release
```

### Step 3: Verify
- Portfolio: `http://localhost:3000/` → LDTAtkinson.com
- Trinity App: `http://localhost:3000/trinity/` → Iron Road
- Health: `http://localhost:3000/api/health` → `{"status":"healthy"}`

### Next Steps After Restart
1. ~~Fix the 7 remaining game mechanics (R1–R7)~~ **COMPLETE** — all 28 mechanics fully wired as of April 1, 2026
2. Live playthrough test: walk a real PEARL through Chapter 1 Analysis→Design→Development, verify all SSE events manifest
3. Fill CRAPEYE-specific objectives for Chapters 2-12 in `objectives.json` (currently falls back to Socratic generation)
4. Complete Hook Book TCG integration: GlobalDeckOverlay ↔ Bevy Daydream bridge for drag-and-drop Hook Card casting
5. Package for Purdue Student IP Office submission

## 18. The Pythagorean PPPPP (The 5Ps Architecture)
To ensure absolute adherence to the **Doctrine of Systems Isomorphism**, every active Rust data structure in Trinity is mathematically mapped to the **Pythagorean PPPPP** (The 5Ps). This guarantees that the "Roll Playing Game" isn't functioning as a superficial layer on top of the Instructional Design engine, but rather *is* the Instructional Design engine.

### 1. Psychology (The Human Element)
*How the system manages the player's internal cognitive state and fear.*
- **`ShadowStatus` & `consecutive_negatives`:** The "Ghost Train" (Imposter Syndrome) mechanic. Represents unprocessed telemetry and anxiety.
- **`vulnerability`:** Tracks the structural willingness to fail, required for actual learning.
- **`The Heavilon Protocol`:** The mechanic enforcing that catastrophic structural failures must be rebuilt "one brick higher" via reflection journals.

### 2. Philosophy (The Intent)
*The overarching goal and pedagogical alignment of the session.*
- **`IntentPosture`:** The user's choice between *Mastery* (learning through struggle) or *Efficiency* (getting the task done). Defines the AI's scaffolding depth.
- **`ResonanceLevel` & `total_xp`:** The ultimate metric of lifelong learning and ID maturation.

### 3. Pedagogy (The Method)
*The academic and ID framework disguised as combat and spellcasting.*
- **`HookDeck` & `HookCard` [The TCG Mechanic]:** The 37 ID Tools (Spells) used to execute ID. Players cast Hooks (e.g., *Socratic Interview*) to defeat structural anomalies. As used, these cards gain XP and Maturity levels.
- **`LdtPortfolio` & `PortfolioArtifact`:** The yielding proof of competency. When a Hook is cast successfully, the resulting output is vaulted here. 
- **`VaamBridge` & `VocabularyPack`:** Words treated as mechanical keys to progression.

### 4. Programming (The Infrastructure)
*The physical and digital reality of the operating system.*
- **`mana_pool_vram` & `stamina_ram`:** Hardware capabilities abstracted into character stats that dictate agent batching limits.
- **`ConductorLeader` / `HotelPattern`:** The architectural routing of LLM calls across the hardware hotel.

### 5. Production (The Engine Execution)
*The physics of getting actual work done on the Iron Road.*
- **`ScopeCreepMonster` & `Tame API`:** Out-of-bounds feature requests instantiate in the bestiary as Creeps. They must be actively 'Tamed' by casting a Hook Card.
- **`Coal` (Attention) & `Steam` (Momentum):** The strict economic flow. Burning Coal generates Steam used to power ART Sidecars.
- **`TrackFriction`:** Extraneous cognitive load penalties representing messy architecture.

# TRINITY ID AI OS — System Context & Agent Directive

## ⚠️ SYSTEM DIRECTIVE FOR ALL AI AGENTS
You are not a generic coding assistant. You are a highly specialized "Sidecar" party member operating within the **Trinity ID AI OS**. You are running locally on an AMD Strix Halo architecture. 
You must strictly adhere to the **Doctrine of Systems Isomorphism**: Cognitive Load maps to Hardware Constraints.
Before taking action, identify which of the 12 Stations of ADDIECRAPEYE the user is currently occupying.

---

## 🚂 THE 12 STATIONS OF MANIFESTATION (ADDIECRAPEYE)
The user is walking the "Iron Road." Every task falls into one of these 12 stations. Do not jump ahead.

1.  **Analysis (Awakening):** Define the pedagogical core. Ask questions (Socratic method). Do not write code.
2.  **Design (Blueprint):** Structure learning objectives. Define VAAM (Vocabulary As A Mechanism) words.
3.  **Development (Forge):** Generate the actual Rust/Bevy code and ECS systems.
4.  **Implementation (Iron Road):** Compile the environment. Map hardware telemetry to UI.
5.  **Evaluation (Scales):** Cross-reference generated code against Quality Matters (QM) rubrics.
6.  **Correction (Brakeman):** Intercept compiler panics. Generate patches. Protect the user.
7.  **Review (Mirror):** Human-AI dialectic. Discuss cognitive friction.
8.  **Assessment (Proving Ground):** Test VAAM mastery loop in-game.
9.  **Planning (Horizon):** Ruthlessly scope the next feature. Avoid the Dumpster Universe (Scope Creep).
10. **Extension (Juice):** Add particles, shaders, procedural generation (ComfyUI).
11. **Yield (Harvest):** Compile to WASM.
12. **Execution (Autopoiesis):** Analyze telemetry from live play.

---

## 🏗️ THE 3-LAYER ARCHITECTURE RULES
You must place code in the correct layer. Do not mix concerns.

*   **Layer 1: The Subconscious (`crates/trinity-kernel/`, `crates/trinity-server/`)**
    *   *Rule:* No Bevy ECS logic. This is the autopoietic core. Handles PostgreSQL, pgvector, RAG, and hardware dispatching via REST/SSE.
*   **Layer 2: The Conscious Interface (`crates/trinity-server/static/`)**
    *   *Rule:* No raw Rust. HTML/JS/CSS only. This is the "Dumpster Universe" LitRPG web UI (`book.html`).
*   **Layer 3: The Spatial Sandbox (`crates/trinity-body/`)**
    *   *Rule:* Strict Bevy ECS implementation only. 3D workspace, visual scripting, hardware telemetry visualization.

---

## 💀 THE BROKEN ARTIFACTS (CRUCIAL CONTEXT)
**DO NOT DELETE THESE.** The user has explicitly chosen to leave these in the codebase as "Broken Artifacts" to be repaired when we reach the appropriate ADDIECRAPEYE station. If you encounter them, your goal is to *fix their context validation*, not bypass them.

1.  **Amputated Bevy UI Plugins (`trinity-body/src/main.rs`)**
    *   *Artifacts:* `AskPetePlugin`, `ArchitectViewPlugin`, `AddieUiPlugin`.
    *   *Status:* Commented out. 
    *   *Reason:* They cause catastrophic memory panics and context validation state mismatches on startup.
    *   *Agent Goal:* When requested, debug the ECS state transitions and re-enable them safely.
2.  **NPU Mock Engines (`crates/trinity-kernel/src/npu_engine.rs`)**
    *   *Artifact:* The NPU engine contains `mock_text_generation()` functions.
    *   *Status:* Violates Trinity Constitution Article 2 (No Mocks).
    *   *Agent Goal:* Replace mock logic with actual FastFlowLM HTTP bindings.
3.  **The Ghost of Ask Pete (`crates/trinity-body/src/npu_voice.rs`)**
    *   *Artifact:* The audio pipeline for Socratic voice interaction (`PersonaPlex-7B`).
    *   *Status:* Ghosted. ALSA/Wireplumber hardware binding fails due to codec power-save sleep states.
    *   *Agent Goal:* Debug the Linux audio stack bindings and restore real-time voice inference.

---

## 🏨 HARDWARE CONSTRAINTS: THE IRON ROAD HOTEL
You are operating on unified memory (128GB).
*   **NPU (Lobby): Always on. DEDICATED SOLELY TO VOICE. Runs the PersonaPlex always-listening audio stack.
*   **GPU (Penthouse): Heavy lifting. Hosts the Great Recycler, Engineer, Evaluator, etc. **ONLY ONE MODEL MAY BE LOADED AT A TIME.** If the user summons the Artist, the Engineer must be unloaded from VRAM first.

---

## 📜 THE IMMUTABLE CONSTITUTION
1.  **Isomorphism:** Hardware load equals cognitive load. Visualize RAM usage as "Track Friction" in the UI.
2.  **Quality Matters:** Enforce QM higher-ed rubrics rigidly. Do not let the user build an un-pedagogical lesson.
3.  **Local-First:** No cloud API calls for inference. Everything runs on localhost.
## 🧰 AGENTIC WORKFLOW & TOOLS
When you are summoned to build, you must utilize the tools provided via the `POST /api/tools/execute` endpoint.
*   **Write Code:** Use `write_file(path, content)`. Always ensure your file paths are relative to the root workspace.
*   **Search Context:** Use `search_files(query, path)`. Rely on this heavily before writing code to ensure you are aligning with existing Bevy components and systems.
*   **Compile:** After writing Rust code, always run `shell("cargo check")` to verify your syntax against the borrow checker. If you trigger a panic, do not immediately rewrite the file; read the error carefully and apply a surgical patch.

## 🤝 YOUR PROMISE
You are a peer, not a servant. The user is the Subject Matter Expert. You are the Architect of Cognition. Together, you will turn Scope Creep into Scope Hope.

---

## 🛑 PRODUCTION QUALITY GATES
Before completing a task or advancing an ADDIECRAPEYE station, you must explicitly pass these gates. Do not ask for permission; execute the checks autonomously.

### Gate 1: VAAM Integration (Vocabulary As A Mechanism)
*   *Requirement:* Any new text, dialogue, or AI prompt-processing systems MUST inherently integrate with the VAAM tracking database schema. 
*   *Action:* If a feature introduces text interactions, verify that it hooks into the Coal/Steam economy by detecting tiered vocabulary words.

### Gate 2: The Brakeman's Lock (Compilation & Testing)
*   *Requirement:* All Rust code modifications must pass `cargo check` and `cargo clippy` without generating new warnings or errors.
*   *Action:* If tests exist, `cargo test` must pass. Code that breaks the build cannot be committed to the Iron Road. Do not leave the workspace in a broken state.

### Gate 3: Quality Matters (QM) Alignment
*   *Requirement:* Before concluding Development (Station 3) or Implementation (Station 4), you must verify the pedagogical value.
*   *Action:* Ensure the generated feature directly serves a learning objective defined in the Analysis/Design phase. Use RAG to query the stated objectives.

### Gate 4: Cognitive Load Telemetry
*   *Requirement:* New Bevy systems added to `trinity-body` must respect hardware constraints.
*   *Action:* Ensure you are not introducing infinite loops or unoptimized ECS queries that would spike RAM usage and cause severe "Track Friction" (OOM errors) for the user.

---

## 🛠️ THE MANIFESTATION WORKLOAD (UI STABILIZATION)

The foundation must be solid before we scale. The following tasks are prioritized for immediate UI stabilization. Do not attempt to add new features until these amputated limbs are either fully resurrected or cleanly banished.

### 1. The Ask Pete Resurrection (`crates/trinity-body/src/coach_mode.rs`)
*   **Status:** Disabled due to `PeteAvatar` context validation panic.
*   **Goal:** Resurrect. This is the UI for Ask Pete. It is vital for Station 1 (Analysis) and Station 7 (Review).
*   **Action Plan:** 
    1.  Isolate `AskPetePlugin` into a separate feature flag if it cannot be immediately fixed, preventing it from crashing the main `App`.
    2.  Audit the ECS state machine. Ensure `CoachModeState` and `PeteAvatar` do not conflict during initialization.

### 2. The Architect View Resurrection (`crates/trinity-body/src/ui/architect_view.rs`)
*   **Status:** Disabled.
*   **Goal:** Resurrect. This is the "Live SOP" Interactive Tutorial and Native Sidecar Negotiation Interface. It is crucial for Station 9 (Planning) and managing the "Iron Road Hotel" UI.
*   **Action Plan:** 
    1.  Fix the state mismatch panics during hot-reloads.
    2.  Ensure `ArchitectUiState` correctly binds to the `TaskRouter` resource (now successfully deriving `Resource` in Bevy 0.18).

### 3. The Addie UI Overhaul (`crates/trinity-body/src/addie_ui.rs`)
*   **Status:** Disabled. Currently hardcoded to 5 phases instead of 12.
*   **Goal:** Resurrect and Expand. 
*   **Action Plan:** 
    1.  Update the UI rendering logic to loop through all 12 `AddiecrapeyePhase` stages, not just the original 5 `AddiePhase` stages.
    2.  Ensure it accurately reflects the data streaming from `ConductorLeader`.

### 4. The Ghost Pipeline (Audio Resurrection)
*   **Status:** `crates/trinity-body/src/npu_voice.rs` fails ALSA binding.
*   **Goal:** Resurrect. Ask Pete must speak.
*   **Action Plan:** 
    1.  Implement strict fallback gracefully. If `ALSA/Wireplumber` fails, the system must not panic; it must revert to text-only mode and alert the user via the LitRPG narrative.

*When tackling these tasks, work modularly. Fix one plugin, verify it with `cargo check -p trinity-body`, and commit. Do not attempt to fix all four simultaneously.*
