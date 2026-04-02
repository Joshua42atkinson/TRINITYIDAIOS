# PLAN 1: Architecture
## Trinity ID AI OS — System Design for Full Operation

---

## 1. The Three Layers

| Layer | Purpose | Status |
|-------|---------|--------|
| **L1: Headless Server** | Audio-only interface, agentic orchestration, model serving | Axum on :3000 runs, inference stubs replaced with real calls, audio I/O exists but no STT/TTS models |
| **L2: Web UI** | Visual game interface (Iron Road book, Yardmaster IDE, ART studio) | 3 HTML pages serve from :3000, VAAM highlighting works, chat sends to llama.cpp |
| **L3: Spatial/XR** | Bevy 3D game engine, future VR/XR | 40K lines exist in archive, currently blocked by rendering issues — deferred |

---

## 2. Inference Routing Architecture

Trinity runs multiple AI models simultaneously. Each model has a specific serving strategy based on its format and size.

```
┌─────────────────────────────────────────────────────────────────┐
│                    TRINITY MAIN SERVER (:3000)                  │
│                         Axum / Rust                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────────┐ │
│  │ /api/chat    │  │ /api/tools   │  │ /api/orchestrate      │ │
│  │ (user ↔ Pete)│  │ (agentic)    │  │ (ADDIECRAPEYE)        │ │
│  └──────┬───────┘  └──────┬───────┘  └───────────┬───────────┘ │
│         │                 │                       │             │
│         ▼                 ▼                       ▼             │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │            CONDUCTOR LEADER (Rust)                      │   │
│  │   Routes requests to the correct model/sidecar          │   │
│  │   Manages ADDIECRAPEYE state machine                    │   │
│  │   Hotel pattern: ONE heavyweight at a time              │   │
│  └─────────────┬──────────────┬────────────────┬───────────┘   │
│                │              │                │               │
└────────────────┼──────────────┼────────────────┼───────────────┘
                 │              │                │
    ┌────────────▼───┐  ┌──────▼──────┐  ┌──────▼──────────────┐
    │ llama-server   │  │ SIDECAR     │  │ SIDECAR             │
    │ (:8080)        │  │ (:8090)     │  │ (vLLM :8000)        │
    │                │  │             │  │                      │
    │ GGUF models:   │  │ Rust +PyO3  │  │ Python (PyO3-spawned)│
    │ • Pete (68GB)  │  │ • Crow 9B   │  │ • Ming 195GB        │
    │ • GPT-OSS 12GB │  │ • REAP 25B  │  │ • Custom /generate  │
    │ • OmniCoder 9B │  │ • Opus 27B  │  │   protocol          │
    │                │  │             │  │                      │
    │ OpenAI-compat  │  │ Quest API   │  │ NOT OpenAI-compat   │
    │ /v1/chat/...   │  │ Tool exec   │  │ Needs custom client │
    └────────────────┘  └─────────────┘  └──────────────────────┘
```

### Serving Strategy by Model

| Model | Size | Format | Serving | Port | Protocol |
|-------|------|--------|---------|------|----------|
| Mistral Small 4 119B (Pete) | 68GB GGUF | Split GGUF | llama-server | 8080 | OpenAI-compat |
| Ming-flash-omni-2.0 (Yardmaster) | ~195GB safetensors | Safetensors | vLLM via PyO3 | 8000 | Custom `/generate` |
| Crow 9B (ART-R) | 5.3GB GGUF | GGUF | llama-server (swappable) | 8081 | OpenAI-compat |
| REAP 25B (ART-R) | 15GB GGUF | GGUF | llama-server (swappable) | 8081 | OpenAI-compat |
| OmniCoder 9B (ART-T) | 5.4GB GGUF | GGUF | llama-server (swappable) | 8082 | OpenAI-compat |
| ComfyUI (ART-A) | N/A | Python | HTTP sidecar | 8188 | ComfyUI REST |
| Blender (ART-A) | N/A | Python | Subprocess / PyO3 | N/A | Script gen |
| MusicUI (ART-T) | N/A | Python | HTTP sidecar | 8086 | REST |

---

## 3. PyO3 Sidecar Isolation Pattern

**Key Principle:** Python is isolated INSIDE the sidecar process. The main Rust server never imports Python. Each sidecar is its own process with its own crash domain.

```
┌──────────────────────────────────────┐
│        SIDECAR PROCESS (Rust)        │
│                                      │
│  ┌────────────────────────────────┐  │
│  │  Axum API (:8090)              │  │
│  │  • /status                     │  │
│  │  • /think                      │  │
│  │  • /quest/execute              │  │
│  │  • /creative/image             │  │
│  └──────────┬─────────────────────┘  │
│             │                        │
│  ┌──────────▼─────────────────────┐  │
│  │  PyO3 Bridge (Rust ↔ Python)   │  │
│  │                                │  │
│  │  Python::with_gil(|py| {       │  │
│  │    // vLLM engine              │  │
│  │    // ComfyUI workflow gen     │  │
│  │    // ONNX Runtime (NPU)       │  │
│  │    // Blender bpy              │  │
│  │  })                            │  │
│  └────────────────────────────────┘  │
│                                      │
│  Python is CONTAINED here.           │
│  If Python panics, this process      │
│  crashes — NOT the main server.      │
└──────────────────────────────────────┘
```

### What Each Sidecar Owns

| Sidecar | Rust Components | PyO3/Python Components |
|---------|----------------|----------------------|
| **P (Conductor/Pete)** | ADDIECRAPEYE state machine, quest orchestration, VAAM | llama-server subprocess (GGUF), future: vision via PyO3 |
| **A (Aesthetics)** | Asset pipeline, file management | ComfyUI HTTP client, Blender subprocess/PyO3 |
| **R (Research)** | Code analysis, document search, RAG | llama-server subprocess for Crow/REAP |
| **T (Tempo)** | Music scheduling, flow state | MusicUI HTTP client, audio processing |
| **Y (Yardmaster)** | Worldbuilding orchestration | vLLM via PyO3 for Ming omni-modal inference |

---

## 4. Inter-Sidecar Communication

Sidecars do NOT call each other directly. All communication flows through the **Conductor** via a shared message bus.

```
                    ┌──────────────────────┐
                    │   CONDUCTOR (Pete)   │
                    │                      │
                    │  ADDIECRAPEYE State   │
                    │  Machine decides     │
                    │  who does what        │
                    └──┬───┬───┬───┬───┬──┘
                       │   │   │   │   │
              ┌────────┘   │   │   │   └────────┐
              ▼            ▼   ▼   ▼            ▼
         ┌────────┐  ┌────┐ ┌───┐ ┌────┐  ┌────────┐
         │  ART-A │  │R   │ │ T │ │ Y  │  │ VAAM   │
         │Aesthet.│  │Res.│ │Tem│ │Yard│  │ Engine │
         └────────┘  └────┘ └───┘ └────┘  └────────┘
```

### Communication Channels

1. **Game State (VAAM):** Shared vocabulary acquisition state. The Conductor reads VAAM scores to decide cognitive load and adjust difficulty.
2. **Graph RAG (SurrealDB):** Entity relationships stored in the graph database. Any sidecar can query context about what the user is building.
3. **Vector DB (Qdrant):** Document embeddings for semantic search. Used by Research (R) to find relevant code/docs.
4. **Quest Board (PostgreSQL):** Persistent quest state. The Conductor writes quests, sidecars claim and execute them.
5. **SSE Broadcast:** Real-time updates to the web UI via Server-Sent Events.

### Recommendation Protocol

When a sidecar wants to recommend a change to another sidecar's domain:

```
1. Sidecar R (Research) discovers a code pattern issue
2. R writes a recommendation to the Quest Board:
   { type: "recommendation", from: "research", to: "yardmaster",
     content: "Function X should use pattern Y", priority: 3 }
3. Conductor picks it up during next ADDIECRAPEYE cycle
4. Conductor routes it to Yardmaster during Development phase
5. Yardmaster executes or rejects (with reason logged to Journal)
```

---

## 5. Memory Architecture

```
┌─────────────────────────────────────────────────┐
│                 128 GB Unified LPDDR5X          │
│                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐  │
│  │ Pete     │  │ Sidecar  │  │ System +     │  │
│  │ 68GB     │  │ (swap)   │  │ PostgreSQL   │  │
│  │ (always  │  │ 5-20GB   │  │ + Qdrant     │  │
│  │  loaded) │  │          │  │ ~10GB        │  │
│  └──────────┘  └──────────┘  └──────────────┘  │
│                                                  │
│  Remaining ~30-50GB available for:              │
│  • vLLM Ming (CPU offload / disk cache)         │
│  • ComfyUI SDXL Turbo (6.5GB)                  │
│  • OS + applications                            │
└─────────────────────────────────────────────────┘
```

**Hotel Pattern:** Only ONE heavyweight sidecar loaded at a time. The Conductor manages swaps via the `manage_hotel_sidecars()` method, which maps ADDIECRAPEYE phases to sidecar roles.

---

## 6. File Organization (Target State)

```
crates/
├── trinity/                 # Main binary — Axum server, routes, UI
├── trinity-protocol/        # Shared types — CharacterSheet, VAAM, quests
├── trinity-inference/       # Inference clients — llama.cpp, vLLM, PyO3 bridge
├── trinity-iron-road/       # Iron Road game — book, narrative, PeteCore, VAAM engine
├── trinity-quest/           # Quest system — board, state, persistence
├── trinity-data/            # Data layer — PostgreSQL, Qdrant, SurrealDB, RAG
├── trinity-sidecar/         # Sidecar binary — role system, quest execution, PyO3
├── trinity-sidecar-conductor/ # Conductor mini-bible and prompts
├── trinity-comfy/           # ART pipeline — ComfyUI, Blender, Music, ADDIECRAPEYE creative
├── trinity-voice/           # Audio I/O — cpal, rodio, PersonaPlex stubs
├── trinity-addie/           # ADDIE tutorial content — genre select, vocab, party config
├── trinity-eye/             # Vision processing — screenshot analysis, UI evaluation
├── trinity-crap/            # Data pipeline — CRAP phases of ADDIECRAPEYE
├── trinity-render/          # Bevy UI (deferred) — dockable, graphics, screens
├── trinity-client/          # Client utilities
├── trinity-dev/             # Dev tools — two-panel UI, Bevy ECS plugins
└── archive/                 # Previous implementations for reference
```
