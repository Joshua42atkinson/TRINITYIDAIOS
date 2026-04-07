# Trinity AI OS — Session Context & Fix List

> **Last Updated**: April 7, 2026
> **Goal**: Finalize media rendering and Purge Legacy code before Purdue demo. Iron Road is NATIVE; Yard is Unshackled.

---

## Current System State

| Component | Status | Detail |
|-----------|--------|--------|
| **LongCat (DiNA)** | 🟡 Sandboxed | Model 74B downloaded (151GB). `sglang-engine` container created. Ready to boot. |
| **Trinity Server** | ✅ Running | Port 3000, health endpoint shows `connected: true` |
| **LDT Portfolio UI** | ✅ Running | React Web App on Port 3001 |
| **Iron Road UI** | ✅ Native | Ported from React to Native Bevy `trinity-daydream`. Holographic LitRPG book UI. |
| **Yardmaster Agent** | ✅ Unshackled | 'Ring 5' sandbox gutted. Pete runs on `Broad` gauge with OpenHands parity. |
| **LongCat Sidecar** | 🟡 Sandboxed | `longcat_omni_sidecar` holds `server.py` proxy & `launch_engine.sh`. |

## System Full Startup Sequence (Web UI + Backend)

To avoid wasting time on startup routing, here is the exact boot sequence required for the web frontend and trinity backend:

1. **LongCat Omni Engine (Port 30000 -> 8010)**  
   ```bash
   distrobox enter sglang-engine -- bash ./longcat_omni_sidecar/launch_engine.sh
   # And inside Trinity root:
   python3 longcat_omni_sidecar/server.py
   ```

2. **Trinity Backend (Port 3000)**  
   In `trinity-genesis/`: 
   ```bash
   cargo run --release -p trinity
   ```
   *(Serves as the main API layer on port 3000)*

3. **LDT Portfolio Web UI (Port 3001)**  
   In `trinity-genesis/LDTAtkinson/client/`: 
   ```bash
   npm run dev -- --port 3001
   ```
   *(The frontend Vite proxy routes internal `/api` traffic directly to `127.0.0.1:3000`)*
| **Media in chat** | ❌ Missing | Created assets (images, audio) don't appear inline in chat bubbles |
| **Kokoro TTS** | ⬚ Available | Port 8200, not currently running |
| **ComfyUI** | ⬚ Available | Port 8188, not currently running |

---

## Fix List (Priority Order)

### 🔴 P0 — Chat Must Work

- [x] **`inference.rs`** — `model: "mistral"` → `model: "Great_Recycler"` (lines 179, 298, 418) — **DONE**
- [ ] **Rebuild Trinity** — `cargo build --release` to pick up the inference.rs model name fix
- [ ] **Verify live chat** — Send a message in Iron Road, confirm AI responds in real time

### 🟠 P1 — Legacy Model Name Purge (Backend)

These are all places that still say "Mistral" when the brain is now Qwen2.5/Gemma-4:

| File | Line | What to fix |
|------|------|-------------|
| `health.rs` | 128 | Default model hint: `"Mistral-Small-4-119B"` → `"Great_Recycler"` |
| `conductor_leader.rs` | 483, 512, 522 | Comments: "Mistral" → "Great Recycler" |
| `tools.rs` | 1029 | Status command: `"Mistral Small 4 (LLM brain)"` → `"Great Recycler (vLLM)"` |
| `tools.rs` | 1534 | Agent label: `"Mistral Small 4 119B"` → `"Great Recycler"` |
| `cow_catcher.rs` | 322 | Test string: `"mistral"` → `"recycler"` |
| `main.rs` | 1247 | Landing page: `"Mistral Small 4 119B"` → `"Great Recycler (Qwen2.5-14B)"` |
| `main.rs` | 2263 | Comment: `"Crow/Mistral :8080"` → `"vLLM Great Recycler :8001"` |
| `main.rs` | 2596 | Comment: `"When Mistral replaces Crow"` → update |

### 🟠 P2 — Legacy Model Name Purge (Frontend)

| File | Line | What to fix |
|------|------|-------------|
| `Yardmaster.jsx` | 200 | `'Launching LM Studio...'` → `'Connecting to vLLM...'` |
| `Yardmaster.jsx` | 204 | `'Loading Mistral 119B...'` → `'Loading Great Recycler...'` |
| `Yardmaster.jsx` | 595 | localStorage default: `"lm_studio"` → `"vllm"` |
| `Yardmaster.jsx` | 604 | Tooltip: `"Starts LM Studio or Ollama"` → `"Connects to vLLM on port 8001"` |
| `SetupWizard.jsx` | ALL | **DELETE** — Trinity is a single download-and-run system, no engine selector needed |
| `CharacterSheet.jsx` | 1293 | `'Supertonic-2 TTS'` → `'Kokoro TTS'` |
| `PhaseWorkspace.jsx` | 264 | Comment: `"Supertonic sidecar"` → `"Kokoro TTS"` |
| `Yardmaster.test.jsx` | 25, 42 | Test fixture: `'Mistral Small 4 119B'` → `'Great Recycler'` |

### 🟠 P3 — Legacy Backend Purge (Dead Code)

These reference backends that no longer exist:

| File | Lines | What to fix |
|------|-------|-------------|
| `inference_router.rs` | 11, 75, 79 | Comments still reference llama-server, LM Studio, Ollama |
| `inference_router.rs` | 556-642 | **Tests** reference ollama, llama-server, lm-studio — update to vLLM-only |
| `tools.rs` | 253-262 | `llama_server_binary()` function — dead code, remove or gate |
| `tools.rs` | 2323-2393 | Vision tool references llama-server on port 8081 — should use vLLM 8001 |
| `main.rs` | 185 | Comment: `"Tracks LM Studio boot"` — dead state machine |
| `main.rs` | 655 | Comment: `"Re-probes llama-server"` |
| `main.rs` | 1750 | Comment: `"Crow 9B on :8080"` |
| `main.rs` | 2390-2413 | **Live code**: Ollama spawn logic — should be removed or gated |
| `main.rs` | 4496 | Ollama URL hardcoded: `"http://127.0.0.1:11434"` |
| `voice.rs` | 6-43 | Header: still mentions Voxtral, Supertonic |
| `voice.rs` | 125-126 | Supertonic fallback definition |
| `voice.rs` | 353-392 | Dead Voxtral-4B code |
| `telephone.rs` | 13-336 | Multiple Supertonic references |
| `voice_loop.rs` | 8 | Header: `"Legacy voice loop — delegated to Supertonic-2"` |
| `rag.rs` | 28 | Comment: `"Embedding generation via llama-server"` |

### 🟡 P4 — Media Rendering in Chat

Currently, when the AI creates content (images via ComfyUI, audio via Kokoro), the user doesn't see it inline in the chat. Both the Iron Road and Yard chats need:

- [ ] Parse tool call results for `image_url`, `audio_url`, `video_url` fields
- [ ] Render `<img>` tags inline when the AI generates an image
- [ ] Render `<audio>` controls inline when the AI generates narration
- [ ] The ArtStudio already has `AssetCard` and `PreviewModal` components — reuse in chat

**Frontend chat components:**
- Iron Road: `PhaseWorkspace.jsx` → uses `/api/chat/stream`
- Yardmaster: `useYardmaster.js` → uses `/api/chat/yardmaster`
- Art Studio: `ArtStudio.jsx` → uses `/api/chat/stream` (already renders media via `AssetCard`)

### 🟡 P5 — Port Map Cleanup

The `inference_router.rs` default backend list (lines 292-339) has 4 backends when we only need 1:

```
Current:
  vllm-recycler  → 8001 (active, working)
  longcat        → 8010 (new implementation proxy target)

Target:
  longcat        → 8010 (sole Omni-Brain handling audio/visual/text natively)
```

Also clean up `default.toml` to remove the `vllm-omni` on 8000 entry.

---

## Architecture Reminders

- **One Brain**: Great Recycler is the sole reasoning engine. Programmer Pete is a sub-agent using the same model via different system prompts.
- **Player Profiles**: Each Iron Road project runs in a sidecar with its own player profile. All generated content stays within that project's sidecar.
- **Static VRAM Budget**: Target is fixed VRAM allocation at boot. All models Apache 2.0.
- **Distrobox**: vLLM runs in distrobox `vllm` (kyuz0/vllm-therock-gfx1151). Ports are directly on 127.0.0.1.
- **NEVER install `turboquant-vllm`** — it's NVIDIA-only and destroys KV cache on AMD ROCm.

## Key Files

| Purpose | File |
|---------|------|
| Inference client (model name, API calls) | `crates/trinity/src/inference.rs` |
| Backend router (port fallback chain) | `crates/trinity/src/inference_router.rs` |
| Agent chat (Yardmaster SSE) | `crates/trinity/src/agent.rs` |
| Iron Road chat (SSE stream) | `crates/trinity/src/main.rs` → `chat_stream()` |
| Health endpoint | `crates/trinity/src/health.rs` |
| Runtime config | `configs/runtime/default.toml` |
| Frontend Iron Road | `frontend/src/components/PhaseWorkspace.jsx` |
| Frontend Yardmaster | `frontend/src/hooks/useYardmaster.js` + `Yardmaster.jsx` |
| Frontend Art Studio | `frontend/src/components/ArtStudio.jsx` |
| vLLM diagnostics | `docs/VLLM_LESSONS_LEARNED.md` |
| Architecture bible | `TRINITY_FANCY_BIBLE.md` §1.9 |

## LongCat Launch Commands (Isolated Sidecar)

```bash
# 1. Start the DiNA Token Architecture (within the sglang sandbox)
distrobox enter sglang-engine -- bash ./longcat_omni_sidecar/launch_engine.sh

# 2. Start the Python Interdiction Proxy (Translates DiNA tokens -> Media)
python3 ./longcat_omni_sidecar/server.py
```
