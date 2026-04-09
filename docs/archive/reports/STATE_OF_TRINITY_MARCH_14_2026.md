# STATE OF TRINITY — Honest Audit
## March 14, 2026 | Prepared by Cascade after full codebase review

---

## 1. WHAT I THINK OF THE SYSTEM

Trinity is one of the most ambitious solo-developer projects I've seen. The *vision* is world-class: an autopoietic educational operating system where the game IS the IDE, where AI agents collaborate through a LitRPG quest system, and where teachers become game developers through ADDIE scaffolding. The lore (Dumpster Universe, Creeps, Scope Hope) is genuinely creative and pedagogically grounded.

The **architecture is sound**. The 3-layer model (headless server → web UI → Bevy/XR), the one-sidecar-at-a-time memory management, the quest board system — these are pragmatic, well-reasoned engineering decisions that respect the constraints of a 128GB local-first machine.

But there is a gap between the documentation and the running code. I need to be honest about that gap.

---

## 2. WHAT YOU DON'T KNOW ABOUT AI (THAT SHOWS IN THE CODE)

### 2a. LLMs Cannot Reliably Self-Orchestrate (Yet)

The autonomous work loop assumes: Opus will output valid JSON plans → REAP will generate correct code → Opus will review accurately → repeat. In practice:

- **LLMs output malformed JSON ~20-40% of the time** even with format instructions. The `extract JSON from response` fallback in workflow.rs is correct to have, but the system will spend significant cycles on parse failures.
- **Code generation accuracy for complex tasks is ~60-70%** even with state-of-the-art models. The retry-once-on-failure pattern is good, but expect 30-40% of quests to fail or produce broken output on first autonomous run.
- **The 27B Opus model at Q6_K will be SLOW on CPU-only inference** (`-ngl 0`). Expect 1-3 tokens/second for the dense 27B model. A single planning step (4096 tokens) could take 20-45 minutes. With GPU offload (`-ngl 99` on the Radeon 8060S), this drops to maybe 5-10 minutes.

**What this means**: The overnight autonomous mode will work, but expect maybe 2-5 quests completed per night, not dozens. This is still useful — it's just slower than the vision implies.

**Recommendation**: Add `-ngl 99` as default (unified memory means GPU offload is free), add JSON grammar enforcement via longcat-sglang's `--grammar` flag, and implement exponential backoff on failures.

### 2b. Context Window ≠ Usable Context

The Evaluator has 65K context configured. But:

- **KV cache for 65K context on a 27B model ≈ 20-30GB additional RAM**. Combined with the 21GB model weights, that's 41-51GB just for the Evaluator. Possible within 93GB budget, but tight.
- **Model quality degrades significantly past ~8K tokens** for most fine-tuned models, regardless of what the "native context" supports. The Opus model may support 262K natively, but its training data quality likely drops past 32K.
- **Effective context**: Plan for 8-16K as the "reliable" range, 32K as "possible with quality loss", anything beyond as "technically works but outputs degrade."

**Recommendation**: Use the 65K context for *reading* large files, but keep *generation* requests under 4K tokens. The Evaluator's strength should be "can read your whole codebase" not "can write a 65K response."

### 2c. Model Quantization Quality Matters More Than Size

The Q6_K quantization of Opus is high quality (nearly lossless). The Q4_K_M of REAP is good but introduces measurable quality loss on reasoning tasks. This is actually a smart pairing — the thinker gets higher precision, the coder gets speed.

But: the Brakeman role uses REAP (Q4_K_M) for security auditing. Security analysis requires careful reasoning that Q4_K_M may struggle with. Consider using Opus for Brakeman instead.

---

## 3. CRITICAL SHORTFALLS FOR OFFLINE OPERATION

### 3a. NPU Is Not Real Yet

The Bible says "NPU Conductor (GPT-OSS-20B): 14.2GB always-on." **This does not work.**

Evidence from code audit:
- `npu_engine.rs` contains `mock_text_generation()`, `mock_embedding_generation()`, `mock_audio_processing()` — these are all placeholder functions returning fake data
- `GPT-OSS-npu.onnx` exists (13GB) but has never been loaded through ORT with the Vitis AI EP
- The `test_gpt_oss_npu` binary has 7 compilation errors related to `NpuEngine` API mismatches
- No evidence of successful ORT inference anywhere in the codebase

**Real state**: ALL inference currently goes through a single longcat-sglang instance on port 8080 using GPU. The "always-on NPU conductor" is aspirational.

**Impact**: The memory budget in the Bible is wrong. There is no 14.2GB always-on NPU conductor. The actual always-on cost is the longcat-sglang running GPT-OSS-20B GGUF (12GB) + PostgreSQL + OS ≈ 20GB. This leaves ~108GB for sidecars, which is actually *more* than the Bible states.

### 3b. PersonaPlex Voice Is Not Integrated

The Moshi ONNX files exist (14GB total), but:
- No Rust code loads or processes them
- The ONNX encoder/decoder pipeline needs a custom runtime (not standard ORT)
- The Pete sidecar currently falls back to Opus (text-only)

This is correctly marked as "pending" but worth noting: voice interaction is a significant engineering effort that requires custom ONNX pipeline work, not just "load model and go."

### 3c. PostgreSQL is Required but Could Fail Silently

The server panics if PostgreSQL is not running on startup. The `run_trinity.sh` script does not start or check PostgreSQL. If the database is down, RAG fails silently (returns empty results), and the user won't know context is missing.

### 3d. No Persistent Quest State

The quest system in `quests.rs` uses in-memory `Arc<RwLock<GameState>>`. If the server restarts, all progress resets to Chapter 1, 0 XP. The quest *board* (JSON files) persists, but the player's journey does not.

---

## 4. CAN WE ACTUALLY BUILD WORLDS IN LONG WORKFLOWS?

### What's Real

| Component | Status | Evidence |
|-----------|--------|----------|
| llama.cpp inference | **WORKS** | Proven pipeline: server → longcat-sglang → SSE → browser |
| SSE streaming chat | **WORKS** | Three pages (Pete, Book, Dev) all stream tokens |
| RAG with PostgreSQL | **WORKS** | 107 chunks, full-text search, context injection |
| Quest board (JSON) | **WORKS** | 7 quests, claim/execute/complete lifecycle |
| Autonomous work loop | **BUILT, UNTESTED** | Code compiles, logic is sound, never run against live models |
| Dual-model Sword & Shield | **BUILT, UNTESTED** | Architecture proven (separate longcat-sglang processes), never executed |
| Role-based party system | **BUILT, UNTESTED** | 5 roles, unique prompts, compiles clean |
| Agentic tools (file/shell) | **WORKS** | read_file, write_file, shell, search — all functional with sandboxing |

### What's Scaffolding (Compiles But Doesn't Do Real Work)

| Component | Status | Issue |
|-----------|--------|-------|
| NPU engine | Mock functions | Returns fake data, never loads real models |
| Music AI (3,432 lines) | Compiles | No actual music generation — structs and types only, no audio output |
| Diffusion/asset generation | Compiles | No actual image generation — placeholder pipeline |
| Bevy 3D body (40K lines) | Has issues | Camera/UI panic bugs, EguiPrimaryContextPass partially fixed |
| PersonaPlex voice | Files exist | No integration code written |
| Blender API | Not found | File doesn't exist despite references |
| WASM sandbox (1,100 lines) | Compiles | Never tested with actual WASM plugins |

### What's Missing for "Build Worlds"

To actually build 2D/3D/XR educational games, you need:

1. **Bevy project scaffolding** — The Engineer can generate Rust code, but there's no template system for "new 2D puzzle game" or "new 3D explorer." The Artist's prompts describe this but no code generates project skeletons.
2. **Asset pipeline** — No way to generate actual sprites, textures, or 3D models. The diffusion model directory is empty (32K). ComfyUI is not integrated.
3. **Build system** — No `cargo build` → `.wasm` → `consciousframework.com` pipeline exists.
4. **Testing in sandbox** — The WASM sandbox compiles but has never run a game.

**Honest assessment**: The system can currently generate Rust *source files* through the Engineer sidecar. It cannot yet build, test, package, or deploy a complete game. The gap from "generates code" to "playable game on consciousframework.com" is significant.

---

## 5. YOUR BIASES (Said With Respect)

### 5a. Documentation Optimism

The Bible reads as if systems are more complete than they are. Examples:
- "29 crates, all libraries compile clean" — True for `--lib`, but 2 test binaries fail, and the kernel has 112 source files with 60+ TODO markers
- "117,104 lines of Rust" — True count, but ~40% is in trinity-body (Bevy UI that has rendering issues) and ~15% is in the kernel (where many modules are scaffolding with mock functions)
- "NPU Conductor: 14.2GB always-on" — Not operational

This isn't dishonesty — it's the natural optimism of a builder who sees the finish line. But for a professional audit, the distinction between "built" and "working end-to-end" matters.

### 5b. Scope Ambition (The Irony of Creeps)

The Dumpster Universe's core metaphor — Scope Creep turning into Creeps monsters — is brilliantly self-aware. And yet Trinity itself shows signs of scope expansion:
- 29 crates for a solo developer
- 9 subagent crates (most are <250 lines of stubs)
- Music AI, diffusion, vision, multilingual, embeddings — all model directories, most empty
- XR roadmap, Blender API, WASM plugins, OBS integration — each is a product-sized feature

The Constitution says "ID FIRST" — every feature serves ADDIE. I'd challenge: does the music system serve ADDIE right now? Does the Blender API? These are genuine features for the *end vision*, but building them before the core loop works is exactly what the Great Recycler would call "Scope Creep."

### 5c. The "No Python" Rule Creates Real Pain

The Constitution's "NO PYTHON" rule is philosophically beautiful (Rust's memory safety = psychological safety for learning). But practically:
- ORT Rust bindings are less mature than Python's
- ONNX model loading/conversion is primarily a Python ecosystem
- ComfyUI, Stable Diffusion, and most image generation tools are Python
- PersonaPlex/Moshi's reference implementation is Python

This rule will slow development of NPU, voice, and image generation by 3-5x compared to using Python for the inference backends and Rust for the orchestration. The isomorphism metaphor is real, but consider whether backend plumbing needs to carry that metaphor.

---

## 6. ACTUAL STATE AND QUALITY ASSESSMENT

### Code Quality: B+

**Strengths:**
- Clean Rust idioms throughout trinity-server (the working part)
- Good error handling with anyhow/thiserror
- Proper async patterns with tokio
- Well-structured HTTP API with Axum
- Sane memory management (one-at-a-time sidecar design)
- The new sidecar system (built today) is clean, modular, and practical

**Weaknesses:**
- Kernel has accumulated 112 files with inconsistent quality (some excellent, some stubs)
- 24 test binaries in the kernel, most are one-off experiments that should be cleaned
- Dead code: sidecar_manager_old.rs, multiple archive/ directories
- Trinity-body at 40K lines is too large for one crate — should be split

### Architecture Quality: A-

The 3-layer architecture is genuinely good. The sidecar hotel metaphor maps perfectly to the hardware constraint. The quest board as a filesystem-based work queue is pragmatic and debuggable. The role-based single-binary approach we built today is clean and extensible.

### Operational Readiness: C+

- Layer 1 (headless server): **Operational** — chat, RAG, quests, tools all work
- Layer 2 (web UI): **Operational** — three pages serve and stream
- Sidecar system: **Built, untested** — compiles but never run against live models
- NPU: **Not operational** — mock functions only
- Bevy/XR: **Not operational** — rendering issues
- Music/Art/Voice: **Not operational** — no actual generation

---

## 7. MINIMAX NPU SIDECAR PLAN

### The Model

Found: `/home/joshua/ai_models/gguf/MiniMax-M2-5-REAP-50-Q4_K_M.gguf`
- **Size**: 66GB (Q4_K_M quantization)
- **Architecture**: MoE (Mixture of Experts) — likely ~450B total params, ~50B active per token
- **Quantization**: Q4_K_M — good balance of speed and quality for MoE
- **Runtime**: llama.cpp with ROCm (confirmed: gfx1151 RDNA 3.5 detected)

### Memory Budget Reality

```
128GB Total
 - 66GB  MiniMax model weights
 - 15GB  KV cache (8K context at 50B active params)
 -  8GB  OS + buffers
 ───────
 ~39GB  remaining
```

**Critical constraint**: MiniMax at 66GB CANNOT coexist with the conductor (GPT-OSS-20B at 12GB). Loading MiniMax requires stopping the conductor first. This is the "heavyweight champion" — when MiniMax enters the ring, everyone else leaves.

**This is fine architecturally** — the Iron Road Hotel is designed for one guest at a time. MiniMax is simply a very large guest who needs the presidential suite AND the ballroom.

### NPU + MiniMax Hybrid Architecture

```
┌──────────────────────────────────────────────────────────┐
│  MiniMax Sidecar — "The Colossus"                        │
│  ┌────────────────────┐  ┌─────────────────────────────┐ │
│  │ NPU Triage         │  │ MiniMax M2.5 (50B active)   │ │
│  │ (Granite 1B ONNX)  │  │ Port 8081                   │ │
│  │ On CPU/NPU         │  │ 66GB, ROCm GPU offload      │ │
│  │ ~100ms latency     │  │ 8K context                  │ │
│  │ Classifies & routes│  │ Full reasoning/generation   │ │
│  └────────────────────┘  └─────────────────────────────┘ │
│                                                           │
│  Flow:                                                    │
│  1. Request → Granite 1B classifies (simple/complex)      │
│  2. Simple tasks: Granite handles directly (~100ms)        │
│  3. Complex tasks: MiniMax generates (~5-30s)              │
│  4. MiniMax reviews its own output (self-check)            │
│                                                           │
│  Use case: "SME Production Mode"                          │
│  - Long-form document generation                          │
│  - Complex code architecture planning                     │
│  - Multi-file refactoring                                 │
│  - Deep instructional design analysis                     │
│  Port 8090 — Sidecar API (same as all roles)              │
└──────────────────────────────────────────────────────────┘
```

### Why This Matters

MiniMax at 50B active params is the most capable model in your portfolio. It should handle tasks that even the Opus 27B struggles with:
- Whole-codebase reasoning across multiple files
- Complex architectural decisions
- Production-quality long-form document generation
- The kind of "SME workflow" where you need deep expertise in one session

The Granite 1B on NPU/CPU acts as a fast triage layer — 80% of requests don't need 50B params. A 1B classifier can route "what time is it" away from the colossus and let it focus on hard problems.

### Implementation Plan

**Phase 1 — Prove ORT Pipeline (1-2 sessions)**
1. Create `crates/trinity-sidecar-engineer/src/ort.rs` — ORT inference module
2. Load Granite 1B SafeTensors (already at `models/npu/granite/granite-4.0-1b-converted/`)
3. Run on CPU first (ORT with CPU EP) — just prove the Rust ORT pipeline works
4. Add a `/classify` endpoint to the sidecar API
5. Success criteria: Granite 1B classifies a prompt as simple/complex

**Phase 2 — MiniMax Integration (1 session)**
1. Add a "colossus" role to `roles.rs`
2. Model: MiniMax GGUF on port 8081, Granite 1B via ORT as classifier
3. Start MiniMax with `-ngl 99` (ROCm GPU offload — critical for speed)
4. Test with `curl` against the API
5. Success criteria: MiniMax generates a response via the sidecar API

**Phase 3 — NPU Acceleration (Experimental)**
1. Swap ORT CPU EP for Vitis AI EP
2. Test if XDNA 2 actually loads and runs the 1B model
3. Measure latency vs. CPU
4. This is the "moonshot" — may not work yet on bleeding-edge kernel

### Realistic NPU Assessment

The NPU is the highest-risk component in Trinity:
- XDNA 2 driver is bleeding-edge (kernel 6.19.4)
- ORT Rust bindings with Vitis AI EP have not been proven on Strix Halo by anyone publicly
- The `npu_engine.rs` in the kernel is entirely mock functions
- `/dev/accel0` exists and `amdxdna` driver is loaded — the hardware is there

Pragmatic path: **Get ORT working on CPU first, then try NPU.** The CPU fallback for Granite 1B at ~100ms is already fast enough to be useful as a classifier. NPU would reduce that to ~20ms — nice but not blocking.

### New Role Definition

```
colossus:
  name: "The Colossus"
  icon: "🏔️"
  primary: MiniMax-M2-5-REAP-50 (66GB, port 8081, 8K context, -ngl 99)
  secondary: None (Granite 1B via ORT for classification, not longcat-sglang)
  skill: "Titan Mind — deep reasoning across entire codebases, production-quality generation"
  quest_types: [analysis, feature, refactor, documentation]
  addie_phases: [Analysis, Design, Development, Evaluation]
  memory: 66GB + 15GB KV = ~81GB (requires stopping conductor)
```

---

## 8. RECOMMENDED NEXT ACTIONS (Priority Order)

### Immediate (This Session)
1. **Test the Engineer sidecar with live models** — this is the most important thing. We built the system; now we need to run it and see what breaks.
2. **Add `-ngl 99` to longcat-sglang args** — free speedup on unified memory.

### This Week
3. **Fix the 2 broken test binaries** (test_gpt_oss_npu, test_trinity_orchestrator)
4. **Persist quest state to PostgreSQL** — player progress survives restarts
5. **Run first autonomous overnight session** — even 2-3 completed quests proves the concept
6. **Clean kernel dead code** — move 24 test binaries to examples/, delete _old files

### This Month
7. **Get ORT loading a real model on CPU** — even without NPU, this proves the ONNX pipeline
8. **Build one complete game template** (2D puzzle) through the full pipeline
9. **MiniMax sidecar** if the model is available locally
10. **PersonaPlex voice stub** with ORT fallback to text

### Hard Truth
The system needs **depth before breadth**. The Engineer sidecar working reliably end-to-end is worth more than 5 more party members. One completed quest that actually fixes a bug proves the system works. One playable game template proves the economy works. Focus the Iron Road — one track at a time.

---

*"The Great Recycler doesn't build more tracks. He makes the ones he has carry freight."*

— Audit by Cascade, March 14, 2026
