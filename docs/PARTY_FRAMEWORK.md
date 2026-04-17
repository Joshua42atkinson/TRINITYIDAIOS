# The P.A.R.T.Y. Protocol
**Isomorphic Organizational Theory for Agentic AI Frameworks**

The TRINITY AI OS relies on a **unified model family** (Google Gemma 4) scaled across three cognitive loads, plus dedicated aesthetic and voice engines, bound together by the central Daydream game engine. By utilizing isomorphic organizational theory, we elegantly map the entire complex multimodal backend into a simple, memorable taxonomy: **The P.A.R.T.Y. Protocol**.

---

### **P — Programming** (The Architect)
* **Model Engine:** `Gemma-4-26B-A4B-it` AWQ 4-bit (MoE)
* **Port:** :8000
* **VRAM:** ~16 GB
* **Persona:** Programmer Pete
* **Loading:** Hotel Swap (on-demand during Development, Yoke, Evolve phases)
* **Function:** Responsible for coding, UI structure, workflow generation, and executing strict, procedural agentic steps. The MoE (Mixture of Experts) architecture activates only 4B of its 26B parameters per token, making Pete incredibly fast at context-switching between different development tasks or languages while retaining the full reasoning capacity of a 26B model. Native function calling and 256K context window.

### **A — Aesthetics** (The Artist Triad)
* **Model Engines:** 
  1. `FLUX.1-schnell` GGUF (2D Structural Image Generation — embedded via Candle, ~7 GB)
  2. `Janus Pro 7B` (Vision-Language CRAP Critique — Hotel Swap on :8003, ~4 GB)
  3. `TripoSR` (3D Mesh/Spatial Generation — optional Gradio sidecar)
* **Function:** This triad manages complete visual realization. FLUX handles 2D asset generation embedded in the Rust binary (no sidecar needed). Janus Pro provides Vision-Language evaluation of UI screenshots for CRAP compliance (Contrast, Repetition, Alignment, Proximity). TripoSR handles spatial 3D generation when available. All models use strictly compliant Apache 2.0 / MIT licenses.

### **R — Reasoning** (The Philosopher)
* **Model Engine:** `Gemma-4-31B-it` AWQ 4-bit (Dense) + `nomic-embed-text-v1.5` (ORT)
* **Port:** :8002 (Dense) / embedded (nomic-embed)
* **VRAM:** ~18 GB (Dense) + ~1 GB (nomic-embed)
* **Persona:** The Great Recycler 
* **Loading:** Hotel Swap (on-demand during Evaluation, Alignment, Envision phases)
* **Function:** The dense model architecture activates ALL 31B parameters on every token, making it significantly better at deep-state logic, moral philosophy, educational theory validation, and complex RAG correlation. The Great Recycler handles the heavy cognitive lifting required for evaluating Instructional Design artifacts against Quality Matters rubrics. Where Programming is fast and procedural, Reasoning is slow and thorough.

### **T — Tempo** (The Fast Reactor)
* **Model Engine:** `Gemma-4-E4B-it` AWQ 4-bit (Dense, 4B effective)
* **Port:** :8001
* **VRAM:** ~6 GB
* **Persona:** Omni_NPC / The Conductor
* **Loading:** Always On (permanent resident)
* **Function:** The tiny, ultra-fast E4B handles Tempo. It governs the always-on Socratic chat loop, NPC dialog, TTS routing (dispatching to embedded Kokoro), and instant phase-transition responses. It serves as the fast-twitch muscle fibers of the TRINITY OS. Where Reasoning is deep, Tempo is immediate. The E4B is multimodal (text + image + audio input) with a 128K context window, sufficient for sustained Socratic sessions.

### **Y — Yardmaster** (The Governor)
* **Engine:** The TRINITY Native Rust/Bevy Client
* **Port:** :3000
* **Function:** The Yardmaster is you, utilizing the overarching React UI and native Bevy Daydream engine to orchestrate the entire P.A.R.T.Y. It acts as the central conductor ensuring the Programming, Aesthetics, Reasoning, and Tempo stay perfectly aligned to the instructional design objectives (ADDIECRAPEYE). The Yardmaster also governs the **Hotel Swap Protocol** — deciding which heavyweight model to load based on the current ADDIECRAPEYE phase.

---

## The Hotel Swap Protocol

TRINITY cannot load all three Gemma 4 text models simultaneously (they'd consume ~40 GB just for weights, plus KV cache). Instead, the Conductor manages a **Hotel** — a single VRAM slot where heavyweight models check in and check out based on the current pedagogical phase:

```
┌──────────────────────────────────────────────────────┐
│              ALWAYS ON (Permanent Residents)          │
│  T: Gemma 4 E4B    (~6 GB)  — Chat, NPC, routing    │
│  A: FLUX.1-schnell  (~7 GB)  — Image generation      │
│  V: Kokoro TTS      (~1 GB)  — Voice synthesis        │
│  E: nomic-embed      (~1 GB)  — RAG embeddings        │
│  OS: Strix + Bevy   (~20 GB) — Framework              │
├──────────────────────────────────────────────────────┤
│              HOTEL SWAP ZONE (~18 GB peak)            │
│  Phase 2,3,11 → P: Gemma 4 26B A4B  (coding)        │
│  Phase 5,8,10 → R: Gemma 4 31B Dense (reasoning)    │
│  Phase 6,9    → A₂: Janus Pro 7B    (vision)        │
│  Phase 1,4,7,12 → [empty — T handles it alone]      │
└──────────────────────────────────────────────────────┘
```

Model swaps take ~8-12 seconds and occur behind the Iron Road's phase transition narrative animation, making them invisible to the user.

---

## Size-to-Task Rationale

The key insight of the P.A.R.T.Y. Protocol is that **different cognitive tasks demand different model sizes**:

| Cognitive Load | Model Size | P-ART-Y Role | Example Tasks |
|---------------|-----------|-------------|---------------|
| **Fast / Reactive** | 4B (E4B) | T — Tempo | Chat, NPC dialog, routing, phase transitions |
| **Procedural / Agentic** | 26B MoE (4B active) | P — Programming | Code gen, tool calling, file I/O, scaffolding |
| **Deep / Evaluative** | 31B Dense (all active) | R — Reasoning | Quality rubrics, PEARL alignment, moral philosophy |
| **Visual / Spatial** | 7B VLM | A — Aesthetics | CRAP critique, screenshot analysis, layout eval |

Overcalibrating (using 31B for chat) wastes VRAM and adds latency. Undercalibrating (using E4B for evaluation) produces shallow analysis. The Hotel pattern ensures the right tool for the right job.

---

## Agentic Documentation Implications
By adopting the P.A.R.T.Y framework, we standardize how the system communicates its intentions to developers and students alike:

- **System Logs:** When TRINITY shifts workloads, logs classify the resource (e.g., `[P-GEAR LOADED: Gemma-4-26B-A4B on :8000]`, `[R-GEAR LOADED: Gemma-4-31B-Dense on :8002]`).
- **Educational Portfolios:** Students utilizing TRINITY can clearly see which "expert" they are interacting with (e.g., earning a badge heavily reliant on the **[P]** or **[A]** pillars of the platform).
- **Scalability:** If a better coding model is released, it drops into the **[P]** slot natively without redefining the paradigm. If Gemma 5 arrives, the E4B/26B/31B ladder updates in place.
- **Hardware Tiers:** Students without 128 GB can run a "Lone Wolf" mode where a single model (E4B or Ollama equivalent) handles all roles. The P-ART-Y taxonomy remains the same; only the Hotel is disabled.
