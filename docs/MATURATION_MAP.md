# Trinity Embedded Inference — Maturation Map

> *March 28, 2026 — "The Golem breathes through its own lungs now."*

## Architecture (Current)

```
┌────────────────────────────────────────────────────────────────┐
│                    TRINITY ID AI OS                             │
│                                                                │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ Layer 1: Headless Server (Rust, Axum, :3000)            │   │
│  │                                                         │   │
│  │  ┌──────────────────────┐  ┌────────────────────────┐   │   │
│  │  │ Embedded Inference   │  │ HTTP Fallback Router   │   │   │
│  │  │ llama-cpp-2 (Vulkan) │  │ → llama-server :8080   │   │   │
│  │  │ PRIMARY — zero HTTP  │  │ → LM Studio :1234      │   │   │
│  │  │ Deferred loading     │  │ → Ollama :11434        │   │   │
│  │  │ Hot-swappable model  │  │ Auto-detect + failover │   │   │
│  │  └──────────────────────┘  └────────────────────────┘   │   │
│  │                                                         │   │
│  │  ┌──────────────────────┐  ┌────────────────────────┐   │   │
│  │  │ AUDIO (ONNX Runtime) │  │ ART (HTTP Sidecars)    │   │   │
│  │  │ Supertonic-2 TTS     │  │ ComfyUI :8188          │   │   │
│  │  │ Whisper STT           │  │ MusicGPT :8189         │   │   │
│  │  │ CPU/NPU — no GPU     │  │ Hunyuan3D :7860        │   │   │
│  │  └──────────────────────┘  └────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                │
│  Boot Order: Server(:3000) → TTS/STT(ONNX) → ComfyUI → LLM   │
│              ↑ instant        ↑ 2s CPU        ↑ probe  ↑ 30-60s│
└────────────────────────────────────────────────────────────────┘
```

## Boot Sequence

1. **Server starts on :3000** — instant, responds to health checks
2. **TTS/STT ONNX models** — load on CPU, ~2 seconds
3. **ComfyUI probe** — HTTP health check, no GPU load
4. **(5s delay)** — settle ONNX models
5. **Mistral 68GB loads** — background task, Vulkan GPU, 30-60s

## vLLM Removal — Complete

| Item | Status |
|------|--------|
| `BackendKind::Vllm` enum variant | ✅ Removed |
| `BackendKind::Sglang` enum variant | ✅ Removed |
| `VLLM_URL` env var fallback | ✅ Removed |
| `configs/runtime/default.toml` vLLM backend | ✅ Removed |
| `trinity-brain.service` vLLM launcher | ✅ Replaced with embedded |
| 5 vLLM launch scripts | ✅ Archived |
| 3 SystemD vLLM service files | ✅ Archived |
| `test_vllm_vision.sh` | ✅ Archived |
| `setup_vllm_distrobox.sh` | ✅ Archived |
| All `*.rs` files — zero vLLM references | ✅ Verified |
| Compilation | ✅ 0 errors |
| Router tests | ✅ 9/9 pass |

## PostgreSQL Removal — Complete

| Item | Status |
|------|--------|
| `PgPool` replaced with `SqlitePool` | ✅ Complete |
| `pgvector` dependencies removed | ✅ Complete |
| RAG vector search moved to in-memory Rust | ✅ Complete |
| Database initializes zero-config local file | ✅ Complete |
| Pure SQLite table structures applied | ✅ Complete |
| `trinity-mcp-server` compatible | ✅ Complete |

---

## Remaining Phases

### 🟡 Phase 2: Model Management API + Yard UI

**Why**: The user needs a load/unload/switch button in the Yardmaster UI.

| Task | Description |
|------|-------------|
| `GET /api/model/status` | Return: loaded model, state (`loading`/`ready`/`unloaded`), GPU memory |
| `POST /api/model/load` | Load GGUF by path into embedded_model |
| `POST /api/model/unload` | Unload model, free GPU memory |
| `GET /api/model/available` | List GGUF files in `~/trinity-models/gguf/` |
| Yardmaster UI | Model management button in the Yard sidebar |

### 🟡 Phase 3: HuggingFace Downloader

**Why**: First-run experience — download models without leaving Trinity.

| Task | Description |
|------|-------------|
| `POST /api/model/download` | Download GGUF from HuggingFace |
| `GET /api/model/download/progress` | SSE stream of download progress |
| First-run wizard | Beautiful UI when no models found |
| Recommended models | Curated list (Mistral Small 4, etc.) |

### 🟢 Phase 4: Bible & Docs Cleanup

**Why**: Preparing for copywriting — remove all vLLM references from documentation.

| Task | Description |
|------|-------------|
| `TRINITY_FANCY_BIBLE.md` | Update inference architecture sections |
| `CONTEXT.md` | Remove vLLM references |
| `HOOK_BOOK.md` | Clean inference references |
| `docs/` subdirectories | Sweep remaining references |
| Copywriting-ready summary | 2-page executive overview |

### 🟢 Phase 5: AppImage Packaging

**Why**: Single-download AppImage like LM Studio itself.

| Task | Description |
|------|-------------|
| `build-appimage.sh` | Bundle trinity binary + frontend |
| Model wizard | First-run download/locate GGUF |
| Desktop integration | Icon, .desktop file, MIME types |
| Clean system test | Verify on fresh install |

---

## Archived Maps

Previous maturation maps archived to `archive/maturation-maps-march-2026/`:
- `MATURATION_MAP_pre_embedded.md` — pre-embedded inference era
- `FORGE_MATURITY_MAP.md` — Bevy/Forge game studio integration
- `LDT_CAPSTONE_MATURITY_MAP.md` — Purdue capstone alignment
