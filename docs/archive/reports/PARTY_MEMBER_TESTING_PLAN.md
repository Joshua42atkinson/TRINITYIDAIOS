# Party Member Testing & Finishing Plan
## March 14, 2026 | Plan-Act-Evaluate Cycle

---

## 1. HARDWARE REALITY (From Verified Specs)

```
128GB LPDDR5X Total
├─ 96GB VRAM (BIOS v1.05 UMA_SPECIFIED) ← GPU/model workspace
├─ 32GB System RAM ← OS, Rust compiler, PostgreSQL, browser
├─ NPU: XDNA 2, 52 TOPS ← Always-on, 1-1.5B ONNX models only
├─ GPU: Radeon 8060S, 40 CUs, RDNA 3.5 ← longcat-sglang with -ngl 99
└─ MiniMax verified: 16.8 tok/s on this hardware
```

**Key constraint**: NPU can only run **1-1.5B AWQ ONNX** models (2GB NPU memory).
**Key advantage**: 96GB VRAM means we can load massive models on GPU.
**Swap time**: ~2-3 min to unload one sidecar and load another (GPU VRAM mapping).

---

## 2. NPU ALWAYS-ON ORCHESTRATOR

The NPU runs a small 1-1.5B model that is ALWAYS loaded — no swapping needed.
This is the "conductor's brain" that runs alongside any sidecar.

### What the NPU Does
1. **Task classification**: Simple/complex routing (~100ms)
2. **Queue management**: Decides which quest to run next
3. **Health monitoring**: Checks sidecar status
4. **Quick responses**: Simple questions answered instantly without GPU

### NPU Model Candidates (already downloaded or available)
| Model | Size | Location | Status |
|-------|------|----------|--------|
| Llama-3.2-1B ONNX | ~1GB | Not downloaded | **RECOMMENDED** (AMD optimized) |
| Qwen2.5-Coder-1.5B ONNX | ~1.2GB | Not downloaded | Code-aware alternative |
| Granite 1B | 6.1GB SafeTensors | models/npu/granite/ | **Wrong format** — needs ONNX conversion |

### NPU Action Items
1. Download `amd/Llama-3.2-1B-Instruct-onnx-ryzenai-npu` (AMD pre-optimized)
2. Test loading with ORT (Python first for speed, then Rust via PyO3)
3. Build `/api/classify` endpoint that routes via NPU
4. NPU runs 24/7 even when sidecars swap

---

## 3. SIDECAR MEMORY BUDGETS (96GB VRAM)

Each sidecar gets the full 96GB VRAM minus the NPU allocation (~1GB).
**Available per sidecar: ~95GB VRAM + 32GB system RAM**

| Sidecar | Model(s) | Model RAM | KV Cache | Total | Fits? |
|---------|----------|-----------|----------|-------|-------|
| **Engineer** | Opus 27B + REAP 25B | 36GB | ~8GB | ~44GB | ✅ Comfortable |
| **Evaluator** | Opus 27B (65K ctx) | 21GB | ~20GB | ~41GB | ✅ Comfortable |
| **Artist** | Opus 27B + ComfyUI | 21GB + 6.5GB | ~8GB | ~36GB | ✅ Comfortable |
| **Brakeman** | REAP 25B | 15GB | ~4GB | ~19GB | ✅ Lots of room |
| **Visionary** | Qwen3.5-35B-A3B + Vision + ComfyUI | 21GB + 6.5GB | ~8GB | ~36GB | ✅ Comfortable |
| **Pete** | Opus 27B | 21GB | ~4GB | ~25GB | ✅ Comfortable |
| **Colossus** | MiniMax 66GB | 66GB | ~15GB | ~81GB | ✅ Tight but fits |

**All sidecars fit within 96GB.** The Colossus is the only tight one.

---

## 4. WHAT "TESTED AND FINISHED" MEANS PER ROLE

### Test Criteria (every role must pass ALL):
- [ ] Model loads and responds to `/think` endpoint
- [ ] Can read files, run shell commands via quest system
- [ ] Completes at least 1 quest successfully
- [ ] Git safety: modified files restored on verification failure
- [ ] Clean shutdown (processes killed, ports freed)

### Role-Specific Test Criteria:

#### ⚙️ Engineer — TESTED (partially)
- [x] Both models load (Opus + REAP)
- [x] /think and /code endpoints respond
- [x] Quest execution runs end-to-end
- [x] Patch mode prevents file truncation
- [x] Git safety restores files on failure
- [ ] Successfully generates code that compiles
- [ ] Completes quest-fix-cli with a working fix

#### 📊 Evaluator
- [ ] Loads Opus with 65K context
- [ ] Generates QM rubric evaluation report
- [ ] Evaluates against Bloom's Taxonomy
- [ ] quest-evaluate-server produces real report

#### 🎨 Artist
- [ ] Loads Opus with 32K context
- [ ] Generates game design document
- [ ] Generates asset manifest
- [ ] quest-design-first-game produces GDD

#### 🛡️ Brakeman
- [ ] Loads REAP with 16K context
- [ ] Generates test suites
- [ ] Runs cargo clippy/test
- [ ] quest-brakeman-security-audit produces findings

#### 👁️ Visionary (+ ComfyUI integration)
- [ ] Loads Qwen3.5-35B-A3B with vision projector
- [ ] Can describe images when given a screenshot path
- [ ] ComfyUI HTTP bridge generates images from text prompts
- [ ] Can review generated images for quality

#### 🎓 Pete
- [ ] Loads Opus with 16K context
- [ ] Responds with Socratic questions
- [ ] Generates formative assessment reports
- [ ] Adapts to Bloom's taxonomy level

---

## 5. VISIONARY + COMFYUI + OBS WORKFLOW

This is the multimedia pipeline — the Artist/Visionary share this capability.

```
┌─────────────────────────────────────────────────────────────┐
│  Visionary Sidecar (:8090)                                  │
│                                                             │
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │ Qwen3.5-35B-A3B  │  │ ComfyUI (Python) │                │
│  │ + Vision Projector│  │ :8188            │                │
│  │ :8081 (GPU)      │  │ SDXL Turbo       │                │
│  │ Describes/Reviews│  │ Generates images  │                │
│  └──────────────────┘  └──────────────────┘                │
│            │                    │                           │
│            ▼                    ▼                           │
│  ┌──────────────────────────────────────────┐              │
│  │ OBS Studio (optional)                     │              │
│  │ Screenshots, video capture, streaming     │              │
│  │ Visionary reviews screenshots via vision  │              │
│  └──────────────────────────────────────────┘              │
│                                                             │
│  Workflow:                                                  │
│  1. Artist/Opus designs game → writes asset specs           │
│  2. Specs → ComfyUI HTTP API → generates sprites/textures  │
│  3. Visionary reviews generated images → quality check      │
│  4. OBS captures game screenshots → Visionary evaluates UI  │
│  5. Iterate until quality meets WCAG/design standards       │
│                                                             │
│  Memory: 35B model (21GB) + SDXL (6.5GB) + OBS (~2GB)     │
│  Total: ~30GB — fits easily in 96GB VRAM budget            │
└─────────────────────────────────────────────────────────────┘
```

### Implementation Steps for ComfyUI Bridge
1. Create `comfyui_client.rs` in the sidecar — HTTP client for ComfyUI API
2. ComfyUI runs as a separate Python process on :8188 (already installed)
3. Sidecar generates workflow JSON → POST to ComfyUI → download result
4. Vision model reviews the generated image via `--mmproj` flag

---

## 6. QUEST ROTATION STRATEGY

### How Sidecar Rotation Works
```
Quest Board has quests tagged by ADDIE phase and required role.

1. NPU orchestrator scans board → groups quests by role
2. Picks the role with most pending quests
3. Loads that sidecar (2-3 min GPU swap)
4. Sidecar processes ALL quests for its role
5. When queue empty → NPU picks next role → swap
6. Repeat

This maximizes GPU utilization by batching work per role.
```

### Example Overnight Schedule
```
18:00  Engineer loads    → works 5 code gen quests (~35 min)
18:35  Swap to Evaluator → reviews 3 quests (~25 min)
19:00  Swap to Artist    → designs 2 game templates (~20 min)
19:20  Swap to Brakeman  → runs tests on Engineer's output (~15 min)
19:35  Swap to Visionary  → reviews generated images (~15 min)
19:50  All quests done   → NPU idles, checks for new quests every 30s
```

---

## 7. TESTING ORDER (Priority)

### Phase 1: Prove Each Role Works (This Week)
1. **Engineer** — finish the quest-fix-cli test (patch mode needs tuning)
2. **Evaluator** — run quest-evaluate-server, verify report output
3. **Artist** — run quest-design-first-game, verify GDD output
4. **Brakeman** — run quest-brakeman-security-audit, verify findings

### Phase 2: Vision + ComfyUI (Next Week)
5. **Visionary** — test vision model with `--mmproj` flag
6. **ComfyUI bridge** — HTTP client for image generation
7. **OBS integration** — screenshot capture + vision review

### Phase 3: NPU Always-On (2 Weeks)
8. Download Llama-3.2-1B ONNX for NPU
9. Test ORT loading via Python
10. Build PyO3 bridge for Rust integration
11. NPU orchestrator for quest routing

### Phase 4: Polish for Purdue Demo (3-4 Weeks)
12. Complete quest chain (Chapters 1-3)
13. One game template built end-to-end
14. WASM export for browser playability
15. Demo walkthrough documented

---

## 8. IMMEDIATE NEXT STEPS

### Right Now
1. Start the Evaluator sidecar and run quest-evaluate-server
2. Start the Artist sidecar and run quest-design-first-game
3. Verify each produces real output files

### To Enable Overnight Work
4. Add role-rotation logic to the autonomous loop
5. Tag quests with required role
6. NPU orchestrator (later — for now, manual rotation is fine)

---

*"The Iron Road Hotel doesn't try to check in all guests at once. It gives each one the full suite, then rotates efficiently."*

— Party Member Testing Plan, March 14, 2026
