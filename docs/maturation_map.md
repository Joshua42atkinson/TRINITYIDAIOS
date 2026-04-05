# TRINITY ID AI OS: Maturation Map

The TRINITY system is a layered, Socratic ecosystem designed to shepherd a user from a raw idea to a fully shipped portfolio product. It relies on a three-tier agentic hierarchy operating seamlessly within the Daydream Engine.

## The Agentic Hierarchy (TRINITY)

To avoid cognitive overload, TRINITY dynamically engages logic engines (models) invisibly via natural Socratic conversation inside the **Iron Road**.

### 1. The Instructional Designer (ID)
### 1. The Instructional Designer (ID)
**Model:** The Great Recycler (Gemma-4-31B-Dense-AWQ via vLLM)  
**Role:** Master Agent & Orchestrator  
**Function:** 
- The ultimate authority of the session, owning 35% of Unified Memory (~44GB).
- Applies the **ADDIECRAPEYE** methodology (Analyze, Design, Develop, Implement, Evaluate | Contrast, Repetition, Alignment, Proximity | Evaluate, Yield, Evolve).
- Interrogates the user via the Socratic Method.
- Maintains the master game state, character sheet, and quest logic.
- Evaluates when to autonomously spawn Subagents without exiting the Iron Road.

### 2. Artificial Intelligence Media (AI)
**Model:** Voxtral / Art Studio (Gemma-4-E4B-Omni & HunyuanImage DiT via vLLM)  
**Role:** Subagent (Delegated Senses & Media)  
**Function:** 
- Acts strictly at the behest of the Master Agent, dynamically load-balanced on port 8000.
- Generates required visual assets (Spot Art, Characters, UI styling) natively via direct tensor inferences.
- Yields the Lite Novel Audiobooks via real-time synthesized voice rendering without relying on legacy sidecars like ComfyUI or Supertonic.
- Artifacts are blindly inserted into the LDT Portfolio. The User never "prompts" the AI Art model directly.

### 3. Operating System Logic (OS)
**Model:** Programmer Pete (Gemma-4-26B-MoE-AWQ via vLLM)  
**Role:** Sub-Subagent (The Golem Muscle)  
**Function:**
- Responsible for fulfilling concrete logic architectures mathematically across 25% of memory.
- Executes bash commands, updates Rust `.rs` files, writes React `.jsx`. 
- Handles the CI/CD pipeline, compiler errors, and the actual "Product" output of the Iron Road delivery matrix.
- Governed by the Recycler to ensure "Pete" only builds what the ID has theoretically approved.

---

## User Journey (Path of Maturation)

### Phase 1: The Yard (Novice)
Users land in the Socratic CLI. They interact purely with the ID (Great Recycler). They answer foundational scope questions. 
- **Active Subsystems:** ID (100%).

### Phase 2: The Iron Road (Apprentice)
Users begin writing the Hook Book. As they establish narrative and visuals, the ID silently delegates tasks. Users notice characters appearing visually and audio narrative generating dynamically.
- **Active Subsystems:** ID (60%), AI Media (40%). 

### Phase 3: The Daydream Forge (Journeyman)
The User crosses from theory to product engineering. The Great Recycler awakens Programmer Pete to translate the ADDIECRAPEYE scaffolding into raw Rust/React logic. The User now plays a game of QA, reviewing Pete's builds against the ID's original theories.
- **Active Subsystems:** ID (40%), OS (60%).

### Phase 4: Autopoiesis (Master)
The User fully commands the TRINITY loop on a single, isolated hardware node. The User feeds complex constraints (PEARL data schemas) into the Recycler, which simultaneously binds Pete to write the codebase while the Omni pipeline generates textures for the 3D meshes. The User does not prompt — they **Direct**, all orchestrated blazingly fast across the 128GB RDNA 3.5 unified compute layer.

---

## Telemetry & "Just Works" Local Persistence

### 1. The Vector Memory Matrix (RAG)
TRINITY operates a constant Vector Database utilizing the `nomic-embed-text-v1.5-AWQ` pipeline hosted dynamically on Port 8005. 
As textbooks, JSON files, or user interactions are read, they are vectorized mathematically and pushed into the local SQLite database. This gives The Great Recycler instant, passive memory of all architectural changes to the OS.

### 2. The beastlogger (Media HUD Telemetry)
The beastlogger acts as the organizational cortex for the HUD. It is a telemetry subsystem designed to perfectly organize the raw output of the Socratic loop:
- **File Organization:** Because Pete executes raw bash logic without human intervention, the beastlogger tracks workspace changes and correctly drops `.rs`, `.md`, and Image outputs into their respective HUD Vaults structurally.
- **Cognitive Mapping:** It tracks the 'Coal' expenditures visually, assuring users understand their cognitive load mapping throughout the Iron Road. It operates entirely via the React state UI interacting with Rust's Tracing events.

### 3. Auto-Bootstrap Distribution (Zero-Config Delivery)
To ensure the OS is "distributable", the entire ecosystem collapses behind `scripts/launch/start_vllm_omni.sh`. When a user launches the ecosystem, the backend dynamically verifies the presence of required AI weights. If uninstalled, it organically pulls the necessary `AWQ` weights via HuggingFace. 
**Offline Architecture Masquerading:** To bypass fragile Python dependencies on diverse systems, the backend utilizes architecture masquerading locally—re-mapping future `.safetensors` into structural envelopes standard `transformers` environments understand, guaranteeing an out-of-the-box local inference classroom without demanding 10-hour C++ engine recompilations.

---

## Appendix A: Purdue Video Presentation Flow (What Works NATIVELY)
> *April 5, 2026 — Verified Native Architecture Video Script*

The system is now fully native, fast, and does not rely on legacy sidecars. For your video, **do not worry about the old UI.** Present the new LDTAtkinson Portfolio as the ultimate manifestation of Instructional Design. 

**Here is your proven, working flow for the video:**

1. **Launch the Core:** 
   - Show the terminal running `cargo run --bin trinity`. Point out it compiles cleanly and runs natively.
   - Show the terminal running `npm run dev` in the `LDTAtkinson/client` directory.
2. **The Portfolio Hub:** 
   - Open your browser to `http://localhost:5173`. Show the gorgeous React interface. Explain that this UI is the "Wrapper" for the TRINITY OS. 
3. **The BeastLogger Telemetry:** 
   - Point out the new BeastLogger overlay. Explain that instead of just "talking to an AI", the system physically meters Cognitive Load (Coal/Steam) entirely autonomously using `tokio` telemetry layers in the backend.
4. **The Four Chariots (The Crown Jewel):**
   - Click the Navigation links to open the Socratic Audiobooks (`/chariot/PLAYERS_HANDBOOK.md` or `TRINITY_SYLLABUS.md`).
   - Show the **Double-Page ELearning UI**. 
   - Explain the Pipeline: *"This wasn't built by pasting text into ChatGPT. I engineered a Python compiler that seamlessly chunks structurally sound markdown, feeds it to the local vLLM-Omni endpoint for audio narration, and dynamically queries the local Aesthetic Triad (Flux) for the spot illustrations."*
5. **The Iron Road Integration:**
   - Finally, click the **Interact** button in the Audiobook UI, which pivots seamlessly to the "Ask Pete" Iron Road workspace. This visually proves you've designed a unified system where Documentation, Socratic Interaction, and UI design are entirely biologically coupled to pedagogical theory.
