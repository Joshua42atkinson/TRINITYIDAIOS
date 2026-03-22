# Trinity Architecture Bible
**Document:** 01-ARCHITECTURE.md  
**Purpose:** Philosophy, patterns, ADDIE-C-R-A-P-E-Y-E framework  
**Lines:** ~500  
**Isomorphic to:** [02-IMPLEMENTATION.md §7](02-IMPLEMENTATION.md), [03-OPERATIONS.md §5](03-OPERATIONS.md)

---

## 1. Core Philosophy: "Hotel Management"

Trinity operates out of a single **128GB Unified Memory "Hotel"**. Running massive LLMs (100B+ parameters) simultaneously will crash the hotel (Out of Memory). Therefore, Trinity uses a strict **"Rotating Star" (One-in, One-out) policy** governed by the **Conductor**.

### 1.1 The Hotel Metaphor

| Hotel Element | Trinity Equivalent | Why It Matters |
|---------------|-------------------|----------------|
| **Lobby** | Conductor Agent (Mistral Small 4) | First point of contact, directs traffic |
| **Guest Rooms** | Model slots (2 concurrent max) | Limited by 128GB LPDDR5X |
| **Check-in/Check-out** | Sidecar start/stop API calls | One model must leave before another enters |
| **Room Service** | Tool system (7 tools) | Agents can request files, shell commands |
| **Concierge** | RAG system (PostgreSQL + pgvector) | Retrieves context from knowledge base |
| **Housekeeping** | NPU Great Recycler | Background book updates, continuous learning |

### 1.2 Why "Hotel" and Not "Swarm"?

Early designs considered a "swarm" of agents all active simultaneously. This fails on 128GB hardware:
- Nemotron-120B (Conductor): ~72GB
- Step-Flash-121B (Engineer): ~75GB  
- Crow-9B (Researcher): ~6GB

**Total: 153GB > 128GB available = CRASH**

The Hotel pattern ensures **graceful degradation**: if a model can't load, the Conductor queues the request and explains to the user.

### 1.3 Rotating Star Policy

```
┌─────────────────────────────────────┐
│     128GB Unified Memory Hotel      │
├─────────────────────────────────────┤
│  Room 1: Conductor (always booked) │
│         Mistral Small 4 | 14GB          │
├─────────────────────────────────────┤
│  Room 2: [Vacant] or [Sidecar]      │
│         Engineer/Artist/Evaluator   │
│         15-36GB depending on role    │
├─────────────────────────────────────┤
│  Queue: Waiting models              │
│         (managed by Conductor)      │
└─────────────────────────────────────┘
```

**Rule:** Room 2 can only hold ONE model at a time. To swap:
1. Conductor calls `sidecar_stop` (unload current)
2. Wait for `/health` to return 503 (confirmed down)
3. Conductor calls `sidecar_start --role [new_role]`
4. Wait for `/health` 200 OK
5. Route request to new model

---

## 2. The Durability Layer

Agents do not hold long-term state in active RAM. The system is a **"Blockchain of Markdown files"**.

### 2.1 Persistence Strategy

| What | Where | Format | Why |
|------|-------|--------|-----|
| Quest progress | PostgreSQL | SQL + JSONB | ACID compliance, queryable |
| Conversation | PostgreSQL | chat_history table | Context for RAG |
| Knowledge base | PostgreSQL + pgvector | 384-dim embeddings | Semantic search |
| Books of the Bible | `docs/books_of_the_bible/*.md` | Markdown | Human-readable, versioned |
| Character sheet | `~/.trinity/character_sheet.json` | JSON | User preferences |
| Configuration | `configs/*.toml` | TOML | Human-editable |
| Code artifacts | Git repository | Rust/Python | Durable, diffable |

### 2.2 Why Not In-Memory State?

If the Conductor crashes (OOM, panic, hardware failure):
- **In-memory state:** LOST. User must restart quest from scratch.
- **Durability layer:** PRESERVED. New Conductor instance loads from PostgreSQL, resumes exactly where user left off.

**Principle:** "Treat every agent as ephemeral. The system is the source of truth."

---

## 3. Three-Layer Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 3: BODY / UI                                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐ │
│  │   Browser   │  │ Bevy App    │  │   PersonaPlex Voice     │ │
│  │  (Today)    │  │ (Future)    │  │   (Ask Pete)            │ │
│  │ index.html  │  │ trinity-body│  │   Audio-to-Audio        │ │
│  │ book.html   │  │ Dockable    │  │   <200ms latency        │ │
│  │ dev.html    │  │ workspace   │  │                         │ │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘ │
│  Purpose: Human interface, LitRPG immersion, spatial computing  │
├─────────────────────────────────────────────────────────────────┤
│  LAYER 2: AI KERNEL (The "Brain")                              │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  Conductor Agent (Mistral Small 4) — ALWAYS LOADED              ││
│  │  • Orchestrates P-ART-Y roles                              ││
│  │  • Routes requests to appropriate sidecar                  ││
│  │  • Manages Hotel check-in/check-out                        ││
│  └─────────────────────────────────────────────────────────────┘│
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌───────────┐│
│  │ Engineer    │ │ Evaluator   │ │  Artist     │ │ Brakeman  ││
│  │ ⚙️ 36GB     │ │ 📊 21GB     │ │  🎨 21GB    │ │ 🛡️ 15GB   ││
│  │ (Sword &    │ │ (QM/       │ │  (GDDs,     │ │ (Tests,   ││
│  │  Shield)    │ │  WCAG)      │ │  Wireframes)│ │  Security)││
│  │ PORT 8082   │ │ PORT 8090   │ │  PORT 8091  │ │ PORT 8092 ││
│  └─────────────┘ └─────────────┘ └─────────────┘ └───────────┘│
│  ┌─────────────┐ ┌─────────────┐                              │
│  │ Pete        │ │ Visionary   │                              │
│  │ 🎓 21GB     │ │ 👁️ 21GB     │                              │
│  │ (Socratic)  │ │ (Vision)    │                              │
│  │ PORT 8093   │ │ PORT 8094   │                              │
│  └─────────────┘ └─────────────┘                              │
│  Purpose: AI reasoning, code generation, creative work        │
├─────────────────────────────────────────────────────────────────┤
│  LAYER 1: HEADLESS SERVER                                      │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  trinity (Axum HTTP on :3000)                               ││
│  │  • Static file serving (index.html, book.html, etc.)        ││
│  │  • API routes (/api/chat, /api/quest/*)                      ││
│  │  • SSE streaming for real-time updates                     ││
│  │  • RAG integration (PostgreSQL + pgvector)                   ││
│  │  • 7 Agentic Tools (read_file, shell, search_files, etc.)   ││
│  └─────────────────────────────────────────────────────────────┘│
│  Purpose: HTTP API, static hosting, database gateway           │
└─────────────────────────────────────────────────────────────────┘
```

### 3.1 Layer Responsibilities

| Layer | Crate | Always Running? | Hardware |
|-------|-------|-----------------|----------|
| 1 | trinity | ✅ Yes | Any (low resource) |
| 2a | Conductor | ✅ Yes | Strix Halo (128GB) |
| 2b | Sidecars | ❌ On-demand | Strix Halo (128GB) |
| 3 | Browser/Bevy | ✅ Yes (when active) | Any with GPU |

### 3.2 Communication Flow

```
User (Browser) → HTTP POST /api/chat → trinity (Layer 1)
                                      ↓
                              Routes to Conductor (Layer 2)
                                      ↓
                              Classifies intent
                                      ↓
                    ┌─────────────────┼─────────────────┐
                    ↓                 ↓                 ↓
               If coding         If creative       If evaluation
                    ↓                 ↓                 ↓
           Start Engineer      Start Artist       Start Evaluator
           (swap sidecar)      (swap sidecar)     (swap sidecar)
                    ↓                 ↓                 ↓
              Return result ← All routes return here
                    ↓
           Update PostgreSQL (durability)
                    ↓
           SSE broadcast to UI
```

---

## 4. ADDIE-C-R-A-P-E-Y-E Framework

Trinity extends the classic ADDIE instructional design model with 7 additional phases forming the acronym **ADDIECRAPEYE**.

### 4.1 The 12 Phases

| # | Phase | Letter | Instructional Purpose | Trinity Implementation | P-ART-Y Agent | Bloom's Level |
|---|-------|--------|----------------------|------------------------|---------------|---------------|
| 1 | **A**nalysis | A | Understand learners, needs, context | Hardware scan → "Mana"/"Agility" assignment | Evaluator (56GB) | Remember/Understand |
| 2 | **D**esign | D | Define objectives, structure, flow | Game Design Document generation | Artist (15GB) | Apply |
| 3 | **D**evelop | V | Create content, assets, code | Bevy ECS code generation | Engineer (36GB) | Analyze |
| 4 | **I**mplementation | I | Deploy, teach, facilitate | Running the built game, testing | Engineer (36GB) | Evaluate |
| 5 | **E**valuation | E | Assess learning, measure outcomes | QM rubric compliance, testing | Evaluator (56GB) | Create |
| 6 | **C**ontrast | C | Compare alternatives, A/B test | Model comparison, approach evaluation | Researcher (9GB) | Analyze |
| 7 | **R**epetition | R | Spiral curriculum, spaced practice | Quest replay, skill reinforcement | Pete (21GB) | Apply |
| 8 | **A**lignment | A | Map to standards, alignment check | IBSTPI, WCAG, Bloom's alignment | Evaluator (56GB) | Evaluate |
| 9 | **P**roximity | P | Just-in-time learning, context-aware | RAG retrieval, moment-of-need | Pete (21GB) | Understand |
| 10 | **E**nvision | E | Future scenarios, forecasting | "What if" expansion planning | Artist (15GB) | Create |
| 11 | **Y**oke | Y | Connect concepts, interdisciplinary | Cross-domain integration | Engineer (36GB) | Analyze |
| 12 | **E**volve | E | Iterate, improve, adapt | Continuous improvement cycle | Evaluator (56GB) | Create |

### 4.2 Phase Detail: Hero's Journey Mapping

Each ADDIE-C-R-A-P-E-Y-E phase maps to Joseph Campbell's 12-stage Hero's Journey:

```
ACT I: DEPARTURE (Chapters 1-5)
├── Ch 1: ORDINARY WORLD → Analysis (A)
│   └── Default state: Teacher with no game dev skills
│   └── ADDIE: Analyze current reality
│
├── Ch 2: CALL TO ADVENTURE → Design (D)  
│   └── Discovery: "You can build games from lessons"
│   └── ADDIE: Design the transformation
│
├── Ch 3: REFUSAL OF THE CALL → Development (V)
│   └── Doubt, overwhelm, "Creeps" attack
│   └── ADDIE: Develop counter-measures
│
├── Ch 4: MEETING THE MENTOR → Implementation (I)
│   └── Great Recycler teaches ADDIE framework
│   └── ADDIE: Implement first lesson
│
└── Ch 5: CROSSING THE THRESHOLD → Evaluation (E)
    └── Commit to building (first ADDIE cycle)
    └── ADDIE: Evaluate readiness

ACT II: INITIATION (Chapters 6-9)
├── Ch 6: TESTS, ALLIES, ENEMIES → Contrast (C)
│   └── Build AI Party, face Creeps
│   └── CR: Compare approaches, contrast strategies
│
├── Ch 7: APPROACH TO THE INMOST CAVE → Repetition (R)
│   └── Deep design work, the hard part
│   └── AP: Repetition of ADDIE cycles, spaced practice
│
├── Ch 8: THE ORDEAL → Alignment (A)
│   └── First complete Bevy game build
│   └── PE: Align with standards, check quality
│
└── Ch 9: THE REWARD → Proximity (P)
    └── Working prototype: "It actually runs!"
    └── Y: Just-in-time celebration, proximity to goal

ACT III: RETURN (Chapters 10-12)
├── Ch 10: THE ROAD BACK → Envision (E)
│   └── Testing, evaluation, iteration
│   └── EN: Envision next version, future scenarios
│
├── Ch 11: THE RESURRECTION → Yoke (Y)
│   └── Final polish, accessibility, QM compliance
│   └── YO: Yoke together all elements
│
└── Ch 12: RETURN WITH THE ELIXIR → Evolve (E)
    └── Publish to consciousframework.com
    └── EV: Evolve the system, share the gift
```

### 4.3 Phase Icons and DC (Difficulty Class)

Each phase has a D20 "Difficulty Class" for skill checks:

| Phase | Icon | DC | Typical Check |
|-------|------|-----|---------------|
| Analysis | 🔍 | 10 | Logistics check (d20 + Traction ≥ 10) |
| Design | 🎨 | 15 | Inspiration check (d20 + Velocity ≥ 15) |
| Development | ⚙️ | 20 | Complexity check (d20 + Combustion ≥ 20) |
| Implementation | 🚀 | 25 | Deployment check (d20 + all stats ≥ 25) |
| Evaluation | 📊 | 30 | Mastery check (d20 + Resonance ≥ 30) |
| Contrast | ⚖️ | 18 | Comparison check |
| Repetition | 🔄 | 12 | Practice check |
| Alignment | 📐 | 22 | Standards check |
| Proximity | 🎯 | 14 | Context check |
| Envision | 🔮 | 24 | Foresight check |
| Yoke | 🔗 | 20 | Integration check |
| Evolve | 🌱 | 28 | Transformation check |

---

## 5. P-ART-Y Role System

The **P-ART-Y** (Party) is the ensemble of AI agents that accompany the user through their Hero's Journey.

### 5.1 Role Definitions

| Role | Icon | Model | Memory | Specialty | ADDIE Phases | Port |
|------|------|----------|--------|-----------|--------------|------|
| **Conductor** | 🚂 | Mistral Small 4 | 14GB | Orchestration, narrative, routing | All (coordinator) | 8080 |
| **Yardmaster** | ⚙️ | Ming-flash-omni-2.0 | 24GB | Code generation, Dev Mode, realtime UI | Develop, Implement, Yoke | 8082 |
| **Evaluator** | 📊 | Qwen2.5-Coder-32B (65K ctx) | 21GB | QM rubrics, WCAG audits, Bloom's alignment | Analysis, Evaluation, Alignment | 8090 |
| **Artist** | 🎨 | Qwen2.5-Coder-32B (32K ctx) | 21GB | GDDs, UI wireframes, 2D/3D/XR specs | Design, Envision | 8091 |
| **Brakeman** | 🛡️ | Qwen3-Coder-25B (16K ctx) | 15GB | Test generation, security audits, `cargo clippy/test` | Evaluation, safety checks | 8092 |
| **Pete** | 🎓 | Qwen2.5-Coder-32B (voice pending) | 21GB | Socratic dialogue, questions not answers | Analysis, Design, Evaluation, Repetition, Proximity | 8093 |
| **Visionary** | 👁️ | Qwen3.5-35B-A3B + mmproj | 21GB | Screenshot/UI/graphics evaluation, vision | Design, Envision | 8094 |

### 5.2 The Yardmaster Pattern (EYE)

Replacing the legacy "Sword & Shield Engineer", the **Yardmaster** operates exclusively on **Ming-flash-omni-2.0** as a single, omni-modal Dev Mode system. 
Ming Omni excels at modifying UX/UI elements in real-time based on code generation while minimizing the context switching latency and high VRAM overhead required by dual-model patterns. This perfectly aligns with the Strix Halo shared memory architecture.

### 5.3 Party Formation Rules

```rust
// From quests.rs — party system logic
pub struct PartyMember {
    pub id: String,
    pub name: String,
    pub role: String,
    pub model: String,
    pub memory_gb: f32,
    pub specialty: Vec<Phase>,  // Which ADDIE phases they excel at
    pub icon: String,
    pub available: bool,        // Unlocked through quest progression
    pub active: bool,           // Currently loaded in Room 2
}
```

**Rules:**
1. Conductor always active (Room 1)
2. Only ONE sidecar active at a time (Room 2)
3. Switching sidecars: unload → wait → load → wait → ready
4. Party members unlock through quest progression
5. Each member has specialty phases (Engineer = Dev/Impl/Yoke)

---

## 6. Iron Road Isomorphism

The **Iron Road** is Trinity's LitRPG narrative layer that teaches ADDIE through gameplay.

### 6.1 Core Isomorphism: Three Mappings

| Instructional Design | AI System | Organizational Structure |
|----------------------|-----------|--------------------------|
| ADDIE phases | Agent roles | Quest chapters |
| Bloom's taxonomy | Model capabilities | Skill trees |
| Learning objectives | Tool outputs | Game mechanics |

**The isomorphism means:** Learn one domain, you've learned all three.

### 6.2 LitRPG Mechanics

| Game Term | Instructional Equivalent | System Implementation |
|-----------|-------------------------|----------------------|
| **Resonance** | Pedagogical mastery level | Integer 1-12, unlocks chapters |
| **Coal** | Stamina/energy | Float 0-100, consumed by actions |
| **Steam** | Progress momentum | Generated by completing objectives |
| **Traction** | Analysis skill | d20 bonus for logistics checks |
| **Velocity** | Design skill | d20 bonus for inspiration checks |
| **Combustion** | Development skill | d20 bonus for complexity checks |
| **Creeps** | Scope creep monsters | Challenges that drain Coal |
| **Cargo Hold** | Working memory limit | 128GB metaphor for project scope |
| **Locomotive** | Player's approach archetype | 4 personality types (Assault, Siege, etc.) |

### 6.3 Book of the Bible Generation

The **Great Recycler** (NPU-resident agent) continuously writes the player's development story:

```
User completes quest objective
        ↓
Conductor logs to PostgreSQL
        ↓
Great Recycler (NPU) reads update
        ↓
Generates LitRPG prose chapter
        ↓
Writes to docs/books_of_the_bible/[quest_id].md
        ↓
SSE broadcast updates /book.html
        ↓
User sees their story unfold in real-time
```

---

## 7. Isomorphic Patterns Summary

These patterns repeat across all Trinity documentation:

| Pattern | ARCHITECTURE (this doc) | IMPLEMENTATION | OPERATIONS | MODELS |
|---------|------------------------|----------------|------------|--------|
| **ADDIE-C-R-A-P-E-Y-E** | §4: 12-phase framework | §7: Code phase implementations | §5: Deployment runbooks | — |
| **P-ART-Y Roles** | §5: Agent definitions | §6: Sidecar configs | §4: Launch commands | §2: Model cards |
| **Hotel Pattern** | §1: Memory philosophy | §8: Model switching logic | §3: Health monitoring | §3: Loading sequences |
| **Iron Road** | §6: LitRPG mechanics | §9: Book generation API | — | — |
| **Sword & Shield** | §5.2: Engineer pattern | §8: Dual-model inference | — | — |

---

## 8. Cross-References

| Section | Reference |
|---------|-----------|
| API endpoints for ADDIE phases | [02-IMPLEMENTATION.md §3.3](02-IMPLEMENTATION.md) |
| PostgreSQL schema for quests | [02-IMPLEMENTATION.md §4.3](02-IMPLEMENTATION.md) |
| Model parameters | [04-MODELS.md §2](04-MODELS.md) |
| Launch sequences | [03-OPERATIONS.md §2](03-OPERATIONS.md) |
| Troubleshooting | [03-OPERATIONS.md §6](03-OPERATIONS.md) |

---

*End of 01-ARCHITECTURE.md — Philosophy & Patterns*
