# PLAN 3 v2: Implementation — vLLM-Only Architecture
## Trinity ID AI OS — Build Order (Supersedes v1)

*"Do it right, do it light. Do it wrong, do it long."*

---

## Phase 0: vLLM + ROCm Installation (BLOCKER — nothing works without this)

| Step | Task | Notes |
|------|------|-------|
| 0.1 | Install PyTorch with ROCm 6.2 for AMD 395+ | `pip3 install torch --index-url https://download.pytorch.org/whl/rocm6.2` |
| 0.2 | Install vLLM | `pip3 install vllm` (or from source if ROCm needs custom build) |
| 0.3 | Verify GPU detection | `python3 -c "import torch; print(torch.cuda.is_available())"` |
| 0.4 | Merge Mistral Small 4 split GGUF | `gguf-split --merge` the 2 shards into one 68GB file |
| 0.5 | Test: `vllm serve` with merged Pete GGUF | Verify OpenAI-compat API works at :8000 |
| 0.6 | Test: `vllm serve` with Ming safetensors | Verify with `--trust-remote-code` |

---

## Phase 1: Pete Talks (Conductor Online via vLLM)

| Step | Task | Files |
|------|------|-------|
| 1.1 | Update `main.rs` — replace `LLAMA_URL` with `VLLM_URL` as sole inference endpoint | `main.rs` |
| 1.2 | Update `inference.rs` — point to vLLM (same OpenAI-compat API, minimal change) | `inference.rs` |
| 1.3 | Update `tools.rs` — replace llama-server launch with vLLM management | `tools.rs` |
| 1.4 | Test `/api/chat` returns real responses from Pete via vLLM | Manual test |
| 1.5 | Test `/api/chat/stream` SSE streaming | Manual test |

---

## Phase 2: ADDIECRAPEYE Orchestrates

| Step | Task | Files |
|------|------|-------|
| 2.1 | Create seed quests for Iron Road tutorial | `quests/board/` |
| 2.2 | Wire `/api/orchestrate` endpoint to `ConductorLeader::orchestrate()` | `main.rs` |
| 2.3 | `call_pete()` already uses HTTP — just verify it works with vLLM URL | `conductor_leader.rs` |
| 2.4 | Test full Analysis → Design → Development cycle | Manual test |

---

## Phase 3: Iron Road Narrative

| Step | Task | Files |
|------|------|-------|
| 3.1 | Implement `book.rs` — append-only chapter ledger | `trinity-iron-road/src/book.rs` |
| 3.2 | Implement `narrative.rs` — LitRPG prose generation via Pete | `trinity-iron-road/src/narrative.rs` |
| 3.3 | Implement `great_recycler.rs` — background journal → book synthesis | `trinity-iron-road/src/great_recycler.rs` |
| 3.4 | Wire into `/api/book/stream` SSE endpoint | `main.rs` |

---

## Phase 4: PyO3 Foundation (vLLM Management)

| Step | Task | Files |
|------|------|-------|
| 4.1 | Create `crates/trinity-python-bridge/` with PyO3 | New crate |
| 4.2 | Implement `VllmManager` — start/stop/swap models via PyO3 | New file |
| 4.3 | Implement `ComfyBridge` — HTTP client to ComfyUI | Consolidate from `trinity-comfy` |
| 4.4 | Implement `BlenderBridge` — subprocess or PyO3 | Already in `blender.rs` |
| 4.5 | Implement `AudioBridge` — Ming talker for STT/TTS | New file |

---

## Phase 5: Ming Online (Yardmaster)

| Step | Task |
|------|------|
| 5.1 | Test Ming safetensors via `vllm serve --trust-remote-code` |
| 5.2 | Wire Ming's custom talker protocol for audio I/O |
| 5.3 | Wire Yardmaster sidecar to vLLM for worldbuilding |
| 5.4 | Test: quest → Ming generates game code |

---

## Phase 6: ART Production Line

| Step | Task |
|------|------|
| 6a | Aesthetics: ComfyUI + Blender via HTTP/subprocess |
| 6b | Research: Crow 9B + REAP 25B via vLLM |
| 6c | Tempo: OmniCoder 9B + MusicUI via vLLM + HTTP |

---

## Phase 7: Voice Pipeline (Level 1 Headless)

| Step | Task |
|------|------|
| 7.1 | STT via Ming's audio encoder or Whisper via vLLM |
| 7.2 | TTS via Ming's talker decoder |
| 7.3 | Build headless game loop: listen → transcribe → Pete → synthesize → speak |

---

## Phase 8: Data Pipeline (Graph RAG + Vector DB)

| Step | Task |
|------|------|
| 8.1 | Implement embeddings via fastembed or vLLM embedding endpoint |
| 8.2 | Wire Qdrant vector DB |
| 8.3 | Wire SurrealDB graph RAG |
| 8.4 | Feed quest events into both databases |

---

## Dependency Graph (Updated)

```
Phase 0 (vLLM Install) ← EVERYTHING DEPENDS ON THIS
  │
  ▼
Phase 1 (Pete Talks via vLLM)
  │
  ├──→ Phase 2 (ADDIECRAPEYE) ──→ Phase 3 (Narrative)
  │
  ▼
Phase 4 (PyO3 Foundation)
  │
  ├──→ Phase 5 (Ming/Yardmaster via vLLM)
  ├──→ Phase 6 (ART Pipeline)
  └──→ Phase 7 (Voice Pipeline)

Phase 8 (Data Pipeline) — parallel with 4-7
```

## Critical Path: 0 → 1 → 2 → 4 → 5
## Quick Wins: 0 → 1 → 2 → 3 (playable Iron Road with Pete only)
