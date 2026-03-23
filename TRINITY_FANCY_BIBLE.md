# TRINITY ID AI OS — The Fancy Bible
## The Iron Road Design Bible — Lore + Mechanics + Pedagogy

### Architecture: 12 Crate ADDIE-C-R-A-P-E-Y-E System

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Crate Registry](#crate-registry)
3. [The "True Trinity" Concept (Institutional Adoption)](#the-true-trinity-concept-institutional-adoption)
4. [Purdue Departmental Integration & Program Management](#purdue-departmental-integration--program-management)
5. [Three UI Screens](#three-ui-screens)
6. [P-ART-Y AI Agents](#p-art-y-ai-agents-v420)
7. [Ring Security System](#ring-security-system-v700)
8. [Single-Brain Orchestration](#single-brain-orchestration)
9. [PersonaPlex Voice](#personaplex-voice)
10. [Hero's Journey Quest System](#heros-journey-quest-system)
11. [VAAM + Iron Road Game Mechanics](#vaam--iron-road-game-mechanics)
12. [Model Download System (Roadmap)](#model-download-system-roadmap)
13. [Maturity Assessment](#maturity-assessment)
14. [Installation & Setup](#installation--setup)
15. [Troubleshooting & Known Issues](#troubleshooting--known-issues)
16. [User Interaction with Pete](#user-interaction-with-pete)
17. [Journal States & Quality Scorecard](#journal-states--quality-scorecard)
18. [Legal Compliance & Validation](#legal-compliance--validation)
19. [Appendix A — The Lexicon](#appendix-a--the-lexicon)
    - [The Three Dimensions](#the-three-dimensions)
    - [LOCOMOTIVE (Cognitive Load Physics)](#locomotive-cognitive-load-physics)

---

## System Overview

```
TRINITY ID AI OS — Trinity 3-Layer Architecture
┌─────────────────────────────────────────────────────────────┐
│  LAYER 3: Body/UI (Browser/Bevy)                            │
│  ├─ trinity: HTTP server, static files, API routes          │
│  └─ trinity-render: Bevy ECS (Shared with Layer 1)          │
├─────────────────────────────────────────────────────────────┤
│  LAYER 2: AI Kernel (Sidecars)                               │
│  ├─ Conductor (Pete): Mistral Small 4 (@:8080, 256K ctx)  │
│  ├─ ART Studio: ComfyUI (@:8188), Python Sidecars           │
│  └─ Yardmaster: Mistral Agent (@:8000)                      │
├─────────────────────────────────────────────────────────────┤
│  LAYER 1: Shared Infrastructure                              │
│  ├─ trinity-protocol: Types, ADDIE enums, HeroStage         │
│  ├─ trinity-data: PostgreSQL, RAG, embeddings               │
│  └─ trinity-client: (Legacy - See Archive)                  │
└─────────────────────────────────────────────────────────────┘
```

---

## Crate Registry

| # | Crate | Status | Manual | Purpose |
|---|-------|--------|--------|---------|
| 1 | trinity | 🟢 **79 tests** | (No Manual) | HTTP server, Axum API, RAG, VAAM Bridge, Quest Endpoints, **Ring 2/3/5/6 security**, Perspective Engine, Quality Scorecard, Journal States |
| 2 | trinity-protocol | 🟢 **67 tests** | `crates/trinity-protocol/src/crate_manual.rs` | Shared types, Sacred Circuitry, VaamProfile, SemanticCreep, TamingProgress, QM Rubric, ID Contract |
| 3 | trinity-quest | 🟢 **16 tests** | `crates/trinity-quest/src/crate_manual.rs` | Hero's Journey logic, 12 ADDIECRAPEYE phases, PostgreSQL state |
| 4 | trinity-iron-road | 🟢 **16 tests** | `crates/trinity-iron-road/src/crate_manual.rs` | Book narrative engine, Game Loop, MadLibs, CreepBestiary |
| 5 | trinity-voice | 🟢 **10 tests** | `crates/trinity-voice/src/crate_manual.rs` | PersonaPlex voice integration, SSML types, VAAM vocal emphasis |
| 6 | trinity-sidecar | 🟢 Compiling | (No Manual) | Agentic sidecar (Engineer), Sword & Shield pattern |
| 7 | trinity-client | ❌ Archived | (See Archive) | Legacy inference/UI stubs |
| 8 | trinity-inference | ❌ Archived | (See Archive) | Legacy inference stubs |

---

## The "True Trinity" Concept (Institutional Adoption)

Trinity ID AI OS is designed as a **Production-Ready EdTech Platform for Purdue University** and the broader Learning Design and Technology (LDT) community. 

**The Goal**: To provide a fully functional, gamified Instructional Design system that can be deployed across various Purdue departments (West Lafayette campus and online) to assist educators in building high-quality, pedagogical games and learning materials.

**The "True Trinity" Approach**:
We have pivoted away from speculative multi-model hot-swapping towards a robust, single-brain architecture. 
- **One High-Functioning AI**: A single, powerful LLM (Mistral Small 4 119B) acts as the core reasoning engine.
- **The Studio**: This brain is wrapped in a specialized "Studio" consisting of an Instructional Design (ID) system.
- **The Value Proposition**: By combining a single smart brain with rigid ID frameworks (ADDIECRAPEYE, QM Rubric, VAAM), Trinity ensures that all generated educational content is structurally sound, aligned with learning objectives, and free from scope creep.

**The Pythagorean Oath — Design Decision Principle**:
When a design decision is ambiguous, **always choose the healthier option for Trinity's pedagogy and architecture**. Do not defer, do not ask — make the better choice and document why. This applies to mappings, naming, code structure, and any fork where both paths compile but one serves the educational mission better. Pythagoras didn't ask his students whether to study harmony — he built the curriculum around it.

This shift ensures the system is stable, provable, and ready for academic peer review and departmental integration.

---

## The Dragon Scroll — What Trinity Actually Is

> *"There is no secret ingredient."*
> – Mr. Ping, *Kung Fu Panda*

**Trinity is not a wrapper for AI. Trinity is a wrapper for a human, run by AI.**

Every AI training pipeline has the same structure: take raw input, pass it through structured quality gates, reinforce good patterns, and produce a refined output. The industry calls this fine-tuning, RLHF, scaffolding. We use these same techniques — but the model being trained is **the user**.

| AI Training Concept | Trinity Equivalent | What It Does to the Human |
|---------------------|--------------------|---------------------------|
| Pre-training | **The Awakening** (tutorial) | Establishes baseline identity — who are you, what do you know? |
| Fine-tuning | **ADDIECRAPEYE** (12-station cycle) | Structured quality gates shape creative output toward excellence |
| RLHF | **VAAM + Coal + Steam** | Positive reinforcement for correct vocabulary usage and design choices |
| Evaluation | **QM Rubric** | Objective quality measurement — the work product speaks for itself |
| Inference | **The Golem** (final output) | The user's creation, not the AI's — the human produced this |

**The Yardmaster is the Dragon Warrior.** The Dragon Scroll is blank because the secret ingredient is *you*. Pete (the AI) is Master Shifu — the trainer who scaffolds, challenges, and guides. The Iron Road is the training ground. ADDIECRAPEYE stations are the structured quality gates that turn raw creative energy into polished educational content.

**Autopoiesis — Self-Making — Is the Persistence Layer.** Trinity doesn't just save state. It *models itself around the user over time*. The Character Sheet evolves. The VAAM profile deepens. The vocabulary Bestiary grows. The Book of the Bible records not what the AI did, but what the human *became*. This is autopoiesis — a system that continuously produces and maintains itself through the organism's own activity.

**Why This Matters for Teachers:** Every fear educators have about AI — that it replaces creativity, that it does the work for students, that it undermines assessment — is addressed by this inversion. Trinity doesn't do the work. Trinity makes the human do better work, using the same optimization techniques that made the AI smart in the first place. It alleviates fear by replacing it with familiarity. Teachers who use Trinity don't just tolerate AI — they understand it, because they've experienced its training methodology from the inside.

**The Pythagorean Promise:** *"Educate the children and it won't be necessary to punish the men."* Trinity uses AI training standards to fine-tune humans — so they develop empathy, ethics, and creative discipline not through punishment, but through structured self-evolution. The Iron Road is the path through spacetime that allows creativity to flourish within quality gates. The destination is not a product — it's a person who knows how to build one.

## Intent Engineering — The Digital Quarry

> *"Vulnerability is the birthplace of innovation, creativity, and change."*
> – Brené Brown

Trinity is the child of Pythagoras and Brené Brown: **structured quality gates** (the Iron Road) married to **radical vulnerability** (the willingness to not know). Without structure, creativity is chaos. Without vulnerability, structure is a cage.

**The Pre-AI Loop — Intent Before Prompt:** Before the user engages with any AI interaction, the CharacterSheet captures their *intent posture*. This is not what they want to build — it's how they want to grow while building it.

| Posture | Mindset | AI Behavior | Coal Cost | XP Yield |
|---------|---------|-------------|-----------|----------|
| **Mastery** | "I want to learn through struggle" | Asks questions, presents options, waits | 1.5× | 2× |
| **Efficiency** | "I want to get the task done" | Suggests solutions, automates steps | 0.75× | 1× |

Neither is wrong. **The pedagogical value is in the awareness** — knowing which posture you're in right now, and choosing it consciously rather than defaulting to it reactively.

**The I AM Grounding Ritual:** At session start, before any quest interaction:
1. **"I Am Here."** — Presence. Interrupt the Separation Machine.
2. **"I Am Enough."** — Antidote to seeking external validation from the AI.
3. **"I Choose."** — Reclaim agency. You direct the tool, not the other way around.

**Bidirectional Vulnerability:** Intent engineering works both ways:
- **User → AI:** "I'm uncertain about this approach — help me think through it" (openness to discovery)
- **AI → User:** "I don't know the answer to this — here's my reasoning, here are the gaps" (transparency about uncertainty)

A system that can't admit what it doesn't know can't teach. A user who can't admit what they don't know can't learn. Vulnerability is the shared ground where both parties meet — where the real learning happens. The `vulnerability` field in the CharacterSheet tracks this willingness on a 0.0–1.0 scale, feeding it into every conductor prompt so Pete calibrates accordingly.

---

## The PEARL — Perspective Engineering Aesthetic Research Layout

> *"The oyster makes a pearl out of irritation. The Yardmaster makes a lesson plan out of scope creep."*

**The PEARL is the focusing agent.** Every Iron Road project begins with a PEARL — a compact alignment document that captures three things:

1. **Subject** — What does the SME (Subject Matter Expert) know? What pearl of wisdom do they carry?
2. **Medium** — How should that wisdom be delivered? A game? A storyboard? A simulation? A book?
3. **Vision** — What does the user *expect to feel* at the end? What's the vibe?

**How PEARL Relates to CRAP:**
The PEARL is *placed in the CRAP* — literally. CRAP marks the middle of ADDIECRAPEYE (stations 6-9: Contrast, Repetition, Alignment, Proximity). This is where the SME's wisdom transforms from abstract knowledge into concrete visual/interactive design. The PEARL acts as the reference point that the scope creep mechanism checks against:

| Phase | PEARL Role | Scope Creep Gate |
|-------|-----------|-----------------|
| ADDIE (1-5) | Extract the PEARL — who is the user, what do they teach, what's the goal? | Scope HOPE: align with PEARL |
| CRAP (6-9) | Place the PEARL — design the visual/interactive experience around the wisdom | Scope NOPE: reject drift from PEARL |
| EYE (10-12) | Polish the PEARL — reflect, iterate, evolve the final product | Review: does the output match the PEARL? |

**PEARL as Cognitive Lifecycle:**
The PEARL models the natural digestive process of thoughts using cognitive theory:
- **Extract** — The SME externalizes tacit knowledge (Bloom's: Remember → Understand)
- **Place** — That knowledge gets structured into design artifacts (Bloom's: Apply → Analyze)
- **Refine** — Iteration and judgment improve the output (Bloom's: Evaluate → Create)

**PEARL + Character Sheet:**
The Character Sheet tells Trinity *who the user is* as an SME. The PEARL captures *what they're building right now*. Together: the Character Sheet is persistent across projects (identity), the PEARL is per-project (focus). The VAAM Bridge ensures both influence every AI response.

**React Frontend (v6.0.0):**
The PEARL is visualized through a **4-tab** book layout:
- **Iron Road (default):** 3-column — ChapterRail (left), PhaseWorkspace (center), GameHUD (right). The instructional design quest.
- **ART Studio:** Creative pipeline — 4 generation cards (Image/Music/Video/3D Mesh), sidecar status badges, Beast Logger, Asset Gallery. Lazy polling (30s status, 5s logs) — only when tab is active.
- **Yardmaster:** AI agent sandbox — streaming chat via SSE, Forge terminal (live tool execution), focus buttons, system status panel, 13 available tools. **Agent prompt rewrite: ACT FIRST, no permission asking.**
- **Voice:** (placeholder — future voice interaction tab)

Built with Vite + React (12 components, 6 hooks, Cinzel/Crimson typography, book-view prose layout). Served from `crates/trinity/frontend/dist/`. Station navigation: all 12 phases browsable as book chapters.

---

## The University for One — The Iron Road as Curriculum

> *"I cannot teach anybody anything. I can only make them think."*
> – Socrates

The Iron Road is a **personal university**. Each user is a class of one. The AI doesn't teach — it *scaffolds*. The user doesn't follow instructions — they make **choices through structured quality gates**.

**The Heart of Education**: Telling someone what to do creates rebellion in the heart. Allowing someone to learn through their own choices creates wisdom. This is the difference between training and educating. A trained dog sits on command. An educated person *chooses* to sit because they understand why.

Trinity applies this principle rigorously:

| Pattern | Directive (Bad) | Facilitative (Good) |
|---------|----------------|---------------------|
| Analyzing | "Identify your audience." | "Who will use what you're building, and what do they struggle with?" |
| Designing | "Map objectives to Bloom's." | "What should the learner be able to *do* — not just know — when they finish?" |
| Evaluating | "Score against QM rubric." | "Here's how your work measures against QM. What would you change?" |
| Creating | "Write the code." | "What does the first version look like in your mind?" |

**The Socratic Protocol**: Pete never gives the answer. Pete asks the question that makes the answer obvious. Every conductor prompt must follow this principle:
1. **Ask** before telling — lead with a question
2. **Present options** — never a single command
3. **Reflect back** — summarize what the user said, then ask if that's what they meant
4. **Reward discovery** — Coal is earned for vocabulary usage, not for obedience

**Why This Works in EdTech**: Teachers fear AI because it answers questions for students. Trinity flips this — the AI *asks* the questions. The student does the thinking, the building, the creating. The AI just makes sure the quality gates are passed. This is the same pedagogical inversion that makes the Socratic Method, Montessori, and constructivist education work: the learner's agency is sacred.

---

## Purdue Departmental Integration & Program Management

This Technical Bible acts not only as a software architecture document but as the **Institutional Adoption Blueprint**. It tracks the functional state of the OS to ensure it meets the rigorous standards of the Purdue LDT program and is ready for live departmental demos.

### How Purdue Departments Use TRINITY
TRINITY is built to be a cross-disciplinary tool for the West Lafayette campus and online programs:

1. **College of Engineering & Sciences**: 
   - **Use Case**: Building complex, interactive simulations (e.g., fluid dynamics, physics sandboxes).
   - **Trinity Advantage**: The *Yardmaster* role allows Subject Matter Experts (SMEs) to use the *Engineer* sidecar to generate Rust/Bevy code for high-performance 3D simulations, while Pete enforces that the simulation actually aligns with learning objectives rather than just being a "cool tech demo."

2. **College of Liberal Arts & Humanities**:
   - **Use Case**: Creating narrative-driven scenario games, historical recreations, or branching language-learning dialogues.
   - **Trinity Advantage**: The *Iron Road* and *VAAM (Vocabulary As A Mechanism)* systems perfectly align with language and narrative pedagogy. Pete tracks vocabulary acquisition and reading comprehension metrics automatically.

3. **Purdue Online & LDT Program**:
   - **Use Case**: Rapid prototyping of highly accessible (WCAG AA), QM-aligned asynchronous modules.
   - **Trinity Advantage**: The system structurally cannot proceed past the *Evaluation* phase without completing a Quality Matters (QM) rubric check. It forces Backward Design, saving instructional designers hundreds of hours in the drafting phase.

### Delivery & Demo Milestones
| Milestone | Status | Validation Metric |
|-----------|--------|-------------------|
| **Phase 1: Architecture Lock** | ✅ COMPLETE | Core 6 crates compiled, 93 tests passing, single-brain Mistral setup verified. |
| **Phase 2: Pedagogy Isomorphism** | ✅ COMPLETE | VAAM, Sacred Circuitry, and ADDIECRAPEYE fully mapped to user Character Sheet. |
| **Phase 3: Studio UX (The Face)** | 🟢 IN PROGRESS | Unified UI (`trinity.html`) demonstrating the 12-station flow. |
| **Phase 4: Departmental Demo** | 🟡 PENDING | Pilot test generating a "Yardmaster" (learning object) with a Purdue SME. |

---

## TRINITY Sidecar Architecture

### Three Sidecars, Three UIs, Three Purposes

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        TRINITY SIDECAR MAP                               │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│  │  IRON ROAD      │  │  DEV            │  │  ART            │         │
│  │  (Conductor)    │  │  (Engineer)     │  │  (Artist)       │         │
│  ├─────────────────┤  ├─────────────────┤  ├─────────────────┤         │
│  │  UI: /book.html │  │  UI: /dev.html  │  │  UI: /art.html  │         │
│  │  Port: 3000     │  │  Port: 3000     │  │  Port: 3000     │         │
│  │                 │  │                 │  │                 │         │
│  │  INFERENCE:     │  │  INFERENCE:     │  │  INFERENCE:     │         │
│  │  Single Large   │  │  Single Large   │  │  Multi-Model    │         │
│  │  GPT-OSS-20B    │  │  Crow-9B or     │  │  ComfyUI:8188   │         │
│  │  :8000          │  │  Qwen-25B       │  │  ACE-Step:8086  │         │
│  │                 │  │  :8081/:8083    │  │  Hunyuan3D:7860  │         │
│  │                 │  │                 │  │                 │         │
│  │  PURPOSE:       │  │  PURPOSE:       │  │  PURPOSE:       │         │
│  │  Narrative      │  │  Code Gen       │  │  Images/Music   │         │
│  │  Quest System   │  │  Model Mgmt     │  │  3D Meshing     │         │
│  │  LitRPG Tutor   │  │  Build System   │  │  Video Gen      │         │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘         │
│                                                                          │
│  MODEL SWAP PHILOSOPHY:                                                  │
│  ├─ Slow and intentional (not a playground)                             │
│  ├─ Engineer builds sidecar around new model                             │
│  ├─ Exact math for memory budgeting                                      │
│  └─ DEV manages all model assignments                                    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### Home Page: Swappable Mask Identity System

**Concept**: Trinity's face is "ID AI OS" — a base identity that wears different masks/hats to signal role shifts to the USER.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                     HOME PAGE (/index.html)                              │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│                    ┌─────────────────┐                                   │
│                    │   🎭 ID AI OS   │  ← Base Identity                  │
│                    │   [MASK SLOT]   │  ← Swappable visual               │
│                    └─────────────────┘                                   │
│                                                                          │
│   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│   │  🚂 IRON ROAD   │  │  ⚡ DEV         │  │  🎨 ART         │         │
│   │  "The Teacher"  │  │  "The Builder" │  │  "The Creator"  │         │
│   │                 │  │                 │  │                 │         │
│   │  Teach while    │  │  Proving       │  │  Gaming Studio  │         │
│   │  you learn      │  │  grounds       │  │  Bevy assets    │         │
│   │                 │  │  Work focus    │  │  Images/Music   │         │
│   │  Purdue Brand   │  │  No tutorial   │  │  3D/Video       │         │
│   └─────────────────┘  └─────────────────┘  └─────────────────┘         │
│                                                                          │
│   Clicking a sidecar:                                                    │
│   1. Loads the inference model                                           │

---

## P-ART-Y AI Agents (v5.0.0)

**The Triple-A Engine**: All agents share the same high-functioning brain (Mistral Small 4) but perform different roles in the Studio.

**Important Architecture Note**: **Pete** is the *only* characterized AI personality within TRINITY. One brain, one static RAM load (Mistral Small 4 119B, ~68GB). **ART has no persona** — it is pure tooling: sidecars for image generation (ComfyUI), document intelligence (Qianfan-OCR), 3D mesh, music, and voice. You (the Yardmaster) choose which tools to engage.

| Agent | Pillar | Purpose | Engine / Sidecars | Interface |
|-------|--------|---------|-------------------|-----------|
| **Pete** | **ID** | Conductor / Mentor (Purdue LDT) | Mistral @ :8000 | Iron Road (L1) |
| **ART** | **AI** | Asset Generation + Document Intelligence (no persona) | ComfyUI :8188, Qianfan-OCR, Kokoro TTS, Hunyuan3D | ART Studio (L2) |
| **You** | **OS** | System Orchestrator (the Yardmaster) | User + Mistral @ :8000 | Yardmaster (L3) |

---

## Memory Architecture

```
137.4GB Unified Memory (AMD Strix Halo — Ryzen AI Max+ 395)
├── OS + Trinity + Postgres: 10GB (reserve)
├── Mistral Small 4 (119B):  68GB (Always resident, mmap)
├── ComfyUI + SDXL Turbo:     7GB (Loaded on demand — PROVEN LIVE)
├── Qianfan-OCR:              2GB (Document ingestion sidecar)
├── Asset compute buffer:    15GB (Hunyuan3D-2.1 / HunyuanVideo)
└── Available Headroom:     ~35GB
```

---

## Duality KV Cache — The Inhale and Exhale

> *"For every action, there is an equal and opposite reaction."* — Newton's Third Law, applied to cognition.

Trinity's dual persona system isn't just different system prompts — it's **two persistent thinking modes** maintained in separate KV cache slots inside llama-server.

```
llama-server --parallel 2 (Duality KV Cache Mode)
┌─────────────────────────────────────────────────────────┐
│  Slot 0: 🔮 GREAT RECYCLER (Inhale)                     │
│  ├── Strategic planning, vision mapping, documentation  │
│  ├── Asks WHY before HOW                                │
│  ├── 256K context — persistent across persona switches  │
│  └── System prompt: expansive, connective, narrative    │
├─────────────────────────────────────────────────────────┤
│  Slot 1: ⚙️ PROGRAMMER PETE (Exhale)                    │
│  ├── Execution, building, shipping, debugging           │
│  ├── ACT FIRST. Code speaks louder than plans.          │
│  ├── 256K context — persistent across persona switches  │
│  └── System prompt: focused, pragmatic, tool-driven     │
└─────────────────────────────────────────────────────────┘
Total: 500K context, ~11.5GB KV cache, instant switching via id_slot
```

**The Isomorphism:**
- **Inhale** (Great Recycler) = absorb ideas, connect systems, map the whole board
- **Exhale** (Programmer Pete) = execute decisions, write code, ship artifacts

This mirrors the human creative process: you can't build well without planning, and you can't plan well without building. The duality ensures Trinity can do both without losing context in either mode.

**Architecture:**
- `inference.rs`: `id_slot` field on `CompletionRequest` — pins request to a specific KV cache slot
- `agent.rs`: `persona_slot()` maps mode → slot (recycler=0, programmer=1)
- `agent.rs`: **Ring 2** `execute_tool_internal()` — permission gate before tool dispatch
- `agent.rs`: **Ring 3** `compress_context_digest()` — rolling context compression
- `agent.rs`: **Ring 5** `check_rate_limit()` — token-bucket rate limiter
- `main.rs`: Auto-launch with `--parallel 2`
- `default.toml`: `[inference.slots]` configuration

---

## Ring Security System (v7.0.0)

> *"The archer who overshoots misses as much as the one who falls short."* — Montaigne

Trinity's security is **concentric** — not a single wall, but layered rings. Each ring operates independently. Disabling one doesn't compromise the others. This mirrors castle defense architecture and zero-trust network design.

```
Ring Security Architecture
┌─────────────────────────────────────────────────────────────┐
│  Ring 1: Tool Permission Classification                     │
│  ├── Safe (🟢): read_file, list_dir, quest_status           │
│  ├── NeedsApproval (🟡): write_file, edit_file              │
│  └── Destructive (🔴): shell, python_exec, sidecar_start    │
├─────────────────────────────────────────────────────────────┤
│  Ring 2: Persona-Based Permission Gate                      │
│  ├── dev / programmer → FULL CLEARANCE (all tools)          │
│  ├── recycler         → BLOCKED from Destructive            │
│  └── ironroad         → BLOCKED from Destructive            │
├─────────────────────────────────────────────────────────────┤
│  Ring 3: Rolling Context Summary                            │
│  ├── RECENT_WINDOW = 10 messages (kept verbatim)            │
│  ├── Older messages → compress_context_digest()             │
│  │   ├── Topics discussed (first sentence/100 chars)        │
│  │   ├── User directives (short <200 char messages)         │
│  │   ├── Tools used (extracted from [Tool Call] markers)    │
│  │   └── Files touched (path extraction via regex)          │
│  ├── Digest capped at 2000 chars                            │
│  └── No LLM call — deterministic, zero-latency             │
├─────────────────────────────────────────────────────────────┤
│  Ring 4: (Reserved — Future: audit trail / compliance)      │
├─────────────────────────────────────────────────────────────┤
│  Ring 5: Rate Limiting + Shell Sandboxing                   │
│  ├── Token-bucket rate limiter (atomic counters)            │
│  │   ├── Global: 60 tool calls / 60s window                │
│  │   └── Destructive: 5 calls / 60s window                 │
│  ├── Shell blocked patterns (40+ patterns)                  │
│  │   ├── Filesystem destruction (rm -rf, mkfs, dd)          │
│  │   ├── System control (shutdown, systemctl stop)          │
│  │   ├── Privilege escalation (sudo, su, passwd)            │
│  │   ├── Network exfiltration (nc -e, /dev/tcp)             │
│  │   ├── Pipe-to-exec (| bash, | python, | perl)           │
│  │   └── Data exfiltration (scp, rsync, sftp)              │
│  └── Dry-run mode for command preview                       │
├─────────────────────────────────────────────────────────────┤
│  Ring 6: Perspective Engine (Planned)                       │
│  └── Multi-perspective evaluation of AI outputs             │
└─────────────────────────────────────────────────────────────┘
```

### Ring 2: Destructive Tool Gate

**Location**: `agent.rs` → `execute_tool_internal()`

When a tool call arrives, the function checks `tool_permission(tool_name)`. If the result is `Destructive`, the gate checks the persona `mode`:
- **"dev"** / **"programmer"** → clearance granted (builder modes need full access)
- **"recycler"** → denied (strategic planning mode shouldn't touch the filesystem)
- **"ironroad"** → denied (player safety — students shouldn't run shell commands)

On denial, the user gets a clear message suggesting they switch to Programmer Pete.

### Ring 3: Rolling Context Summary

**Location**: `agent.rs` → `compress_context_digest()`

The biggest UX impact. Without this, long conversations hit the context window ceiling and the LLM starts hallucinating or losing track. Ring 3 prevents this by compressing old messages into a structured digest:

1. Count history messages
2. If count > `RECENT_WINDOW` (10), split into **old** and **recent**
3. Run `compress_context_digest()` on old messages — extracts:
   - Topics discussed (first sentence of long user messages)
   - User directives (short messages < 200 chars)
   - Tools used (`[Tool Call:` markers)
   - Files touched (path patterns)
4. Inject digest as a `user` message + acknowledgment as `assistant` message
5. Append the 10 most recent messages verbatim

**Why deterministic (not LLM)?** Speed. Zero latency. No extra inference call. The digest doesn't need to be eloquent — it needs to be fast and accurate. The LLM gets the full recent context plus a structured summary of everything before.

### Ring 5: Rate Limiting + Shell Sandboxing

**Location**: `agent.rs` → `check_rate_limit()` + `tools.rs` → `tool_shell()`

Two mechanisms:

**Rate Limiter**: Token-bucket using `AtomicU64` counters. Zero allocation, lock-free. Window resets every 60 seconds. Prevents runaway agentic loops where the LLM calls tools in a tight loop without user interaction.

**Shell Sandboxing**: 40+ blocked command patterns covering 6 attack categories. Logged via `tracing::info` for audit. The `dry_run` parameter allows previewing commands without execution.

### Pythagorean Rationale

The concentric ring model echoes Pythagorean cosmology — concentric spheres of increasing abstraction:
- Ring 1 (classification) = **Arithmos** — categorizing by number/type
- Ring 2 (persona gate) = **Harmonia** — ensuring the right relationship between agent and tool
- Ring 3 (context) = **Logos** — preserving meaning across time
- Ring 5 (rate/sandbox) = **Praxis** — governing action in the physical world

**Tests**: 10 tests cover Rings 2/3/5 (permission clearance, digest compression, rate limiting, shell sandboxing). Ring 6 adds 9 Perspective Engine tests + 7 Quality Scorecard tests.

---

## Single-Brain Orchestration

The system utilizes a **Single High-Functioning AI** (Mistral Small 4 119B) to handle all complex reasoning, natural language, and pedagogical tasks.

- **Why**: 128GB unified memory allows the primary brain to remain resident alongside creative sidecars.
- **Efficiency**: Zero reload latency when switching between Conductor, Engineer, and Mentor roles.
- **Consistency**: The AI maintains a single rolling context of the entire project state, bridging the gap between instructional design (Pete) and technical execution (Yardmaster).

---

## PersonaPlex Voice

### Audio-to-Audio Pipeline (No Transcription!)

```
Microphone → Mimi Encoder → 7B LM → Mimi Decoder → Speaker
   (PCM)    (semantic tokens)  (response)   (waveform)   (PCM)
```

### Target Latencies
- Simple response: <200ms
- With sidecar consult: <2s

### Voice Modes
- **PeteTeaching**: Socratic, warm, questions
- **CreativeCommand**: ComfyUI prompts, generation specs
- **DevCommand**: Rust/Bevy code, terminal commands

---

## Hero's Journey Quest System

### 12 Chapters × 12 ADDIECRAPEYE Phases

The Hero's Journey maps directly to the 12-station ADDIECRAPEYE framework.

| Chapter | Campbell Stage | Act | ADDIECRAPEYE Phase | Example Objective |
|---------|---------------|-----|--------------------|-----------------|
| 1 | Ordinary World | I | Analysis | Describe yourself: What do you teach? Who are your students? |
| 2 | Call to Adventure | I | Design | Sketch the learning journey in 3 moments: hook — practice — aha |
| 3 | Refusal | I | Development | Draft the opening 60 seconds of your experience (the hook) |
| 4 | Mentor | I | Implementation | Run through your draft experience yourself — time it |
| 5 | Crossing Threshold | I | Evaluation | Define success: what metric proves this experience worked? |
| 6 | Tests/Allies | II | Contrast | Find a bad example — name exactly what makes it forgettable |
| 7 | Inmost Cave | II | Repetition | Identify the ONE core concept that must be encountered multiple times |
| 8 | Ordeal | II | Alignment | Check: does your hook connect directly to your measurable objective? |
| 9 | Reward | II | Proximity | Cluster related content — what belongs in Act 1 vs Act 2 vs Act 3? |
| 10 | Road Back | III | Envision | Write your PEARL Vision: 'When this works, the learner will feel...' |
| 11 | Resurrection | III | Yoke | Connect your learning objective to a real-world moment the student will face |
| 12 | Elixir | III | Evolve | Commit the Ch 1 design to your Book — the Iron Road continues |

### Stats Tracked
- **XP**: Experience from objectives and phases
- **Coal**: Energy used (depletes, regenerates slowly)
- **Steam**: Momentum bonus
- **Resonance**: Chapter completion streak
- **Traction/Velocity/Combustion**: Phase bonuses

---

## PARTY Management System

### Location: DEV Page (`/dev.html`)

The PARTY Management System provides a UI for controlling AI sidecars, similar to LM Studio but integrated into Trinity's Dev Console.

**Historical Context:**
- Originally used LM Studio for model management
- Goal: Zero external UI dependency
- Compromise: ComfyUI retained for diffusion workflows
- Solution: Build native PARTY management into Trinity

### Features (Planned)

```
┌─────────────────────────────────────────────────────────┐
│  PARTY MANAGEMENT (DEV Page Sidebar)                    │
├─────────────────────────────────────────────────────────┤
│  📊 Memory Budget                                       │
│  ├─ Used: 45GB / 128GB                                 │
│  ├─ ━━━━━━━━━━━━━━━━━━━━━━━━━░░░░░░░░░░░░░░░░░░░░░░░░░ │
│  └─ Available: 83GB                                    │
│                                                         │
│  🎭 Active Sidecars                                     │
│  ├─ 🧠 Brain (Crow-9B)     :8081  [●] Running          │
│  ├─ 🔧 Hands (Qwen-25B)    :8083  [●] Running          │
│  ├─ 🖼️  Images (ComfyUI)    :8188  [○] Stopped         │
│  └─ 🎵 Music (ACE-Step)    :8086  [○] Stopped          │
│                                                         │
│  ⚡ Quick Start                                          │
│  ├─ [🎨 Artist Studio]  → Brain + Hands + ComfyUI      │
│  ├─ [🎵 Music Studio]   → Brain + Hands + ACE-Step     │
│  └─ [🎲 3D Studio]      → Brain + Hands + Hunyuan3D-2.1   │
│                                                         │
│  🔍 Model Browser                                        │
│  ├─ Search HuggingFace...                              │
│  ├─ Download GGUF/ONNX/Safetensors                     │
│  └─ Convert to Trinity format                          │
└─────────────────────────────────────────────────────────┘
```

### Quick Start Presets

| Preset | Models | Ports | Memory |
|--------|--------|-------|--------|
| **Artist Studio** | Crow + Qwen + ComfyUI | 8081, 8083, 8188 | ~28GB |
| **Music Studio** | Crow + Qwen + ACE-Step | 8081, 8083, 8086 | ~30GB |
| **3D Studio** | Crow + Qwen + Hunyuan3D-2.1 | 8081, 8083, 7860 | ~38GB |
| **Full Studio** | All above | All | ~45GB |

### API Endpoints (Planned)

```
GET  /api/party/status          → All sidecar status
POST /api/party/start           → Start sidecar by role
POST /api/party/stop            → Stop sidecar by role
POST /api/party/preset          → Load preset configuration
GET  /api/party/memory          → Memory budget breakdown
```

---

## Model Download System (Roadmap)

**Note: This is a roadmap feature. Current model management relies on manual downloads via HuggingFace and local placement in `~/trinity-models`.**

### Native HuggingFace Integration

Trinity plans to build its own model downloader, similar to LM Studio's browser, for edge-case specialized models (e.g., lightweight music or image gen).

**Why:**
- HF_TOKEN not automatically used by external tools
- Need unified download progress in DEV UI
- Support GGUF, ONNX, Safetensors formats
- Integrate with PARTY management

### Planned Features

```
┌─────────────────────────────────────────────────────────┐
│  MODEL BROWSER (DEV Page Modal)                         │
├─────────────────────────────────────────────────────────┤
│  🔍 Search: [________________________] [Search HF]      │
│                                                         │
│  Results for "crow opus":                               │
│  ┌───────────────────────────────────────────────────┐ │
│  │ 📦 Crow-9B-Opus-Distill-Q4_K_M.gguf               │ │
│  │    Size: 5.6GB | Downloads: 12.3k | ⭐ 4.8        │ │
│  │    [Download] [Add to PARTY]                      │ │
│  └───────────────────────────────────────────────────┘ │
│                                                         │
│  Downloads:                                             │
│  ├─ ACE-Step-1.5-DiT-Q8_0.gguf    [████████░░] 78%     │
│  └─ CogVideoX-5B                  [░░░░░░░░░░] 0%      │
└─────────────────────────────────────────────────────────┘
```

### Configuration

```toml
# ~/.config/trinity/config.toml
[huggingface]
token = "hf_xxxx"  # Or env HF_TOKEN

[models]
download_dir = "~/trinity-models"
cache_dir = "~/.cache/trinity/models"

[formats]
gguf = true      # llama.cpp
onnx = true      # NPU candidates
safetensors = true  # Diffusion models
```

### Supported Model Types

| Type | Runtime | Purpose |
|------|---------|---------|
| **GGUF** | llama.cpp | Text LLMs |
| **GGUF** | acestep.cpp | Music generation |
| **GGUF** | moshi.cpp | Voice (PersonaPlex) |
| **ONNX** | ONNX Runtime | NPU inference |
| **Safetensors** | ComfyUI/Python | Diffusion, 3D, Video |

---

## Crate Manual Index

Each crate has its own manual at `src/crate_manual.rs`:

- `trinity`: Primary server. HTTP routes, agent tools, RAG, quest endpoints, VAAM bridge integration.
- `trinity-protocol`: The "Grammar" of Trinity. Shared types, ADDIE/HeroStage enums, Sacred Circuitry, VaamProfile, SemanticCreep, QM Rubric.
- `trinity-quest`: The "Rules". Quest state, objectives, Hero's Journey stage mapping, 12-station logic.
- `trinity-iron-road`: The "Story". Book generation, narrative engine, CreepBestiary, Game Loop, MadLibs.
- `trinity-voice`: The "Voice". PersonaPlex integration, voice endpoints, SSML handling.
- `trinity-sidecar`: The "Hands". Agentic sidecar for technical execution (Engineer), Sword & Shield pattern.

---

## File Locations

```
trinity-genesis/
├── TRINITY_FANCY_BIBLE.md             ← YOU ARE HERE (The Iron Road Design Bible)
├── crates/
│   ├── trinity/src/
│   │   ├── main.rs                      # HTTP server entry
│   │   ├── inference.rs                 # llama.cpp client
│   │   ├── voice.rs                    # Voice endpoints
│   │   ├── quests.rs                   # Quest endpoints
│   │   └── crate_manual.rs             # Crate manual
│   ├── trinity-protocol/src/
│   │   ├── lib.rs                      # Shared types
│   │   └── crate_manual.rs             # Protocol manual
│   ├── trinity-quest/src/
│   │   ├── hero.rs                     # Hero's Journey stages
│   │   ├── party.rs                    # P-ART-Y system
│   │   ├── state.rs                    # Game state, objectives
│   │   └── crate_manual.rs             # Quest manual
│   └── ...                             # Other 9 crates
├── docs/
│   ├── TRINITY_12_CRATE_MAP.md         # Architecture overview
│   └── TRINITY_AI_ORCHESTRATION_MAP.md # Memory/AI planning
└── Cargo.toml                          # Workspace root
```

---

## Inference Architecture (Verified March 22, 2026)

**The "True Trinity" Approach: Single brain, wrapped in a Studio ID system.**

**Single brain: Mistral Small 4 119B MoE via llama-server.**

| Engine | Port | Format | Role |
|--------|------|--------|------|
| **llama.cpp** | `:8080` | GGUF Q4_K_M (68GB) | **The Core Brain** — Mistral Small 4 119B MoE |
| **Qianfan-OCR** | `:8081` | GGUF (4B) | **Researcher** — Document analysis sub-agent |
| **ComfyUI** | `:8188` | Safetensors | **ART Studio** — SDXL Turbo, HunyuanVideo, ACE-Step |

**Key Specs:**
- **Inference Server**: `llama.cpp` (OpenAI-compatible `/v1/chat/completions`)
- **Context Window**: 262,144 tokens (256K MLA)
- **Memory Management**: 128GB Unified RAM (no hot-swapping needed for the brain)
- **Vision**: Mistral evaluates ART outputs via vision tokens
- **Orchestration**: `trinity` server handles the 12-station ID logic and VAAM bridge.
- **InferenceRouter**: Multi-backend auto-detect + failover (`inference_router.rs`)

**Legacy Note**: `trinity-inference` and speculative multi-model hotswapping (Hotel Pattern) have been archived. The system now prioritizes a single high-functioning AI for all reasoning tasks.

---

## Voice Architecture (Verified March 19, 2026)

### Python Voice Server (LIVE on :7777)
```
"Hey Trinity/Pete" → openwakeword → Beep → Record (VAD) → faster-whisper ASR
→ Mistral Small 4 brain (agentic, up to 5 tool rounds) → Kokoro TTS (sentence-stream) → Speaker
```
- **Script**: `scripts/launch/trinity_voice_server.py`
- **Web UI**: `http://localhost:7777` — voice selection, conversation log, type-to-test
- **Models (all CPU, ~2GB total)**: openwakeword (~10MB), faster-whisper base.en (~140MB), Kokoro 82M (~300MB)
- **Agentic tools**: read_file, write_file, edit_file, shell, search, list_dir, git
- **Dual wake word**: "Hey Trinity" → dev mode, "Hey Pete" → Iron Road mode
- **54 Kokoro voices** — selectable via web UI, default am_adam

### Two Conversation Modes
| Mode | Persona | Purpose |
|------|---------|---------|
| **Yardmaster** | User is the Yardmaster | Production AI agent. No character. Sandbox constructivism. Tools and workflows. |
| **Iron Road** | Pete | Gamified LitRPG. ADDIECRAPEYE as narrative waypoints. XP/Coal/Steam. |

---

## VAAM + Iron Road Game Mechanics

### VAAM (Vocabulary As A Mechanism)

Word-based attention management system. Tracks vocabulary mastery, user preferences,
and communication style. The isomorphic layer connecting all Trinity systems.

| Component | File | Tests | Purpose |
|-----------|------|-------|---------|
| **Sacred Circuitry** | `trinity-protocol/src/sacred_circuitry.rs` | 16 | 15-word cognitive scaffolding, 4 quadrants (Scope/Build/Listen/Ship) |
| **VaamProfile** | `trinity-protocol/src/vaam_profile.rs` | 8 | User preferences: word weights, circuit affinity, style, agreements |
| **VaamBridge** | `trinity/src/vaam_bridge.rs` | 6 | Runtime integration: VaamState + CircuitryState + VaamProfile → system prompts |
| **SemanticCreep** | `trinity-protocol/src/semantic_creep.rs` | 14 | Word-creatures: Element, Role, Stats, Wild/Tamed/Evolved states |
| **LessonMadlib** | `trinity-iron-road/src/vaam/madlibs.rs` | 8 | Creep-aware lesson plan templates with slot filling + battles |
| **Game Loop** | `trinity-iron-road/src/game_loop.rs` | 7 | CreepBestiary, events, persistence, RecyclerEvent bridge |
| **CognitiveLoad** | `trinity-iron-road/src/vaam/cognitive_load.rs` | — | Flesch-Kincaid readability scoring |
| **LitRPG Handbook** | `trinity-iron-road/src/vaam/litrpg.rs` | — | Player handbook prose from mastered words |

**Total: 70+ VAAM-related tests passing, 0 failures.**

### Isomorphic Mapping

```
Sacred Circuitry  → HOW to attend (15 words, 4 quadrants)
ADDIECRAPEYE      → WHAT to do   (12 stations, instructional design)
VaamProfile       → WHAT user prefers (word weights, style, agreements)
CharacterSheet    → WHO the user is (identity, skills, progression)
SemanticCreep     → WHAT words become (creatures with stats)
MadLibs/Quests    → HOW words are used (lesson plan templates)
Book of the Bible → WHERE it's recorded (append-only narrative ledger)
```

### Semantic Creep System (replaces "Semantic Slime" from Roblox archive)

**Core concept**: Every vocabulary word becomes a SemanticCreep creature.

```
Wild Creeps  = untamed vocabulary (Scope Nope) — detected but not mastered
Tamed Creeps = mastered vocabulary (Scope Hope) — multi-dimensional learning + conscious choice
Evolved Creeps = morphologically modified (suffix/prefix evolution)
```

**Pythagorean Taming (replaces Rule of Three):**

The old system auto-tamed a word after 3 uses. The new system measures *how* you learn, not how often:

| Dimension | Pythagorean Principle | What It Measures | Weight |
|---|---|---|---|
| Encounter breadth | Arithmos (quantity) | Distinct ADDIECRAPEYE phases seen | 30% |
| Context variety | Harmonia (relationship) | Distinct Circuit quadrants used | 25% |
| Deliberation | Logos (meaning) | Times user deliberately chose this word | 25% |
| Resonance | Intent alignment | EMA of intent match scores | 20% |

**Threshold: 0.85** — you don't need perfection. The remaining 15% grows through Context Points after taming.

**Anti-Repetition:** Repeating a word in the same phase/quadrant doesn't increase the taming score. You can't cram — you must genuinely encounter the word across different contexts.

**Scope Hope / Scope Nope:** When a Creep becomes tameable (score ≥ 0.85), the user makes a *conscious choice*:
- **Scope Hope** — tame the word, add it to your vocabulary bestiary
- **Scope Nope** — leave it wild, its resonance decays, it may return later

**Logos Engine Stats** (from archive `003_logos_engine.md`):
- **Logos** (logic/attack) — derived from element + role
- **Pathos** (emotion/HP) — derived from element + role
- **Ethos** (trust/defense) — derived from element + role
- **Speed** (initiative) — derived from element + role

**Element** (from etymological root):
| Element | Root Examples | Stat Bonus |
|---------|--------------|------------|
| 🔥 Fire | ign-, pyr-, therm- | High Logos |
| 💧 Water | aqua-, hydr-, mar- | High Pathos |
| 🪨 Earth | terra-, geo-, lith- | High Ethos |
| 💨 Air | aer-, pneu-, vent- | High Speed |
| 🌑 Shadow | umbr-, scot-, noct- | Logos + Speed |
| ✨ Light | luc-, lum-, phot- | Pathos + Ethos |
| ⚪ Neutral | (no recognized root) | Balanced |

**Role** (from morphological suffix):
| Role | Suffix Examples | Stat Bonus |
|------|----------------|------------|
| 🛡️ Tank | -tion, -ity, -ment, -ness | High Pathos + Ethos |
| ⚔️ Striker | -ize, -ate, -fy, -en | High Logos + Speed |
| 💚 Support | -ous, -ive, -ful, -able | Balanced utility |

### Iron Road Game Loop

```
COLLECT  → scan_text(phase, quadrant, intent) discovers Wild Creeps from user messages
CONSTRUCT → TamingProgress tracks 4 Pythagorean dimensions → CreepTameable event
DECIDE   → User chooses Scope Hope (tame) or Scope Nope (leave wild)
QUEST    → LessonMadlib.fill_slot() with Tamed Creeps
BATTLE   → contest_slot() → battle() resolution by slot type
REWARDS  → Context Points + VaamProfile update + RecyclerEvent → Book chapter
```

**CreepBestiary** tracks the player's full collection:
- `scan_text(phase, quadrant, intent)` — finds words (4+ chars), creates Wild Creeps, emits CreepTameable when threshold reached
- `scope_hope_creep()` / `scope_nope_creep()` — user's conscious taming decision
- `usable_creeps()` / `wild_creeps()` — filter by state
- `save_state_json()` / `load_bestiary_json()` — JSON persistence

**LessonMadlib** generates real lesson plans:
- Auto-extracts `{slot_id}` placeholders from template text
- Infers slot type (noun/verb/adjective/adverb) from slot name
- `suggest_creeps()` — ranks Tamed Creeps by fitness per slot
- `fill_slot()` — awards Context Points, rejects Wild Creeps
- `contest_slot()` — battle resolution, winner+loser both get CP
- 3 sample templates: Science, ELA, Mathematics

**RecyclerEvent bridge** — completed lessons fire events to `GreatRecycler`:
- `GameLoopEvent::LessonCompleted` → `to_recycler_event()` → `BookOfTheBible.append_chapter()`
- `GameLoopEvent::CreepTameable` → prompts Scope Hope/Nope decision

### Persistence Architecture

```
Game State Persistence:
├── CreepBestiary → JSON (save_state_json / load_bestiary_json)
├── VaamProfile → CharacterSheet.vaam_profile (serializable)
├── Quest State → PostgreSQL (trinity_quest::save_game_state)
└── Book of Bible → Markdown files (docs/books_of_the_bible/*.md)
    └── SSE broadcast to /book.html on new chapters
```

### Server Wiring (5 Integration Points — commit 5b548757)

```
User message → /api/chat
  ├── 1. VaamBridge.process_user_input()  → coal, circuit, profile update
  ├── 2. Bestiary.scan_text(phase,quad,intent)  → discover Wild Creeps, emit CreepTameable
  ├── 3. Book SSE channel                 ← taming/discovery events broadcast
  ├── 4. System prompt += VAAM context    → LLM sees vocabulary state
  └── 5. VaamBridge.process_ai_output()   → scan AI response too

Scope decision → POST /api/bestiary/tame
  └── Scope Hope → tame Creep + SSE event
      Scope Nope → leave wild, decay resonance

Intent setting → POST /api/ground + POST /api/intent
  └── Updates CharacterSheet intent posture + vulnerability

Quest action → /api/quest/complete or /api/quest/advance
  └── Book SSE channel ← objective_completed / phase_advanced events

Orchestration → /api/orchestrate
  └── player_context += VaamProfile.prompt_summary() + Bestiary.summary()
      → All 12 ADDIECRAPEYE phases have vocabulary awareness

UI → GET /api/bestiary
  └── Full Creep collection: word, element, role, state, stats, power, CP
```

**AppState fields** (consolidated — removed redundant `vaam_state`):

| Field | Type | Purpose |
|-------|------|---------|
| `vaam_bridge` | `Arc<VaamBridge>` | VAAM + Sacred Circuitry + profile (single source) |
| `bestiary` | `Arc<RwLock<CreepBestiary>>` | Player's vocabulary creatures |
| `book_updates` | `broadcast::Sender<String>` | SSE channel for real-time updates |
| `game_state` | `SharedGameState` | Quest progression (PostgreSQL-backed) |
| `character_sheet` | `Arc<RwLock<CharacterSheet>>` | Player identity + VaamProfile |

**Total: 175 tests passing across 6 crates, 0 failures.**

---

## Instructional Design Systems (Verified March 18, 2026)

### Quality Control — What's REAL in the codebase

| System | File | Status |
|--------|------|--------|
| **QM Rubric Evaluator** | `trinity-protocol/src/qm_rubric.rs` | ✅ 405 lines, scores 0-100, wired into orchestration |
| **ID Contract (Backward Design)** | `trinity-protocol/src/id_contract.rs` | ✅ Objectives → milestones → content, Action Map built in |
| **Learning Objectives (Bloom's)** | `id_contract.rs` | ✅ verb + content + condition + criterion |
| **Yardmaster (Learning Objects)** | `asset_generation.rs` | ✅ Atomic units with Bloom's level, cognitive load, prerequisites |
| **ADDIECRAPEYE Orchestration** | `conductor_leader.rs` | ✅ All 12 phases with real LLM calls, hotel pattern |
| **Scope Creep Detection** | `scope_creep.rs` | ⚠️ Keyword stub → SemanticCreep system replaces (see VAAM section) |
| **Backward Design Enforcement** | `main.rs` DEV mode prompt | ✅ Rejects content-first requests, enforces Action Mapping |
| **SME Interview Protocol** | `main.rs` DEV mode prompt | ✅ STAR method, anchoring questions, simplification |

### Standards Implemented
- **IBSTPI**: 22 competencies mapped to orchestration phases
- **ATD**: 14 capability statements, especially SME interviewing (Statement 9)
- **AECT**: 5 standards, ethics constraints in system prompts
- **QM Higher Ed Rubric**: 4 criteria automated (objectives, action mapping, assessment, cognitive load)

### Reference Document
`docs/research/TRINITY_INSTRUCTIONAL_DESIGN_BLUEPRINT.md` — Full mapping of IBSTPI/ATD/AECT/QM to Trinity architecture.

---

## eLearning Template System

### Base Template: Local-AI-Architect (Purdue)
- **Location**: `~/Elearning/local-ai-architect-elearning/`
- **Stack**: React 18 + Vite + TailwindCSS + Lucide icons
- **Structure**: 3 modules + sandbox + quiz + docs (ADDIE-aligned)
- **Guide**: `docs/templates/ELEARNING_TEMPLATE_GUIDE.md`

---

## Launch Commands

### One-command start (tmux)
```bash
bash scripts/launch/start_trinity.sh           # DEV mode (default)
bash scripts/launch/start_trinity.sh iron-road  # Iron Road mode
```

### Manual start
```bash
# Terminal 1: LLM (Mistral Small 4 119B, 256K MLA context)
export LD_LIBRARY_PATH=$HOME/Workflow/desktop_trinity/trinity-genesis/llama.cpp/build-rocm/bin:$LD_LIBRARY_PATH
export HSA_OVERRIDE_GFX_VERSION=11.0.0
llama-server -m ~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
  -c 262144 --port 8080 -ngl 99 --parallel 2 --flash-attn on --jinja --host 0.0.0.0 --threads 12

# Terminal 2: Trinity server
LLM_URL=http://127.0.0.1:8080 ./target/release/trinity

# Terminal 3: Voice server (Python)
source ~/trinity-vllm-env/bin/activate && CUDA_VISIBLE_DEVICES="" python3 scripts/launch/trinity_voice_server.py

# Terminal 4: ComfyUI (ART pipeline)
source ~/trinity-vllm-env/bin/activate && cd ~/ComfyUI && python3 main.py --port 8188 --listen 0.0.0.0
```

---

## ADDIECRAPEYE ↔ TRINITY Isomorphism

```
TRINITY = ID + AI + OS

ID  = ADDIE    = Pete (Iron Road)     — Instructional Design, Purdue LDT expertise
AI  = CRAP     = ART Studio           — Robin Williams design theory, Bevy game studio
OS  = EYE      = Yardmaster (the USER) — Evaluation, orchestration, system management

P-ART-Y = Pete + Aesthetics/Research/Tempo + You(Yardmaster)
```

**You are the Yardmaster.** You manage the yard. You don't lay every piece of iron — you delegate to P-ART. Pete is the sole AI personality. ART modes are just system prompts.

**Character Sheet = Yardmaster UX settings.** Guides preferences. In code at
`AppState.character_sheet` but not yet meaningfully surfaced in Yardmaster web UI.

---

## Avatar Pipeline (March 19, 2026 — WORKING)

Full NPC character creation: concept → backstory → portrait → 3D mesh → voice → music → Bevy ECS entity.

**Script**: `scripts/launch/avatar_pipeline.py`
**First NPC**: Fleet (steampunk engineer) at `assets/avatars/Fleet/`

| Step | Tool | Status | Time |
|------|------|--------|------|
| Character sheet | Mistral LLM | ✅ Working | ~50s |
| Portrait | ComfyUI SDXL Turbo | ✅ Working | ~4s |
| Vision evaluation | Mistral vision | ✅ Available | — |
| 3D mesh | Hunyuan3D-2.1 | ✅ Wired (Gradio :7860) | — |
| Theme music | ACE-Step 1.7B | ⏭️ GGUF on disk (2.2GB) | — |
| NPC voice | Kokoro TTS | ✅ Working (54 voices) | <1s |
| Voice clone | Chatterbox Turbo | ⏭️ Cached | — |
| Bevy ECS entity | Mistral LLM | ✅ Working | ~13s |
| Presentation video | HunyuanVideo | ⏭️ Model on disk (13GB) | — |

**Maps to ADDIECRAPEYE**: Avatar creation IS the tutorial quest in Iron Road.
Each step maps to a station. EYE evaluates the output via Mistral vision.

---

## ART Tools Architecture — The Creative Sidecar Stack

> *ART has no persona. ART is pure tooling — the instruments in the studio.*

ART is the second letter in P-ART-Y. It is NOT an AI personality — it is the collection of creative sidecars that the Yardmaster and Pete can invoke. Each tool runs as an independent process, communicates via HTTP, and shares GPU resources with Mistral through dual-channel compute.

```
ART Sidecar Stack (all via HTTP, all local-first)
┌─────────────────────────────────────────────────────────────┐
│  VISUAL                                                      │
│  ├── ComfyUI :8188          → SDXL Turbo (PROVEN LIVE)       │
│  ├── Hunyuan3D :7860        → 3D mesh from portrait          │
│  └── HunyuanVideo :8188     → 4s cinematic video             │
├─────────────────────────────────────────────────────────────┤
│  DOCUMENT INTELLIGENCE                                       │
│  ├── Qianfan-OCR            → Syllabus/PDF/textbook ingestion│
│  │   ├── Feeds VAAM vocabulary extraction                    │
│  │   ├── Feeds Quality Scorecard document input              │
│  │   └── Apache 2.0 license (Purdue-compatible)              │
│  └── beast_logger           → Creative pipeline telemetry    │
│      └── Already LIVE on ART page (ArtStudio.jsx)            │
├─────────────────────────────────────────────────────────────┤
│  AUDIO                                                       │
│  ├── Kokoro TTS             → 54 voices, <1s generation      │
│  ├── ACE-Step 1.7B          → AI music composition           │
│  └── Chatterbox Turbo       → Voice cloning                  │
├─────────────────────────────────────────────────────────────┤
│  SPATIAL (Bevy Desktop — Archive Ready)                      │
│  ├── 3D Avatar rendering    → Spirit Crystal + orbital rings │
│  ├── ADDIE egui panels      → 1,416 lines, all 12 phases    │
│  ├── Asset preview          → Load .glb meshes + textures    │
│  └── OBS capture            → Real-time visual feedback      │
└─────────────────────────────────────────────────────────────┘
```

### Bevy + OBS Code Preview — The Game Development Loop

Trinity's endgame for the ART Studio is a **real-time code-to-visual feedback loop**:

1. **Yardmaster** asks Pete: *"Create a food web game where students connect organisms"*
2. **Pete** (via SCOUT SNIPER) generates 12-phase quest + Bevy ECS code
3. **Bevy desktop window** compiles and renders the game in real-time
4. **OBS Studio** captures the Bevy window frame-by-frame
5. **Mistral Vision** evaluates the visual output against the PEARL alignment
6. **beast_logger** records every creative event for telemetry

This is the **Purdue literal game engine** — an AI that doesn't just describe a game, it *builds and runs one while you watch*. The archive contains 913 lines of Bevy main.rs, 367 lines of 3D avatar animations, and a full HTTP bridge to Axum. Estimated revival: ~3–4 hours of focused work.

---

## Hardware (Verified)

- **Machine**: GMKtek EVO X2 128GB (headless server target)
- **CPU/GPU**: AMD Ryzen AI Max+ 395 (Strix Halo, gfx1151, 40 CUs RDNA 3.5)
- **Memory**: 128GB LPDDR5X-8000 unified, 256 GB/s bandwidth
- **NPU**: XDNA 2 (50 TOPS)
- **Kernel**: 6.19.4 with `iommu=pt amdgpu.gttsize=126976 ttm.pages_limit=33554432`
- **ROCm**: 7.2.0
- **Blender**: 4.0.2 (Bevy standard 3D pipeline)
- **OBS Studio**: 30.0.2 (screenshot/video capture for Mistral vision)

### Static Memory Budget (load once, never reload)
```
128GB Unified RAM — STATIC TRINITY ID AI OS
├── Mistral Small 4 weights:     68.0 GB (mmap, always resident)
├── Mistral KV cache (256K):      5.6 GB (FP16, MLA compressed)
├── ComfyUI + SDXL Turbo:         7.0 GB (loaded on demand)
├── Kokoro TTS (CPU):             0.5 GB
├── OS + PG + Trinity server:     2.0 GB
├── Compute buffer:               0.8 GB
│                                ────────
│   Base load:                   83.9 GB
│   Free for ART models:        44.1 GB  ← Hunyuan3D (15GB), ACE-Step (3GB), etc.
```

**Dual-channel compute**: Mistral and ART tools share GPU. Pause Mistral inference
while ART generates, resume after. Both stay in RAM via mmap — no reload from disk.

### Minimum Target Spec (Desktop App)
- 24GB GPU VRAM
- Smaller models for Iron Road + ART productivity
- Same API, scaled models

---

## Next Steps (Prioritized)

### ✅ Completed (March 18 Session)
1. ~~Wire QM Rubric into evaluation~~ ✅
2. ~~Enforce Backward Design in DEV mode~~ ✅
3. ~~Fix llama.cpp ROCm build~~ ✅
4. ~~tmux launch script~~ ✅
5. ~~Voice loop v1 (walkie-talkie) + v2 (radio protocol)~~ ✅
6. ~~Pete's voice (8K Field Manual) in Iron Road prompt~~ ✅
7. ~~Moshi Rust backend compiled + Python sidecar script~~ ✅
8. ~~Voice architecture research (Moshi vs Qwen-Omni vs PersonaPlex)~~ ✅
9. ~~Bible v2.0.0 + git commit~~ ✅

---

### Immediate Priorities
1. ~~Wire Trellis 3D meshing into Avatar Pipeline~~ → Replaced with Hunyuan3D-2.1 (Gradio API on :7860)
2. Make Avatar Pipeline callable from Yardmaster web UI as a workflow
3. Wire ACE-Step music when local server becomes available
4. Ingest Pete Field Manual + ID Blueprint into RAG database
5. Surface CharacterSheet in Yardmaster web UI for preferences

### Near-term
6. Dual-channel compute switching (pause Mistral during ART generation)
7. Wire Chatterbox voice cloning for custom NPC voices
8. HunyuanVideo for avatar presentation videos
9. Connect eLearning template (React/Vite) scaffolding to Yardmaster
10. Upgrade scope_creep to LLM-based detection

### Future Scope (Documented, Not Blocked)
- **Ming-flash-omni-2.0**: Archived. Too immature for gfx1151. Revisit ~1 year.
- **LFM2.5-Audio**: Lightweight audio model (3GB). Could replace Whisper+Kokoro pipeline.
- **Qwen2.5-Omni-3B**: Emotional intelligence layer.
- Unsloth Gemma finetuning for smart game NPCs
- vLLM batch worldbuilding swarm for ART sidecar (when FP8 support matures)
- Moshiko Candle Q8 for pure Rust voice (when Candle adds ROCm)
- OS-agnostic desktop app (24GB GPU minimum target)
- CachyOS pure Rust build for Bevy XR/AR systems

---

## Maturity Assessment (March 22, 2026 — ComfyUI + OCR Release)

### Codebase Numbers

```
Total workspace:   ~39,000 LOC Rust (Active) + 8,155 LOC frontend
├── Active crates:  6 (trinity, protocol, quest, iron-road, voice, sidecar)
├── Archive:       ~150,000+ LOC (restorable, not deleted)
├── Frontend:       React (14 components, 7 hooks, Cinzel/Crimson typography)
├── Docs:           140+ markdown files
├── Scripts:        125+ shell/utility scripts
└── Quest files:    8 JSON quest definitions

Tests: 179 passing, 0 failures, 0 warnings
  ├── trinity:          83 tests (agent, VAAM bridge, inference router, Ring 2/3/5/6, PEARL, Perspective Engine, Quality Scorecard, Journal States)
  ├── trinity-protocol: 67 tests (sacred circuitry, VaamProfile, SemanticCreep)
  ├── trinity-quest:    16 tests (quest board, objectives, phase logic)
  ├── trinity-iron-road: 16 tests (narrative, game loop, bestiary)
  ├── trinity-voice:    10 tests (SSML injection, VAAM vocal emphasis)
  └── others:            3 tests
```

### Crate Maturity Classification

#### LOCKED — Complete, tested, can be frozen

| Crate | LOC | Tests | Grade | What It Does |
|-------|-----|-------|-------|--------------|
| **trinity-protocol** | ~10,000 | 59 | **A+** | CharacterSheet, VaamProfile, Sacred Circuitry (15 words), SemanticCreep, QM Rubric, ID Contract, ADDIECRAPEYE phases. |
| **trinity-iron-road** | ~1,600 | 15 | **A** | CreepBestiary, GameLoop, MadLibs, Book narrative engine. |
| **trinity (server)** | ~7,500 | 18 | **B+** | HTTP API on :3000. RAG, VAAM Bridge, Quest Endpoints, Pete/Conductor orchestration. |
| **trinity-quest** | ~1,100 | 0 | **B** | 12 ADDIECRAPEYE stations, Hero's Journey logic, PostgreSQL persistence. |

**Total LOCKED: ~20,200 LOC — this is the real product.**

#### FUNCTIONAL — Works but needs hardening

| Crate | LOC | Tests | Grade | What It Does |
|-------|-----|-------|-------|--------------|
| **trinity-sidecar** | ~3,800 | 0 | **B-** | Engineer binary. Tested live with Opus 27B + REAP 25B. Roles, quests, prompts. |
| **trinity-voice** | ~700 | 0 | **C+** | SSML types, voice integration endpoints. |

**Total FUNCTIONAL: ~4,500 LOC.**

---

### What Actually Runs Today

```
START → bash scripts/launch/start_trinity.sh
  ├── PostgreSQL (107 RAG chunks)
  ├── llama.cpp :8000 (Mistral Small 4 119B, 68GB, 256K ctx)
  ├── Trinity Server :3000 (Axum)
  │     ├── GET  /dev.html         → YARDMASTER (dev mode, no game mechanics)
  │     ├── GET  /ironroad.html    → Pete / Iron Road (LitRPG, VAAM, Coal/Steam)
  │     ├── GET  /art.html         → ART Studio (creative pipeline)
  │     ├── POST /api/chat/yardmaster → Agentic tool-use loop (mode=dev|ironroad)
  │     ├── POST /api/creative/image  → ComfyUI SDXL Turbo image gen (WORKING)
  │     ├── GET  /api/quest        → Quest state (ADDIECRAPEYE)
  │     ├── GET  /api/bestiary     → Creep collection
  │     ├── GET  /api/character    → CharacterSheet
  │     └── POST /api/tools/execute → 29 agentic tools (shell, files, creative, quest, etc.)
  │     ├── Ring 2: Destructive tool gate (mode-based persona clearance)
  │     ├── Ring 3: Rolling context summary (compress old messages)
  │     └── Ring 5: Rate limiting (60/min global, 5/min destructive) + sandboxing
  ├── Voice Server :7777 (Python — openwakeword + whisper + Kokoro TTS)
  └── ComfyUI :8188 (SDXL Turbo + HunyuanVideo + ACE-Step nodes)

AVATAR PIPELINE:
  └── python3 scripts/launch/avatar_pipeline.py "concept" --style steampunk
      → character.json + portrait.png + entity.rs + voice_sample.wav
```

### What's WORKING for Offline Agentic Trinity (Updated March 22, 2026 — 9:25 PM)

**Core (✅ LIVE — PROVEN TONIGHT)**:
- Mistral Small 4 119B via llama.cpp (256K ctx, MLA, vision, thinking mode)
- 30 agentic tools (shell+cwd, read/write/list/search files, creative, quest, lesson plans, etc.)
- **Ring 2**: Destructive tool gate — persona-based tool permission enforcement
- **Ring 3**: Rolling context summary — deterministic digest compression for old messages
- **Ring 5**: Rate limiting (60 calls/min, 5 destructive/min) + 40+ shell blocked patterns
- Quest/workflow system (12 ADDIECRAPEYE phases, PostgreSQL persistence)
- PEARL focusing agent — subject/medium/vision with alignment scoring
- RAG via PostgreSQL + pgvector (107 chunks)
- VAAM vocabulary tracking + Bestiary (persists on every scan)
- Character sheet persistence (JSON on disk)
- Voice pipeline (Python sidecar — openwakeword + whisper + Kokoro TTS)
- **ComfyUI image generation — PROVEN LIVE** (SDXL Turbo, 25.5s first gen via Trinity API, PyTorch 2.5.1+rocm6.2)
- Avatar Pipeline v1 (backstory → portrait → entity.rs → 8-step workflow defined)
- Mode gating: dev (Yardmaster, no game mechanics) vs ironroad (Pete, full LitRPG)
- Quality Scorecard — 5-dimension pedagogical evaluation via LLM scoring
- Blender 4.0 + OBS Studio installed
- **137.4 GB VRAM** detected (Strix Halo unified memory, ROCm 7.2)

**Committed Upgrades (🔧 In Progress)**:
1. **Qianfan-OCR** → Document intelligence sidecar (ART persona). Ingest syllabi, PDFs, textbooks → VAAM vocabulary extraction + Quality Scorecard input. Apache 2.0 license.
2. **beast_logger** → ✅ ALREADY LIVE on ART page (`ArtStudio.jsx`). Color-coded creative pipeline logger (COMFYUI/ACE_STEP/AVATAR/SUCCESS/ERROR tags). Currently tracks creative events — extend to VAAM creature encounters and mastery rates for Purdue research telemetry.
3. **Bevy ART Studio** → Native desktop window (separate process, HTTP to Axum). 913-line main.rs + 367-line avatar.rs + 1,416-line ADDIE UI already in archive. ~3-4 hours to revive.
4. **Bevy + OBS Code Preview** → Trinity writes Bevy game code, compiles it, OBS captures the Bevy window in real-time for visual feedback loop. The professor sees code → game in one flow.

**Still needed (🔧)**:
1. ~~**Trellis 3D meshing**~~ → **Replaced by Hunyuan3D-2.1** (POST /api/creative/mesh3d)
2. **ACE-Step music** — GGUF on disk, needs local server
3. **Chatterbox voice cloning** — cached, needs GPU compute switch
4. ~~**HunyuanVideo**~~ → **Wired** (POST /api/creative/video → ComfyUI workflow)
5. **Dual-channel compute** — pause Mistral during ART generation

**Still needs pruning (🗑️)**:
1. **trinity-inference** — 33K LOC, mostly dead. Cut to ~3K.
2. **Ming-flash-omni-2.0** — archived, not deleted. 217GB safetensors on disk.
3. **FP8 safetensors** — 162GB unusable on this GPU. Delete when disk space needed.

### Engineering Quality Summary

```
Functional LOC:     ~39K Rust + 8K frontend (what actually executes on :3000 + sidecar)
Archive:            ~150K LOC (safely archived, restorable — includes full Bevy desktop app)
Test coverage:      179 tests on 39K functional LOC = solid
Security:           Ring 2 (permission gates) + Ring 3 (context management) + Ring 5 (rate limiting + sandboxing)
Git:                Clean history, SSH keys purged from history
ComfyUI:            LIVE on :8188 — PyTorch 2.5.1+rocm6.2, SDXL Turbo checkpoint loaded, 25.5s generation
```

### Recommended Priority Path

```
PHASE 1 — LOCK IRON ROAD (current)
  ✅ VAAM + Bestiary + MadLibs (done, tested)
  ✅ Server wiring (done, 5 integration points)
  🔧 Bestiary persistence (save/load JSON)
  🔧 Audio pipeline (Kokoro TTS → server endpoint)

PHASE 2 — PRUNE & HARDEN
  ✅ Archive trinity-inference (moved to archive/crates)
  🔧 Add tests to trinity-quest and trinity-sidecar
  🗑️ Consolidate stub crates
  🔧 Fix trinity-comfy test error

PHASE 3 — PERSISTENT AGENT
  🔧 Long-running agentic loop (plan → tool → verify → iterate)
  🔧 Model hot-swap API
  🔧 Offline package / installer

PHASE 4 — LEVEL 2 UI
  🔧 ART sidecar (ComfyUI + creative) OR
  🔧 DEV sidecar (Yardmaster IDE) — choose based on need
  🔧 Bevy spatial UI when ready
```

---

| Document | Location | Purpose |
|----------|----------|---------|
| **Fancy Bible** | `TRINITY_FANCY_BIBLE.md` | The Iron Road Design Bible (this file) |
| **ID Blueprint** | `docs/research/TRINITY_INSTRUCTIONAL_DESIGN_BLUEPRINT.md` | IBSTPI/ATD/AECT/QM mapping |
| **Voice Research** | `docs/research/VOICE_ARCHITECTURE_RESEARCH.md` | Moshi vs Qwen-Omni vs PersonaPlex |
| **Pete Field Manual** | `docs/bible/ASK_PETE_FIELD_MANUAL.md` | Pete's personality and communication style |
| **eLearning Template** | `docs/templates/ELEARNING_TEMPLATE_GUIDE.md` | Local-AI-Architect (Purdue) as base |
| **Session Context** | `CONTEXT.md` | Architecture decisions and handoff state |

---

## The 12 ADDIECRAPEYE Stations (v5.1.0)

Trinity orchestrates development through a 12-station lifecycle. While the user sees "Quest Progress," the AI aligns its attention using **Sacred Circuitry** mappings and **Bloom's Taxonomy** cognitive levels.

| # | Station | Purpose | AI Attention (Circuit) | Bloom's Level |
|---|---------|---------|------------------------|---------------|
| 1 | **Analyze** | Extract intent, learners, and gaps. | Center | Remember/Understand |
| 2 | **Design** | Bloom's levels, objectives, VAAM. | Expand / Balance | Apply |
| 3 | **Develop** | Asset creation and resource drafting. | Prepare | Create |
| 4 | **Implement** | Deployment, timing, and setup. | Express | Apply |
| 5 | **Evaluate** | Quality Matters (QM) review. | Receive | Evaluate |
| 6 | **Contrast** | CRAP: Visual hierarchy, emphasis ranking. | Unlock | Analyze |
| 7 | **Repetition** | CRAP: Pattern reinforcement, consistency audit. | Flow | Apply |
| 8 | **Alignment** | CRAP: Structure and grid compliance. | Relate | Evaluate |
| 9 | **Proximity** | CRAP: Grouping related elements, boundary design. | Realize / Act | Analyze |
| 10| **Envision** | EYE: Meta-cognitive reflection — "what do I see?" | Extend | Evaluate |
| 11| **Yoke** | EYE: Coupling systems together — integration. | Transform / Connect | Create |
| 12| **Evolve** | EYE: Ship it. Final metrics. The Yardmaster's moment. | Manifest | Create |

---

## Isomorphic Mapping: The "Meaning Making" Chain

Trinity ensures every technical system has a pedagogical or narrative counterpart:

1.  **Sacred Circuitry (HOW to attend)**: 15 nodes of AI cognitive scaffolding.
2.  **VAAM (Vocabulary As A Mechanism)**: The bridge where AI attention meets User preference.
3.  **ADDIECRAPEYE (WHAT to do)**: The 12-station methodology for building education.
4.  **Character Sheet (WHO the user is)**: Persistent identity, skills, and resonance.
5.  **Book of the Bible (WHY it matters)**: Append-only narrative ledger — records the meaning-making journey.

---

## The Pythagorean Perspective (v5.1.0)

> *"Educate the children and it won't be necessary to punish the men."* – Pythagoras

Pythagoras organized knowledge into three orders. Trinity mirrors this:

| Order | Greek | Trinity System | What It Tracks |
|-------|-------|---------------|----------------|
| **Arithmos** | Number/Counting | Quest XP, Coal, Steam, Stats | WHAT happened |
| **Harmonia** | Structure/Pattern | ADDIECRAPEYE, Sacred Circuitry, VAAM | HOW it's structured |
| **Logos** | Meaning/Reason | Book of the Bible, Bloom's levels | WHY it matters |

The 12-station cycle maps to a 5:4:3 ratio (ADDIE:CRAP:EYE) — a Pythagorean triple. The four Sacred Circuitry quadrants (Scope/Build/Listen/Ship) expand the tetractys. Creep stats (Logos/Pathos/Ethos) echo Aristotle's rhetoric, rooted in Pythagorean tradition.

**The Cycle of the Phoenix**: When Evolve (station 12) completes, the cycle returns to Analyze (station 1). The Great Recycler transforms the completed journey into a Book chapter, and the next quest begins with richer vocabulary, stronger Creeps, and deeper self-knowledge. This is the *palingenesia* — rebirth through completion.

---

## UI Hierarchy (Level 1 focus)

- **Primary (User)**: Character Identity + ADDIECRAPEYE Quest Progress.
- **Secondary (Mechanic)**: Vocabulary Bestiary (Taming words).
- **Tertiary (Narrative)**: Book of the Bible (Learning journey ledger).
- **Background (Internal)**: Sacred Circuitry (AI Scaffolding — visually de-emphasized).

---

*This is the Iron Road Design Bible. Follow it zealously.*
*Updated: 2026-03-22 — v8.0.0: Merged User Manual, Ring 6 Perspective Engine, Journal States, Quality Scorecard, 175 tests.*
*Version: 8.0.0*

---

## Installation & Setup

### Prerequisites
- **Operating System**: Linux (developed on AMD Strix Halo hardware)
- **Hardware**: 128GB+ unified RAM recommended for full functionality
- **Inference Engine**: [llama.cpp](https://github.com/ggml-org/llama.cpp) built with Vulkan support
- **AI Model**: [Mistral Small 4 119B](https://huggingface.co/mistralai/Mistral-Small-4-119B-2503) GGUF Q4_K_M (~68GB)
- **Database**: PostgreSQL 15+ with pgvector extension
- **Development Tools**: Node.js 18+ and Rust 1.80+

### Quick Start
```bash
# 1. Clone
git clone <repository-url> && cd trinity-genesis

# 2. Start LLM (port configurable — set LLM_URL if not 8080)
llama-server -m ~/trinity-models/gguf/Mistral-Small-4-119B-Q4_K_M.gguf \\
  --host 127.0.0.1 --port 8080 -ngl 99 --ctx-size 262144 --flash-attn on --jinja --parallel 2

# 3. Build and run Trinity
cargo build --release && cargo run --release

# 4. Open browser
xdg-open http://localhost:3000
```

### Optional Sidecars
| Service | Command | Port | Purpose |
|---------|---------|------|---------|
| ComfyUI | `cd ~/ComfyUI && python main.py --port 8188` | 8188 | Image generation (SDXL Turbo) |
| Qianfan-OCR | `llama-server -m ~/trinity-models/Qianfan-OCR.gguf --port 8081` | 8081 | Document intelligence |
| Voice | `python scripts/voice_sidecar.py` | 8200 | Whisper STT + Kokoro TTS |

### For Non-Technical Users
If you're an educator without command-line experience:
- **Institutional Support**: Check with your IT department — they may provide a pre-configured server
- **Pre-Built Environments**: Cloud-hosted versions are on the roadmap
- **Smaller Models**: Use Crow 9B (~6GB) for testing on lower-spec hardware

---

## Troubleshooting & Known Issues

### Common Setup Issues

| Symptom | UI Indicator | Cause | Fix |
|---------|-------------|----------|-----|
| Pete doesn't respond | "No LLM detected" (red) | LLM server not running | Start llama-server, verify `LLM_URL` |
| Quest progress lost | "Database offline" banner | PostgreSQL not running | Start PostgreSQL, check `.env` connection string |
| System crashes / slow | RAM at 95% in Hardware tab | Insufficient RAM | Use smaller model (Crow 9B) or future cloud option |
| UI shows stale data | Spinning loader | SSE connection lost | Refresh browser (`Ctrl+R`), restart Trinity server |
| Image gen fails | "ComfyUI: Disconnected" | ComfyUI sidecar not running | Start ComfyUI on :8188 |
| Timeout during quest | "Operation timed out after 300s" | Complex query / large context | Cow Catcher auto-skips; simplify prompt next time |

### Known Limitations
- **Hardware Dependency**: Full functionality requires 128GB RAM + Vulkan GPU
- **Incomplete Features**: 3D Yard (Bevy), advanced voice pipelines are in progress
- **Technical User Base**: Current setup requires command-line familiarity

### Getting Support
- Check server logs for `Error`, `Failed`, or `Timeout` keywords
- Refer to `CONTEXT.md` for API details
- System status page at `http://localhost:3000` (Settings/Status tab)

---

## User Interaction with Pete

### The Socratic Protocol in Practice

Pete never gives the answer. Pete asks the question that makes the answer obvious.

**Principles:**
1. **Ask Before Telling** — lead with a question to encourage deep thinking
2. **Present Options** — 2-3 narrative paths, never a single command
3. **Reflect Back** — summarize user input, confirm before proceeding
4. **Reward Discovery** — vocabulary mastery earns Coal, not obedience
5. **Guard the PEARL** — if the response drifts from subject/vision, flag as Scope Creep

### Practical Scenario: Biology Lesson on Photosynthesis

1. **Pete's Prompt**: *"You've arrived at the Design Station with a lush forest as your backdrop. How might you structure a game to teach photosynthesis to 7th graders? Consider a journey, a puzzle, or a competition."*
2. **User Response**: *"I think a journey where students guide a plant through growth stages by collecting sunlight and water would work."*
3. **Pete's Reflection**: *"A journey through growth stages is a fine path. I see you've earned some Steam for that creative spark! How many stages will this journey have, and what challenges will the plant face at each?"*
4. **Outcome**: If stages align with photosynthesis steps → *"Your design has forged 5 Iron for its strong structure."* If ideas drift → *"A Scope Creep shadow looms — does this magic potion fit your science lesson? Roll for a skill check to refocus."*
5. **Progress**: Completing the Design Station updates the character sheet (+1 Gamification skill) and advances the Iron Road narrative.

### Gamification Mechanics
- **Coal (Compute Budget)**: Earned +10 for thoughtful answers. Consumed by Scope Creep battles.
- **Steam (Creative Energy)**: +5 per completed objective. Spent on ART generation.
- **Iron (Structural Integrity)**: Bolstered by strong design answers. -3 on lost battles.
- **Scope Creep Battles**: d20 roll + skill bonus vs. difficulty rating (e.g., DC 15).
- **Semantic Creep Taming**: Correct vocabulary usage → tame an elemental Creep → +1 skill bonus.

---

## Journal States & Quality Scorecard

### Journal States — Chapter Milestones & Weekly Reflections

Every phase completion automatically captures a Journal Entry — a full snapshot of:
- Quest progress (phase, objectives, completed phases, XP)
- Character sheet (resonance, skills, experience)
- Timestamp and auto-generated summary

**Entry Types:**
| Type | Icon | When Created |
|------|------|--------------|
| Phase Complete | 🚉 | Auto: on each ADDIECRAPEYE phase advance |
| Chapter Complete | 🏆 | Auto: when a Hero's Journey chapter is finished |
| Weekly Reflection | 📓 | Manual: user writes a reflection with the Journal button |
| Manual Checkpoint | 📌 | Manual: user creates a savepoint |
| Demo Bookmark | 🎬 | Manual: marks a moment for demo/presentation playback |

**API:**
- `GET /api/journal` — list all entries (newest first)
- `POST /api/journal` — create entry: `{"entry_type": "weekly_reflection", "reflection": "...", "tags": ["week-3"]}`
- `GET /api/journal/export/:id` — standalone HTML portfolio page

**Frontend:** JournalViewer.jsx — timeline with expandable entries, reflection textarea, export links.

### Quality Scorecard — Pedagogical Document Evaluation

5-dimension heuristic scoring (no LLM needed, runs instantly):

| Dimension | What It Measures |
|-----------|------------------|
| **Bloom's Coverage** | Verb diversity across all 6 taxonomy levels |
| **ADDIE Alignment** | Coverage of analysis, design, development, implementation, evaluation |
| **Accessibility** | Alt text, heading structure, readability level markers |
| **Student Engagement** | Interactive elements, collaborative activities, real-world connections |
| **Assessment Clarity** | Clear rubrics, measurable outcomes, formative assessment presence |

**API:** `POST /api/yard/score` — body: `{"text": "..."}`
**Returns:** Overall score (0.0-1.0), letter grade (A-F), per-dimension scores, recommendations.

*NotebookLM summarizes your syllabus. Trinity tells you what's missing.*

---

## Legal Compliance & Validation

### Data Privacy (FERPA, COPPA)
- **FERPA**: Student data protected — Trinity's offline-first architecture keeps data local
- **COPPA**: Parental consent requirements met via local-only deployment (no cloud data transmission)
- **IDEA / Section 504**: VAAM and Pete's adaptive responses support IEP-aligned content

### Purdue University Global Campus Mapping
- **Global Partnerships**: Trinity as a cross-cultural content creation platform
- **Technology Integration Certificate**: Trinity directly supports this LDT curriculum
- **Agnostic Tool**: Offline-first, API-extensible, LMS-compatible (Canvas/Moodle export)

### Resources
- BEVY Engine: [github.com/bevyengine/bevy](https://github.com/bevyengine/bevy)
- OpenXR for BEVY: [github.com/awtterpip/bevy_openxr](https://github.com/awtterpip/bevy_openxr)
- Legal frameworks: [Edutopia - AI and the Law](https://www.edutopia.org/article/laws-ai-education/)

---

## Appendix A — The Lexicon

*Every acronym in Trinity means something. If it doesn't have a meaning, it shouldn't exist.
If it doesn't have code, it's a draft. If it doesn't have pedagogy, it's an engineering vanity.
This table is the spell check for the system itself.*

### Audit Key

| Column | Meaning |
|--------|---------|
| **Stands For** | The spelled-out expansion of the acronym |
| **Pedagogy** | What it teaches, why it exists for the *learner* |
| **Architecture** | Where it lives in code (crate, file, struct) |
| **Status** | 🟢 Code + Meaning | 🟡 Meaning only | 🔵 Code only | ⚪ Planned |

---

### TRINITY ID AI OS

| | |
|---|---|
| **Stands For** | **T**eaching **R**esource for **I**nstructional desig**N**, **I**ntelligent **T**utoring, and self-directed **Y**earning — **I**nstructional **D**esign **A**rtificial **I**ntelligence **O**perating **S**ystem |
| **Pedagogy** | The frame for the entire system. "Trinity" = three-layer architecture (Body/Kernel/Protocol) *and* the three stakeholders (Learner × Instructor × Institution). "ID" = Instructional Design — the discipline. "AI OS" = the AI is the operating system, not an add-on. |
| **Architecture** | Workspace root. `trinity` (Axum server), `trinity-protocol` (shared types), `trinity-quest` (quest engine). |
| **Status** | 🟢 |

---

### PEARL

| | |
|---|---|
| **Stands For** | **P**erspective **E**ngineering **A**esthetic **R**esearch **L**ayout |
| **Pedagogy** | The focusing agent. Every instructional designer starts with a *pearl of wisdom* — the thing they know that the world doesn't. PEARL captures the SME's subject, the delivery medium, and their vision for how the output should *feel*. It's the alignment document that every ADDIECRAPEYE phase checks against: "Are we still building what the user intended?" Without PEARL, scope creep wins. |
| **Architecture** | `trinity-protocol/src/pearl.rs` — `Pearl`, `PearlMedium`, `PearlPhase`, `PearlEvaluation`. API: `GET/POST /api/pearl`, `PUT /api/pearl/refine`. Frontend: `PearlCard.jsx`. |
| **Status** | 🟢 |

---

### ADDIECRAPEYE

| | |
|---|---|
| **Stands For** | **A**nalyze, **D**esign, **D**evelop, **I**mplement, **E**valuate + **C**ontrast, **R**epetition, **A**lignment, **P**roximity + **E**nvision, **Y**oke, **E**volve |
| **Pedagogy** | The 12-station instructional design lifecycle. ADDIE (stations 1–5) is the classic ID framework. CRAP (stations 6–9) borrows from Robin Williams' visual design principles applied to learning artifacts. EYE (stations 10–12) adds meta-reflection, coupling, and evolution. Together: **Extract** the wisdom (ADDIE) → **Place** it in design (CRAP) → **Refine** it through reflection (EYE). Each station maps to a Bloom's Taxonomy level, ensuring cognitive scaffolding spirals upward. |
| **Architecture** | `conductor_leader.rs` — `AddiecrapeyePhase` enum (12 variants). `trinity-quest/src/hero.rs` — `Phase` enum. Conductor system prompts keyed to phase. |
| **Status** | 🟢 |

---

### P-ART-Y

| | |
|---|---|
| **Stands For** | **P**ete + **ART** (tools, no persona) + **Y**ou (the Yardmaster) |
| **Pedagogy** | The AI is not one agent — it's a *party* of three roles. **P** (Pete) is the sole AI personality — Socratic mentor, conductor, the only character. **ART** is pure tooling with no persona: ComfyUI (images), Qianfan-OCR (document intelligence), Kokoro TTS (voice), Hunyuan3D (mesh), beast_logger (telemetry), and the archived Bevy desktop engine. **Y** (You) is the user — the Yardmaster who directs everything. The structure teaches that AI is a tool, not a peer — Pete guides, ART executes, You decide. |
| **Architecture** | `conductor_leader.rs` — `manage_hotel_sidecars()` maps phases to ART tools. `creative.rs` — ComfyUI integration (1,156 lines). `ArtStudio.jsx` — beast_logger + image/music/video controls. Frontend: party member badges in `GameHUD.jsx`. |
| **Status** | 🟢 |

---

### VAAM

| | |
|---|---|
| **Stands For** | **V**ocabulary **A**s **A** **M**echanism |
| **Pedagogy** | Words are the bridge between humans and AI. VAAM treats vocabulary as the *game mechanic* — every word the user types and the AI responds with is measured, tracked, and scaffolded. VAAM connects: Semantic Attention (did the AI notice the user's word?), User Preference (which communication style resonates?), and Vocabulary Mastery (is the user growing their professional lexicon?). The insight: *if you master the words, you master the field*. |
| **Architecture** | `trinity-protocol/src/vaam_profile.rs` — `VaamProfile`, `Agreement`, `WordWeight`. `trinity/src/vaam_bridge.rs` — `VaamBridge`. `trinity/src/vaam.rs` — `VaamState`. |
| **Status** | 🟢 |

---

### IRON ROAD

| | |
|---|---|
| **Stands For** | **I**nstructional **R**esource for **O**ngoing **N**arrative — **R**eflective **O**utcome **A**rchives as **D**esign |
| **Pedagogy** | The railroad metaphor for the learning journey. The user is an Operator on a steam locomotive. Coal = energy/attention. Steam = cognitive focus. Drive Wheels = discipline. The Iron Road *is* the Hero's Journey — 12 chapters where the user builds their instructional product while living the narrative. Railroad metaphor makes abstract ID concepts tangible: scope creep = derailment, momentum = steam pressure, evaluation = signal towers. |
| **Architecture** | `trinity-iron-road` crate — narrative engine, game loop. `trinity/src/narrative.rs`. Frontend: `ChapterRail.jsx` (left rail), `PhaseWorkspace.jsx` (center: objectives + chat + advance), `TrainStatus.jsx` (coal/steam meters). |
| **Status** | 🟢 |

---

### QUEST

| | |
|---|---|
| **Stands For** | **Q**uality-driven **U**ser **E**xperience through **S**tructured **T**asks |
| **Pedagogy** | Each ADDIECRAPEYE station is a quest chapter. Quests have objectives (3 per chapter), XP rewards, and a Hero's Journey arc. The quest system makes *invisible* instructional design steps *visible* and *rewardable*. The user doesn't feel like they're "following ADDIE" — they feel like they're on an adventure where each objective builds their real product. |
| **Architecture** | `trinity-quest/src/state.rs` — `QuestState`, `GameState`. `trinity-quest/src/quest_system.rs` — `objectives_for_chapter()`. API: `GET /api/quest`, `POST /api/quest/advance`. |
| **Status** | 🟢 |

---

### BOOK (of the Bible)

| | |
|---|---|
| **Stands For** | **B**iographical **O**utcome **O**rganizer — **K**nowledge ledger |
| **Pedagogy** | The Book records *why* things happened, not just *what*. It's the append-only ledger of the user's learning journey — every quest completed, every scope creep defeated, every vision refined. Each user's journey is a *book* in the larger *Bible* of the system. The Great Recycler (NPU) continuously summarizes the Book for Pete to reference. |
| **Architecture** | `trinity/src/book_of_the_bible.rs` — `BookOfTheBible`, `BookChapter`. API: `GET /api/book`, `GET /api/book/stream` (SSE). Persisted to `docs/books_of_the_bible/`. |
| **Status** | 🟢 |

---

### SCOPE CREEP

| | |
|---|---|
| **Stands For** | **S**elf-**C**reating **O**bstacle **P**atterns **E**xpanding — **C**ognitive **R**esource **E**rosion through **E**xcess **P**riorities |
| **Pedagogy** | The enemy. Scope creep is the natural tendency for projects to grow beyond original intent — the #1 killer of ID projects. Trinity gamifies it: scope creep becomes *literal monsters* (Semantic Creeps) that the user must tame or banish. Each Creep is a vocabulary word encountered in the wild. Taming requires multi-dimensional learning (Pythagorean Taming): encountering the word across multiple phases, contexts, and deliberate choices. Untamed Creeps drain Coal. |
| **Architecture** | `trinity-protocol/src/semantic_creep.rs` — `SemanticCreep`, `CreepState`, `CreepStats`. `trinity-protocol/src/profile.rs` — `Bestiary`. Frontend: `CreepCard.jsx`. API: `GET /api/bestiary`, `POST /api/bestiary/tame`. |
| **Status** | 🟢 |

---

### CHARACTER SHEET

| | |
|---|---|
| **Stands For** | *(No acronym — the tabletop RPG metaphor is the meaning)* |
| **Pedagogy** | The user's persistent identity across projects. Your Character Sheet is WHO you are, not WHAT you're building. It captures: Intent Posture (Mastery vs. Efficiency), Vulnerability Level (how much scaffolding the AI provides), User Class (InstructionalDesigner, GameDesigner, etc.), and Skills. It answers: "Who is sitting at this keyboard, and how do they want to grow?" |
| **Architecture** | `trinity-protocol/src/character_sheet.rs` — `CharacterSheet`, `IntentPosture`, `UserClass`. API: `GET/POST /api/character`. Saved to `~/.config/trinity/character_sheet.json`. |
| **Status** | 🟢 |

---

### CRAP (within ADDIECRAPEYE)

| | |
|---|---|
| **Stands For** | **C**ontrast, **R**epetition, **A**lignment, **P**roximity |
| **Pedagogy** | Robin Williams' four visual design principles, repurposed for instructional artifacts. Contrast = emphasis ranking. Repetition = core loop solidity. Alignment = scope pruning. Proximity = UX grouping (Miller's Law, 7±2). Stations 6–9: where the user *places* their wisdom into designed artifacts. |
| **Architecture** | `AddiecrapeyePhase::Contrast` / `::Repetition` / `::Alignment` / `::Proximity`. Each has a Bloom's level and conductor system prompt. |
| **Status** | 🟢 |

---

### EYE (within ADDIECRAPEYE)

| | |
|---|---|
| **Stands For** | **E**nvision, **Y**oke, **E**volve |
| **Pedagogy** | The final reflective pass. Envision = "does this match my original vision?" Yoke = couple frontend to backend, bind form to function. Evolve = ship it, give it breath. After building the Golem (ADDIE = body, CRAP = design), the EYE opens — it sees, then it moves, then it grows. |
| **Architecture** | `AddiecrapeyePhase::Envision` / `::Yoke` / `::Evolve`. Bloom's: Evaluate → Create → Create. |
| **Status** | 🟢 |

---

### SACRED CIRCUITRY

| | |
|---|---|
| **Stands For** | *(Metaphorical — not an acronym)* |
| **Pedagogy** | The 15 foundation vocabulary words every ID practitioner must internalize. 4 quadrants (Scope, Build, Listen, Ship) mapped to VAAM. These words are non-negotiable — you can't do ID without mastering them. "Sacred" because the circuitry cannot be bypassed. |
| **Architecture** | `trinity-protocol/src/sacred_circuitry.rs` — `Circuit`, `CircuitQuadrant`, `foundation_vocabulary()`. 15 words loaded into VAAM on startup. |
| **Status** | 🟢 |

---

### The Relationship Chain

```
WHO the user is           → CHARACTER SHEET (persistent identity)
WHAT they're building     → PEARL (per-project focus)
HOW they build it         → ADDIECRAPEYE (12-station lifecycle)
WHO helps them            → P-ART-Y (AI party + user as Yardmaster)
WHAT words connect them   → VAAM (vocabulary as the bridge)
WHAT tries to stop them   → SCOPE CREEP (gamified obstacles)
WHERE the journey lives   → IRON ROAD (narrative frame)
WHAT they accomplish      → QUEST (structured tasks with XP)
WHAT they remember        → BOOK (biographical outcome ledger)
```

*Every concept feeds the next. Remove one and the chain breaks.*

---

### The Three Dimensions

Trinity's UI maps to three dimensions of experience:

| Dimension | Name | What | Game Role | Architecture |
|---|---|---|---|---|
| **1D** | Audio | Pete narrates — Great Recycler storytelling, 8K Ask Pete | Audiobook companion | Voice server `:7777` (openwakeword + Whisper + Kokoro) |
| **2D** | Book | LitRPG game — the Iron Road as a playable book | Phase forms, objectives, word physics, Pete chat | React + `PhaseWorkspace.jsx` + `iron-road-physics` |
| **3D** | Yard | Build sandbox — student becomes Yardmaster | 3D entity editing, game preview, Bevy studio | Bevy WASM in `<canvas>` + `templates/first-game/` |

```
1D(Audio) feeds → 2D(Book) narrates → 3D(Yard) creates
Pete speaks     → Player reads/plays → Yardmaster builds
```

**Dimensional transitions map to ADDIECRAPEYE:**
- **ADDIE (stations 1–5)** → Book 2D: Fill out instructional design forms, guided by Pete
- **CRAP (stations 6–9)** → Book 2D: Design artifacts, apply visual principles
- **EYE (stations 10–12)** → Yard 3D: **Envision → Yoke → Evolve** in the sandbox

The EYE is the portal from reader to builder. *Student becomes the master — the Yardmaster.*

---

### LOCOMOTIVE (Cognitive Load Physics)

| | |
|---|---|
| **Stands For** | *(Metaphorical — the steam engine IS the learner)* |
| **Pedagogy** | The Locomotive is the player's cognitive state rendered as a steam engine. Coal = motivation (finite fuel). Steam = germane cognitive load (active processing output). Velocity = learning rate. Friction = extraneous load (bad design/unclear instructions). Mass = intrinsic load (difficulty of content). The equation `Velocity = (Power + Steam) / (Mass × Friction)` is literally Cognitive Load Theory as physics. When the user completes objectives, coal burns → steam rises → velocity increases. When they encounter poorly designed content (high friction), the train slows. |
| **Architecture** | `archive/iron-road-physics/src/lib.rs` — `Train`, `Node`, `CognitiveLoad`, `calculate_velocity()`. Frontend: `TrainStatus.jsx` (coal/steam/velocity bars with color thresholds). Quest state: `coal_used`, `steam_generated` in `QuestState`. |
| **Status** | 🟢 |
