# TRINITY ID AI OS: Maturation Map

> **Updated: April 14, 2026 — The Gemma 4 Model Harmony Synthesis**

The TRINITY system is a layered, Socratic ecosystem designed to shepherd a user from a raw idea to a fully shipped portfolio product (The EYE Package). Previously reliant on the unstable LongCat-Next 74B Monolith and a fragmented multi-vendor sidecar matrix, TRINITY has matured into its definitive **Gemma 4 P-ART-Y Architecture** — a unified model family where three sizes of the same architecture serve three distinct cognitive loads, orchestrated by the Hotel Swap Protocol on the AMD Strix Halo (128GB Unified Memory).

This map acts as the definitive roadmap for the August LDT Capstone and Purdue Inventions pitch.

---

## 1. The Pedagogy: Cognitive Thermodynamics & VAAM
Trinity is an educational engine that forces physical interaction. It utilizes:
- **Cognitive Load Theory (CLT)**: Managed via "Traction" and "Friction" (Coal/Steam). The OS monitors user overwhelm and uses the "Ghost Train" dynamic to pause progression.
- **Vocabulary As A Mechanism (VAAM)**: Abstract concepts are mapped to 3D physical models in the Bevy XR engine via `PEARL`. Learning requires spatial interaction.
- **ADDIECRAPEYE Constraint Protocol**: The immutable 12-Station State Machine preventing AI hallucination by anchoring output to rigorous analytical checkpoints.

---

## 2. The Architectural Hierarchy (The Gemma 4 P-ART-Y Matrix)

### Design Philosophy: Size-to-Task Harmony

TRINITY uses three sizes of the **same model family** (Google Gemma 4) rather than a heterogeneous mix of unrelated models. This eliminates tokenizer fragmentation, prompt engineering drift, and license complexity:

- **Same tokenizer** across all three models — PEARLs, VAAM glossaries, and RLHF bias strings work identically
- **Same function calling schema** — Native tool calling with identical JSON format
- **Same quantization pipeline** — AWQ 4-bit across the entire family
- **Apache 2.0 everywhere** — No licensing risk for Purdue or LDT distribution

### VRAM Budget: 128GB Unified Memory (AMD Strix Halo gfx1151)

The Hotel Swap Protocol ensures only ONE heavyweight model occupies the swap zone at any time. The always-on Tempo layer plus embedded models consume ~8 GB permanently; the heaviest swap-in model (31B Dense) peaks at ~18 GB.

| P-ART-Y Role | Model | Port | Size (Q4) | Context | Loading | Function |
|-------------|-------|------|-----------|---------|---------|----------|
| **T — Tempo** | `Gemma-4-E4B-it` AWQ 4-bit | :8001 | ~6 GB | 128K | **Always On** | Real-time chat, NPC dialog, Socratic questioning, TTS routing. The fast-twitch always-on brain. |
| **P — Programming** | `Gemma-4-26B-A4B-it` AWQ 4-bit | :8000 | ~16 GB | 256K | **Hotel Swap** | Code generation, agentic tool calling, React/Rust scaffolding. MoE = 4B inference speed at 26B quality. |
| **R — Reasoning** | `Gemma-4-31B-it` AWQ 4-bit | :8002 | ~18 GB | 256K | **Hotel Swap** | Deep Socratic evaluation, RAG correlation, pedagogical scoring, Quality Matters alignment. Maximum reasoning depth. |
| **A — Aesthetics** | `FLUX.1-schnell` GGUF | embedded | ~7 GB | N/A | **Always On** | 2D image generation via embedded Candle crate. No sidecar needed. |
| **A₂ — Vision Critique** | `Janus Pro 7B` | :8003 | ~4 GB | Image | **Hotel Swap** | Vision-Language CRAP evaluation of UI screenshots (Contrast, Proximity critique). |
| **Voice** | `Kokoro TTS` ONNX | embedded | ~1 GB | N/A | **Always On** | Text-to-speech via embedded ORT crate. No sidecar. |
| **Embeddings** | `nomic-embed-text-v1.5` | embedded | ~1 GB | N/A | **Always On** | RAG semantic search via embedded ORT crate. No sidecar. |
| **Overhead** | *Strix OS / Bevy Engine / Rust* | :3000 | ~20 GB | N/A | **Always On** | Framework |

**Always-On Baseline:** ~35 GB (Tempo + Flux + Voice + Embed + OS)
**Peak with Hotel Guest:** ~53 GB (Baseline + 31B Dense at 18 GB)
**Total Protected Utilization:** **~53 GB / 128 GB** (41% peak, 75 GB safety buffer)

---

## 3. The Hotel Swap Protocol (Phase-Aware Model Loading)

The Conductor (`conductor_leader.rs`) maps each ADDIECRAPEYE phase to a P-ART-Y gear. Only one heavyweight model occupies the Hotel Swap Zone at any given time. Model swaps happen behind the Iron Road's phase transition animation (~8-12 seconds, invisible to the user).

| Phase | Gear | Model in Swap Zone | Why This Model |
|-------|------|-------------------|----------------|
| 1. Analysis | T only | *None* | Socratic questioning is lightweight. E4B's 128K context suffices. |
| 2. Design | **P** | Gemma 4 26B A4B | Designing objectives requires structured output and tool calling. |
| 3. Development | **P** | Gemma 4 26B A4B | Active code generation. MoE handles multi-file reasoning efficiently. |
| 4. Implementation | T only | *Unload P* | Testing walkthrough. E4B provides fast dialog. |
| 5. Evaluation | **R** | Gemma 4 31B Dense | Quality evaluation requires maximum reasoning depth. |
| 6. Contrast | **A₂** | Janus Pro 7B | Visual hierarchy critique. CRAP compliance analysis of screenshots. |
| 7. Repetition | T only | *Unload A₂* | Pattern consistency audit. Conversational. E4B sufficient. |
| 8. Alignment | **R** | Gemma 4 31B Dense | Scope pruning requires deep analytical reasoning. |
| 9. Proximity | **A₂** | Janus Pro 7B | UX grouping analysis. Spatial layout evaluation. |
| 10. Envision | **R** | Gemma 4 31B Dense | Meta-cognitive reflection. Comparing output vs. original PEARL. |
| 11. Yoke | **P** | Gemma 4 26B A4B | Final assembly. Integration code. Tool-heavy compilation. |
| 12. Evolve | T only | *Unload P* | Celebration and export. E4B handles the wrap-up. |

---

## 4. Workflow Choreography (Meaning-Making)

The Trinity logic is governed by 9 core workflows, mapped chronologically to the ADDIECRAPEYE constraint protocol. Each workflow invokes specific models via the Hotel Swap Protocol.

### A. Initialization (The Analysis Phase)
*Goal: Secure the boundaries of the environment before any Socratic engagement.*
1. `/session-start`: Validates the Tempo backbone (:8001) and embedded sidecars. Verifies memory allocations.
2. `/build-and-test`: Establishes the Iron Road physics loop by compiling the Bevy/Axum components.

### B. The Socratic Squeeze (Design & Development Phases)
*Goal: Extract scope from the learner. No AI generation occurs until the user proves intent.*
3. `/first-playable`: Engages the **Tempo E4B** (:8001) for Socratic questioning. The user proves intent before any model swaps occur.
4. `/wire-pete-socratic`: Validates that the Conductor respects the current Phase (e.g. asking Design questions vs. Evaluation questions). When code generation is needed, the **Programming gear (26B A4B)** is loaded on :8000.
5. `/fill-objectives`: Utilizes the embedded nomic-embed RAG to pull structural constraints from `TRINITY_SYLLABUS.md` into the user's PEARL state.

### C. The Layout Critique (CRAP: Contrast, Repetition, Alignment, Proximity)
*Goal: The user must physically align visual logic. The AI must evaluate the spatial reality.*
6. `/fix-frontend-component`: The Socratic engine pauses. The user submits visual layouts. **Janus Pro (:8003)** performs Vision-Language analysis on the layout, ensuring Contrast and Proximity rules are respected.

### D. The Execution (EYE: Envision, Yoke, Evolve)
*Goal: The compiler physically instantiates the validated concepts.*
7. `/fix-rust-backend`: Dispatches the verified schema to the **Programming gear (Gemma 4 26B A4B on :8000)** for sequential compilation.
8. `/research-implementation`: Fills any gaps using semantic architectural sourcing via the **Reasoning gear (Gemma 4 31B on :8002)**.
9. `/commit-wrap`: Finalizes the session, packages the HTML5/XR "EYE Package", mints the progress to the Character Sheet, and releases the Hotel Swap Zone.

---

## 5. Current State & Immediate Priorities (April 14)

### ✅ What is Working
- The Axum & Bevy Rust backend (Port 3000).
- The LitRPG/Iron Road UI structures (26 React components).
- Vaulting of completed artifacts to the Character Sheet.
- Multi-backend inference router with auto-detect and failover.
- RLHF prompt steering wired into all three chat paths.
- Scope Creep auto-intercept in both chat_stream and agent loop.
- EYE Package export (HTML5 Quiz, Adventure, DOCX, ZIP).

### 🔴 P0 — Model Identity Alignment (This Session)
- ✅ **DONE**: Unified MATURATION_MAP to reflect Gemma 4 P-ART-Y matrix.
- ⏳ **IN PROGRESS**: Align PARTY_FRAMEWORK.md with identical model names/ports.
- ⏳ **IN PROGRESS**: Update `default.toml` config with PartyRole annotations.
- ⏳ **IN PROGRESS**: Add `PartyRole` enum to `inference_router.rs`.

### 🟡 P1 — Hotel Swap Wiring
- Unwire "Lone Wolf" mode in `conductor_leader.rs` `manage_hotel_sidecars()`.
- Write launch scripts for the three Gemma 4 models.
- Update `vllm_fleet.rs` to track all four ports (:8000-:8003).

### 🟢 P2 — Integration Testing
- Janus Pro CRAP evaluation end-to-end test.
- Hotel swap latency measurement (target: <12s behind phase animation).
- LM Studio fallback path for Tier 1/2 student hardware.
