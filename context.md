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
- Removed `#[allow(dead_code)]` shields fully. Re-wired `translate_daydream_to_forge` into the dispatcher dynamically parsing GameState.
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
