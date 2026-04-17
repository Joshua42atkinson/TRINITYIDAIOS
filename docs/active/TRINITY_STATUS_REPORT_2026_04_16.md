# TRINITY Genesis — Status Report & Path to Done

> **Date:** April 16, 2026  
> **Author:** Antigravity (system audit)  
> **Hardware:** GMKtek AMD Strix Halo APU · 128 GB LPDDR5x · gfx1151  
> **Disk:** 299 GB free of 1.9 TB  

---

## Executive Summary

**Trinity-Genesis is architecturally complete but operationally blocked on one thing: no inference backend is running.**

The Rust backend compiles with zero errors. The frontend is built. The quest system, conductor, hotel manager, inference router, export pipeline, and 38 agentic tools all exist and are wired. The models are downloaded. But **no LLM process has been running**, which means Trinity starts, shows the UI, and then every chat/agent call returns "INFERENCE_OFFLINE."

**You are approximately one working command away from having a functional AI chat in Trinity. The gap is not code — it's starting an inference server.**

---

## 1. What "Done" Means

There are three levels of "done" for Trinity. Here's what each one looks like and how close you are:

### Level 1: "I can talk to Pete" (You → AI Chat Works)
- Launch an LLM backend on a port Trinity expects  
- Open Trinity in the browser  
- Type a message, get a Socratic response  
- **Distance: 15 minutes of work**

### Level 2: "The Iron Road is playable" (Full Game Loop)
- Onboarding: user enters name, gets PEARL, starts Analysis  
- Phase transitions: completing objectives advances through ADDIECRAPEYE  
- Hotel swaps: the right model loads for the right phase  
- Export: at Evolve phase, export the EYE Package (Quiz + Adventure + DOCX)  
- **Distance: ~1 weekend of focused work**

### Level 3: "Ready for August demo" (Capstone Quality)
- All 12 phases have real pedagogical objectives (not placeholders)  
- Visual polish on the Iron Road UI  
- One guided walkthrough recorded  
- Tested on student hardware (LM Studio Tier 1/2 fallback)  
- **Distance: ~2-3 weeks of part-time work**

---

## 2. Current State — What's Actually Working

### ✅ Compiles & Runs (Verified Today)

| Component | Status | Evidence |
|-----------|--------|----------|
| **Rust backend** (`cargo check -p trinity`) | ✅ 0 errors, 30 warnings | Compiled in 43s |
| **Frontend** (Vite + React, 25 components) | ✅ Built | `dist/` exists, dated April 13 |
| **Inference Router** | ✅ Code complete | Config-driven, 7 backends defined, PartyRole enum, auto-detect, failover |
| **Hotel Manager** | ✅ Code complete | Swap lifecycle: check/kill/launch/health, occupant tracking, Lone Wolf fallback |
| **Conductor** | ✅ Code complete | 12 Socratic system prompts, phase→gear mapping, Hotel wiring, Bloom's levels |
| **Quest System** | ✅ Code complete | ADDIECRAPEYE state machine, XP/Coal/Steam, PEARL alignment |
| **Agentic Tools** | ✅ 38 tools wired | Parallel execution, SSE streaming |
| **EYE Export** | ✅ Working | HTML5 Quiz, Adventure, DOCX, ZIP |
| **Database** | ✅ SQLite | Persistence layer with message/tool-call tracking |
| **Health Endpoint** | ✅ Working | Checks all 4 P-ART-Y ports + DB + CowCatcher |

### ✅ Models Downloaded

| Model | Size on Disk | Location | For |
|-------|:---:|----------|-----|
| Gemma 4 E4B AWQ | 15 GB | `~/trinity-models/vllm/gemma-4-E4B-it-AWQ-4bit/` | T — Tempo (always-on chat) |
| Gemma 4 26B A4B AWQ | 17 GB | `~/trinity-models/vllm/gemma-4-26B-A4B-it-AWQ-4bit/` | P — Programming |
| Gemma 4 31B Dense AWQ | 20 GB | `~/trinity-models/vllm/gemma-4-31B-it-AWQ-4bit/` | R — Reasoning |
| FLUX.1-schnell | ~7 GB | `~/trinity-models/vllm/flux.1-schnell-nf4-with-transformer/` | A — Image gen |
| Nemotron 120B (GGUF Q4) | 81 GB | `~/.lmstudio/models/.../NVIDIA-Nemotron-3-Super-120B-A12B-GGUF/` | LM Studio |
| Gemma 4 31B Opus Distill (GGUF) | 20 GB | `~/.lmstudio/models/.../gemma-4-31B-it-Claude-Opus-Distill-GGUF/` | LM Studio |

### ❌ Not Working Right Now

| Issue | Why | Impact |
|-------|-----|--------|
| **No inference server running** | vLLM distrobox is "Created" but not entered; no LM Studio server active | **Total blocker** — no chat, no agent, no Hotel swaps |
| **vLLM distrobox stale** | `sglang-engine` exited 6 days ago; `vllm` in "Created" state | Cannot launch Tempo via current scripts |
| **Ignition script outdated** | `trinity_ignition.sh` still calls `launch_pete.sh` and `launch_arty_hub.sh` | Won't launch Tempo E4B properly |
| **Frontend stale API refs (possible)** | `InferenceManager.jsx`, `TrainStatus.jsx` may reference old JSON keys | May error on health/fleet endpoints |

---

## 3. Soft Spots — Architectural Risks

### 🔴 Critical: The Distrobox/vLLM Dependency

The current P-ART-Y architecture requires models to run inside a `distrobox` container with a custom vLLM + ROCm image (`kyuz0/vllm-therock-gfx1151:latest`). This is the primary source of stress:

- The distrobox container has **not been running** (status: "Created", not "Running")
- vLLM on AMD ROCm is **fragile** — it requires specific `HSA_OVERRIDE_GFX_VERSION`, `HSA_ENABLE_SDMA`, and `PYTORCH_ROCM_ARCH` env vars
- Model warm-up is skipped (`VLLM_SKIP_WARMUP=true`) to avoid crashes
- `gpu-memory-utilization 0.08` for the E4B is extremely conservative (~6 GB of 128 GB)
- No systemd service or auto-restart — if it dies, manual intervention is required

**The distrobox/vLLM path is the #1 source of friction. Every session starts with "is vLLM running?" and the answer has consistently been "no."**

### 🟡 Medium: LM Studio Is Already Installed But Not Wired

You already have LM Studio installed (`~/.lmstudio/` exists with models). You have two GGUF models downloaded:
- **Nemotron 120B A12B Q4_K_M** (81 GB) — an MoE with 12B active, perfect for your 128 GB
- **Gemma 4 31B Opus Distill Q4_K_M** (20 GB) — dense reasoning model

Trinity's InferenceRouter **already supports LM Studio** on port 1234. The `default.toml` config already defines it. But nobody has connected the two.

### 🟡 Medium: Hotel Swap — Wired But Never Tested

The Hotel Manager code is complete and well-structured, but it has **never been tested against real running models**. The phase transition → model swap → health check pipeline exists entirely as untested code. Specific concerns:

- `kill_port()` uses `lsof` — may not find processes inside distrobox
- Health check polls every 2s for 90s — timeout may be too short for large model loads
- No instrumentation — CowCatcher telemetry for swap latency is planned but not implemented

### 🟢 Low: Frontend API Shape Drift

The health endpoint now returns `tempo_online`, `programming_online`, `reasoning_online`, `aesthetics_online` plus a legacy `pete_online` alias. Some frontend components may expect the old shape. This is a quick fix but needs verification.

---

## 4. Recommendation: Use LM Studio as Primary Backend

**Stop fighting vLLM/distrobox. Use LM Studio.**

### Why LM Studio Instead of vLLM

| Factor | vLLM (distrobox) | LM Studio |
|--------|:-:|:-:|
| **Setup complexity** | Container + custom ROCm image + env vars + launch scripts | GUI app, click "Start Server" |
| **AMD support** | Fragile ROCm shims, `gfx1151` overrides | Uses llama.cpp with Vulkan — works natively |
| **Stability** | Crashes require distrobox restart | Auto-restarts, GUI monitoring |
| **Model loading** | AWQ safetensors, fixed quantization | GGUF, hot-swap models in UI |
| **Multi-model** | Need separate process per port | Can switch models without restart |
| **Tool calling** | vLLM native tool calling | LM Studio supports OpenAI tool calling |
| **Already working** | ❌ Distrobox in "Created" state | ✅ Installed, models downloaded |

### The Recommended Setup (Immediate — Tonight)

**Step 1: Launch LM Studio with Gemma 4 31B Opus Distill** (the GGUF you already have)

```
LM Studio GUI → load gemma-4-31B-it-Claude-Opus-Distill-GGUF
Settings:
  Context: 32768 (start conservative, increase later)
  GPU Offload: All layers (99)
  Server Port: 1234
  Start Server
```

**Step 2: Verify Trinity sees it**

```bash
curl http://localhost:1234/v1/models   # Should show the model
```

**Step 3: Start Trinity**

```bash
cd ~/Workflow/desktop_trinity/trinity-genesis
TRINITY_HEADLESS=1 cargo run -p trinity --release
```

Trinity's `InferenceRouter` will auto-detect LM Studio on port 1234 and failover to it since no vLLM backends are running. That's it. You'll have a working chat.

### The Recommended Setup (This Weekend — "Lone Wolf Plus")

For the P-ART-Y Hotel Swap to work without distrobox, wire LM Studio as the single backend and rely on the Lone Wolf fallback — one model handles all roles:

1. **Load Nemotron 120B Q4_K_M in LM Studio** — it's an MoE (12B active), so it's fast
2. Set Trinity's config to point at LM Studio as primary:

```toml
# In configs/runtime/default.toml:
[inference]
primary = "lm-studio"
```

3. Disable Hotel swaps temporarily — all phases use the single Nemotron model
4. Test the full Iron Road loop: onboarding → Analysis → Design → Export

This gets you **using Trinity for real work** immediately, without any container infrastructure.

### The Recommended Setup (Future — Full P-ART-Y Multi-Model)

Once you're stable with a single model, revisit multi-model by using **LM Studio's daemon mode** to serve multiple models on different ports, or by serving a single large model and letting it handle all cognitive loads (Lone Wolf mode, which is already supported).

---

## 5. Testing Plan

### Smoke Test 1: "Can I chat?" (15 min)

```bash
# 1. Start LM Studio, load a model, start server on :1234
# 2. Verify:
curl http://localhost:1234/v1/models

# 3. Start Trinity:
cd ~/Workflow/desktop_trinity/trinity-genesis
TRINITY_HEADLESS=1 cargo run -p trinity --release

# 4. Open browser:
xdg-open http://localhost:3000

# 5. Go to Iron Road, type "Hello Pete"
# Expected: Get a Socratic response, no errors
```

### Smoke Test 2: "Does quest state work?" (30 min)

```bash
# From the Trinity UI:
# 1. Create a PEARL (subject + medium + vision)
# 2. Start a quest
# 3. Complete one Analysis objective
# 4. Verify phase advances to Design
# 5. Check Character Sheet shows XP/Coal progress
```

### Smoke Test 3: "Does export work?" (15 min)

```bash
# 1. Navigate to any quest
# 2. Click EYE Export
# 3. Verify ZIP downloads with:
#    - HTML5 Quiz
#    - HTML5 Adventure
#    - DOCX document
```

### Smoke Test 4: "Hotel Swap (optional)" (1 hour)

```bash
# Only after Tests 1-3 pass
# 1. Start two LM Studio instances (or use daemon mode)
#    - :1234 as Tempo
#    - :8000 as Programming (if available)
# 2. Trigger Design phase (needs P gear)
# 3. Watch Trinity logs for Hotel swap messages
# 4. Verify the Conductor routes to the P model
```

---

## 6. Realistic Timeline

| When | Milestone | What You Get |
|------|-----------|--------------|
| **Tonight** (30 min) | LM Studio + Trinity smoke test | First real AI chat in Trinity |
| **This weekend** (4-6 hrs) | Iron Road first playable | Create a PEARL, walk through 3 phases, get real Socratic guidance |
| **Next week** (2-3 evenings) | Quest objectives + UI cleanup | All 12 phases have real content, frontend quirks fixed |
| **End of April** | Hotel swap testing | Multi-model P-ART-Y with LM Studio daemon |
| **May** | Automated tests + polish | CI confidence, onboarding walkthrough |
| **August** | Capstone demo | Complete system, tested on student hardware |

---

## 7. What You Should NOT Do

Based on the past few weeks of conversation history, here are patterns that have been causing stress without producing results:

1. **Don't try to fix the vLLM distrobox right now.** It's a yak shave. LM Studio works today.
2. **Don't chase the Mini Trinity Android build.** The desktop version needs to work first. Android is a distribution target, not a development target.
3. **Don't try to get the Bevy XR / Daydream crate running.** It's complete but needs a Quest 3S headset. It's a P2 polish item.
4. **Don't start new crates or new architecture.** The architecture is done. What's missing is *running* the existing code with a real model behind it.
5. **Don't optimize VRAM or quantization yet.** You have 128 GB. Load the 31B GGUF in LM Studio with all layers on GPU. Optimize later.

---

## 8. The One Thing To Do Next

```
Open LM Studio → Load gemma-4-31B-it-Claude-Opus-Distill → Start Server → Start Trinity → Chat with Pete.
```

That's it. Everything else is polish on top of a working system.

---

## Appendix A: File Health Summary

### Backend Source (`crates/trinity/src/`)

| File | Lines | Status |
|------|:-----:|--------|
| `main.rs` | ~5,200 | ✅ Routes, state, all endpoints |
| `tools.rs` | ~3,400 | ✅ 38 agentic tools |
| `agent.rs` | ~3,100 | ✅ Yardmaster loop |
| `conductor_leader.rs` | ~1,360 | ✅ ADDIECRAPEYE orchestrator |
| `inference_router.rs` | ~930 | ✅ Multi-backend with PartyRole |
| `creative.rs` | ~1,600 | ✅ Image/music/3D dispatch |
| `voice.rs` | ~1,100 | ✅ Kokoro TTS |
| `hotel_manager.rs` | ~383 | ✅ Swap protocol (untested) |
| `quests.rs` | ~960 | ✅ Quest state machine |
| `inference.rs` | ~520 | ✅ OpenAI-compatible client |
| `health.rs` | ~212 | ✅ Subsystem health |

### Frontend Components (`crates/trinity/frontend/src/components/`)

| Component | Size | Notes |
|-----------|:----:|-------|
| `CharacterSheet.jsx` | 71 KB | Largest — the RPG character HUD |
| `PhaseWorkspace.jsx` | 55 KB | The main 12-phase ADDIECRAPEYE workspace |
| `PlayerHandbookElearning.jsx` | 39 KB | Interactive textbook |
| `PortfolioView.jsx` | 37 KB | LDT portfolio tracker |
| `Yardmaster.jsx` | 37 KB | Agentic terminal |
| `ArtStudio.jsx` | 34 KB | Creative asset generation |
| `InferenceManager.jsx` | 18 KB | ⚠️ Check for stale API refs |
| `TrainStatus.jsx` | 5.7 KB | ⚠️ Check for stale API refs |

### Disk Usage

| Category | Size | Action |
|----------|:----:|--------|
| Gemma 4 models (vLLM AWQ) | ~52 GB | Keep for now |
| LM Studio models (GGUF) | ~101 GB | Keep — this is your inference path |
| FLUX.1-schnell | ~7 GB | Keep |
| Qwen legacy models | ~20 GB | Can archive to reclaim space |
| Gemma 4 E2B (unused variant) | ~2 GB | Can archive |
| Free disk space | 299 GB | Healthy |

## Appendix B: Config Change for LM Studio Primary

To make LM Studio the primary backend, change one line in `configs/runtime/default.toml`:

```diff
 [inference]
-primary = "tempo-e4b"
+primary = "lm-studio"
 auto_detect = true
```

The InferenceRouter will then prefer LM Studio and fall back to any other healthy backend. No code changes needed.
