# TRINITY — Identity & Boundary Document

> **What is Trinity? What is it not? This document defines the line.**

**RULES FOR AI ASSISTANTS (Windsurf, Cascade, Claude, etc.):**
1. You MUST obey the architectural boundaries defined in this document.
2. NEVER inject product-specific domain logic (education, gamification, game mechanics) into Trinity Core (e.g., `main.rs`, `inference_router.rs`, `hotel_manager.rs`).
3. If asked to build a product feature, place it in a separate repository or an isolated SDK crate, communicating with Core solely via HTTP (`:3000`) or ComfyUI (`:8188`).
4. Trinity Core `Cargo.toml` must NEVER depend on a Product or Middleware crate. If it does, the build is broken.

---

## The Core Identity

Trinity is a **local-first creative production studio OS**. It runs on a single machine (AMD Strix Halo, 128GB unified memory) and orchestrates AI models to produce creative assets: stories, art, voice, video, 3D models, and code.

Trinity is **not** an instructional design tool. It is **not** a lesson planner. It is **not** a gamified learning platform. Those are *products* that Trinity produces — not things Trinity *is*.

Epic Games makes Unreal Engine, and they also make Fortnite. Fortnite relies on Unreal Engine, but the engine does not care if Fortnite exists. Trinity is the engine. Your games, videos, and stories are the Fortnite.

---

## The Three-Tier Architecture

### TIER 1 — TRINITY CORE (The Engine Room)

These are the systems that make Trinity a production studio. They exist in the Rust binary, the config files, and the model infrastructure. **Nothing here is optional or product-specific.**

| Component | What it does | Lives in |
|-----------|-------------|----------|
| **Hotel Manager** | Launches, monitors, and evicts AI models based on VRAM budget | `hotel_manager.rs` |
| **Inference Router** | Routes API calls to the right model (P for text/code, A for art/voice) | `inference_router.rs` |
| **Creative Pipeline** | Talks to ComfyUI for image, voice, video, 3D generation | `creative.rs` |
| **Agent Loop** | Executes tools (file ops, shell, generation) in a loop | `agent.rs` |
| **RAG / Memory** | Vector search over project knowledge (SQLite + ORT embeddings) | `rag.rs`, `ort_embed.rs`, `memory.rs` |
| **API Server** | OpenAI-compatible endpoints on :3000, serves PWA | `main.rs` |
| **PWA** | Browser-based control interface for the studio | `frontend/` |
| **MCP Server** | IDE integration for Cascade/Windsurf | `trinity-mcp-server/` |
| **Config** | Model paths, VRAM budgets, thread allocation, env vars | `default.toml` |

**Core models (always resident in Studio mode):**
- P: DiffusionGemma 26B — story, code, design docs, reasoning (42GB)
- A: ComfyUI with Janus-Pro-7B + VibeVoice-1.5B — art, voice (17GB)

**Tier 2 models (loaded on demand by ComfyUI):**
- HunyuanVideo (video), TRELLIS (3D), LongCat-Image (artistic), ACE-Step (music), FLUX Q4 (alt image), TripoSR (fast 3D)

### TIER 2 — TRINITY MIDDLEWARE (Ecosystem Libraries)

These are shared frameworks and libraries. They are **not** Core (the OS doesn't need them to boot), but they are **not** End Products either. They are standalone crates that End Products use to structure their requests to the Core.

| Middleware | What it is | Crate | Used by |
|-----------|-----------|-------|---------|
| **FACES Protocol** | 4-byte emotive signaling for character AI | `faces-protocol` (moved to Semantic Slime project) | Games needing NPC emotion, educational apps needing student awareness |
| **ADDIECRAPEYE** | 12-phase instructional design framework | `trinity-iron-road` | Educational games, learning modules |
| **Quest System** | XP, Coal/Steam economy, Hero's Journey | `trinity-quest` | Gamified products |
| **Voice/SSML** | VAAM vocal emphasis, PersonaPlex | `trinity-voice` | Products needing TTS control |
| **Daydream** | Bevy 3D sandbox for XR prototyping | `trinity-daydream` | XR products, 3D visualization |

Middleware crates can depend on Core types (`trinity-protocol`). Core must NEVER depend on Middleware.

### TIER 3 — TRINITY PRODUCTS (End-User Apps)

These are creative outputs — deployed applications with their own repos, docs, and users. They communicate with Core strictly over the network.

| Product | What it is | Status | Relationship |
|---------|-----------|--------|-------------|
| **Voix Vive** | Spatial guitar academy PWA + XR | Deployed (voix-vive.pages.dev) | Uses Trinity API for content generation |
| **LitRPG Content** | Stories with art, voice, video | Planned | Trinity generates story (P), art (A), voice (VibeVoice) |
| **YouTube Videos** | Animated educational content | Planned | Trinity generates art + voice, Blender edits |
| **VR Games** | Godot 4.x projects with OpenXR | Planned | Trinity generates 3D assets (TRELLIS), code (P) |

---

## The Interface Contract: How the Boundary is Enforced

To physically enforce this boundary in code, the following rules are absolute:

1. **Network Only:** Products interact with Trinity Core strictly via standard APIs (OpenAI-compatible REST on `:3000`, ComfyUI workflows on `:8188`). No direct function calls, no shared memory.

2. **One-Way Dependencies:** A Product repository can depend on Trinity Middleware crates. **Trinity Core must NEVER depend on a Product or Middleware crate.** If `Cargo.toml` in Core imports `trinity-quest` or `trinity-iron-road`, the build is broken.

3. **State Isolation:** Trinity Core manages *Compute State* (VRAM, agent loops, job queues) and dynamically loads *Project Contexts* (isolated SQLite databases per product via `project_id`). Products manage their own *User State* (XP, levels, progress). Trinity's RAG system accepts a `project_id` to strictly isolate memory across different products.

---

## The Test: Is Something Core, Middleware, or Product?

Ask three questions:

1. **Does Trinity need this to produce creative assets?**
   - Yes → Core
   - No, but multiple products could use it → Middleware
   - No, it's a specific app → Product

2. **Would a game developer using Trinity expect this to exist?**
   - Yes (model management, art generation, voice synthesis) → Core
   - Maybe (emotional detection, quest framework) → Middleware
   - No (a specific guitar academy) → Product

3. **If I deleted this, would Trinity stop being a production studio?**
   - Yes → Core
   - No, but products would have to reinvent it → Middleware
   - No, and only one product cares → Product

### FACES — Case Study

FACES (Fuzzy Affective Cognitive Emotional State) is a sophisticated emotional detection protocol with 283 tests, 12 source files, and zero dependencies.

**Is it core? No.** Trinity doesn't need emotional detection to generate art, voice, video, or code.

**Is it middleware? Yes.** Multiple products could use it — games needing NPC emotion, educational apps needing student awareness, interactive stories needing character responses. It's a reusable library, not a specific app.

**Where it belongs:** Moved to the Semantic Slime project as `faces-protocol` on July 5, 2026. It is now an external product dependency, not a Trinity Middleware crate.

### ADDIECRAPEYE — Case Study

ADDIECRAPEYE is a 12-phase instructional design framework mapping ADDIE + CRAP + EYE to the Hero's Journey.

**Is it core? No.** Trinity doesn't need a 12-phase quest system to generate creative assets.

**Is it middleware? Yes.** Multiple educational products could use it — learning modules, gamified courses, any product wanting a structured design process.

**Where it belongs:** As a Middleware crate (`trinity-iron-road`). The creative pipeline (story → art → voice → video → assemble) is the core process. ADDIECRAPEYE is one way to structure that process for educational products.

---

## State & Storage: Who Owns What

| State Type | Owner | Examples | Storage |
|-----------|-------|----------|---------|
| **Compute State** | Core | VRAM budgets, model occupancy, job queues | In-memory (`hotel_manager.rs`) |
| **System Memory** | Core | RAG index of source code, project knowledge | SQLite (`rag.rs`, `memory.rs`) — isolated by `project_id` |
| **User State** | Product | XP, levels, progress, character sheets | Product's own database |
| **Creative Assets** | Shared | Generated images, voice, video, 3D | File system — saved to Product's workspace folder by Agent Loop |

Trinity Core's RAG system is the identity-critical memory system. It uses ORT embeddings (`ort_embed.rs`) with `all-MiniLM-L6-v2` INT8 ONNX (137MB) for 384-dimensional semantic search. This is what gives Trinity project awareness across sessions. The `memory.rs` system persists context to SQLite, enabling the Agent Loop to recall prior work. These are Core — they are what make Trinity a studio rather than a stateless API proxy.

---

## What This Means for Development

### Core gets maintained, optimized, and documented
- Hotel manager, inference router, creative pipeline, RAG, memory, API, PWA
- These are what make Trinity a studio
- Docs describe how to use the studio, not how to design a lesson

### Middleware gets published as standalone crates
- FACES, ADDIECRAPEYE, Quest, Voice, Daydream
- Each can be used by any product, independently of Trinity Core
- Open-source under Apache 2.0

### Products get built, deployed, and monetized
- Each product is a separate project with its own repo
- Products use Trinity's API (:3000) and ComfyUI (:8188) for generation
- Products can depend on Middleware crates, but never on Core directly
- Products can be open-source or commercial
- Products are where the money comes from

### The studio doesn't depend on any product or middleware
- If FACES is never used in a product, Trinity still works
- If ADDIECRAPEYE is never used in a product, Trinity still works
- If Voix Vive shuts down, Trinity still works
- Trinity is the factory. Middleware is the tooling. Products are the goods.

---

## The Creative Pipeline (Core Process)

This is the core process that Trinity supports:

```
 0. INTENT   (API / PWA)
    → A Product or human triggers a generation request via REST/JSON

 1. STORY    (P: DiffusionGemma)
    → Script, dialogue, game design doc, code

 2. ART      (A: Janus-Pro-7B → ESRGAN upscale)
    → Character art, backgrounds, diagrams, textures

 3. VOICE    (A: VibeVoice-1.5B)
    → Multi-speaker narration, character voices, 90-min capacity

 4. VIDEO    (Tier 2: HunyuanVideo)
    → Animated scenes, technique demonstrations

 5. 3D       (Tier 2: TRELLIS → glTF)
    → Game assets, VR environments, character models

 6. MUSIC    (Tier 2: ACE-Step)
    → Soundtrack, SFX, ambient audio

 7. ASSEMBLE (CPU: Blender + Godot)
    → Video editing, 3D compositing, game development

 8. REVIEW   (Cloud API)
    → External quality check, critique, iteration
    → EXCEPTION to local-first rule: utilized only because local models
      do not yet have the reasoning capacity for reliable final critique.
      This is the ONLY step where data leaves the OS.

 9. VAULT    (File Ops)
    → The Agent Loop saves the final asset to the designated Product
      Workspace folder and returns the file path/URI to the API caller
```

Everything else — quest systems, gamification, emotional detection, instructional design frameworks — is **middleware or product** that uses this pipeline.

---

*This document is the authoritative definition of Trinity's identity. All other docs must conform to it.*
*Last updated: 2026-07-05 — integrated 3-tier architecture, interface contract, state isolation, and pipeline bookends.*
