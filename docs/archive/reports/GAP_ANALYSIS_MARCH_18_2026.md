# Trinity ID AI OS — Gap Analysis
## March 18, 2026 — Honest Audit

---

## 1. HARDWARE (VERIFIED FROM FILESYSTEM)

| Spec | Value |
|------|-------|
| Machine | GMKtek EVO X2 128GB |
| CPU/GPU | AMD Strix Halo Zen5 AMD 395+ |
| Unified Memory | 128 GB LPDDR5X |
| NPU | XDNA 2 (55 TOPS) |

---

## 2. MODELS ON DISK (VERIFIED FROM `ls -lh`)

| Model | File | Size | Format | Role |
|-------|------|------|--------|------|
| Mistral Small 4 119B MoE | `Mistral-Small-4-119B-2603-Q4_K_M-0000{1,2}-of-00002.gguf` | 68 GB (37+31) | GGUF split | **P — Conductor (Pete)** |
| Ming-flash-omni-2.0 | `Ming-flash-omni-2.0/model-0000{1..42}-of-00042.safetensors` | ~195 GB | Safetensors | **Y — Yardmaster** |
| Crow 9B Opus | `Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf` | 5.3 GB | GGUF | **A-R-T (R — Research)** |
| Qwen3-Coder-REAP-25B MoE | `Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf` | 15 GB | GGUF | **A-R-T (R — Research)** |
| OmniCoder 9B | `OmniCoder-9B-Q4_K_M.gguf` | 5.4 GB | GGUF | **A-R-T (T — Tempo)** |
| GPT-OSS 20B | `gpt-oss-20b-UD-Q4_K_XL.gguf` | 12 GB | GGUF | Legacy conductor |
| Qwen3.5-27B Claude Opus | `Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled.i1-Q6_K.gguf` | 21 GB | GGUF | Evaluator / Shield |
| Qwen3.5-35B-A3B | `Qwen3.5-35B-A3B-Q4_K_M.gguf` | 20 GB | GGUF | Visionary |
| MiniMax-M2-5-REAP-50 | `MiniMax-M2-5-REAP-50-Q4_K_M.gguf` | 66 GB | GGUF | Reserve |
| Step-3.5-Flash-REAP-121B | `Step-3.5-Flash-REAP-121B-A11B.Q4_K_S.gguf` | 83 GB | GGUF | Reserve |

**External Tools (not AI models):**
- ComfyUI — installed at `/home/joshua/ComfyUI/` (A — Aesthetics)
- Blender — TBD (A — Aesthetics)
- MusicUI — TBD (T — Tempo)

---

## 3. WHAT ACTUALLY WORKS RIGHT NOW

### 3a. Axum HTTP Server (`crates/trinity/src/main.rs`)
- **STATUS: RUNS** on port 3000
- Serves static HTML files (projects.html, dev.html, creator.html)
- Chat endpoint (`/api/chat`) talks to a **llama.cpp server at port 8080**
- Streaming SSE endpoint (`/api/chat/stream`) for agentic tool-use loop
- RAG ingestion and search via PostgreSQL
- Tool system: read_file, write_file, list_dir, shell, search_files, sidecar_status, sidecar_start
- Book SSE stream (`/api/book/stream`) — endpoint exists but book data is placeholder

### 3b. Sidecar System (`crates/trinity-sidecar/`)
- **STATUS: COMPILES, PARTIALLY FUNCTIONAL**
- Full Axum API on port 8090 with quest management, creative endpoints, autonomous loop
- WorkflowEngine with quest board, claim, execute
- llama.cpp client for primary/secondary model inference
- Role-based prompts (engineer, artist, evaluator, pete, brakeman, visionary)
- **BUT**: tool_sidecar_start in main.rs still launches `gpt-oss-20b` on port 8080 — NOT Mistral Small 4

### 3c. Inference Clients
- `inference.rs` — **WORKS** — OpenAI-compatible client for llama.cpp at port 8080
- `vllm_batcher.rs` — **EXISTS, NOT WIRED** — Clean vLLM client, but main.rs doesn't use it
- `vllm_client.rs` — **EXISTS, NOT WIRED** — More robust vLLM client with retries, also not used

### 3d. Voice Pipeline (`crates/trinity-voice/`)
- `audio.rs` — **REAL CODE** — cpal input/rodio output, async channels, audio I/O works
- `personaplex.rs` — **STUB** — All methods return hardcoded placeholders

### 3e. ADDIECRAPEYE State Machine (`crates/trinity/src/conductor_leader.rs`)
- **STATUS: COMPILES, ALL 12 PHASE HANDLERS ARE STUBS**
- The state machine structure exists (Analysis → Design → ... → Execution → cycle)
- `shift_gears()` correctly maps phases to sidecar roles
- But every `analyze_quest()`, `design_quest()`, etc. returns hardcoded `OrchestrationResponse` with no model calls

### 3f. Iron Road Narrative (`crates/trinity-iron-road/`)
- `pete_core.rs` — **REAL CODE** — `PeteCore` calls vLLM via `VllmEngineClient` for Socratic dialogue
- `vaam/cognitive_load.rs` — **REAL CODE** — CognitiveLoadManager with tier tracking
- `vaam/madlibs.rs` — **REAL CODE** — MadLib template system for vocabulary
- `vaam/litrpg.rs` — **REAL CODE** — Handbook section generator from mastered words
- `book.rs` — **EMPTY**
- `narrative.rs` — **EMPTY**
- `great_recycler.rs` — **EMPTY**

---

## 4. WHAT IS BROKEN OR MISSING

### CRITICAL (Blocks Level 1 Headless)

| # | Gap | Impact |
|---|-----|--------|
| C1 | **vLLM is not installed** on this machine. `pip3 show vllm` returns nothing. `torch` is CPU-only (2.10.0+cpu). | Cannot serve Ming (195GB safetensors) at all. |
| C2 | **main.rs hardcodes llama.cpp at port 8080** as the sole inference backend. No vLLM routing. | Even if vLLM were running, the server wouldn't talk to it. |
| C3 | **sidecar_start launches gpt-oss-20b**, not Mistral Small 4. The model path, context length, and split-file handling are all wrong for the new conductor. | Cannot start the actual Conductor model. |
| C4 | **ADDIECRAPEYE phase handlers are all stubs** returning hardcoded responses. No model calls, no real orchestration. | The PARTY cannot autonomously progress through stations. |
| C5 | **book.rs, narrative.rs, great_recycler.rs are EMPTY**. | The Iron Road cannot generate chapters, record the user's journey, or synthesize the lite-novel. |
| C6 | **PersonaPlex is fully stubbed**. No actual speech-to-text or text-to-speech. | Level 1 headless (audio-only) mode is impossible. |

### IMPORTANT (Blocks Full Gameplay)

| # | Gap | Impact |
|---|-----|--------|
| I1 | No embeddings pipeline (embeddings.rs is empty, qdrant/surreal modules empty). | RAG uses PostgreSQL full-text search only, no semantic vector search. |
| I2 | Creative endpoints (ComfyUI, Blender, Music) exist in sidecar API but have no real client implementations wired to external tools. | ART production line cannot generate assets. |
| I3 | Quest board has no seed quests for the Iron Road tutorial. | A new user has nothing to play. |
| I4 | `character_sheet.rs` model assignments still reference old models (gpt-oss-20b as default in some paths). | Backend data doesn't match actual PARTY. |

---

## 5. MING-FLASH-OMNI-2.0 ANALYSIS

Ming ships with its own vLLM integration scripts in `talker/`:
- `talker_vllm_server.py` — FastAPI server using `AsyncLLMEngine` 
- `vllm_infer.py` — Inference generator with streaming
- `async_vllm_infer.py` — Async batch inference
- `talker_vllm_client.py` — HTTP client

**Architecture:** Ming is a `BailingMM2NativeForConditionalGeneration` model:
- LLM backbone: `BailingMoeV2ForCausalLM` — 256 experts, 8 active per token, 4096 hidden, 32 layers
- Vision: `Qwen3MoeVisionTransformer` — 27 layers, patch_size 16
- Audio: Whisper encoder config (32 layers, 1280 state)
- Max position embeddings: 32768

**Key fact:** Ming's vLLM server does NOT use OpenAI-compatible `/v1/chat/completions`. It uses a custom `/generate` endpoint that takes `prompt_token_ids` and `prompt_embeds` directly. This means our `vllm_batcher.rs` (which calls `/v1/chat/completions`) **will not work with Ming as-is**.

---

## 6. MISTRAL SMALL 4 ANALYSIS

- 119B parameter MoE, Q4_K_M quantized across 2 GGUF shards (68GB total)
- ~6.5B active parameters per token → 40+ tok/s on Strix Halo
- 256K context window with Q4 KV cache quantization
- Vision capable (multimodal)
- Runs via **llama.cpp** (GGUF format), NOT vLLM
- llama-server binary exists at `/home/joshua/Workflow/desktop_trinity/bin/llama-server`

---

## 7. IMPLEMENTATION PRIORITY ORDER

### Phase A: Get Pete (Conductor) Running — Mistral Small 4 via llama.cpp
1. Update `tool_sidecar_start` to launch Mistral Small 4 (split GGUF) instead of gpt-oss-20b
2. Update inference.rs to handle the new model's response format
3. Test chat via `/api/chat` with real Conductor model
4. This alone gives us a working Level 1 text interface with Pete

### Phase B: Wire ADDIECRAPEYE to Pete
1. Replace stub phase handlers in `conductor_leader.rs` with real LLM calls through the inference client
2. Each phase handler builds a phase-appropriate system prompt and calls Pete
3. Pete's responses drive the quest progression

### Phase C: Install vLLM + Serve Ming (Yardmaster)
1. Install vLLM with ROCm/AMD support (requires PyTorch with ROCm)
2. Write a Ming-specific HTTP client in Rust that speaks Ming's custom `/generate` protocol
3. Add a `VLLM_URL` env var to main.rs alongside `LLAMA_URL`
4. Route Yardmaster requests to vLLM, Conductor requests to llama.cpp

### Phase D: Implement Iron Road Narrative
1. Implement `book.rs` — append-only chapter ledger
2. Implement `narrative.rs` — LitRPG prose generation from quest events
3. Implement `great_recycler.rs` — synthesize journal + quests into book chapters
4. Wire into the existing SSE `/api/book/stream` endpoint

### Phase E: Voice Pipeline (Level 1 Headless)
1. Integrate a real STT model (Whisper or Ming's audio encoder)
2. Integrate a real TTS model (could use Ming's talker component)
3. Replace PersonaPlex stubs with actual model inference
4. Build the audio-only game loop: listen → transcribe → Pete responds → synthesize → speak

### Phase F: ART Production Line
1. Wire ComfyUI HTTP client to sidecar creative endpoints
2. Wire Blender Python bridge
3. Wire MusicUI integration
4. Load Crow 9B + REAP 25B + OmniCoder 9B as the R and T models
