# MINI-TRINITY: EDGE-DEVICE PRODUCT DESIGN
**Status:** DRAFT (Awaiting Developer Interview)
**Date:** April 14, 2026

## 1. The Edge-Device Architecture (8-10GB Constraint)
To run a fully autonomous gaming engine and LDT learning assistant on an edge device (e.g., mobile phones, low-end laptops), we must completely eliminate Python, vLLM, and Distroboxes. The solution is **Rust + `ort` (ONNX Runtime) + Tauri**.

### 1.1 The Multimodal ONNX Agent Swarm (Under 6GB Total)
Instead of one massive model, we load three small, specialized models concurrently using ONNX Runtime. 

1. **The Reasoning Agent (DeepSeek R1 Distill 1.5B ONNX)**
   - *RAM Cost:* ~1.5 GB
   - *Role:* Logic, code generation, ADDIECRAPEYE constraint checking, objective mapping. This keeps the engine perfectly aligned to the curriculum without needing 30B param reasoning.
2. **The Storytelling Agent (Gemma 2 2B Instruct ONNX)**
   - *RAM Cost:* ~2.0 GB
   - *Role:* Driving the prose, character dialogue, emotional resonance, and quest narrative. Very fluent in RPG context.
3. **The Multi-Modal Artist (Janus Pro 1B ONNX)**
   - *RAM Cost:* ~1.5 GB
   - *Role:* YES, **Janus 1B makes images!** It is an *autoregressive unified model*, meaning it can take text and output VQ visual tokens (images), AND it can take images, understand them, and output text. This completely removes the need for the bulky SDXL (which requires 6-8GB alone).
4. **The RAG Architect (Nomic-Embed v1.5 ONNX)**
   - *RAM Cost:* ~300 MB
   - *Role:* Memory search and mapping the Player Handbook to gameplay.

**Total Swarm Memory:** ~5.3 GB. This leaves ~2.7 GB for Tauri, the OS, and Fyrox/Bevy, keeping us safely within the 8 GB budget.

---

## 2. The Copy-and-Paste Refactor Plan

We will embrace Option A and restructure `/desktop_trinity` into a Cargo Super-Workspace composed of `genesis`, `mini`, and `core`.

### Step 1: Initialize the Super-Workspace
In `/desktop_trinity`, we will create a `Cargo.toml` workspace that unifies everything.
```bash
cargo new trinity-core --lib
cargo new trinity-mini
```

### Step 2: Extracting the Brain to `trinity-core`
We will **COPY** (not move, to preserve Genesis) the pure-logic files from `trinity-genesis/crates/trinity/src/` and other crates into `trinity-core`. 
*Files to copy:*
- `conductor_leader.rs` (ADDIECRAPEYE state machine)
- `quests.rs` & `crates/trinity-quest/` logic
- `vaam.rs` & `vaam_bridge.rs`
- `skills.rs` & `character_sheet.rs`
- `export.rs` (EYE Package compiling)

### Step 3: Write the Mini-Trinity Heartbeat
In `trinity-mini`, we will build a headless Tauri App.
Instead of `hotel_manager.rs` and `inference_router.rs`, `trinity-mini` uses a single `onnx_swarm.rs` that loads the models statically into memory on startup and handles inference via native CPU/NPU using Tauri commands.

---

## 3. UI & The "Engine-to-Game" Pipeline
You mentioned swapping out Bevy for **Fyrox** and utilizing **Tauri** for mobile readiness. The primary deliverable for users of Mini Trinity is that **the output is ALWAYS a game**. 

1. **The LDT Setup Phase:** Users converse with the Socratic AI via an Iron Road-themed HTML/React mobile-responsive UI inside Tauri.
2. **The Generation Phase:** The AI Swarm designs the curriculum, mapping ADDIECRAPEYE to a narrative tree.
3. **The Playable Result:** Once the player finishes setup, we compile the assets. If using **Fyrox**, we can dynamically load a standalone scene file (`.rgs`) filled with the generated dialogue and Janus images, allowing the user to literally "play" their lesson plan!

---

## 4. Developer Clarification Interview (Please Answer These!)
To finalize the blueprint for the next session, I need your executive direction on the following questions:

**Q1: The Image Generator:** I heavily recommend Janus 1B because it can both SEE and DRAW, fitting perfectly in the 8GB limit. If we use SDXL or SD 1.5, we will blow past 8GB RAM on phones immediately. Are you okay with committing to Janus 1B for the images?

**Q2: The "Small LLM" Breakdown:** I proposed DeepSeek R1 (1.5B) for strict LDT logic and Gemma (2B) for character narrative. Do you approve of this setup, or do you have a specific small model in mind for storytelling (like Llama-3.2-1B)?

**Q3: Bevy vs. Fyrox vs. Pure Web:** If the end product is *always a game* and must run on mobile via Tauri: 
- Do we want a 3D environment generated (Fyrox/Bevy)? 
- Or do we want an elegant 2D RPG Visual Novel UI (similar to Iron Road) built purely in React/Tauri that can generate immediately? It is 100x easier to export a web-based game for an LDT student.

**Q4: Static vs Dynamic Loading:** When running on phone hardware, memory is tight. Do we want all 4 ONNX models to live constantly side-by-side in RAM (fastest, but pushes memory limits), or should the edge device swap them out as needed (e.g. unload the Storyteller while generating an image)?

**Q5: The Target File Structure:** Once you are ready, I will create `trinity-core` and physically paste the code. Are there any other specific files from `trinity-genesis` you know for sure must go into `trinity-core`?
