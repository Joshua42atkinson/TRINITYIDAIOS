# Trinity AI OS — Session Context

> **Last Updated**: April 8, 2026
> **Purpose**: Ground truth for any AI agent starting a session. Read THIS first — not old docs.

---

## Architecture: One Server, One Port

**Everything runs on `127.0.0.1:3000`** via the Trinity Rust backend (`crates/trinity/src/main.rs`).

```
Browser → localhost:3000
          ↓
  1. /api/*           → 85+ Rust API handlers (Axum)
  2. /trinity/*       → Trinity React UI (crates/trinity/frontend/dist/)
  3. /* (fallback)    → LDTAtkinson Portfolio (LDTAtkinson/client/dist/)
```

There is **no port 3001**. There is **no separate Vite dev server** in production. The Rust backend serves both the Trinity capstone UI and the portfolio as static files.

### Two React Frontends (Same Server)

| Frontend | Location | Served At | Stack | Purpose |
|----------|----------|-----------|-------|---------|
| **Trinity UI** | `crates/trinity/frontend/` | `/trinity/*` | React + Vite (vanilla CSS) | Capstone app — Iron Road, Yardmaster, Art Studio |
| **LDT Portfolio** | `LDTAtkinson/client/` | `/*` (fallback) | React + Vite + Tailwind | Professional portfolio — ldtatkinson.com |

---

## Dual Brain Sidecar Model (128GB Strix Halo APU)

### The P.A.R.T.Y. Framework

Trinity's AI is organized into 5 roles across 2 sidecars:

| Letter | Name | Role | Sidecar | Detail |
|--------|------|------|---------|--------|
| **P** | **Pete** | Instructional Designer. The Great Recycler. DM of the Iron Road. | SGLang (8010) | LongCat-Next 74B MoE · **Parallel 2 KV cache** (2× 156K MLA). Pete does the majority of Trinity's work: Socratic mentoring, LitRPG narration, DiNA image gen. He IS the Great Recycler. He breaks character as "Programmer Pete" to get things done and keep users on track. **Pete is NOT a software engineer.** |
| **A** | **Aesthetics** | Support visual/spatial models. | vLLM (8000) | FLUX.1-schnell, CogVideoX-2B, TripoSR. Hotloaded creative generators. |
| **R** | **Research** | Embeddings & permanence. Balances A and T. | vLLM (8000) | RAG embeddings (nomic-embed), vector storage. Ensures the information Pete delivers is balanced between visual (A) and audio (T). |
| **T** | **Tempo** | Audio & music generation. | vLLM (8000) | Acestep 1.5. The audio counterpart to Aesthetics. |
| **Y** | **Yardmaster** | Software engineering subagent. | vLLM (8000) | Qwen3-Coder REAP 25B A3B (GGUF). The engineer that Pete is NOT. Writes Rust, builds React, runs cargo check. |

> **Key insight**: Pete (LongCat) is an *instructional designer*, not a software engineer.
> Instructional design IS NOT software engineering. When Pete needs code written,
> he delegates to the Yardmaster (REAP). Pete focuses on pedagogy, narrative, and
> the Socratic loop. REAP focuses on compiling code.

> ⚠️ **Open Question**: DiNA image generation blocks SGLang for ~10 min.
> Can vLLM run Yardmaster REAP concurrently? Needs testing.

---

## Startup Sequence

```bash
# 1. SGLang — Pete (Port 8010) — ~2.5 min, ~84GB
#    LongCat-Next 74B MoE · Parallel 2 KV cache (2× 156K MLA)
#    Serves: P (Pete/Recycler) — Socratic brain, narrative, DiNA images
distrobox enter sglang-engine -- bash ./longcat_omni_sidecar/launch_engine.sh

# 2. vLLM — A.R.T.Y. Hub (Port 8000) — ~24GB
#    Serves: A (Aesthetics) + R (Research/RAG) + T (Tempo) + Y (Yardmaster REAP)
#    NOT YET WIRED — needs concurrent operation testing with SGLang
# vllm serve Hotload-Hub --port 8000 --max-model-len 32768

# 3. Trinity Backend (Port 3000) — serves API + both UIs
cargo run --release -p trinity
```

---

## Source Code Map

### Rust Backend (`crates/trinity/src/` — 39 files, ~24,000 LOC)

| File | Purpose |
|------|---------|
| `main.rs` | Server entry — Axum setup, 85+ routes, static serving, SSE |
| `conductor_leader.rs` | ADDIECRAPEYE phase orchestrator |
| `agent.rs` | Yardmaster autonomous agent |
| `inference_router.rs` | Multi-backend inference routing |
| `quests.rs` | Iron Road quest engine |
| `narrative.rs` | LitRPG narrative weaving |
| `vaam.rs` / `vaam_bridge.rs` | Vocabulary mining, Coal tracking |
| `scope_creep.rs` | PEARL-aware drift detection |
| `perspective.rs` | Friction/vulnerability tracking |
| `creative.rs` | Image/video/music generation pipeline |
| `voice.rs` / `telephone.rs` | Voice synthesis, WebSocket audio |
| `cow_catcher.rs` | Runtime safety, self-repair |
| `character_sheet.rs` / `character_api.rs` | Player identity CRUD |
| `persistence.rs` / `journal.rs` | SQLite persistence |
| `export.rs` / `eye_container.rs` | EYE HTML5 package export |
| `tools.rs` | 38 core tools (file I/O, cargo check, etc.) |

### Trinity Frontend (`crates/trinity/frontend/src/` — 24 components, ~8,900 LOC)

| Component | Purpose |
|-----------|---------|
| `App.jsx` | Root — SubjectPicker, mode switching |
| `PhaseWorkspace.jsx` | ADDIECRAPEYE Zen-mode chat |
| `CharacterSheet.jsx` | Player identity + Hook Deck |
| `Yardmaster.jsx` | Agent IDE |
| `ArtStudio.jsx` | Creative pipeline UI |
| `GameHUD.jsx` | Coal/Steam/XP overlay |
| `NavBar.jsx` | Navigation |
| `PlayerHandbookElearning.jsx` | Interactive handbook viewer |

### Config

| File | Purpose |
|------|---------|
| `configs/runtime/default.toml` | Inference backend config (ports, model names) |
| `run-trinity.sh` | One-click startup |

---

## Architecture Reminders

- **One Brain, Two Personas**: LongCat is both Instructor (Socratic) and Storyteller (LitRPG). System prompt switching, no model swap.
- **The User IS the protagonist**: ADDIECRAPEYE phases are narrative chapters.
- **Port 3000 ONLY**: No port 3001, no separate Vite dev server. The Rust backend serves everything.
- **Static VRAM Budget**: LongCat owns GPU (~84GB). Pete runs on CPU (zero VRAM).
- **Distrobox**: LongCat runs in `sglang-engine` container. Ports on 127.0.0.1.
- **DO NOT create new UI projects**. Edit the existing frontends in their locations.
