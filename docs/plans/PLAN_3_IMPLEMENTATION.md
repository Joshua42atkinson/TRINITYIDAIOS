# PLAN 3: Implementation
## Trinity ID AI OS — Build Order for Full Operation

*"Do it right, do it light. Do it wrong, do it long."*

---

## Prerequisites (from Plans 1 & 2)

Before implementation begins:
- [ ] Plan 2 Hygiene complete (stale refs fixed, workspace clean)
- [ ] Plan 1 Architecture understood by executing model (this document serves as the spec)

---

## Phase 1: Pete Talks (Conductor Online)
**Goal:** User can chat with Mistral Small 4 via the Iron Road UI.
**Effort:** Small — mostly done, needs a live test.

| Step | Task | Files | Status |
|------|------|-------|--------|
| 1.1 | Launch llama-server with Mistral Small 4 split GGUF manually | Shell command | Ready to test |
| 1.2 | Verify `/api/chat` returns real responses from Pete | `main.rs`, `inference.rs` | Code ready, needs live test |
| 1.3 | Verify `/api/chat/stream` SSE works with Pete | `agent.rs` | Code ready |
| 1.4 | Test tool-use loop (Pete calls tools, gets results, responds) | `agent.rs`, `tools.rs` | Code ready |

**Launch command:**
```bash
./bin/llama-server \
  -m ~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
  -c 32768 --port 8080 -ngl 99 -fa -ctk q4_0 -ctv q4_0 --no-mmap --ctx-shift
```

**Verify command:**
```bash
curl http://127.0.0.1:8080/health
curl -X POST http://127.0.0.1:3000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello Pete, what is ADDIECRAPEYE?", "use_rag": true}'
```

---

## Phase 2: ADDIECRAPEYE Orchestrates (State Machine Live)
**Goal:** The ConductorLeader drives quest progression through all 12 phases with real LLM calls.
**Effort:** Medium — `call_pete()` is wired, needs quest seeding and API endpoint.

| Step | Task | Files |
|------|------|-------|
| 2.1 | Create seed quests for the Iron Road tutorial (JSON files in `quests/board/`) | New quest files |
| 2.2 | Wire `/api/orchestrate` endpoint in main.rs to call `ConductorLeader::orchestrate()` | `main.rs` |
| 2.3 | Wire the Iron Road UI "State your action" input to call `/api/orchestrate` instead of `/api/chat` for quest-bound interactions | `projects.html` |
| 2.4 | Test full cycle: Analysis → Design → Development (first 3 stations) | Manual test |

---

## Phase 3: Iron Road Narrative (The Book Writes Itself)
**Goal:** Quest events generate LitRPG prose chapters stored in the Book of the Bible.
**Effort:** Medium — 3 empty files to implement.

| Step | Task | Files |
|------|------|-------|
| 3.1 | Implement `book.rs` — append-only markdown chapter ledger with SSE broadcast | `trinity-iron-road/src/book.rs` |
| 3.2 | Implement `narrative.rs` — takes quest events + CharacterSheet → LitRPG prose via Pete | `trinity-iron-road/src/narrative.rs` |
| 3.3 | Implement `great_recycler.rs` — background task that synthesizes Journal entries into book chapters | `trinity-iron-road/src/great_recycler.rs` |
| 3.4 | Wire into `/api/book/stream` SSE endpoint (already exists in main.rs, currently returns placeholder) | `main.rs` |
| 3.5 | Update Iron Road UI right page to render book chapters from SSE stream | `projects.html` |

---

## Phase 4: PyO3 Foundation (Python Bridge Crate)
**Goal:** Create a shared `trinity-python-bridge` crate that isolates all Python interaction behind a clean Rust API.
**Effort:** Medium — foundational work that unlocks Phases 5, 6, and 7.

| Step | Task | Files |
|------|------|-------|
| 4.1 | Create `crates/trinity-python-bridge/` with PyO3 dependency | New crate |
| 4.2 | Implement `VllmBridge` — spawns vLLM `AsyncLLMEngine` in-process via PyO3 | New file |
| 4.3 | Implement `ComfyBridge` — HTTP client to ComfyUI (already partly in `trinity-comfy`) | Consolidate |
| 4.4 | Implement `BlenderBridge` — subprocess or PyO3 to Blender bpy | Already in `blender.rs` |
| 4.5 | Implement `AudioBridge` — PyO3 wrapper for Ming's talker component (STT/TTS) | New file |
| 4.6 | Add `python` feature flag to sidecar crate that enables PyO3 | `trinity-sidecar/Cargo.toml` |

**Key design rule:** The PyO3 bridge runs INSIDE the sidecar process. The main trinity server NEVER imports it. If Python crashes, only the sidecar dies.

---

## Phase 5: Ming Online (Yardmaster via vLLM)
**Goal:** Ming-flash-omni-2.0 serves inference via vLLM, callable from the Yardmaster sidecar.
**Effort:** Large — requires vLLM install with ROCm, custom client for Ming's protocol.

| Step | Task | Files |
|------|------|-------|
| 5.1 | Install PyTorch with ROCm support for AMD 395+ | System install |
| 5.2 | Install vLLM from source with ROCm backend | System install |
| 5.3 | Test Ming's `talker_vllm_server.py` standalone | `~/trinity-models/safetensors/Ming-flash-omni-2.0/talker/` |
| 5.4 | Implement `MingClient` in `trinity-python-bridge` — wraps Ming's custom `/generate` protocol | New file |
| 5.5 | Wire MingClient into the Yardmaster sidecar role | `trinity-sidecar/src/workflow.rs` |
| 5.6 | Test: Yardmaster receives a Development phase quest → Ming generates code | Manual test |

**Note:** Ming uses `TokensPrompt` with `prompt_token_ids` + `multi_modal_data`, NOT OpenAI-compatible `/v1/chat/completions`. The `vllm_batcher.rs` client will NOT work for Ming. We need the custom client from 5.4.

---

## Phase 6: ART Production Line
**Goal:** Aesthetics (ComfyUI + Blender), Research (Crow + REAP), and Tempo (OmniCoder + MusicUI) all operational.
**Effort:** Medium per sub-component.

### 6a. Aesthetics (A)
| Step | Task |
|------|------|
| 6a.1 | Verify ComfyUI runs and accepts workflow JSON at `:8188` |
| 6a.2 | Wire `ComfyUIClient` (exists in `trinity-comfy/src/comfyui.rs`) into sidecar creative endpoints |
| 6a.3 | Test: Artist sidecar generates an image from a prompt |
| 6a.4 | Wire Blender subprocess for 3D asset generation |

### 6b. Research (R)
| Step | Task |
|------|------|
| 6b.1 | Launch Crow 9B on a secondary llama-server port (`:8081`) |
| 6b.2 | Launch REAP 25B on same port (swap based on task) |
| 6b.3 | Wire into sidecar `/think` and `/code` endpoints |
| 6b.4 | Test: Research agent analyzes a code file and suggests improvements |

### 6c. Tempo (T)
| Step | Task |
|------|------|
| 6c.1 | Set up MusicUI/ACE-Step on `:8086` |
| 6c.2 | Wire `MusicClient` (exists in `trinity-comfy/src/music.rs`) into sidecar |
| 6c.3 | Wire OmniCoder 9B for code-to-music mapping |
| 6c.4 | Test: Generate background music based on cognitive load state |

---

## Phase 7: Voice Pipeline (Level 1 Headless)
**Goal:** Full audio-only interface — user speaks, Pete responds with voice, Iron Road narrates.
**Effort:** Large — requires real STT/TTS models.

| Step | Task | Files |
|------|------|-------|
| 7.1 | Replace PersonaPlex stubs with real STT (Whisper or Ming's audio encoder via PyO3) | `trinity-voice/src/personaplex.rs` |
| 7.2 | Implement real TTS (Ming's talker decoder or external TTS model) | `trinity-voice/src/personaplex.rs` |
| 7.3 | Build the headless game loop: `listen → transcribe → Pete → synthesize → speak` | New file in `trinity/src/voice_loop.rs` |
| 7.4 | Wire Iron Road narration: book chapters read aloud via TTS | `projects.html` TTS button + backend route |
| 7.5 | Test: User says "Start the Iron Road" → Pete narrates Chapter 1 → User responds vocally | Manual test |

---

## Phase 8: Data Pipeline (Graph RAG + Vector DB)
**Goal:** Semantic search and entity relationships power the inter-sidecar communication.
**Effort:** Medium — SurrealDB and Qdrant clients exist, need wiring.

| Step | Task | Files |
|------|------|-------|
| 8.1 | Implement `embeddings.rs` — generate embeddings via fastembed or PyO3 | `trinity-data/src/embeddings.rs` |
| 8.2 | Wire Qdrant vector DB for document search (client exists in `rag/vector_db.rs`) | Integration |
| 8.3 | Wire SurrealDB graph RAG for entity relationships (client exists in `rag/graph_rag.rs`) | Integration |
| 8.4 | Feed quest events, journal entries, and code changes into both databases | `trinity-quest/`, `trinity-iron-road/` |
| 8.5 | Expose `/api/search` endpoint for semantic RAG queries | `main.rs` |

---

## Dependency Graph

```
Phase 1 (Pete Talks)
  │
  ▼
Phase 2 (ADDIECRAPEYE)  ──→  Phase 3 (Narrative)
  │
  ▼
Phase 4 (PyO3 Foundation)
  │
  ├──→ Phase 5 (Ming/Yardmaster)
  ├──→ Phase 6 (ART Pipeline)
  └──→ Phase 7 (Voice Pipeline)
  
Phase 8 (Data Pipeline) can run in parallel with 4-7
```

---

## Estimated Effort

| Phase | Sessions | Blocking? |
|-------|----------|-----------|
| 1. Pete Talks | 1 (just a test) | YES — everything depends on this |
| 2. ADDIECRAPEYE | 1-2 | YES — orchestration drives all work |
| 3. Narrative | 1-2 | No — enhances gameplay |
| 4. PyO3 Foundation | 2-3 | YES — unlocks 5, 6, 7 |
| 5. Ming/Yardmaster | 3-4 | No — Pete can solo most tasks |
| 6. ART Pipeline | 2-3 | No — assets can be manual initially |
| 7. Voice Pipeline | 2-3 | No — web UI works without voice |
| 8. Data Pipeline | 1-2 | No — PostgreSQL RAG works for now |

**Critical path:** 1 → 2 → 4 → 5 (Pete → ADDIECRAPEYE → PyO3 → Ming)
**Quick wins:** 1 → 2 → 3 (Pete → ADDIECRAPEYE → Narrative = playable Iron Road)
