# TRINITY ID AI OS — Master Pivot Document

**Date:** July 2, 2026 (updated July 5, 2026)
**Author:** Joshua Atkinson + Cascade
**Status:** Strategic pivot — Purdue academic tool → AMD Strix Halo reference implementation → Spatial computing platform for imagination amplification

> **Update note (July 5, 2026):** This document captures the strategic thinking at the time of the pivot. Some implementation details have since changed:
> - **FACES Protocol** moved to the Semantic Slime project as `faces-protocol`. The `trinity-faces` crate no longer exists in this workspace. The "Image" reflection concept remains in Trinity's philosophy.
> - **Qwythos-9B (R)** removed from the architecture. P (DiffusionGemma) handles both Socratic questioning and execution. Hermes 4 70B (H) handles planning via LM Studio.
> - **Hotel modes** restructured to Studio/Solo/Closed (3 modes, no more Team/Swap).
> - **P-ART-Y** terminology replaced with P+A+H (Producer + Art Dept + Hermes).
> - See `docs/active/MASTER_TASK_LIST.md` for the current authoritative architecture.

---

## 0. THE THESIS: AI CAN COMPUTE, TRINITY IS FOR THE IMAGINATION

### The Core Insight

AI can compute. That's not the valuable part. The most valuable tool a human has is **imagination** — the ability to envision something that doesn't exist yet, hold it in mind, refine it, and bring it into being. Every piece of great work — every product, every curriculum, every building, every piece of music — started as an image in someone's imagination.

The problem with current AI tools: they jump straight to output. "Give me a thing." The AI generates, the human reacts, the result is mediocre because the AI never understood what the human actually imagined.

**Trinity's approach: the AI questions first, then builds.** Like a good doctoral advisor or senior consultant — they don't start writing until they've drawn out your vision through Socratic dialogue. The human is the SME. The AI is the curious apprentice that reflects back what it hears, asks for more detail, and only *then* begins work.

If LLMs were trained to treat the user as an SME and use Socratic questioning to explore the detail of the content before working, all work thereafter would be 100% better due to vision and planning within the human imagination.

### The Triple Reflection

Trinity's cognitive framework is a triple reflection — three mirrors that together produce full-spectrum understanding before any work begins:

| Reflection | Function | What It Does |
|-----------|----------|-------------|
| **FACES** (Image) | Set and setting | Establishes the emotional and atmospheric context. The AI understands *how it feels* to be in this work. 4-byte protocol, 38,400 states, NPU-accelerated. |
| **LitRPG** (Narrative) | Perspective engineering | Lightweight quest/role scaffolding for workflows. Not heavy gamification — just structural framing that gives work a narrative arc. The user is the protagonist, the work is the quest, the AI is the companion. |
| **Socratic** (Depth) | Reflective questioning | The AI treats the user as SME, questions them before working, draws out the full vision from the human imagination. Reflective depth in content creation and intentional product improvements. |

**How they work together:**
1. **Socratic** draws out the vision from the human imagination (depth) — on the phone
2. **FACES** establishes the emotional context of the work (image) — on the NPU
3. **LitRPG** frames the work as a quest with structure (narrative) — on the phone
4. **ADDIECRAPEYE** executes the work in 12 phases (action) — on the desktop
5. **Strix Halo** provides the compute (NPU for FACES, GPU for language, 128GB for memory) — the engine
6. **Android XR** makes the imagination visible in space (output) — on the glasses

### The Three-Device Architecture (HPT Framework)

Trinity is a **Human Performance Technology wrapper** — AI for compute, human for imagination. The system spans three devices, each with a distinct role:

```
┌─────────────────────────────────────────────────────────────────────┐
│  PHONE (Pixel 10 Pro XL) — THE DIRECTOR                           │
│  "Trinity in your pocket" — human input device                     │
│                                                                     │
│  Kotlin + ADK + Gemini Nano                                         │
│  ├── Socratic questioning engine (interviews Joshua for vision)    │
│  ├── ADDIECRAPEYE phase tracker (directs the workflow)             │
│  ├── FACES state display (ASCII face on phone screen)              │
│  ├── Quest management (LitRPG framing)                             │
│  └── WebSocket client → Strix Halo (sends commands, receives FACES)│
│                                                                     │
│  Human input: voice + touch                                        │
│  Human output: imagination, direction, content expertise           │
└──────────────────────────┬──────────────────────────────────────────┘
                           │ WiFi / WebSocket
                           │ Commands up, FACES states down
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│  DESKTOP (Strix Halo) — THE ENGINE                                │
│  "AI for compute" — the heavy lifter                               │
│                                                                     │
│  Rust (trinity crate, trinity-faces crate)                         │
│  ├── GPU → LLM inference (7B-70B)                                 │
│  ├── NPU → FACES-Embed (~66M, INT8)                               │
│  ├── CPU → Rust orchestration, ADDIECRAPEYE conductor              │
│  ├── telephone.rs — WebSocket audio pipeline                       │
│  └── conductor_leader.rs — ADDIECRAPEYE prompts                    │
│                                                                     │
│  AI input: commands from phone                                     │
│  AI output: FACES states, LLM responses, generated content         │
└──────────────────────────┬──────────────────────────────────────────┘
                           │ WiFi6E / WebSocket
                           │ FACES states + content
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│  XR (XREAL Aura) — THE CANVAS                                     │
│  "EYE phase — interactive review and sensory experience"           │
│                                                                     │
│  Kotlin + Jetpack Compose for XR + ARCore + ADK                    │
│  ├── SpatialPanel → FACES ASCII face in space                     │
│  ├── Orbiter → state info, congruence, confidence                  │
│  ├── SpatialGltfModel → 3D avatar with FACES expression            │
│  ├── ARCore → anchor FACES panels to physical locations           │
│  ├── EYE phase UI — Envision, Yoke, Evolve                        │
│  ├── ADK + Gemini Nano → standalone FACES detection (fallback)    │
│  └── WebSocket client → Strix Halo (receives FACES + content)     │
│                                                                     │
│  Human input: hand gestures (pinch), voice, gaze                  │
│  Human output: spatial review, sensory immersion, EYE phase       │
└─────────────────────────────────────────────────────────────────────┘
```

**The pipeline flows:**
```
Human Imagination (the most valuable resource)
       ↓
  PHONE: Socratic questioning (AI draws out the vision)
       ↓
  DESKTOP: FACES (AI understands the emotional context, NPU)
       ↓
  PHONE: LitRPG (AI frames the work as a quest)
       ↓
  DESKTOP: ADDIECRAPEYE (AI executes in 12 phases, GPU + CPU)
       ↓
  XR GLASSES: EYE phase (Envision, Yoke, Evolve — imagination visible in space)
```

**Data flows:**
- **Socratic questioning:** Phone → Desktop (voice/text commands)
- **FACES states:** Desktop → Phone + XR (4-byte WebSocket stream)
- **Content output:** Desktop → XR (for EYE phase spatial review)
- **Human feedback:** XR → Phone (next direction from spatial review)

### Why This Is Different

| Current AI Tools | Trinity |
|-----------------|---------|
| Human prompts, AI generates | AI questions, human imagines, AI reflects, then generates |
| Output-focused | Vision-focused, then output |
| AI is the expert | Human is the SME, AI is the apprentice |
| No emotional context | FACES provides set and setting |
| No narrative structure | LitRPG provides quest scaffolding |
| Flat interaction | Socratic depth before work begins |
| Runs on any hardware | Designed for Strix Halo's NPU+GPU parallelism |
| Output stays on screen | Output becomes visible in XR space |

### The Two-Year Arc

This is why Joshua has been working on AI for two years. Not to build a chatbot. Not to build a benchmark. To build a system that amplifies human imagination:

- **Year 1 (2025):** Trinity ID AI OS — AI agent runtime, ADDIECRAPEYE, autopoietic engine, PEARL, MCP server
- **Year 2 (2026):** Voix Vive XR — spatial computing prototype, Bevy/OpenXR, Android XR, Kotlin/Jetpack XR SDK
- **Convergence:** Trinity FACES on Strix Halo (base station) + spatial XR client (Android XR) = imagination amplification system for professional communication, education, and workspace

The vision: **AI that helps you imagine better, then helps you build what you imagined, then shows it to you in space.**

---

## 1. THE HONEST ASSESSMENT

### What's Right

**ADDIECRAPEYE phase prompts** (`crates/trinity/src/conductor_leader.rs:70-179`) — Real ID methodology as LLM constraints. 12 phases, each with Bloom's level, Socratic instructions, guardrails. Crown jewel.

**Quest objectives** (`crates/trinity-quest/src/objectives.json`) — Concrete ID work, not gamified fluff.

**PEARL** (`crates/trinity-protocol/src/pearl.rs`) — Subject + Medium + Vision. Clean, one per project.

**EYE export** (`crates/trinity/src/export.rs`) — Real self-contained HTML5 deliverables.

**Inference router** — Auto-detection, health probing, failover. Production-grade.

**Living code textbook** — Heavily-commented Rust as curriculum. Novel.

### What's Wrong

**Scope creep detector** (`crates/trinity/src/scope_creep.rs:56-99`) — Triggers on "also"/"add" in >7 word messages. False positives.

**VAAM** (`crates/trinity/src/vaam.rs`) — Keyword matching, not comprehension. Copy-paste = earn Coal.

**Quality scorecard** (`crates/trinity/src/quality_scorecard.rs`) — Heuristic word-patterns. Gameable.

**No knowledge tracing** — Tracks clicks, not learning. Gamified chat, not ITS.

**Monolithic files** — main.rs 223KB/5,251 lines. Unmaintainable.

**Test coverage** — 2 test files for ~26K LOC.

### Isomorphic Layering

**One good isomorphism:** ADDIECRAPEYE 12 ↔ Hero's Journey 12 ↔ Bloom's 6. Three frameworks, one spine. Assists.

**5-6 parallel frameworks that don't map:** P-ART-Y (infra not pedagogy), Sacred Circuitry (15≠12, Bashar credibility risk), Cognitive Thermodynamics (invented physics), VAAM (keyword scanner), Four Horses (no code), Cow Catcher/Hook Book/Bible Car (file header noise).

**Core problem:** ~70 concepts to understand before use. Teacher needs 3: PEARL, current phase, export.

---

## 2. THE PURDUE PROBLEM

Sent three times. No engagement. Scope incomprehensible to busy academics — presents as 8 things at once. Dr. Wanju interested but can't use it.

**Three paths:** Finish for Purdue (rejected, 6-12mo committees), Homeschool (viable secondary, 4.3M students), Personal use first (strategic).

**Conclusion:** Scope isn't code, it's concept. Sent cosmology when they needed a tool.

---

## 3. PIVOT 1: AI AS AUDIENCE

Change audience from humans to AI. Code doesn't change — audience changes.

**Already built for AI:** Autopoietic engine (self-modifying code, `crates/trinity-mcp-server/src/autopoietic.rs:1-323`), Self-Work MCP (`self_work.rs:1-319`), TODO Parser (AI consuming developer intent), pedagogical schema (AI-consumable datasets).

**Original vision said it:** "Finished when Trinity improves its own curriculum." "Simulates a creator. Persistent entity that lives on this machine."

**Reframing:** Pete teaches AI agents (not teachers). ADDIECRAPEYE = cognitive scaffold (not game). EYE = structured artifact (not quiz). VAAM = agent mastery tracking. PEARL = agent task contract.

**Why better:** Solves delivery (MCP, no UI). Makes layering disappear (AI tracks 15+12+5 in context window). Novel contribution (AI teaching AI to teach). Living textbook works (128K context = 223KB is one prompt).

**Limitation:** Alone doesn't solve hardware problem. Needs hardware partner.

---

## 4. PIVOT 2: AMD AS TARGET

### The Document

Joshua already wrote `~/Downloads/Optimizing GMKtech Evo X2 AI Capabilities.md` — a Strix Halo whitepaper naming Trinity as culminating architecture. Describes Crate Economy (77.5GB/50GB), autopoietic QLoRA, "No Python Snakes" Rust constitution. **Joshua already wrote the pitch.**

### Why AMD > Purdue

| Factor | Purdue | AMD |
|--------|--------|-----|
| Need | Curriculum tools | Killer apps selling hardware |
| Speed | 6-12 months | Weeks |
| Understands | Pedagogy | Hardware + software |
| Offers | Pilot | Hardware, support, co-marketing |
| Leverage | One professor | Entire product line needs software |

### AMD's Problem

Hardware impressive, software ecosystem not. NVIDIA has CUDA + decade tooling. AMD has ROCm catching up. gfx1151 not in stable ROCm. Need apps showcasing unified memory, bypassing PyTorch fragility, demonstrating autopoietic use, connecting to MCP, Rust-native. Trinity does 4 of 5.

### The Reframe

Trinity = Strix Halo-optimized AI agent runtime, ID as vertical. Product is runtime. Domain is differentiator.

### Key Hardware Facts

**Specs:** Ryzen AI Max+ 395, 16 Zen 5 cores, RDNA 3.5 (40 CUs), XDNA 2 NPU (50 TOPS), 128GB LPDDR5X, 256 GB/s, 59.39 FP16 TFLOPS.

**Memory unlock:** `ttm.pages_limit=31457280` (120GB), `ttm.page_pool_size=15728640` (60GB pre-alloc), `amdgpu.vm_fragment_size=8`, `amd_iommu=off`.

**Benchmarks:** Vulkan RADV short context 85 t/s. ROCm hipBLASLt 130K context 13 t/s. ROCm mandatory for >100K tokens.

**Critical flags:** `--no-mmap` (load: hours→22s), `TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL=1` (19x SDPA), unset `PYTORCH_HIP_ALLOC_CONF`.

**Crate Economy:** 124GB → 77.5GB host + 50GB dynamic sandbox. Kernel evaluates payload, cleanup(), hot-swap via curl. Zero downtime.

**Autopoietic QLoRA:** Nightly MXFP4, `Mxfp4Config(dequantize=True)`, peak 32-38GB, fits sandbox.

**Lemonade SDK:** AMD middleware, OpenAI-compatible + MCP, auto-profiles hardware.

---

## 5. THE FACES DISCOVERY: NPU + GPU PARALLELISM

### The Insight

Every reviewer says NPU competes with GPU for memory. Joshua's insight: **run them in parallel on independent compute paths.**

```
GPU (40 CUs)     →  LLM token generation (full speed, no contention)
NPU (50 TOPS)    →  FACES emotive detection + expression rendering
CPU (16 cores)   →  Rust orchestration (Trinity kernel)
```

FACES is 4 bytes. FACES-Embed is ~66M params (DistilBERT). Fits NPU without touching GPU bandwidth. **No other consumer hardware has both an NPU and ROCm GPU.** NVIDIA DGX Spark has no NPU. Apple M4 Max has no ROCm. (Note: the NPU+GPU parallel compute concept itself is not unique to Trinity — see Appendix J for existing projects. Trinity's uniqueness is in running *independent* workload types in parallel, not splitting a single LLM task.)

### What FACES Is

**FACES = Fact Align Computational Emotive System** — 4-byte (32-bit) protocol for emotional/intent states via ASCII geometry.

| Byte | Component | Description |
|------|-----------|-------------|
| 0 | Aura/Color | 8-bit ANSI/Hex mood index |
| 1 | Container/Shape | Boundary: `()` `[]` `{}` `||` `<>` |
| 2 | Focus/Eyes | Attention: `oo` `><` `OO` `..` `^^` `--` |
| 3 | Action/Mouth | Intent: `_` `v` `~` `-` `.` |

**38,400 distinct states.** Render: `{O~O}` = fluid, open, contemplative, soft.

### Theoretical Foundations

- **Pareidolia:** Brain wired for faces in simple patterns. ASCII triggers empathy, bypasses Uncanny Valley, negligible compute.
- **Mehrabian 7-38-55:** FACES proxies the 55% text-based AI misses.
- **Brene Brown:** Vulnerability = `OO` + `{}`. Defensiveness = `[]` + `><`.
- **Alan Watts:** Fluid `{}` + wavy `~` = continuous transitions over binary switching.
- **Ekman FACS:** AU4+AU7 = `><`. AU5 = `OO`. AU12 = `~`.
- **Mian Xiang:** Five Elements = Container shapes (Wood=`||`, Fire=`<>`, Earth=`[]`, Metal=`()`, Water=`{}`).
- **Committee Mapping:** Aura=Heart=Mastery, Container=Mind=Autonomy, Focus=Body=Acquisition, Action=Will=Vocabulary.
- **Congruence/Incongruence:** Text matches FACES = congruence. Diverges = sarcasm, fatigue, high thought-load.

### FACES-Embed

~66M param DistilBERT encoder-only. Maps expressions to 38,400 states. ONNX for NPU. Sub-millisecond. Reduces LLM context by 1,500-2,500 tokens/session.

### What Trinity Already Has

- `crates/trinity-protocol/src/character_sheet.rs:87-172` — `VoiceEmotion` enum (6 states) + `detect_emotion()` keyword detector
- `crates/trinity-protocol/src/types.rs:37-46` — `EmotionData` struct (5 float dimensions)
- `crates/trinity/src/voice.rs:666-668` — `check_npu_availability()` checks `/dev/xdna`
- `crates/trinity/src/voice.rs:71` — `npu_available` field in `VoiceStatus`
- `crates/trinity-voice/src/lib.rs:13` — "PersonaPlex ONNX types retained for future NPU path"

### The Gap

| Component | Status | Gap |
|-----------|--------|-----|
| FACES protocol spec | Done | None |
| FACES Rust engine | Done (terminal) | Integrate into Trinity |
| FACES-Embed ONNX | Designed | Train + export |
| NPU detection | Done | Add XRT inference |
| Trinity emotion | Keyword matching | Upgrade to FACES 4-byte |
| GPU/NPU parallel | Not built | Core demo |
| LLM to FACES bridge | Not built | Output to state mapping |

---

## 6. FINAL ARCHITECTURE: TRINITY FACES FOR STRIX HALO

### Three-Layer Stack

```
+-------------------------------------------+
|         MCP INTERFACE LAYER               |
|  Any AI agent connects via MCP            |
+------------------+------------------------+
|         TRINITY KERNEL (Rust)             |
|  ADDIECRAPEYE | Inference Router |        |
|  Autopoietic Engine | PEARL |             |
|  Crate Economy (50GB) | FACES Engine      |
+------------------+------------------------+
|         HARDWARE COMPUTE LAYER            |
|  GPU: LLM tokens | NPU: FACES | CPU: OS   |
+-------------------------------------------+
```

### Unique Value Proposition

**Trinity FACES runs independent heterogeneous workloads in parallel on Strix Halo — emotive AI on NPU while LLM generates on GPU.** This is distinct from existing projects (halo, strix-halo-pipeline, hal0) that split a *single LLM task* across NPU+GPU. Trinity runs a *different workload class* on each accelerator. All local, all Rust, all AMD hardware. See Appendix J for competitive analysis.

---

## 7. WHAT TO KEEP, CUT, BUILD

### Keep

- ADDIECRAPEYE phase system (AI instructions)
- PEARL (agent task alignment)
- MCP server (delivery mechanism)
- Autopoietic engine (self-improvement)
- Pedagogical schema (AI datasets)
- Cross-dataset search (query routing)
- Quality Scorecard (AI self-evaluation)
- EYE export (structured artifacts)
- Inference router (multi-backend)
- Voice/emotion system (upgrade to FACES)
- NPU detection (already built)

### Cut

- React frontend (terminal demo for AMD)
- LitRPG narrative (human overhead)
- Sacred Circuitry (doesn't sell hardware)
- Four Horses (no code, no runtime)
- Cow Catcher / Hook Book / Bible Car headers (file noise)
- Caddy / Cloudflare tunnel (not relevant)
- Daydream Bevy 3D (unless showcasing spatial AI)

### Build

1. FACES integration into `trinity-protocol`
2. NPU inference path via AMDXDNA driver
3. LLM to FACES bridge (output to 4-byte state)
4. Split-terminal demo (LLM left, FACES right)
5. Lemonade SDK integration
6. ROCm-aware crate swapping (77.5GB/50GB)
7. Nightly QLoRA pipeline (MXFP4)
8. Benchmark script (Evo X2 + Trinity vs DGX Spark)

---

## 8. THE DEMO PLAN

### What AMD Sees

One terminal. Two panes. Two minutes.

**Left:** LLM streaming at 80+ t/s (GPU, full speed)
**Right:** FACES rendering real-time — ASCII face morphing as emotional state shifts

```
GPU:  [Generating] "I understand your frustration..."  82.4 t/s
NPU:  [FACES]     Aura:245 | Container:{} | Focus:OO | Action:~
      Render:      {O~O}    open, contemplative, soft
```

Angry input:
```
GPU:  [Generating] "That is a serious concern..."  79.1 t/s
NPU:  [FACES]     Aura:196 | Container:[] | Focus:>< | Action:-
      Render:      [>-<]    rigid, intense, tight
```

**Status bar:** ttm.pages_limit | GPU mem | NPU power | tokens/s | FACES latency

### Why It Works

1. Proves NPU + GPU parallel with independent workloads (emotive AI + LLM, not prefill/decode split)
2. AI has real-time emotional awareness
3. All local, all Rust, all AMD
4. No competitor runs emotive AI on NPU alongside GPU LLM
5. Two minutes = executive attention span

### Build Phases

**Phase 1 (1-2 days):** Port `faces_engine.rs` into `trinity-protocol`. Replace keyword `detect_emotion()` with FACES 4-byte mapping. Heuristic LLM-to-FACES mapping.

**Phase 2 (2-3 days):** Load FACES-Embed ONNX via AMDXDNA. Run on `/dev/xdna`. Benchmark GPU t/s before/after.

**Phase 3 (1 day):** Split terminal UI. Telemetry status bar. Record video.

**Phase 4 (1 day):** One-page pitch doc. Link video. Send to AMD dev relations.

---

## 9. THE AMD PITCH

### One Sentence

> Trinity is an imagination amplification system — AI that questions you like a doctoral advisor, understands the emotional context of your work via FACES on the NPU, runs language generation on the GPU, and renders the result in XR space. Strix Halo is the only hardware that can do all of this in parallel, on one machine, with no cloud.

### One Paragraph

Trinity ID AI OS is a Rust-native imagination amplification system for Strix Halo. Most AI tools jump straight to output — Trinity questions first. The AI treats the user as the domain expert, uses Socratic dialogue to draw out their full vision, then executes through a 12-phase instructional design framework. While the GPU runs LLM inference, the XDNA 2 NPU runs FACES — a 4-byte emotive AI protocol that gives the AI real-time emotional awareness of the work's context. The output doesn't stay on a screen — it renders in Android XR space, where the imagination becomes visible. This is the future of professional communication, education, and workspace: AI that helps you imagine better, build what you imagined, and see it in space. We'd like to partner with AMD to ship Trinity as a reference implementation for the Evo X2.

### The Ask

A GMKtec Evo X2 unit for development. In exchange: a reference implementation that makes people buy the hardware.

### What AMD Gets

1. **Reference implementation** — Trinity on Evo X2 as demo unit at events
2. **NPU utilization story** — only software running emotive AI (not LLM prefill/decode) on XDNA 2 alongside GPU
3. **Rust-native credibility** — AMD ROCm docs reference Rust; Trinity is pure Rust
4. **MCP ecosystem presence** — connects Strix Halo to emerging agent standard
5. **Autopoietic differentiator** — 24/7 self-improving agents justify 128GB unified memory
6. **Spatial computing story** — Strix Halo as base station for Android XR. Imagination amplification isn't a terminal demo — it's a spatial product. AMD hardware powers the entire pipeline from NPU emotion detection to GPU LLM inference to XR rendering.
7. **Imagination amplification narrative** — not "AI replaces humans" but "AI helps humans imagine better." AMD becomes the hardware of human creativity, not the hardware of automation.

---

## 10. USING ADDIECRAPEYE ON THE PIVOT

### Analysis (Remember/Understand)
- **SME:** Joshua. Solo dev, working Rust codebase, deep Strix Halo knowledge, already wrote the pitch doc.
- **Gap:** Code but no demo. AMD won't read code. They need to see it run.
- **Audience:** AMD product managers and dev relations. Understand hardware, respect benchmarks, need reference implementations.

### Design (Apply)
- **Objective:** "AMD engineer clones repo, runs one command, watches Trinity load model, stream tokens on GPU, render FACES on NPU — under 2 minutes."
- **Medium:** Terminal demo (not web, not 3D, not slides)
- **Assessment:** Does hardware look good? Does NPU show value? Is it all local?

### Development (Create)
- Port FACES into Trinity protocol
- Wire NPU inference via `/dev/xdna`
- Build split-terminal demo
- Add telemetry status bar

### Implementation (Apply)
- Run on desktop (still available)
- Record 2-minute video
- Test: does GPU t/s drop when NPU active? (Shouldn't)

### Evaluation (Evaluate)
- Does AMD hardware look good? Yes.
- Does it show NPU value? Yes.
- Reproducible? One command.
- Ship it.

### CRAP (Contrast/Repetition/Alignment/Proximity)
- **Contrast:** Every Strix Halo review ignores NPU. We make it the star.
- **Repetition:** FACES updates every token batch — consistent, visible.
- **Alignment:** Demo scope = NPU+GPU parallel. Nothing else. No creep.
- **Proximity:** Two panes, one screen. GPU left, NPU right. Status bar connects.

### EYE (Envision/Yoke/Evolve)
- **Envision:** "AMD sees their hardware do something no other software does — emotive AI on NPU alongside LLM on GPU."
- **Yoke:** FACES protocol → Trinity kernel → Strix Halo. One pipeline.
- **Evolve:** Ship video. Send pitch. Get the laptop.

---

## 11. CODEBASE INVENTORY

### Main Repo: `/home/joshua/Workflow/TRINITYIDAIOS`

**Core crates:**
- `crates/trinity/` — Main server (main.rs 223KB, agent.rs 109KB, tools.rs 116KB)
- `crates/trinity-protocol/` — Shared types, PEARL, character sheet
- `crates/trinity-quest/` — Quest state, objectives.json
- `crates/trinity-mcp-server/` — MCP, autopoietic, self-work, pedagogical schema
- `crates/trinity-voice/` — Audio types, NPU path refs
- `crates/trinity-iron-road/` — Narrative, VAAM, Pete core
- `crates/trinity-daydream/` — Bevy 3D client (cut for AMD pivot)

**Key files for pivot:**
- `crates/trinity/src/conductor_leader.rs:70-179` — ADDIECRAPEYE prompts (KEEP)
- `crates/trinity-protocol/src/pearl.rs` — PEARL (KEEP)
- `crates/trinity-mcp-server/src/autopoietic.rs:1-323` — Self-modify engine (KEEP)
- `crates/trinity-mcp-server/src/self_work.rs:1-319` — AI workflows (KEEP)
- `crates/trinity-mcp-server/src/pedagogical_schema.rs` — AI datasets (KEEP)
- `crates/trinity-mcp-server/src/cross_dataset.rs` — Query routing (KEEP)
- `crates/trinity/src/quality_scorecard.rs` — Self-eval (KEEP)
- `crates/trinity/src/eye_container.rs` — Export (KEEP)
- `crates/trinity/src/voice.rs:666-668` — NPU detection (KEEP, UPGRADE)
- `crates/trinity-protocol/src/character_sheet.rs:87-172` — VoiceEmotion (UPGRADE to FACES)
- `crates/trinity-protocol/src/types.rs:37-46` — EmotionData (UPGRADE to FACES)
- `crates/trinity/src/scope_creep.rs` — Keyword detector (CUT/rework)
- `crates/trinity/src/vaam.rs` — Keyword scanner (CUT/rework)

### Genesis Archive: `/home/joshua/Workflow/desktop_trinity/trinity-genesis`

Original AI-facing vision. Key refs:
- `docs/CRITIQUE_AND_ROADMAP.md:64-66` — "Finished when Trinity improves own curriculum"
- `docs/ROADMAP.md:37` — "System should write its own code"
- `docs/RESEARCH_ANALYSIS.md:73-75` — "Simulates a creator. Persistent entity."
- `docs/VAAM_BLUEPRINT.md` — Cognitive Load as physics
- `docs/phoenix_protocol.md` — Self-healing watchdog
- `crates/trinity-kernel/src/todo_parser.rs` — TODO to tasks

### FACES Documents: `~/Downloads/`

- `FACES Master Specification and Funding Proposal.docx` — Full protocol, theory, 4-byte spec, FACS/Mian Xiang mapping
- `FACES Engine Zero-Dependency Codebase.docx` — C and Rust impls
- `faces_engine.rs.docx` — Rust FacesState with render() and to_hex()
- `FACES Project Completion and Market Deployment Strategy.docx` — AGPLv3, pricing, commercial
- `FACES Protocol_ Catalyst Application.docx` — Android XR app, Purdue talking points
- `FACES Mechanical ELIZA Therapist Codebase.docx` — ELIZA + FACES integration
- `FACES Sovereign Systems Architect Income and Automation Framework.docx` — Income strategy
- `Master_Research_Engineering_Summary_FACES_and_AI.docx` — Master summary

### Strix Halo Document: `~/Downloads/Optimizing GMKtech Evo X2 AI Capabilities.md`

156-line whitepaper. Kernel params, ROCm optimization, TTM memory math, benchmarks, Trinity as culminating architecture. The document that proved Joshua already had the AMD pitch written.

---

## 12. GLOSSARY

**ADDIECRAPEYE** — 12-phase instructional design framework: Analysis, Design, Development, Implementation, Evaluation, Contrast, Repetition, Alignment, Proximity, Envision, Yoke, Evolve. Trinity's cognitive scaffold.

**Autopoietic** — Self-creating/self-improving. Trinity's ability to modify its own code, fine-tune its own models, and improve without human intervention.

**Crate Economy** — Memory management architecture: 77.5GB host + 50GB dynamic sandbox for hot-swapping AI model weights on Strix Halo.

**FACES** — Fact Align Computational Emotive System. 4-byte protocol for emotional states via ASCII geometry. 38,400 distinct states. The "Image" reflection in Trinity's triple reflection.

**FACES-Embed** — ~66M param DistilBERT encoder-only model for mapping expressions to FACES states. ONNX for NPU deployment.

**FACS** — Facial Action Coding System. Paul Ekman's empirical AU framework mapped to FACES bytes.

**gfx1151** — AMD ISA architecture designation for RDNA 3.5 (Radeon 8060S in Strix Halo).

**Imagination Amplification** — Trinity's core thesis. AI doesn't replace human creativity — it helps humans imagine better by questioning them (Socratic), understanding emotional context (FACES), and framing work as quests (LitRPG) before generating output.

**Lemonade SDK** — AMD's middleware for local AI orchestration. OpenAI-compatible + MCP.

**LitRPG** — Literary Role-Playing Game. In Trinity, lightweight quest/role scaffolding for workflows. The "Narrative" reflection — gives work a narrative arc without heavy gamification.

**MCP** — Model Context Protocol. Emerging standard for AI agent tool connectivity.

**Mehrabian 7-38-55** — 7% words, 38% prosody, 55% body/face. FACES proxies the 55%.

**Mian Xiang** — Chinese face reading. Five Elements mapped to FACES Container shapes.

**NPU** — Neural Processing Unit. XDNA 2 on Strix Halo, 50 TOPS. Runs FACES.

**PEARL** — Pedagogical Engine for Artifact Representation and Alignment Logic. Subject + Medium + Vision. One per project.

**ROCm** — Radeon Open Compute. AMD's GPU compute stack (vs NVIDIA CUDA).

**Socratic Reflection** — The "Depth" reflection in Trinity's triple reflection. AI treats user as SME, questions them before working, draws out the full vision from the human imagination.

**Strix Halo** — AMD Ryzen AI Max+ 395. 128GB unified memory, RDNA 3.5 + XDNA 2 NPU. Trinity's base station.

**Triple Reflection** — Trinity's cognitive framework: FACES (Image) + LitRPG (Narrative) + Socratic (Depth). Three mirrors that produce full-spectrum understanding before work begins.

**TTM** — Translation Table Maps. Linux kernel subsystem for GPU memory allocation. `ttm.pages_limit` unlocks 120GB.

**VAAM** — Vocabulary Acquisition Autonomy Mastery. Originally keyword-based game economy, repurposed as agent concept mastery tracking.

**VoiceEmotion** — Trinity's current 6-state emotion enum (Neutral, Warm, Urgent, Sarcastic, Celebratory, Contemplatory). To be upgraded to FACES 4-byte.

**WMMA** — Wave Matrix Multiply Accumulate. RDNA 3.5 compiler intrinsics for hardware-accelerated matrix math. Enables 59.39 FP16 TFLOPS.

**XDNA 2** — AMD's NPU architecture in Strix Halo. 50 TOPS for power-efficient inference.

---

## 12.5. THE SPATIAL COMPUTING ROADMAP: STRIX HALO + ANDROID XR

### The Architecture

```
┌─────────────────────────────────┐         ┌─────────────────────────────────┐
│   STRIX HALO (Base Station)     │         │   ANDROID XR (Spatial Client)   │
│   GMKtec Evo X2                 │         │   XREAL Aura / Pixel 10 Pro     │
│                                 │         │                                 │
│   GPU: LLM inference            │  WiFi6E │   Display: Optical see-through  │
│   NPU: FACES emotive AI         │ ←─────→ │   NPU: Tensor/Snapdragon        │
│   CPU: Rust orchestration       │  MCP /  │   Audio: Open-ear + mic array   │
│   128GB: Unified memory         │  gRPC   │   Tracking: Hand (25 joints)    │
│   Autopoietic: 24/7 self-improve│         │   OS: Android XR                │
│                                 │         │                                 │
│   Trinity Kernel (Rust)         │         │   Trinity XR Client (Rust/Bevy) │
└─────────────────────────────────┘         └─────────────────────────────────┘
```

Strix Halo is the compute-heavy base station. Android XR is the display and interaction layer. FACES is the emotional bridge between them. The user imagines, Trinity questions and builds on Strix Halo, the result renders in XR space.

### What Already Exists (Bertrand-Masterclass Codebase)

**Location:** `/home/joshua/Workflow/Bertrand-Masterclass`

This is not a separate project — it's the XR client prototype for Trinity. Two years of work converged on this architecture.

#### Bevy + OpenXR Engine (`apps/spatial-engine-bevy/`)

Working Rust code, zero errors, zero warnings:

| File | Purpose | Trinity Integration |
|------|---------|---------------------|
| `fretboard.rs` (12KB) | 3D holographic rendering, 78 potholes, emissive materials, scale highlighting | Generalizes to any spatial UI — not just guitar. FACES state could drive pothole colors. |
| `pitch_detection.rs` (8KB) | YIN algorithm + cpal 48kHz, verified detecting real audio | Audio input pipeline for FACES voice prosody detection |
| `hand_tracking.rs` (6KB) | OpenXR hand tracking, fingertip → position mapping | Hand tracking for spatial interaction with Trinity output |
| `spatial_audio.rs` (2KB) | SpatialListener at camera, 3D positioned audio | FACES emotional feedback as spatial audio |
| `environment_manager.rs` (9KB) | Zen Garden / Studio / Stage scenes | Set and setting — FACES drives environment selection |
| `modes.rs` (2KB) | State machine with scene switching | LitRPG quest phase switching |
| `xr_shell.rs` (3KB) | XR environment setup, floor, lighting, tonemapping | Base XR shell for Trinity spatial output |
| `bin/desktop.rs` | Desktop emulator (test without XR hardware) | Demo development without dev kit |
| `bin/xr.rs` | Native OpenXR entry, ALPHA_BLEND for optical see-through | Production XR entry point |

**Gated behind `extras` feature (code exists, needs Bevy 0.18 ParamSet fixes):**
- `system_menu.rs` (9KB) — Menu system → LitRPG quest selector
- `holographic_ui.rs` (6KB) — Note display panel → FACES state display in XR
- `spatial_ui.rs` (6KB) — Render-to-texture 3D panels → Trinity content panels in space
- `widgets.rs` (4KB) — Glass panels + holographic buttons → Trinity XR UI components
- `interaction.rs` (2KB) — Laser pointer + hit cursor → Spatial interaction with Trinity
- `sensor_fusion.rs` (4KB) — Hand velocity + evaluation → Somatic input for FACES
- `truebadour_ai.rs` (2KB) — AI response + context window → Socratic questioning in XR
- `audio_transducer.rs` (2KB) — Note → audio playback → FACES audio feedback
- `ipc.rs` (4KB) — WebSocket IPC server → MCP transport to Strix Halo base station

#### Kotlin + Jetpack XR SDK (`apps/xr-prototype/android-xr/`)

Native Android XR app using Google's official SDK:

| File | Purpose | Trinity Integration |
|------|---------|---------------------|
| `MainActivity.kt` | Session.create(), hand tracking permissions, Oboe audio | Android XR entry point for Trinity client |
| `HandTrackingManager.kt` | ARCore hand joints → position mapping | Spatial input for Trinity XR |
| `VoixViveXrApp.kt` | Compose spatial UI | Trinity UI in XR space |
| `PitchDetectionEngine.kt` | Oboe 48kHz + YIN | Audio input for FACES on mobile NPU |
| `XrFretboardRenderer.kt` | OpenGL ES 3.0 renderer | Spatial rendering scaffold |

#### WebXR Prototype (`apps/xr-prototype/`)

Three.js + WebXR API, works in Chrome, no build required. Useful for rapid prototyping and web-based demos.

#### Companion App (`apps/companion-app/`)

React PWA deployed to Cloudflare Pages (`voix-vive.pages.dev`). 224 tests passing. Features:
- Two-tier hands-free voice system (keyword + AI intent interpretation)
- Google OAuth → Gemini (zero API cost, student's own quota)
- 700/700 EN/FR i18n key parity
- `[TOOL:XXX]` tag system for AI-driven UI control

### The FACES Gap in XR

The XR maturation map explicitly states: **"No face tracking — Aura has no face tracking. Can't detect winces, concentration, jaw tension."**

This is exactly the gap FACES fills:
- **On Strix Halo NPU:** FACES-Embed analyzes the LLM's output and determines the AI's emotional state. This state is sent to the XR client and rendered as the AI companion's expression.
- **On mobile NPU (future):** FACES-Embed could detect the user's emotional state from voice prosody (since face tracking isn't available on Aura).
- **In XR space:** FACES state drives the environment — tense work = Studio, creative flow = Stage, contemplation = Zen Garden. Set and setting, driven by emotion.

### Roadmap: Terminal Demo → Spatial Product

| Phase | Timeline | What Ships | Hardware |
|-------|----------|-----------|----------|
| **Phase 1: Terminal Demo** | Weeks | FACES on NPU + LLM on GPU, split terminal, 2-min video | Strix Halo only |
| **Phase 2: AMD Pitch** | Days after Phase 1 | One-page pitch + video, send to AMD dev relations | Strix Halo only |
| **Phase 3: XR Prototype** | 1-2 months | Trinity output rendered in Bevy desktop emulator with FACES-driven environment | Strix Halo + desktop |
| **Phase 4: Android XR Client** | 2-3 months | Trinity MCP server on Strix Halo, Bevy XR client on Android XR, FACES state streamed between them | Strix Halo + Android XR device |
| **Phase 5: Spatial Product** | 3-6 months | Full imagination amplification in XR space. User talks, AI questions (Socratic), FACES sets the mood, LitRPG frames the quest, output renders in space. | Strix Halo + XREAL Aura |

### Note on BE/DO/PLAY

Bertrand's BE/DO/PLAY pedagogy is his method, developed for guitar education. Trinity does not adopt it. Trinity's own triple reflection (FACES + LitRPG + Socratic) serves the same structural purpose but is original to Trinity and generalizes beyond music education.

---

## 13. DETAILED SESSION REVIEW (CHUNKED)

Each chunk represents a distinct conversation turn with what was discussed, what was decided, and the rationale.

---

### CHUNK 1: Initial Assessment Request

**User asked:** "and there is AI stuff here: /home/joshua/Workflow/desktop_trinity/trinity-genesis"

**What happened:** Explored the genesis archive — the original Trinity codebase from late 2025. Found 9 crates (trinity-brain, trinity-body, trinity-kernel, trinity-protocol, trinity-skills, trinity-coder, trinity-cli, trinity-client, iron-road-physics). Read README, Fancy Bible V3, Cargo.toml, all roadmap docs, critique docs, VAAM blueprint, research analysis, session turnover, usability audit, overnight task, ADDIE analyses, filtered TODOs.

**Key discoveries:**
- The genesis archive was the **original AI-facing vision** — designed for autonomy, self-coding, autopoiesis
- `docs/CRITIQUE_AND_ROADMAP.md:64-66` explicitly stated: "The project is finished when Trinity can improve its own educational curriculum"
- `docs/ROADMAP.md:37` stated: "The system should increasingly write its own code and content"
- `docs/RESEARCH_ANALYSIS.md:73-75` stated: "This architecture does not merely automate a task; it simulates a creator. Trinity is a persistent entity that lives on this machine"
- The usability audit (`docs/USABILITY_TODO.md`) showed 80% architecture / 20% production readiness
- The overnight task doc showed Trinity was designed to autonomously document its own codebase overnight
- The `todo_parser.rs` module was explicitly described as "Trinity reads its own conscience"

**Decision:** Recognized that the genesis archive proved the AI-as-audience concept was the original vision, not a new idea. The human-facing wrapper was added for Purdue, not because the system needed it.

**Rationale:** The genesis archive is the "true genesis" (as the Fancy Bible V3 states). The current TRINITYIDAIOS repo inherited the AI-facing architecture but buried it under human-facing layers. The pivot isn't inventing something new — it's returning to the original design.

---

### CHUNK 2: The AMD Pivot Proposal

**User asked:** "what if I made the target audience AMD? ... focus on connecting to and improving their products for strix halo, combining them into an instructional design suite that I can give them for a free laptop"

**What happened:** Read the Strix Halo whitepaper (`~/Downloads/Optimizing GMKtech Evo X2 AI Capabilities.md`). This 156-line document was a comprehensive technical whitepaper covering kernel parameters, ROCm optimization, TTM memory math, compiler intrinsics, benchmarks, and — critically — **named Trinity ID AI OS as the culminating architecture** (lines 131-152).

**Key discoveries from the document:**
- Trinity was already positioned as the "ultimate culmination" of all Strix Halo hardware optimizations
- The "Crate Economy" (77.5GB host / 50GB dynamic sandbox) was already described
- Autopoietic nightly fine-tuning with QLoRA MXFP4 was already specified
- The "No Python Snakes in the Grass" Rust-first constitution was already articulated
- The document concluded: "Trinity OS proves that when these cutting-edge hardware parameters... are meticulously aligned, the GMKtec Evo X2 transcends standard edge computation. It evolves into a continuous, self-optimizing, autopoietic intelligent system."

**Decision:** Pivot target audience from Purdue academics to AMD product/dev relations team. Trinity becomes a Strix Halo reference implementation, not a curriculum tool.

**Rationale:**
- AMD needs killer apps that sell hardware, not curriculum tools
- AMD's decision cycle is weeks, not 6-12 months
- AMD understands both hardware and software ecosystems
- AMD can offer hardware (Evo X2), engineering support, co-marketing
- The entire Strix Halo product line needs software ecosystem support
- Joshua had already written the pitch without realizing it was a product strategy
- NVIDIA DGX Spark is the competitor; Strix Halo needs differentiators

**The reframe:** "Trinity is not an instructional design tool. Trinity is an AI agent operating system that happens to specialize in instructional design. The ADDIECRAPEYE framework becomes the domain expertise that makes Trinity's agents useful, not the product itself. The product is the runtime. The domain is the differentiator."

---

### CHUNK 3: What to Keep, Cut, Build

**User asked:** "how would we need to change our system, and what would be bloat"

**What happened:** Systematically evaluated every component against the AMD target. Categorized into Keep (core value that AMD needs), Cut (human-facing overhead), Build (what AMD specifically requires).

**Decision framework:**
- **Keep test:** Does this component showcase Strix Halo capabilities or serve AI agents? → Keep
- **Cut test:** Does this component exist only for human interaction or academic credibility? → Cut
- **Build test:** Does AMD specifically need this to see hardware value? → Build

**Keep decisions (11 components):**
1. ADDIECRAPEYE — already AI instructions, not human UI
2. PEARL — agent task alignment contract
3. MCP server — the delivery mechanism for AI agents
4. Autopoietic engine — self-improvement loop, AMD differentiator
5. Pedagogical schema — AI-consumable datasets
6. Cross-dataset search — AI query routing
7. Quality Scorecard — AI self-evaluation
8. EYE export — structured artifact output
9. Inference router — multi-backend AI serving
10. Voice/emotion system — upgrade target for FACES
11. NPU detection — already built, already checks `/dev/xdna`

**Cut decisions (7 components):**
1. React frontend — terminal demo is more impressive for AMD
2. LitRPG narrative — game framing is human overhead
3. Sacred Circuitry — cosmology doesn't sell hardware
4. Four Horses of Awareness — no code, no runtime behavior
5. Cow Catcher/Hook Book/Bible Car headers — file noise
6. Caddy/Cloudflare tunnel — not relevant for local runtime
7. Daydream Bevy 3D — unless it showcases unified memory for spatial AI

**Build decisions (8 items):**
1. FACES integration into trinity-protocol
2. NPU inference path via AMDXDNA driver
3. LLM-to-FACES bridge (output to 4-byte state)
4. Split-terminal demo (LLM left, FACES right)
5. Lemonade SDK integration
6. ROCm-aware crate swapping (77.5GB/50GB)
7. Nightly QLoRA pipeline (MXFP4)
8. Benchmark script (Evo X2 + Trinity vs DGX Spark)

**Rationale for cuts:** Every cut component was evaluated against the question "would an AMD product manager care about this?" If the answer was no, it was cut. The cuts are not permanent — they're scope discipline for the demo. The LitRPG layer and Daydream 3D could return if they showcase spatial AI on Strix Halo.

---

### CHUNK 4: Using Pedagogy on the Pivot

**User asked:** "how do we use our pedagogy to help us here, as we rebuild and pivot our audience?"

**What happened:** Applied ADDIECRAPEYE to the pivot itself. This is meta-pedagogy — using the ID framework to design the pivot strategy.

**Decision:** Run ADDIECRAPEYE on the pivot as if the pivot were a curriculum project.

**The self-application:**
- **Analysis:** SME is Joshua. Gap is code-but-no-demo. Audience is AMD PMs.
- **Design:** One objective — "AMD engineer sees FACES on NPU + LLM on GPU in 2 minutes." Medium is terminal demo.
- **Development:** Port FACES, wire NPU, build split-terminal.
- **Implementation:** Run on desktop, record video, verify no GPU contention.
- **Evaluation:** Does hardware look good? Does NPU show value? Ship it.
- **Contrast:** Every Strix Halo review ignores NPU. We make it the star.
- **Repetition:** FACES updates every token batch — consistent, visible.
- **Alignment:** Demo scope = NPU+GPU parallel. Nothing else. No creep.
- **Proximity:** Two panes, one screen. GPU left, NPU right.
- **Envision:** "AMD sees their hardware do something no other software does — emotive AI on NPU alongside LLM on GPU."
- **Yoke:** FACES → Trinity kernel → Strix Halo. One pipeline.
- **Evolve:** Ship video. Send pitch. Get the laptop.

**Rationale:** Using your own pedagogy on yourself proves the system works. If ADDIECRAPEYE can guide a pivot strategy, it can guide AI agent behavior. This is the eat-your-own-dog-food principle applied to instructional design.

---

### CHUNK 5: FACES Integration

**User asked:** "we need to include FACES with trinity AMD pivot, use our FACES emotive AI system as the NPU, with GPU main system running LLM"

**What happened:** Searched Downloads for FACES documents. Found 12+ FACES-related files. Extracted text from .docx files using Python zipfile/XML parsing. Read the FACES Master Specification, Engine Codebase, Rust implementation, Project Completion Strategy, and Catalyst Application.

**Key discoveries:**
- FACES is a 4-byte (32-bit) protocol: Aura, Container, Focus, Action
- 38,400 distinct coordinate states from 4 bytes
- Rust implementation already exists (`faces_engine.rs` with `FacesState` struct)
- FACES-Embed: ~66M param DistilBERT ONNX model designed for NPU
- Theoretical foundations: Pareidolia, Mehrabian 7-38-55, Brene Brown, Alan Watts, Ekman FACS, Mian Xiang
- Committee Mapping: Aura=Heart=Mastery, Container=Mind=Autonomy, Focus=Body=Acquisition, Action=Will=Vocabulary
- Trinity already has `VoiceEmotion` (6 states), `EmotionData` (5 floats), `check_npu_availability()`, `npu_available` field

**The insight:** Every Strix Halo reviewer says NPU competes with GPU for memory. But FACES is 4 bytes. FACES-Embed is ~66M params (DistilBERT). That fits the NPU's compute budget without touching GPU bandwidth. Run them in parallel:

```
GPU (40 CUs)     →  LLM token generation (full speed, no contention)
NPU (50 TOPS)    →  FACES emotive detection + expression rendering
CPU (16 cores)   →  Rust orchestration (Trinity kernel)
```

**No other consumer hardware can do this.** NVIDIA DGX Spark has no NPU. Apple M4 Max has no ROCm.

**Decision:** FACES becomes the NPU workload. LLM stays on GPU. This is the demo that sells Strix Halo.

**Rationale:**
- Solves the "NPU is useless" problem that every reviewer identifies
- FACES is lightweight enough (4 bytes, ~66M model) to not impact GPU bandwidth
- The parallel compute story is unique to Strix Halo's heterogeneous architecture
- FACES already has a Rust implementation ready to integrate
- Trinity already has NPU detection and emotion types — the plumbing exists
- The demo is visually compelling (ASCII face morphing in real-time)

---

### CHUNK 6: Master Document Request

**User asked:** "can we summarize everything and make a master pivot doc? I like long and detailed documents, ones that I can read and remember everything I forget."

**What happened:** Created this document. Initially timed out due to size, so chunked into multiple writes/edits.

**Decision:** Create a single comprehensive document that serves as the source of truth for the pivot.

**Rationale:** Joshua explicitly likes long detailed documents for when he forgets context. This document is designed to be re-read after context loss and bring the reader back to full understanding.

---

### CHUNK 7: Chunked Review + Appendices Request

**User asked:** "can we please chunk up the session review a bit more, and have some appendages or something for extra detail on decision making processes, this allows for better vision management agenticly."

**What happened:** This current expansion. Breaking the session into granular chunks (above) and adding detailed appendices (below) for every major decision point.

**Rationale:** For agentic vision management, an AI agent (or Joshua after forgetting) needs to understand not just WHAT was decided but WHY. The appendices provide the reasoning trail that allows future sessions to either continue the strategy or intelligently override it with new information.

---

## APPENDIX A: DECISION MATRIX

Every major decision from the session, with options considered, choice made, and rationale.

### A.1: Target Audience

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| Purdue academics | Altruistic, existing relationship, Dr. Wanju interested | 6-12mo cycles, can't use the tool, sent cosmology not tool, no hardware | **Rejected as primary** |
| Homeschool parents | 4.3M students, direct purchase, motivated, underserved | Same delivery problem, needs UI, lower technical bar but still non-zero | Secondary market |
| AI agents (MCP) | Solves delivery, novel contribution, aligns with market | No hardware partner, commercially ungrounded alone | **Adopted as mechanism** |
| AMD product team | Fast decisions, understands hardware, needs killer apps, can offer hardware | Requires demo not code, higher bar for polish | **Selected as primary** |

**Final strategy:** AMD as primary target, AI agents as the delivery mechanism, Purdue as warm contact (send video when ready), homeschool as future market.

### A.2: Product Identity

| Option | Description | Verdict |
|--------|-------------|---------|
| ID tool with AI backend | Current framing — Pete teaches teachers | **Rejected** |
| AI agent runtime with ID vertical | Pivot framing — runtime is product, ID is differentiator | **Selected** |
| Pure AI agent OS (no ID) | Drop ID entirely | **Rejected** — ID is the domain expertise that makes agents useful |

**Rationale:** The ID domain is what makes Trinity's agents different from generic agent frameworks. Without it, Trinity is just another MCP server. With it, Trinity is the only agent runtime that knows how to design instruction.

### A.3: Demo Medium

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| React web UI | Polished, interactive | AMD doesn't need web UI, too much overhead to build | **Rejected** |
| Bevy 3D (Daydream) | Visually impressive | Too much scope, doesn't showcase NPU specifically | **Rejected** |
| Terminal (split pane) | Minimal scope, shows hardware metrics, FACES renders in ASCII natively | Less visually polished | **Selected** |
| Video only | No reproducibility | AMD can't verify claims | **Rejected** |

**Rationale:** Terminal demo is the minimal viable demo. FACES renders in ASCII — terminal is its native medium. Split-pane shows GPU and NPU working simultaneously. One command to reproduce. AMD engineers respect terminal demos more than web UIs.

### A.4: NPU Workload

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| Whisper STT on NPU | Useful, already referenced in voice.rs | Doesn't showcase emotive AI, not visually compelling | Future |
| FACES-Embed on NPU | ~66M fits NPU budget, 4-byte output is visual, unique differentiator | Needs ONNX model training/export | **Selected** |
| Background RAG on NPU | Useful for 24/7 operation | Not visible in demo, not a differentiator | Future |
| Nothing on NPU | What every other reviewer does | Wastes hardware, no story | **Rejected** |

**Rationale:** FACES on NPU is the only option that simultaneously (a) uses the NPU, (b) doesn't compete with GPU bandwidth, (c) is visually compelling in a demo, and (d) no competitor can replicate.

### A.5: Framework Disposition

| Framework | Role | Keep/Cut | Rationale |
|-----------|------|----------|-----------|
| ADDIECRAPEYE (12 phases) | Cognitive scaffold | **Keep** | Core IP, already AI instructions, maps to Bloom's |
| PEARL | Task alignment | **Keep** | Clean abstraction, one per project, agent contract |
| Hero's Journey (12 chapters) | Narrative structure | **Cut for demo** | Human-facing narrative, not needed for AMD demo |
| Bloom's Taxonomy (6 levels) | Cognitive levels | **Keep** | Embedded in ADDIECRAPEYE, academic credibility |
| VAAM | Vocabulary economy | **Rework** | Keyword matching is broken; concept is sound for agent mastery tracking |
| Sacred Circuitry (15 circuits) | Cognitive scaffolding | **Cut** | 15≠12, orthogonal to spine, Bashar sourcing risk, doesn't sell hardware |
| Cognitive Thermodynamics | Physics metaphor | **Cut** | Invented physics, not real pedagogy, doesn't map to anything |
| P-ART-Y (5 roles) | Infrastructure roles | **Cut from product** | Infrastructure documentation, not user-facing |
| Four Horses of Awareness | Doc metaphor | **Cut** | No code, no runtime, pure documentation overhead |
| Cow Catcher | Telemetry | **Cut from headers** | Keep the mechanism if useful, remove from file headers |
| Hook Book / Bible Car | Doc organization | **Cut** | No runtime behavior, adds cognitive overhead |
| Scope Creep Detector | Guardrail | **Rework for AI** | Keyword matching is broken; rework as agent context drift guard |

---

## APPENDIX B: CODE ASSESSMENT DETAILS

Specific findings from each file reviewed during the session.

### B.1: ADDIECRAPEYE Phase Prompts
**File:** `crates/trinity/src/conductor_leader.rs:70-179`

**Finding:** The `phase_system_prompt()` function is the crown jewel. Each phase has:
- Specific Bloom's Taxonomy cognitive level (Remember through Create)
- Concrete Socratic instructions (not vague)
- Explicit guardrails ("Do NOT suggest solutions. Do NOT build anything.")
- Clear advancement criterion

**Assessment:** This is real ID methodology encoded as LLM constraints. The Analysis prompt ("Ask 3 focused questions: WHO, WHAT, WHY. Do NOT suggest solutions") is better ID methodology than most LMS platforms attempt.

**Pivot impact:** These prompts are already AI instructions. They don't need to change. They need to be exposed via MCP so AI agents can consume them directly.

### B.2: Scope Creep Detector
**File:** `crates/trinity/src/scope_creep.rs:56-99`

**Finding:** Triggers on the word "also" and "add" in any message over 7 words. If a teacher says during Development, "I want to add a vocabulary section to my game" — that's the work, not scope creep.

**Assessment:** Produces false positives that interrupt legitimate pedagogical discussion. The phase-scaled penalties are a good idea, but detection needs semantic understanding, not keyword matching.

**Pivot impact:** Rework as agent context drift guard. Instead of keyword matching, use embedding similarity to detect when an AI agent's output has drifted from the PEARL-defined task scope.

### B.3: VAAM
**File:** `crates/trinity/src/vaam.rs`

**Finding:** Scans for vocabulary words and awards Coal. Proves the user *typed* a word, not that they *understand* it. A student could copy-paste "pedagogy" and earn Coal.

**Assessment:** Undermines credibility of the entire mastery tracking system. The concept (vocabulary as physical entities with mass/friction/steam) is sound, but the implementation is a keyword scanner dressed as a mechanism.

**Pivot impact:** Rework for AI agents. Instead of keyword matching, track concept mastery through agent output quality over time. An agent that correctly applies Bloom's verbs in generated content demonstrates mastery; one that misuses them doesn't.

### B.4: Quality Scorecard
**File:** `crates/trinity/src/quality_scorecard.rs`

**Finding:** Uses word patterns and structure detection. A document that mentions "Bloom's" six times scores well on Bloom's coverage even if it never actually aligns objectives to cognitive levels.

**Assessment:** Heuristic-based, gameable. Needs AI-evaluated semantic understanding.

**Pivot impact:** Keep for AI self-evaluation. An AI agent generating instructional content can use the scorecard as a self-check before emitting artifacts. The heuristics are a starting point; upgrade with LLM-based evaluation.

### B.5: PEARL
**File:** `crates/trinity-protocol/src/pearl.rs`

**Finding:** Subject + Medium + Vision. Clean abstraction. `PearlMedium` enum (Game, Storyboard, Simulation, LessonPlan, Assessment, Book, Other) with suggested tools per medium.

**Assessment:** Well-designed. One focus artifact per project, checked at every phase. This is the contract that prevents scope drift.

**Pivot impact:** Keep as-is. For AI agents, PEARL becomes the task alignment contract — the agent's "mission statement" that gets checked at every ADDIECRAPEYE phase transition.

### B.6: EYE Export
**File:** `crates/trinity/src/export.rs`, `crates/trinity/src/eye_container.rs`

**Finding:** Produces self-contained HTML5 files. Quiz export injects vocabulary and objectives into a working interactive page.

**Assessment:** Real product output, not a stub. This is what makes Trinity produce deliverables, not just chat.

**Pivot impact:** Keep. For AI agents, EYE export produces structured artifacts that other agents can consume. The HTML5 format is one output; add JSON and MCP-native formats.

### B.7: Autopoietic Engine
**File:** `crates/trinity-mcp-server/src/autopoietic.rs:1-323`

**Finding:** Complete self-modifying code engine. Stages mutations, validates syntax, compiles, backs up, promotes to live. Immutable file protection, failure circuit breakers (max 3 consecutive failures), version tracking with backup/restore.

**Assessment:** This is the differentiator. No other AI agent runtime can safely modify its own source code. The safety mechanisms (immutable files, circuit breakers, staging) are well-designed.

**Pivot impact:** Keep as core AMD differentiator. The autopoietic loop justifies 128GB unified memory — 24/7 self-improving agents that fine-tune their own models overnight.

### B.8: Self-Work MCP
**File:** `crates/trinity-mcp-server/src/self_work.rs:1-319`

**Finding:** Workflows for analyze_code, modify_code, test_code, search_docs, compile_check, update_status. These are AI-facing workflows — no human clicks these buttons.

**Assessment:** Already designed for AI consumption. This is the MCP interface that AI agents use to interact with Trinity.

**Pivot impact:** Keep. This is the delivery mechanism. Any AI agent that speaks MCP can connect to Trinity and use these workflows.

### B.9: Pedagogical Schema
**File:** `crates/trinity-mcp-server/src/pedagogical_schema.rs`

**Finding:** Edu-ConvoKit conversations table, Blooms concepts table, RICO screen patterns table. SQLite-based with indexes on conversation_id, speaker, intent, bloom_level.

**Assessment:** AI-consumable datasets, not human reading material. The cross-dataset search in `cross_dataset.rs` routes queries to the right dataset programmatically.

**Pivot impact:** Keep. These datasets give Trinity's agents domain knowledge that generic agent frameworks don't have.

### B.10: Voice/Emotion System
**File:** `crates/trinity/src/voice.rs`, `crates/trinity-protocol/src/character_sheet.rs:87-172`, `crates/trinity-protocol/src/types.rs:37-46`

**Finding:**
- `VoiceEmotion` enum: 6 states (Neutral, Warm, Urgent, Sarcastic, Celebratory, Contemplative)
- `detect_emotion()`: keyword-based detector (checks for "congratulations", "warning", "perhaps", etc.)
- `EmotionData` struct: 5 float dimensions (happiness, anger, sadness, fear, surprise)
- `check_npu_availability()`: checks `/dev/xdna`
- `npu_available` field in `VoiceStatus`
- Kokoro TTS, Whisper STT, Piper TTS pipelines
- "PersonaPlex ONNX types retained for future NPU path"

**Assessment:** The plumbing exists for NPU-aware emotion processing. The emotion detection is too simple (keyword matching) and needs upgrading to FACES 4-byte protocol. The NPU detection is already there.

**Pivot impact:** This is the integration point. Replace `VoiceEmotion` with `FacesState`. Replace `detect_emotion()` with FACES-Embed inference on NPU. The voice pipeline and NPU awareness are already built.

### B.11: Genesis Archive
**Path:** `/home/joshua/Workflow/desktop_trinity/trinity-genesis`

**Finding:** 9 crates, comprehensive docs, original AI-facing vision. Key documents:
- `docs/CRITIQUE_AND_ROADMAP.md` — "Finished when Trinity improves own curriculum"
- `docs/ROADMAP.md` — "System should write its own code"
- `docs/RESEARCH_ANALYSIS.md` — "Simulates a creator"
- `docs/VAAM_BLUEPRINT.md` — Cognitive Load as physics (Mass/Friction/Steam)
- `docs/phoenix_protocol.md` — Self-healing watchdog (trinity-monitor)
- `docs/ASK_PETE_ADDIE.md` — Maturity assessment of Educator skill (1/5)
- `docs/USABILITY_TODO.md` — 80% architecture, 20% production
- `docs/OVERNIGHT_TASK.md` — Autonomous overnight documentation
- `crates/trinity-kernel/src/todo_parser.rs` — "Trinity reads its own conscience"
- `crates/trinity-brain/ADDIE.md` — Brain maturity 4/5, "Socratic Mirror" architecture

**Assessment:** The genesis archive proves the AI-facing vision was original. The human-facing layers in TRINITYIDAIOS were added for Purdue, not because the architecture needed them.

**Pivot impact:** The genesis archive is the reference for what to return to. The autopoietic engine, self-work MCP, and TODO parser concepts all originated here.

### B.12: Strix Halo Whitepaper
**File:** `~/Downloads/Optimizing GMKtech Evo X2 AI Capabilities.md`

**Finding:** 156-line technical whitepaper. Covers:
- Hardware specs (Ryzen AI Max+ 395, 128GB, 256 GB/s, 59.39 TFLOPS)
- Kernel parameters (ttm.pages_limit, page_pool_size, vm_fragment_size, amd_iommu)
- ROCm optimization (TheROCk nightlies, AOTriton, hipBLASLt)
- Benchmarks (Vulkan vs ROCm across context depths)
- Lemonade SDK (AMD middleware, OpenAI-compatible + MCP)
- Kyuz0 containers (Fedora toolbx, --no-mmap, rocWMMA)
- ML stability (BF16 issues, FP32 bottleneck, 31% improvement via autoresearch)
- Spatial AI XR Stack (Godot, StereoKit, SpatialClaw, TRELLIS 2)
- **Trinity ID AI OS as culminating architecture** (lines 131-152)
- Crate Economy (77.5GB/50GB split)
- Autopoietic nightly QLoRA (MXFP4, dequantize=True, 32-38GB peak)

**Assessment:** Joshua already wrote the AMD pitch. The document positions Trinity as the killer app for Strix Halo. It just needed to be recognized as a product strategy.

**Pivot impact:** This document IS the pitch. The demo proves what the document claims.

---

## APPENDIX C: FACES PROTOCOL FULL SPECIFICATION

### C.1: The 4-Byte Payload

The FACES state is encapsulated in a 32-bit (4-byte) structure:

| Byte | Component | Functional Description | Range |
|------|-----------|----------------------|-------|
| 0 | Aura/Color | 8-bit ANSI/Hex color index representing general mood or biological state | 0-255 |
| 1 | Container/Shape | Defines physical boundary and temperament of the primitive | 5 values |
| 2 | Focus/Eyes | Defines attentional state and intensity | 6 values |
| 3 | Action/Mouth | Defines communicative intent or mechanical output state | 5 values |

**Total coordinate states:** 256 × 5 × 6 × 5 = 38,400

### C.2: Container Geometries (Byte 1)

| Value | Symbol | Element | Meaning |
|-------|--------|---------|---------|
| 0 | `()` | Metal | Organic parentheses — symmetry, focus, structured |
| 1 | `[]` | Earth | Rigid brackets — stability, assertiveness, boundaries |
| 2 | `{}` | Water | Fluid braces — adaptability, depth, vulnerability |
| 3 | `\|\|` | Wood | Walled pipes — growth, vision, shielding |
| 4 | `<>` | Fire | Sharp brackets — dynamic energy, passion, intensity |

### C.3: Focus States (Byte 2)

| Value | Symbol | FACS Mapping | Meaning |
|-------|--------|-------------|---------|
| 0 | `oo` | Neutral | Relaxed attention, baseline |
| 1 | `><` | AU4+AU7 | Intense, piercing, focused, defensive |
| 2 | `OO` | AU5 | Open, vulnerable, surprised, receptive |
| 3 | `..` | — | Distant, unfocused, disengaged |
| 4 | `^^` | — | Upward, hopeful, curious |
| 5 | `--` | — | Closed, tired, skeptical |

### C.4: Action States (Byte 3)

| Value | Symbol | FACS Mapping | Meaning |
|-------|--------|-------------|---------|
| 0 | `_` | — | Flat, neutral, waiting |
| 1 | `v` | AU15 | Downward, sad, disappointed |
| 2 | `~` | AU12 | Wavy, playful, wry, amused |
| 3 | `-` | — | Tight, thoughtful, controlled |
| 4 | `.` | — | Small, precise, minimal |

### C.5: The 5-Character Render

The FACES state renders as a 5-character ASCII string:

```
[container_left][focus_left][action][focus_right][container_right]
```

Examples:
- `{O~O}` — fluid, open, playful = happy, receptive, amused
- `[>-<]` — rigid, piercing, tight = angry, defensive, controlled
- `(o_o)` — organic, relaxed, flat = neutral, calm, waiting
- `|^.^|` — walled, hopeful, playful = guarded optimism
- `<O.v>` — sharp, open, sad = surprised disappointment

### C.6: Aura Color Mapping (Byte 0)

The Aura byte maps to ANSI 256-color or hex color values:

| Aura Range | Color Family | Emotional Association |
|------------|-------------|----------------------|
| 196-207 | Red | Anger, urgency, danger, passion |
| 208-219 | Orange | Warning, energy, attention |
| 220-231 | Yellow | Caution, alertness, curiosity |
| 232-255 | White/Bright | Neutral, clear, focused |
| 245-255 | Light gray | Calm, composed, professional |
| 33-51 | Blue | Sadness, calm, trust, depth |
| 52-69 | Green | Growth, balance, harmony |
| 90-105 | Magenta | Creativity, nonconformity |
| 129-145 | Purple | Wisdom, contemplation, mystery |

### C.7: Committee Mapping (4 Channels)

The FACES 4 bytes map to Joshua's "Committee" framework:

| Byte | FACES Component | Committee Channel | VAAM Mapping | Function |
|------|----------------|-------------------|-------------|----------|
| 0 | Aura | Heart | Mastery (M) | Emotional climate, vibe, feeling |
| 1 | Container | Mind | Autonomy (A) | Cognitive boundaries, schemas, rules |
| 2 | Focus | Body | Acquisition (Ac) | Sensory processing, attention, load |
| 3 | Action | Will | Vocabulary (V) | Kinetic expression, verbalization, output |

### C.8: Congruence and Incongruence

**Congruence:** Text sentiment matches FACES visual state. Positive text with playful face = sincere happiness.

**Incongruence:** Visual state intentionally diverges from text. High-energy text paired with "tired" visual = cognitive fatigue or sarcasm. This is the mechanism for detecting complex emotional states that simple text analysis cannot capture.

### C.9: FACES-Embed Model

- **Architecture:** Encoder-only (BERT-style)
- **Parameters:** ~66M (DistilBERT-base + 4 classification heads)
- **Input:** Text or audio features
- **Output:** 4-byte FACES state (Aura, Container, Focus, Action)
- **Deployment:** ONNX runtime, optimized for NPU (XDNA 2)
- **Latency:** Sub-millisecond on NPU
- **Context savings:** 1,500-2,500 tokens per multi-turn session (emotion encoded in 4 bytes instead of text descriptions)
- **Training data:** FACS-annotated facial expressions + text-emotion pairs

### C.10: Existing Rust Implementation

The `faces_engine.rs` (from Downloads) provides:

```rust
pub struct FacesState {
    pub aura: u8,
    pub container: u8,
    pub focus: u8,
    pub action: u8,
}
```

Methods: `new()`, `get_container()`, `get_focus()`, `get_action()`, `render()`, `to_hex()`. Uses `stty` for raw terminal input. Supports 3 input profiles (Arrows, WASD, Numpad) cycled with Tab.

**Integration plan:** Port `FacesState` into `trinity-protocol/src/faces.rs`. Add `From<VoiceEmotion> for FacesState` conversion. Add `detect_faces_state(text: &str) -> FacesState` to replace `detect_emotion()`. Add NPU inference path using FACES-Embed ONNX model.

---

## APPENDIX D: HARDWARE OPTIMIZATION DETAILS

### D.1: Memory Unlock Mathematics

The GMKtec Evo X2's BIOS restricts GPU memory. Linux kernel parameters override this:

```
Pages Limit = (120 × 1024 × 1024 × 1024) / 4096 = 31,457,280 pages
```

| Parameter | Value | Effect |
|-----------|-------|--------|
| `ttm.pages_limit` | 31457280 | Unlocks 120GB for GPU (8GB buffer for OS) |
| `ttm.page_pool_size` | 15728640 | Pre-allocates 60GB contiguous, reduces fragmentation |
| `amdgpu.vm_fragment_size` | 8 | 2M chunks for tensor mapping (default 4 = 64K) |
| `amd_iommu` | off | ~6% Vulkan speed increase, reduces latency |

### D.2: Inference Backend Selection

| Backend | Best For | Context Depth | Performance |
|---------|----------|---------------|-------------|
| Vulkan (RADV) | Short context, conversational | <8K tokens | 85 t/s generation |
| Vulkan (AMDVLK) | Short context, fallback | <8K tokens | 82 t/s generation |
| ROCm (hipBLASLt) | Long context, extreme depth | >100K tokens | 13 t/s at 130K context |
| ROCm (tuned) | Production, all-purpose | All | 51 t/s prompt at 130K |

**Rule:** Use Vulkan for demo (short context, fast TTFT). Use ROCm for autopoietic loop (long context, extreme depth).

### D.3: Critical Environment Variables

| Variable | Value | Effect |
|----------|-------|--------|
| `TORCH_ROCM_AOTRITON_ENABLE_EXPERIMENTAL` | 1 | 19x SDPA speedup (44ms → 2.3ms) |
| `ROCBLAS_USE_HIPBLASLT` | 1 | Resolves rocBLAS regressions |
| `PYTORCH_HIP_ALLOC_CONF` | (unset) | Prevents crashes on Strix Halo |
| `HSA_OVERRIDE_GFX_VERSION` | (not needed with TheROCk) | Legacy override, no longer required |

### D.4: llama.cpp Optimization

| Flag | Effect |
|------|--------|
| `--no-mmap` | Pins tensors to GTT, load: hours → 22 seconds |
| `-DGGML_HIP_ROCWMMA_FATTN=ON` | Exploits RDNA 3.5 WMMA intrinsics |
| `--ctx-size 0` | Maps model's native maximum context |

### D.5: Crate Economy Memory Budget

```
Total available (after kernel params):  ~124 GB
                                         ────────
Host OS + Rust runtime:                  ~17.5 GB
RAG vector databases:                    ~20 GB
Background orchestrators:                ~40 GB
                                         ────────
Host total:                              ~77.5 GB

Dynamic AI sandbox (crate swapping):     ~50 GB
                                         ────────
                                         ~124 GB total
```

The kernel evaluates each crate's payload, verifies it fits the 50GB budget, issues `cleanup()` to clear the current context, and hot-swaps new model weights via curl to the llama.cpp router API.

### D.6: Autopoietic QLoRA Pipeline

| Parameter | Value |
|-----------|-------|
| Model | GPT-OSS 20B (or similar) |
| Method | QLoRA |
| Precision | MXFP4 (Microscaling Formats) |
| Config | `Mxfp4Config(dequantize=True)` |
| Dequantization target | BF16 in GPU memory |
| Peak memory | 32-38 GB |
| Sandbox budget | 50 GB |
| Schedule | Nightly cron (idle period) |
| Data source | JSON ledgers of user interactions |

### D.7: Competitive Hardware Comparison

| Platform | Memory | Bandwidth | FP16 TFLOPS | NPU | Price |
|----------|--------|-----------|-------------|-----|-------|
| GMKtec Evo X2 (Strix Halo) | 128 GB unified | 256 GB/s | 59.39 | 50 TOPS (XDNA 2) | ~$1,800 |
| NVIDIA DGX Spark | 128 GB unified | 273 GB/s | 62.50 | None | ~$4,000 |
| Apple Mac Studio (M4 Max) | 128 GB unified | 546 GB/s | 34.08 | None (Neural Engine) | ~$2,000 |
| NVIDIA RTX PRO 6000 | 96 GB GDDR6 ECC | 1792 GB/s | 251.90 | None | ~$9,000 |

**Trinity's advantage:** Only Strix Halo has both NPU (for FACES) and ROCm (for GPU inference). DGX Spark has no NPU. Mac Studio has no ROCm. RTX PRO 6000 is enterprise-priced.

---

## APPENDIX E: RISK ANALYSIS

### E.1: Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| FACES-Embed ONNX fails on XDNA 2 | Medium | High | Fall back to heuristic FACES mapping (text → 4-byte). Still demonstrates NPU+GPU parallelism concept. |
| GPU token speed drops with NPU active | Low | Critical | Benchmark before/after. NPU has dedicated compute path; should not impact GPU. If it does, document the tradeoff honestly. |
| ROCm nightly instability on gfx1151 | Medium | Medium | Use Vulkan (RADV) for demo. ROCm for autopoietic loop only. Vulkan gives 85 t/s for short context. |
| `--no-mmap` doesn't work as described | Low | High | Already documented in whitepaper. Test on desktop first. |
| Lemonade SDK integration complexity | Medium | Low | Lemonade is OpenAI-compatible. Trinity's inference router already speaks OpenAI API. Should be a configuration change, not a rewrite. |

### E.2: Strategic Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| AMD doesn't respond to pitch | Medium | High | The demo video is still valuable. Post on YouTube, Reddit r/LocalLLaMA, Strix Halo Wiki. AMD dev relations monitors these channels. |
| AMD responds but wants different use case | Medium | Medium | The runtime is flexible. If they want a different vertical (not ID), the ADDIECRAPEYE scaffold can be swapped. The FACES+NPU parallelism is vertical-agnostic. |
| Competitor ships NPU+GPU demo first | Low | High | No known competitor is working on this. FACES is Joshua's IP. The 4-byte protocol is novel. |
| Purdue sees the pivot as abandonment | Low | Low | Send Dr. Wanju the demo video. The pivot makes Trinity better for everyone, including Purdue. If AMD partnership happens, Purdue gets a free reference implementation. |

### E.3: Personal Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| Desktop breaks (only remaining hardware) | Medium | Critical | Build demo on desktop, record video immediately. Video is the deliverable, not the live demo. |
| Scope creep during demo build | High | Medium | ADDIECRAPEYE Alignment phase: demo scope = NPU+GPU parallel. Nothing else. Use PEARL to check scope at every step. |
| Burnout from pivot fatigue | Medium | High | The pivot simplifies scope, not expands it. Cutting 7 components reduces work. Building 4 phases (5-7 days total) is achievable. |

---

## APPENDIX F: COMPETITIVE LANDSCAPE

### F.1: AI Agent Runtimes

| Framework | Language | MCP | NPU | ID Domain | Autopoietic |
|-----------|----------|-----|-----|-----------|-------------|
| **Trinity FACES** | Rust | Yes | Yes (XDNA 2) | Yes (ADDIECRAPEYE) | Yes |
| Rig | Rust | Yes | No | No | No |
| ADK-Rust | Rust | No | No | No | No |
| Crane | Rust | No | No | No | No |
| LangChain | Python | Yes | No | No | No |
| AutoGen | Python | No | No | No | No |
| CrewAI | Python | No | No | No | No |

**Trinity's unique combination:** Only runtime with NPU support, ID domain expertise, and autopoietic self-improvement. All in Rust.

### F.2: Strix Halo Software Ecosystem

| Software | Purpose | NPU Usage | Trinity Overlap |
|----------|---------|-----------|-----------------|
| Lemonade SDK | AI orchestration | No (routes to GPU) | Trinity integrates with Lemonade |
| Kyuz0 containers | Environment isolation | No | Trinity's "No Python" constitution aligns |
| llama.cpp | LLM inference | No | Trinity's inference router wraps this |
| vLLM | Batch inference | No | Trinity's inference router wraps this |
| ComfyUI | Image generation | No | Future crate in Crate Economy |
| **Trinity FACES** | **Agent runtime + emotive AI** | **Yes (FACES on NPU)** | **—** |

**Trinity is the only software in the Strix Halo ecosystem running independent workload types on NPU and GPU simultaneously** (emotive AI + LLM, not prefill/decode split of a single LLM).

### F.3: Emotive AI Protocols

| Protocol | Compute | Granularity | NPU-Ready | Open Source |
|----------|---------|-------------|-----------|-------------|
| **FACES** | 4 bytes | 38,400 states | Yes (~66M ONNX) | AGPLv3 |
| FACS (3D blendshapes) | High (GPU) | ~7,000 AUs | No | Proprietary |
| EmotionML (W3C) | XML tags | ~20 categories | N/A | Open standard |
| DeepFace (Python) | High (GPU) | 7 emotions | No | MIT |

**FACES advantages:** Lowest compute (4 bytes), highest granularity per byte, NPU-native, Rust implementation, theoretical grounding (Mehrabian, FACS, Mian Xiang).

---

## APPENDIX G: AGENTIC VISION MANAGEMENT NOTES

### G.1: Purpose of This Document for Agents

This document is designed to be consumed by AI agents (including future Cascade sessions) to restore full context after context loss. The structure supports:

1. **Quick orientation:** Read sections 1-6 for the what and why
2. **Decision archaeology:** Read Appendix A for why each choice was made
3. **Code navigation:** Read Appendix B for file-level findings
4. **Implementation planning:** Read sections 7-8 for what to build and how
5. **Protocol reference:** Read Appendix C for FACES specification
6. **Hardware reference:** Read Appendix D for Strix Halo optimization
7. **Risk awareness:** Read Appendix E for known risks and mitigations
8. **Market context:** Read Appendix F for competitive landscape

### G.2: Agent Instructions

When resuming work on the Trinity AMD pivot:

1. **Read this document first** — it is the source of truth for the pivot strategy
2. **Check the codebase inventory** (Section 11) before searching for files
3. **Follow the build phases** (Section 8) in order — do not skip ahead
4. **Use ADDIECRAPEYE** (Section 10) to structure your work
5. **Check the decision matrix** (Appendix A) before proposing changes to the strategy
6. **If you disagree with a decision:** Check the rationale in Appendix A first. If you have new information, update the decision matrix with your override and rationale.
7. **Do not re-introduce cut components** without explicit user approval. The cuts are scope discipline, not permanent judgments.
8. **The demo is the deliverable.** Code without demo = no AMD pitch. Prioritize building the demo over expanding the codebase.

### G.3: Context Restoration Protocol

If Joshua says "I forgot where we are" or similar:

1. Point him to this document
2. Summarize: "We're pivoting Trinity from Purdue to AMD. FACES on NPU + LLM on GPU is the demo. We're in [current build phase]."
3. Ask: "Do you want to continue building, or review the strategy?"

### G.4: Session Continuation Checklist

Before starting a new session on this pivot:

- [ ] This document exists and is current
- [ ] The build phase (Section 8) is known
- [ ] The FACES Rust implementation is accessible
- [ ] The Strix Halo whitepaper is accessible
- [ ] The desktop (development machine) is operational
- [ ] LM Studio or equivalent LLM backend is running
- [ ] `/dev/xdna` exists (if on Strix Halo hardware) OR NPU simulation is available

### G.5: Key Files to Monitor

| File | Why Monitor | Current State |
|------|-------------|---------------|
| `docs/active/MASTER_PIVOT_DOCUMENT.md` | This document | Source of truth |
| `crates/trinity-protocol/src/character_sheet.rs` | VoiceEmotion → FACES upgrade target | Has 6-state enum + keyword detector |
| `crates/trinity/src/voice.rs` | NPU detection + voice pipeline | Has `check_npu_availability()` |
| `crates/trinity-mcp-server/src/autopoietic.rs` | Self-improvement engine | Complete, 323 lines |
| `crates/trinity/src/conductor_leader.rs` | ADDIECRAPEYE prompts | Complete, crown jewel |
| `~/Downloads/Optimizing GMKtech Evo X2 AI Capabilities.md` | Strix Halo whitepaper / AMD pitch | Already written by Joshua |
| `~/Downloads/faces_engine.rs.docx` | FACES Rust implementation | Ready to port into trinity-protocol |

---

## APPENDIX H: FACES COMMERCIAL STRATEGY (FROM DOWNLOADS)

### H.1: AGPLv3 Dual Licensing

The FACES protocol uses AGPLv3 dual licensing ("Shelf" mechanism):
- **Open access:** GitHub under AGPLv3. Free for personal/open-source use.
- **SaaS trigger:** AGPLv3 closes the SaaS loophole. Any networked service using FACES must open-source their entire stack OR purchase a commercial license.
- **Sales pipeline:** Corporations integrating FACES into proprietary environments are legally compelled to buy a commercial license.

### H.2: Pricing Tiers (from FACES docs)

| Tier | License | Price | Deliverables |
|------|---------|-------|--------------|
| 1 | Indie/Solo | $199 one-time | 1 developer, 1 app, perpetual |
| 2 | SaaS/Commercial | $1,200/yr or $2,400 one-time | Full SaaS rights, enterprise support |
| 3 | Custom NPU Integration | $15,000 flat | Hardware-level NPU optimization, consulting |

**For AMD:** Tier 3 is the relevant tier. The AMD partnership would include custom NPU integration for XDNA 2, making Trinity FACES the reference implementation for NPU emotive AI on Strix Halo.

### H.3: IP Protection

- FACES protocol is Joshua's original IP
- The 4-byte standard, 5-character matrix, and theoretical mappings are novel
- AGPLv3 ensures derivative works remain open
- Commercial license revenue funds continued development
- The Rust implementation is zero-dependency, making it easy to audit and embed

---

## APPENDIX I: SECURITY CONSIDERATIONS

### I.1: Known Vulnerabilities (from prior session memory)

The current TRINITYIDAIOS codebase has known security issues if exposed publicly:

- **Unauthenticated tool execution:** `/api/tools/execute` is publicly accessible
- **Arbitrary URL injection:** `/api/models/switch` accepts arbitrary URLs via `InferenceRouter::set_active_url`, enabling prompt exfiltration to attacker-controlled backends
- **No tenant isolation:** Shared global `AppState` with single session, unsafe for multi-user

### I.2: Pivot Security Posture

The AMD pivot **improves** security because:
- The demo runs **locally** (no public exposure needed)
- MCP is the interface (not HTTP routes), and MCP has its own auth
- The terminal demo has no web attack surface
- The autopoietic engine's immutable file protection prevents unauthorized code modification

### I.3: Remaining Security Work

Before any public deployment (even after AMD partnership):
- Add authentication to all API routes
- Validate URLs in `InferenceRouter::set_active_url` (allowlist)
- Implement per-session `AppState` instead of global
- Add rate limiting
- Audit the autopoietic engine's file access scope

**For the AMD demo:** Local-only execution. No security work needed. The demo runs on one machine, accessed via terminal.

---

## APPENDIX J: NPU + GPU PARALLEL COMPUTE — COMPETITIVE REALITY (July 2026 RESEARCH)

### J.1: Why This Appendix Exists

The original master pivot document claimed "Trinity FACES is the only software demonstrating parallel NPU + GPU compute on Strix Halo." Web research on July 2, 2026 proved this **false**. Multiple open-source projects already demonstrate NPU + GPU parallel compute on Strix Halo. This appendix documents what exists, what Trinity's actual differentiator is, and how the pitch must be amended.

### J.2: Existing NPU + GPU Projects on Strix Halo

#### Project 1: halo (muchdevsuchcode)
**URL:** `https://muchdevsuchcode.github.io/halo/`
**GitHub:** (linked from site)

**What it does:**
- Proves zero-copy GPU↔NPU memory sharing on Strix Halo (verified byte-exact at 1/64/256 MiB)
- Wires NPU into llama.cpp as a real ggml backend (`-DGGML_NPU=ON`)
- NPU appears in `llama-server --list-devices` alongside iGPU
- Runs int8 transformer compute on XDNA2: GEMM → quantized Linear → FFN → attention → full transformer layer → KV-cached generation → speculative decode
- Architecture: prefill on NPU (dedicated matrix engine), decode on iGPU (wide memory + 64MB L3), sharing KV cache in unified memory with no copy
- Open source, MLIR-AIE toolchain, no CUDA, no closed kernels

**Honest assessment from author:** NPU runs 984/987 matmuls of Gemma 4 12B but end-to-end is ~0.31 tok/s prefill vs iGPU's 329 tok/s. "The gap is engineering, not feasibility." NPU is ~450× slower at raw matmul than iGPU. "The NPU's value is parallelism / freeing the GPU / power, not out-running the iGPU."

**Relevance to Trinity:** This project validates the NPU+GPU parallel compute concept. It also validates that the NPU should be used for complementary work, not competing with the GPU — exactly Trinity's thesis.

#### Project 2: strix-halo-pipeline (mikealanni)
**URL:** `https://github.com/mikealanni/strix-halo-pipeline`

**What it does:**
- "First-of-its-kind" NPU+GPU LLM inference pipeline on Linux
- FastFlowLM on NPU (1.7B model, ~750 tok/s prefill, ~34 tok/s decode)
- llama.cpp on GPU via Vulkan (8B model, ~41 tok/s)
- Pipeline: NPU streams tokens immediately (~700ms TTFT), GPU warms KV cache in background, then continues with larger model
- OpenAI-compatible API, binds to 0.0.0.0
- Modes: strix-speculative (NPU draft → GPU continue), npu-only, gpu-only, auto

**Honest assessment from author:** "For a single request, execution is overlapped but sequential — NPU generates first, GPU continues. True simultaneous same-token generation would require shared KV cache between FLM and llama.cpp, which doesn't exist." GPU cache warmup runs *during* NPU streaming via asyncio.

**Relevance to Trinity:** This is the closest competitor to what Trinity claims. It runs NPU and GPU simultaneously, but for the *same LLM task* (draft + verify). Trinity's approach is different: NPU does emotion detection (completely different workload), GPU does LLM inference (unrelated task).

#### Project 3: hal0 (bogdan-d)
**URL:** `https://github.com/bogdan-d/hal0`

**What it does:**
- Dashboard for managing Strix Halo AI workloads
- NPU multi-role via FLM trio: chat + transcription + embedding concurrently on one NPU hardware context (~2GB NPU memory)
- Three NPU slots: agent (chat), stt-npu (Whisper), embed-npu (embedding model)
- GPU slots for larger models via llama.cpp/Vulkan/ROCm
- Hardware-aware probe detects GPU/NPU/unified memory
- 128GB Ryzen AI Max+ 395 is "the reference deployment — not a hopeful port"

**Relevance to Trinity:** hal0 runs multiple *different* workload types on NPU (chat + STT + embedding), which is closer to Trinity's independent-workload concept. But hal0's NPU workloads are all LLM-adjacent (chat, transcription, embeddings). None are emotive AI. Trinity's FACES is a different workload class entirely.

#### Project 4: AMD Lemonade SDK (hybrid execution)
**URL:** `https://rocm.docs.amd.com/en/docs-7.2.0/how-to/system-optimization/strixhalo.html`

**What it does:**
- AMD's own middleware for Strix Halo
- "Hybrid execution" mode: NPU for prefill, GPU for decode
- Windows-only (DirectML + OGA)
- OpenAI-compatible API + MCP support
- Auto-profiles hardware

**Limitation:** Windows-only. Context length capped at 2,000-3,000 tokens for NPU models. Only supports ONNX quantization, not GGUF.

**Relevance to Trinity:** Lemonade is AMD's official tool. Trinity should integrate with it, not compete against it. Trinity's FACES pipeline can run alongside Lemonade's hybrid execution.

#### Project 5: sypherin/strix-halo-setup
**URL:** `https://github.com/sypherin/strix-halo-setup`

**What it does:**
- Comprehensive Strix Halo setup guide with systemd services
- FastFlowLM on NPU (port 52625) for small models (Qwen3.5:4b, Whisper)
- llama.cpp on GPU (Vulkan, port 8001) for large models
- ComfyUI on ROCm (port 7860) for image generation
- Notes FLM as "installed, NOT running" — setup is documented but not always active

**Relevance to Trinity:** Shows the community is already configuring NPU + GPU as separate services. Trinity's approach of running them as parallel pipelines within one process is more integrated.

### J.3: What Trinity FACES Actually Is (and Isn't)

| Claim | True? | Correction |
|-------|-------|------------|
| "Only software demonstrating parallel NPU + GPU compute on Strix Halo" | **FALSE** | At least 5 projects do this (halo, strix-halo-pipeline, hal0, Lemonade, sypherin setup) |
| "Only software running independent workload types on NPU + GPU" | **PARTIALLY TRUE** | hal0 runs chat+STT+embedding on NPU (different types), but all are LLM-adjacent. FACES (emotive AI) is a different workload class entirely. |
| "Only Rust-native NPU + GPU parallel runtime" | **TRUE** | All existing projects are Python or C++. Trinity is pure Rust. |
| "Only NPU workload for emotive AI detection" | **TRUE** | No existing project runs emotion detection on NPU. All use NPU for LLM prefill/decode or transcription. |
| "Only NPU + GPU parallel with autopoietic self-improvement" | **TRUE** | No existing project has self-modifying code or nightly QLoRA. |
| "Only NPU + GPU parallel with ID domain expertise" | **TRUE** | No existing project has ADDIECRAPEYE or pedagogical scaffolding. |
| "No other consumer hardware has both NPU and ROCm GPU" | **TRUE** | DGX Spark has no NPU. Mac Studio has no ROCm. This is a hardware fact, not a software claim. |

### J.4: The Corrected Pitch

**Old (false):** "Trinity FACES is the only software demonstrating parallel NPU + GPU compute on Strix Halo."

**New (accurate):** "Trinity FACES is the first software to run independent heterogeneous workloads — emotive AI on NPU, LLM on GPU — as parallel pipelines on Strix Halo. While projects like halo and strix-halo-pipeline split a single LLM task across both accelerators, Trinity runs a fundamentally different workload class on each: the NPU handles real-time emotional state detection via the FACES 4-byte protocol, while the GPU runs LLM inference at full speed. This is the only Rust-native runtime doing so, the only one with autopoietic self-improvement, and the only one with instructional design domain expertise."

### J.5: Strategic Implications

1. **The NPU+GPU parallel compute concept is proven.** Trinity doesn't need to prove it's possible — halo and strix-halo-pipeline already did. Trinity needs to prove it's *useful for a different purpose*.

2. **The competitive landscape is active.** This isn't a blue ocean. AMD's NPU software ecosystem is maturing rapidly in 2026. Trinity needs to move fast or find a different angle.

3. **Trinity's real moat is the combination, not any single feature:**
   - NPU + GPU parallel compute (not unique alone)
   - + Emotive AI workload on NPU (unique workload class)
   - + Rust-native (unique in NPU+GPU space)
   - + Autopoietic self-improvement (unique everywhere)
   - + ID domain expertise (unique everywhere)
   - + MCP integration (emerging standard)
   - + FACES 4-byte protocol IP (Joshua's original work)

4. **The pitch should acknowledge the ecosystem, not claim to be alone in it.** Saying "we're the only ones doing this" when GitHub proves otherwise destroys credibility with AMD engineers who know the landscape. Saying "we're doing something different with the same hardware" is more credible and more interesting.

5. **Potential collaboration, not competition.** The halo project's author explicitly says "the NPU's value is parallelism / freeing the GPU / power." Trinity FACES is the *embodiment* of that thesis. Trinity could cite halo as validation, not pretend it doesn't exist.

### J.6: Updated Competitive Landscape Table

| Project | NPU+GPU | Workload Type | Language | Autopoietic | ID Domain | Emotive AI |
|---------|---------|---------------|----------|-------------|-----------|------------|
| **Trinity FACES** | Yes | Independent (emotive + LLM) | Rust | Yes | Yes | Yes (FACES) |
| halo | Yes | Same-task (prefill+decode) | Python/C++ | No | No | No |
| strix-halo-pipeline | Yes | Same-task (draft+verify) | Python | No | No | No |
| hal0 | Yes | Multi-role (chat+STT+embed) | Python | No | No | No |
| Lemonade (hybrid) | Yes (Win only) | Same-task (prefill+decode) | Python | No | No | No |
| sypherin setup | Configured | Separate services | Python | No | No | No |
| FastFlowLM | NPU only | LLM only | C++ | No | No | No |
| llama.cpp | GPU only | LLM only | C++ | No | No | No |

**Trinity's unique combination:** Independent workload types + Rust-native + autopoietic + ID domain + emotive AI. No single competitor has more than one of these five attributes.

---

*"The goal is not to ship features, but to enable learning. The audience changed. The goal didn't."*

*"The project is finished when Trinity can improve its own educational curriculum." — Genesis archive, Dec 2025*

*"This architecture does not merely automate a task; it simulates a creator." — Genesis archive, Dec 2025*

*"AI can compute. Trinity is for the imagination. That is the best and most valuable tool a human has." — Joshua Atkinson, July 2026*

*"Trinity FACES runs independent heterogeneous workloads — emotive AI on NPU, LLM on GPU — as parallel pipelines on Strix Halo. Not the first to use both accelerators, but the first to run different workload classes on each." — This document, July 2026 (amended after competitive research)"*

---

## APPENDIX K: ANDROID XR ARCHITECTURE — XREAL AURA, JETPACK XR, ADK (July 2026 RESEARCH)

### K.1: Why This Appendix Exists

The original master pivot treated "Android XR" as a vague output target. Research on July 2, 2026 revealed specific hardware (XREAL Aura), specific frameworks (Jetpack XR SDK, SceneCore, Compose for XR), and a specific agent SDK (Google ADK for Kotlin/Android with Gemini Nano). This appendix documents what exists, what Trinity uses, and how the three-device architecture is implemented.

### K.2: Hardware — XREAL Aura

| Spec | Value |
|---|---|
| **Display** | Optical see-through (OST) — Sony Micro-OLED, 1920×1200 per eye |
| **FOV** | 70° (virtually borderless) |
| **Weight** | < 95g |
| **Hand tracking** | World-facing cameras ×2 (XR_EXT_hand_tracking) |
| **Spatial anchoring** | 6DoF tracking |
| **Platform** | Android XR + Snapdragon Reality Elite + X1S Coprocessor |
| **Input** | Hands (pinch gesture), voice, touchpad on compute puck |
| **Launch** | Fall 2026 |

**Why optical see-through matters for Trinity:** The user sees the real world directly through glass — zero passthrough latency. FACES panels overlay on the physical environment. This is ideal for the EYE phase: the user reviews AI-generated content in space while remaining grounded in their physical workspace.

### K.3: XR Development Stack — Kotlin-First

The XR client is a **separate Kotlin application** from the Rust base station. The Rust crate (`trinity-faces`) runs on Strix Halo; the Kotlin XR client runs on the XREAL Aura compute puck.

| Layer | Technology | Role in Trinity |
|---|---|---|
| **Jetpack XR SDK** | Android XR developer framework | Core XR session management |
| **Jetpack SceneCore** | 3D scene graph API | FACES entity placement in space |
| **Compose for XR** | Declarative spatial UI | FACES panel rendering (SpatialPanel, Orbiter) |
| **Material Design for XR** | Spatial Material components | UI components that adapt for XR |
| **ARCore for Jetpack XR** | Plane detection, hit-testing, spatial anchors | Anchor FACES panels to physical locations |
| **ADK for Android** | Agent Development Kit + Gemini Nano | On-device FACES detection fallback |

**Key Compose for XR components for FACES:**

| Composable | FACES Use |
|---|---|
| `SpatialPanel` | Render FACES ASCII face `(o_o)[><v]` in a 2D panel floating in 3D space |
| `Orbiter` | Contextual UI attached to panel — FACES state info, congruence, confidence |
| `SpatialGltfModel` | 3D `.glb` avatar model with expression matching FACES state |
| `SceneCoreEntity` | Place FACES entities at specific 3D coordinates |
| `SpatialRow/Column/Box` | Multi-panel FACES dashboard layout |
| `SpatialDialog` | Consent Gate UI in XR space |
| `SubspaceModifier` | Size, position, movable, resizable for FACES panels |

### K.4: Google ADK + Gemini Nano — Third Compute Tier

Google's ADK (Agent Development Kit) v0.1.0 for Kotlin/Android enables on-device AI agent orchestration with Gemini Nano (available on 140M+ Android devices, including Pixel 10 Pro XL).

**ADK opens a third compute tier in Trinity's architecture:**

```
Tier 1: Strix Halo (base station)
  GPU → LLM inference (7B-70B)
  NPU → FACES-Embed (~66M, INT8)
  CPU → Rust orchestration (trinity-faces crate)

Tier 2: XREAL Aura puck / Pixel 10 Pro XL (edge)
  Kotlin + ADK → lightweight agent orchestration
  Gemini Nano → on-device FACES detection (fallback when base station offline)
  Jetpack Compose for XR → FACES state rendering

Tier 3: Cloud (optional)
  Gemini Pro → complex reasoning, teacher model for FACES-Embed distillation
```

**Critical insight:** Gemini Nano on the XREAL puck or Pixel phone could run a lightweight FACES detection model *without the Strix Halo base station*. The glasses can function standalone for basic emotion detection, with the base station providing heavy LLM + FACES-Embed for full fidelity.

**ADK dependency:** `com.google.adk:google-adk-kotlin-core-android:0.1.0`

**On-device agent pattern:**
```kotlin
val onDeviceModel = GenaiPrompt.create(generativeModel, name = "gemini-nano")
val agent = LlmAgent(
    name = "faces_socratic_agent",
    model = onDeviceModel,
    instruction = Instruction("You are a Socratic questioning agent...")
)
```

**Hybrid orchestration:** Cloud Gemini as main orchestrator (on Strix Halo GPU), on-device Gemini Nano for privacy-sensitive sub-agents (on phone/glasses). ADK manages the orchestration, context handling, and error handling.

### K.5: Existing Reusable Assets (Bertrand-Masterclass)

The Bertrand-Masterclass workspace (`/home/joshua/Workflow/Bertrand-Masterclass`) contains a working Bevy 0.18 + OpenXR engine targeting XREAL Aura. Several files were originally ported FROM Trinity OS and evolved in the Voix Vive context. The following patterns and concepts are reusable for Trinity's XR client — **keeping Trinity's identity separate from Voix Vive**:

**Direct file reuse (Rust → Rust):**

| Bertrand File | Trinity Target | What to Extract |
|---|---|---|
| `AndroidManifest.xml` | Template for `apps/trinity-xr/` | XREAL Aura feature declarations (hand tracking, spatial anchoring, XR_ACTIVITY) |
| `ipc.rs` WebSocket pattern | Enhance `crates/trinity/src/` | WebSocket broadcast pattern for FACES state streaming (warp + crossbeam + tokio) |

**Concept transfer (Rust patterns → Kotlin):**

| Bertrand Concept | Kotlin Target | How It Maps |
|---|---|---|
| `spatial_ui.rs` (render-to-texture panels) | `SpatialPanel` composable | Bevy 3D panels → Compose for XR spatial panels |
| `holographic_ui.rs` (note/AI/dialogue panels) | FACES state panel + Socratic dialogue panel | Holographic panels → SpatialPanel + Orbiter |
| `environment_manager.rs` (scene state machine) | FACES environment modes in XR | Scene switching → Compose state-based environment switching |
| `hand_tracking.rs` (OpenXR hand tracking) | ARCore hand tracking + pinch | OpenXR hand joints → ARCore hit-testing + gesture detection |
| `system_menu.rs` (third-eye anchored menu) | Orbiter with FACES state selector | Third-eye menu → Orbiter attached to SpatialPanel |

**Not reused (Voix Vive specific):** `fretboard.rs`, `pitch_detection.rs`, `FallingNote.gd`, `Fretboard.gd`, `vertiscale_patterns.json`, BE/DO/PLAY mode system.

### K.6: Proposed Project Structure

```
TRINITYIDAIOS/
├── crates/                          (existing Rust crates — the engine)
│   ├── trinity/                     (Axum server, agent, inference)
│   ├── trinity-faces/               (FACES protocol — W1+W2 done)
│   ├── trinity-daydream/            (Bevy 3D/XR — desktop + OpenXR)
│   ├── trinity-protocol/            (shared types)
│   ├── trinity-voice/               (audio/TTS)
│   ├── trinity-mcp-server/          (MCP interface)
│   ├── trinity-quest/               (quest system)
│   └── trinity-iron-road/           (iron road mode)
│
├── apps/                            (NEW — Kotlin/Android clients)
│   ├── trinity-phone/               (Pixel 10 Pro XL — THE DIRECTOR)
│   │   ├── app/src/main/java/com/trinity/phone/
│   │   │   ├── SocraticSession.kt      (ADK agent, interviews Joshua)
│   │   │   ├── FacesDisplay.kt         (ASCII face on phone)
│   │   │   ├── PhaseTracker.kt         (ADDIECRAPEYE phase state)
│   │   │   ├── TrinityClient.kt        (WebSocket → Strix Halo)
│   │   │   └── QuestView.kt            (LitRPG quest framing)
│   │   └── build.gradle.kts
│   │
│   └── trinity-xr/                  (XREAL Aura — THE CANVAS)
│       ├── app/src/main/java/com/trinity/xr/
│       │   ├── FacesSpatialPanel.kt   (SpatialPanel with FACES face)
│       │   ├── FacesOrbiter.kt        (state info + congruence)
│       │   ├── FacesAvatar.kt         (3D glTF avatar)
│       │   ├── EyePhaseView.kt        (Envision/Yoke/Evolve UI)
│       │   ├── FacesAnchoring.kt      (ARCore spatial anchors)
│       │   ├── FacesClient.kt         (WebSocket → Strix Halo)
│       │   └── StandaloneFaces.kt     (ADK + Gemini Nano fallback)
│       ├── app/src/main/assets/models/
│       │   └── faces_avatar.glb
│       └── build.gradle.kts
│
├── docs/active/                     (existing — updated with XR research)
└── .windsurf/workflows/             (existing — W2-W10)
```

### K.7: HPT Framing — Trinity as Wrapper

```
┌─────────────────────────────────────────────────────┐
│ TRINITY — Human Performance Technology Wrapper      │
│                                                     │
│  ┌───────────┐   ┌───────────┐   ┌───────────┐     │
│  │  HUMAN    │   │    AI     │   │  OUTPUT   │     │
│  │           │   │           │   │           │     │
│  │ Imagination│  │ Compute   │   │ XR Space  │     │
│  │ Direction  │→ │ FACES     │→ │ EYE Phase │     │
│  │ Content    │  │ LLM       │   │ Review    │     │
│  │ Expertise  │  │ ADDIECRP  │   │ Sensory   │     │
│  │            │   │           │   │           │     │
│  │ PHONE      │   │ DESKTOP   │   │ XR        │     │
│  │ (Director) │   │ (Engine)  │   │ (Canvas)  │     │
│  └───────────┘   └───────────┘   └───────────┘     │
│                                                     │
│  Socratic questioning: Phone → Desktop              │
│  FACES states: Desktop → Phone + XR                 │
│  Content: Desktop → XR for EYE review               │
│  Human feedback: XR → Phone (next direction)        │
└─────────────────────────────────────────────────────┘
```

### K.8: Catalyst Program

Joshua has applied for the Android XR Developer Catalyst Program using voix_vive.com. Dev kit access would provide actual XREAL Aura hardware for testing, accelerating the XR client development from scaffold to working prototype.

### K.9: Development Testing on Pixel 10 Pro XL

The Pixel 10 Pro XL is available now for ADK + Gemini Nano testing. The phone app (THE DIRECTOR) can be developed and tested before XREAL dev kits ship:

1. **Phase 1 (now):** Build `trinity-phone` app with ADK + Gemini Nano — test Socratic questioning on device
2. **Phase 2 (when dev kit arrives):** Build `trinity-xr` app with Jetpack Compose for XR — test FACES rendering in space
3. **Phase 3 (integration):** Connect both to Strix Halo base station — full three-device pipeline

### K.10: Archive Strategy

After development is complete, the project will archive bloat (unused files, experimental code, stale configs). This is deferred until all workflows (W1-W10) and the three-device architecture are functional, as some assets may prove useful during development. The archive will be a cleanup pass, not a deletion pass.
