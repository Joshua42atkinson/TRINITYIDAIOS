# 📖 THE HOOK BOOK
### *A Spell Book for the Instructional Designer*
> **The PEARL inside the PEARL** — everything Trinity can do, everything it can become.

---

## What Is a Hook?

A **Hook** is a capability that Trinity provides to the user — a tool, a workflow, a system behavior that transforms *intent* into *product*. Each Hook is a spell the user can cast through Story Mode, Iron Road, ART Studio, or the Yardmaster.

Hooks are organized by **School** (domain of application) and **Tier** (current readiness):

| Tier | Meaning |
|------|---------|
| 🟢 **Cast** | Working now. Verified in production. |
| 🟡 **Scribed** | Architecture exists. Needs wiring or polish. |
| 🔴 **Prophesied** | Designed. Waiting for implementation. |

---

## 🏫 School of Pedagogy — *The Teacher's Arsenal*

| Hook | Tier | What It Does |
|------|:----:|-------------|
| **Socratic Interview** | 🟢 | Great Recycler asks WHY before you build. Forces reflection. Produces structured SME transcripts. |
| **12-Station Quest** | 🟢 | ADDIECRAPEYE framework as a playable quest. Each station = one ID phase with objectives and deliverables. |
| **Bloom's Extraction** | 🟢 | Every AI response is tagged with Bloom's taxonomy level. System tracks cognitive progression. |
| **Scope Creep Combat** | 🟢 | Out-of-scope ideas spawn as Bestiary entries. Scout (hope) tames them. Sniper (nope) bags and tags. |
| **Quality Scorecard** | 🟢 | Real-time alignment scoring against QM, AECT, and IBSTPI standards. Live dashboard, not a checkbox. |
| **PEARL Review** | 🟢 | 5-dimension quality gate: Purpose, Evidence, Alignment, Rigor, Learner-centricity. No phase advances without it. |
| **Design Doc Export** | 🟢 | One-click export of the entire project as a structured design document. Markdown + metadata. |
| **CLT Engine** | 🟢 | Cognitive Load Theory management: reduces extraneous load (clean UI), manages intrinsic load (scaffolded phases), maximizes germane load (real learning). |
| **Express Wizard** | 🟢 | 10-minute lesson plan generator. Subject → objectives → activities → assessment. Fast path for working professionals. |
| **Adaptive Difficulty** | 🟡 | Phase complexity adjusts based on user performance. Struggling users get more scaffolding. Advanced users get challenge tasks. |
| **Peer Review Mode** | 🔴 | Multiple students review each other's PEARLs. Cross-pollination of design thinking. Multiplayer pedagogy. |
| **Competency Mapping** | 🔴 | Auto-map deliverables to institutional competency frameworks (AECT, IBSTPI, QM, custom). |

---

## 🎨 School of Creation — *The ART Studio*

| Hook | Tier | What It Does |
|------|:----:|-------------|
| **Image Generation** | 🟢 | SDXL via ComfyUI. Text → image for course materials, presentations, game assets. |
| **Music Composition** | 🟢 | MusicGPT. Text → original music for learning modules, game soundtracks, presentations. |
| **Video Generation** | 🟡 | Hunyuan Video. Text/image → short-form video for instructional content. |
| **3D Asset Generation** | 🟡 | Hunyuan3D. Text → 3D models for VR/XR educational environments. |
| **Voice Narration** | 🟢 | Supertonic-2 TTS, native ONNX. 10 voices. Real-time narration of Pete's responses. |
| **Asset Pipeline** | 🟢 | All creative outputs stored in the local asset library. Reusable across projects. |
| **Bevy Game Scaffold** | 🟡 | Generate a working Bevy game project from instructional design data. Course → game. |
| **VR/XR Scene Builder** | 🔴 | Generate immersive VR/XR educational environments from design documents. The endgame. |
| **Interactive Simulation** | 🔴 | Bevy-powered simulations that teach through play. Physics, chemistry, history — any domain. |

---

## ⚙️ School of Systems — *The Yardmaster's Workshop*

| Hook | Tier | What It Does |
|------|:----:|-------------|
| **30 Agentic Tools** | 🟢 | File I/O, shell execution, web search, image generation, quest management, journal, and more. Pete can DO things, not just talk. |
| **Model Switching** | 🟢 | Hot-swap LLM models from the Yardmaster. Mistral, Llama, Qwen — whatever fits the task. |
| **Vector Database** | 🟢 | Semantic search across all user-generated content. Your knowledge graph grows with you. |
| **Context Window (500K+)** | 🟢 | Mistral Small 4 119B with massive context. Entire textbooks fit in one conversation. |
| **Local Inference (40+ t/s)** | 🟢 | 119B parameter model running at 40+ tokens/second on consumer hardware. No cloud. |
| **Edge Guard** | 🟢 | Kernel-level security middleware. Route-by-route access control. Red Hat security posture. |
| **CowCatcher** | 🟢 | Input sanitization engine. Blocks prompt injection, path traversal, and code execution attacks. |
| **Journal System** | 🟢 | Timestamped, phase-tagged entries. The student's learning log is the system's training data. |
| **Multi-Model Routing** | 🟡 | Route different tasks to different models. Reflection → large model. Code → specialized model. |
| **Plugin Architecture** | 🔴 | Users build custom tools and hooks. The Yardmaster is an IDE for building IDE capabilities. |
| **Classroom Orchestrator** | 🔴 | Professor dashboard. View all student progress, send prompts, review PEARLs. vLLM multi-user. |
| **Corporate Presenter** | 🔴 | Export any design document as a presentation deck. Bevy-powered slide system with live AI narration. |

---

## 🎭 School of Identity — *The Character Sheet*

| Hook | Tier | What It Does |
|------|:----:|-------------|
| **Character Sheet** | 🟢 | Living portfolio. Stats, artifacts, progression. The student's professional identity. |
| **XP Economy** | 🟢 | Experience points from completed objectives, tamed creeps, and PEARL reviews. Real progression. |
| **Steam/Coal Resources** | 🟢 | RLHF-driven resource economy. Good decisions generate steam (momentum). Bad ones burn coal (learning fuel). Both are valuable. |
| **Ghost Train Detection** | 🟢 | Shadow Status system detects imposter syndrome patterns and intervenes with metacognitive scaffolding. |
| **Achievement Badges** | 🟡 | Phase completion, creep taming, PEARL quality — each tracked and displayed. |
| **Graduation Protocol** | 🔴 | When the student completes the Iron Road, they graduate to the Yardmaster. The textbook becomes their IDE. |
| **Professional Portfolio Export** | 🔴 | One-click export of the Character Sheet as a professional portfolio website. Show employers what you built. |

---

## 🚀 The Endgame — What Trinity Scales To

```
TODAY (v1.0 — Single User, Local)
├── One student, one machine, one AI mentor
├── 194K LOC Rust, 264 tests, 73 API endpoints
└── Fully functional prototype, zero cloud dependencies

THIS YEAR (v2.0 — Multi-User, Institutional)
├── vLLM backend → multiple students, one server
├── Professor dashboard → classroom orchestration
├── Competency mapping → institutional accreditation support
└── Peer review → multiplayer pedagogy

NEXT YEAR (v3.0 — VR/XR Education)
├── Bevy game engine → immersive learning environments
├── VR/XR scene builder → spatial education design
├── AI-generated simulations → learn by playing
└── Corporate training → boardroom to classroom, one system

THE VISION (v∞ — The Living Textbook)
├── Every student gets Trinity on their machine
├── Every teacher customizes it with the Yardmaster
├── Every institution runs it behind their own walls
├── The textbook APPRECIATES — it gets smarter, richer, more valuable
└── The student OWNS it — forever. Local-first. Yours.
```

---

## How the Hook Book Lives

The Hook Book is not static. It is the **Player's Handbook appendix** — a living catalog that grows as Trinity grows. Each Hook is a node in the vector database, meaning Pete can *discuss* any Hook, *plan* implementations using Hooks, and *combine* Hooks into workflows.

When a user opens Story Mode and says:
> *"I want to build a VR chemistry lab for my 9th graders"*

Pete activates:
1. **Socratic Interview** (School of Pedagogy) — WHY this lab? What's the learning objective?
2. **12-Station Quest** (School of Pedagogy) — Walk through ADDIECRAPEYE to design it properly
3. **3D Asset Generation** (School of Creation) — Generate molecular models
4. **Bevy Game Scaffold** (School of Systems) — Build the VR environment
5. **Quality Scorecard** (School of Pedagogy) — Validate against QM standards
6. **Design Doc Export** (School of Systems) — Ship the documentation
7. **Character Sheet** (School of Identity) — Record the XP, update the portfolio

Seven Hooks, one conversation, one product. That's the vision.

---

## 👻 Ghost Train Integration — Imposter Syndrome as a System Event

The **Ghost Train** is Trinity's Shadow Status detector. When a user's behavior patterns suggest imposter syndrome — hesitation, self-deprecation, abandoning work — the system responds with data, not platitudes.

**How the Hook Book fights the Ghost Train:**

```
User pattern detected: "I can't do this" + 3 abandoned objectives
  → Ghost Train Alert activated
    → Pete pulls the user's Hook casting history from the vector DB
      → "You've cast 11 Hooks across 3 Schools this session.
         You completed a Socratic Interview, exported a Design Doc,
         and tamed 2 scope creeps. That's not imposter syndrome.
         That's a practitioner at work."
```

The Hook Book becomes **evidence against the ghost**. Every Hook cast is a logged competency demonstration. The system turns the user's own work history into metacognitive ammunition:

| Ghost Train Signal | Hook Book Response |
|---|---|
| "I'm not good enough" | "You've cast Quality Scorecard 4 times with rising scores" |
| "This is too complex" | "You successfully chained 7 Hooks in your last session" |
| "I don't belong here" | "Your Character Sheet shows 3 PEARLs at Distinction level" |
| Abandoning objectives | "The Graveyard has 12 artifacts — each one taught you something" |

---

## 🪦 The Graveyard — Where Old Hooks Become New Hooks

The **Archive Graveyard** is not a cemetery. It's a **composting system**. Every completed, failed, or abandoned artifact goes to the Graveyard, where it becomes raw material for future Hook chains. This is the Great Recycler's domain.

```
Lifecycle of a Hook Casting:

  📖 User selects Hook (e.g., "Socratic Interview")
    → Pete guides the casting through Story Mode
      → Artifact produced (design doc, assessment, lesson plan)
        → Artifact stored in Vector DB with full metadata
          → Quest advances, XP awarded, Character Sheet updated

  If the artifact is later replaced by a better version:
    → Old version moves to the Graveyard
      → Vector DB retains the embedding
        → Great Recycler can reference it:
          "Last time you designed a rubric for this topic,
           you struggled with criterion alignment. Let's
           address that first this time."
```

**Graveyard + Hook Book = Institutional Memory**

The Graveyard answers: *"What has this user tried before?"*
The Hook Book answers: *"What can this user try next?"*

Together, they give Pete the ability to say:
> *"Based on your 6 completed PEARLs (Graveyard) and your current quest phase (Iron Road), I recommend casting the Express Wizard (Hook Book) to generate a quick assessment, then using the Quality Scorecard to validate it before you present."*

---

## 🚂 Agentic Quest System — Hook Chains for Long-Horizon Tasks

This is the architectural heart of Trinity's agentic capability. A **Hook Chain** is an ordered sequence of Hook castings that Pete plans and executes across multiple sessions.

### How It Works

```
User: "I need to build a complete online course for my department"

Pete's internal planning (Great Recycler, Slot 0):
  1. Check Hook Book → which Hooks match this task?
  2. Check Graveyard → has this user done similar work?
  3. Check Quest State → which ADDIECRAPEYE phase are we in?
  4. Generate Hook Chain → ordered sequence of castings

Planned Hook Chain:
  ┌─────────────────────────────────────────────┐
  │ STATION 1: ANALYZE                          │
  │   → Socratic Interview (School of Pedagogy) │
  │   → Vector DB search for prior work         │
  ├─────────────────────────────────────────────┤
  │ STATION 2: DESIGN                           │
  │   → 12-Station Quest (phase 2)              │
  │   → Bloom's Extraction on objectives        │
  │   → Scope Creep Combat (boundary setting)   │
  ├─────────────────────────────────────────────┤
  │ STATION 3: DEVELOP                          │
  │   → Express Wizard (lesson scaffolds)       │
  │   → Image Generation (visuals)              │
  │   → Voice Narration (multimedia)            │
  ├─────────────────────────────────────────────┤
  │ STATION 4: IMPLEMENT                        │
  │   → Bevy Game Scaffold (interactive sim)    │
  │   → Design Doc Export (deliverable)         │
  ├─────────────────────────────────────────────┤
  │ STATION 5: EVALUATE                         │
  │   → Quality Scorecard (QM alignment)        │
  │   → PEARL Review (5-dimension gate)         │
  │   → Character Sheet update (XP + portfolio) │
  └─────────────────────────────────────────────┘
```

### Hook Chain Storage

Successful Hook Chains are stored in the vector database as **recipes** — reusable patterns that Pete can recommend to future users (or the same user in a new context):

| Recipe | Hooks Used | Domain |
|--------|:----------:|--------|
| "Quick Lesson Plan" | Socratic → Express Wizard → Export | K-12 |
| "Full Course Build" | Socratic → 12-Station → Image Gen → Bevy → PEARL | Higher Ed |
| "Corporate Training Module" | Socratic → Scorecard → Voice → Export | Corporate |
| "VR Lab Experience" | Socratic → 3D Gen → Bevy → Simulation → PEARL | STEM |

### The Scout Sniper Pattern

During any Hook Chain, scope creep is managed by the **Scout Sniper** class:

```
Mid-chain, user says: "Oh, we should also add a VR field trip!"

  → Great Recycler detects scope expansion
    → Scope Creep spawns in Bestiary
      → Pete activates Scout Sniper:
        SCOUT (hope): "That's a 🟡 Scribed Hook (VR/XR Scene Builder).
                       We can tame it — add it as a Phase 2 goal."
        SNIPER (nope): "That's out of scope for this sprint.
                        Bagged and tagged in the Graveyard for later."

  → User decides → Hook Chain adjusts
    → Long-horizon task stays on track
```

This is how Trinity manages **long-horizon agentic tasks** without losing coherence: the Hook Book provides the vocabulary, the Quest System provides the structure, the Graveyard provides the memory, and the Ghost Train provides the safety net.

---

> *"The Hook Book is to Trinity what a spell book is to a wizard — it doesn't just list what you can do. It reveals what you can BECOME."*

---

**TRINITY** — *Textbook · Reflective · Instructional · Narrative · Intelligence · Technology — Yours*

*The Iron Road awaits. Choose your Hook.*
