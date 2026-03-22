# Trinity Technical Bible — Master Navigation Hub
**Version:** 4.0.0 (Chunked Isomorphic Architecture)
**Date:** March 16, 2026
**Status:** Production Reference

---

## Quick Navigation

| Document | Purpose | Lines | Key Sections |
|----------|---------|-------|--------------|
| **[01-ARCHITECTURE.md](01-ARCHITECTURE.md)** | Philosophy, patterns, ADDIE/CRAPEYE | ~500 | Hotel Pattern, Iron Road, ADDIE-C-R-A-P-E-Y-E phases |
| **[02-IMPLEMENTATION.md](02-IMPLEMENTATION.md)** | APIs, schemas, configs, code specs | ~600 | API endpoints, PostgreSQL schema, model params |
| **[03-OPERATIONS.md](03-OPERATIONS.md)** | Commands, deployment, troubleshooting | ~400 | Launch scripts, health checks, common issues |
| **[04-MODELS.md](04-MODELS.md)** | Model cards, delegation matrix | ~300 | 165GB portfolio, task→model mapping |
| **[05-CROW-CONTINUITY.md](05-CROW-CONTINUITY.md)** | CROW architecture, Iron Road ledger | ~400 | Vision observer, narrative orchestrator, continuity engine |

**Isomorphic Principle:** Each document mirrors the same structural patterns (ADDIE, P-ART-Y roles, Hotel Management) so concepts learned in one transfer to others.

---

## 1. Core Philosophy: "Hotel Management"

Trinity operates out of a single 128GB Unified Memory "Hotel". Running massive LLMs (100B+ parameters) simultaneously will crash the hotel (Out of Memory). Therefore, Trinity uses a strict **"Rotating Star" (One-in, One-out) policy** governed by the **Conductor**.

> **See details:** [01-ARCHITECTURE.md §2](01-ARCHITECTURE.md#2-the-hotel-pattern)

---

## 2. System Architecture: 3-Layer Model

```
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 1: HEADLESS SERVER (trinity)                            │
│  • Axum HTTP API on :3000                                       │
│  • PostgreSQL RAG with pgvector                                │
│  • Agentic tools (7 tools: read_file, shell, search, etc.)      │
│  • Web UI served (index.html, book.html, dev.html)             │
├─────────────────────────────────────────────────────────────────┤
│  LAYER 2: AI KERNEL (trinity-kernel + sidecars)               │
│  • Conductor (Mistral Small 4) — orchestrates quest workflows      │
│  • Sidecars: Engineer, Evaluator, Artist, Brakeman, Pete       │
│  • One-in, One-out model switching (Hotel Pattern)            │
├─────────────────────────────────────────────────────────────────┤
│  LAYER 3: BODY/UI (trinity-body / browser)                     │
│  • Bevy dockable workspace (future)                            │
│  • Browser: Chat, Iron Road Book, Dev Console                  │
│  • PersonaPlex-7B voice interface (Ask Pete)                  │
└─────────────────────────────────────────────────────────────────┘
```

> **See details:** [01-ARCHITECTURE.md §3](01-ARCHITECTURE.md#3-three-layer-architecture)

---

## 3. ADDIE-C-R-A-P-E-Y-E: The 12-Phase Workflow

Trinity uses an extended ADDIE framework called **ADDIECRAPEYE**:

| Phase | Letter | Focus | Agent | Bloom's Level |
|-------|--------|-------|-------|---------------|
| **A**nalysis | A | Needs, learners, context | Evaluator (56GB) | Remember/Understand |
| **D**esign | D | Objectives, structure | Artist/CRAP (21GB) | Apply |
| **D**evelop | V | Create content | Yardmaster/EYE (24GB) | Analyze |
| **I**mplement | I | Deploy, teach | Yardmaster/EYE (24GB) | Evaluate |
| **E**valuate | E | Assess, measure | Evaluator (56GB) | Create |
| **C**ontrast | C | Compare alternatives | Researcher (9GB) | Analyze |
| **R**epetition | R | Spiral curriculum | Pete/ADDIE (21GB) | Apply |
| **A**lignment | A | Map to standards | Evaluator (56GB) | Evaluate |
| **P**roximity | P | Just-in-time learning | Pete/ADDIE (21GB) | Understand |
| **E**nvision | E | Future scenarios | Artist/CRAP (21GB) | Create |
| **Y**oke | Y | Connect concepts | Yardmaster/EYE (24GB) | Analyze |
| **E**volve | E | Iterate, improve | Evaluator (56GB) | Create |

> **See details:** [01-ARCHITECTURE.md §4](01-ARCHITECTURE.md#4-addiecrapeye-framework)

---

## 4. Implementation Quick Reference

### 4.1 API Endpoints (Trinity Server)

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/health` | GET | llama.cpp + PostgreSQL status |
| `/api/chat` | POST | Standard chat |
| `/api/chat/agent` | POST | Agentic chat with tools |
| `/api/quest/complete` | POST | Mark objective done |
| `/api/book/stream` | GET | SSE Iron Road updates |

> **See full spec:** [02-IMPLEMENTATION.md §3](02-IMPLEMENTATION.md#3-api-specification)

### 4.2 PostgreSQL Schema (pgvector)

```sql
-- Core RAG tables
document_embeddings (id, doc_path, content, embedding vector(384))
quest_state (player_id, chapter, phase, xp, resonance)
party_members (id, name, role, model, active)
```

> **See full schema:** [02-IMPLEMENTATION.md §4](02-IMPLEMENTATION.md#4-database-schema)

### 4.3 Model Launch Commands

```bash
# Conductor / base brain (Mistral Small 4) — main orchestrator
llama-server -m ~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf -ngl 99 -c 32768 --port 8080

# Yardmaster / EYE (Ming-flash-omni-2.0) — dev mode code generation & vision
llama-server -m ~/trinity-models/gguf/Ming-flash-omni-2.0-Q4_K_M.gguf -ngl 99 -c 32768 --port 8082
```

> **See all models:** [04-MODELS.md](04-MODELS.md)

---

## 5. Operational Quick Reference

### 5.1 Launch Commands

```bash
# Start everything
./run_trinity.sh

# Or manual sequence
./llama.cpp/build/bin/llama-server -m ~/trinity-models/gguf/Mistral-Small-24B-Instruct-2501-Q4_K_M.gguf -ngl 99 -c 32768 --port 8080
cargo run -p trinity --bin trinity
```

### 5.2 Health Checks

```bash
# Check llama.cpp
curl http://localhost:8080/health

# Check Trinity Server
curl http://localhost:3000/api/health

# Check database
psql postgres://trinity:trinity@localhost/trinity -c "SELECT COUNT(*) FROM document_embeddings;"
```

> **See full operations:** [03-OPERATIONS.md](03-OPERATIONS.md)

---

## 6. P-ART-Y: The AI Party System

| Role | Model(s) | Memory | Specialty | Icon |
|------|----------|--------|-----------|------|
| **Conductor** | Mistral Small 4 | 14GB | Orchestration, narrative | 🚂 |
| **Yardmaster (EYE)** | Ming-flash-omni-2.0 | 24GB | Dev Mode, UI manipulation, code | ⚙️ |
| **Evaluator** | Qwen2.5-Coder-32B (65K ctx) | 21GB | QM rubrics, WCAG, Bloom's | 📊 |
| **Artist (CRAP)** | Qwen2.5-Coder-32B (32K ctx) | 21GB | GDDs, UI wireframes, 2D/3D/XR | 🎨 |
| **Brakeman** | Qwen3-Coder-25B (16K ctx) | 15GB | Tests, security, clippy | 🛡️ |
| **Pete (ADDIE)** | Qwen2.5-Coder-32B (voice pending) | 21GB | Purdue Pete (ID): Socratic dialogue | 🎓 |
| **Visionary** | Qwen3.5-35B + mmproj | 21GB | Vision, UI evaluation | 👁️ |
| **CROW** | Crow-9B + mmproj | 6GB | Continuity, narrative, vision | 🦅 |

**Memory Budget:** 72GB loaded (3 models), 56GB swap space for swapping

> **See full party spec:** [01-ARCHITECTURE.md §5](01-ARCHITECTURE.md#5-p-art-y-role-system)

---

## 7. Isomorphic Patterns Across Documents

Each Bible document uses the same structural patterns:

| Pattern | ARCHITECTURE | IMPLEMENTATION | OPERATIONS | MODELS |
|---------|--------------|----------------|------------|--------|
| **ADDIE-C-R-A-P-E-Y-E** | §4: Framework overview | §7: Development phase specs | §5: Deployment phases | — |
| **P-ART-Y Roles** | §5: Role definitions | §6: Agent configs | §4: Launch commands | Full cards |
| **Hotel Pattern** | §2: Memory philosophy | §8: Model switching | §3: Health monitoring | §3: Loading |
| **Iron Road** | §3: LitRPG integration | §9: Book generation | — | — |

This isomorphism means: **learn the pattern once, apply everywhere.**

---

## 8. Document Cross-Reference Matrix

| If you need... | Go to... | Key Lines |
|----------------|----------|-----------|
| Philosophy, "why" design decisions | [01-ARCHITECTURE.md](01-ARCHITECTURE.md) | 1-200 |
| ADDIE-C-R-A-P-E-Y-E deep dive | [01-ARCHITECTURE.md §4](01-ARCHITECTURE.md#4-addiecrapeye-framework) | 200-400 |
| API specs, endpoints | [02-IMPLEMENTATION.md §3](02-IMPLEMENTATION.md#3-api-specification) | 100-300 |
| PostgreSQL schema | [02-IMPLEMENTATION.md §4](02-IMPLEMENTATION.md#4-database-schema) | 300-500 |
| Configuration structs | [02-IMPLEMENTATION.md §5](02-IMPLEMENTATION.md#5-configuration-system) | 500-700 |
| Launch commands | [03-OPERATIONS.md §2](03-OPERATIONS.md#2-launch-sequences) | 50-150 |
| Troubleshooting | [03-OPERATIONS.md §6](03-OPERATIONS.md#6-troubleshooting) | 300-400 |
| Model cards, which model for what task | [04-MODELS.md §2](04-MODELS.md#2-model-cards) | 50-200 |
| Delegation matrix (task → model) | [04-MODELS.md §3](04-MODELS.md#3-intelligent-delegation) | 200-300 |

---

## 9. File Locations (Quick Reference)

| What | Where | Document |
|------|-------|----------|
| Master Bible | `TRINITY_TECHNICAL_BIBLE.md` | This file |
| Architecture | `docs/bible/01-ARCHITECTURE.md` | [View](01-ARCHITECTURE.md) |
| Implementation | `docs/bible/02-IMPLEMENTATION.md` | [View](02-IMPLEMENTATION.md) |
| Operations | `docs/bible/03-OPERATIONS.md` | [View](03-OPERATIONS.md) |
| Models | `docs/bible/04-MODELS.md` | [View](04-MODELS.md) |
| §17 Comment Standard | `docs/bible/§17-STANDARD.md` | [View](§17-STANDARD.md) |

---

## 10. Version History

| Version | Date | Changes | Lines |
|---------|------|---------|-------|
| 3.0.0 | Mar 10 | Initial comprehensive Bible | 1,145 |
| 3.1.0 | Mar 12 | Sidecar architecture, P-ART-Y | 1,145 |
| 3.2.0 | Mar 14 | §17 comment standard | 1,145 |
| 3.3.0 | Mar 16 | Appendix C implementation details | 1,504 |
| **4.0.0** | **Mar 16** | **Chunked isomorphic architecture** | **Master: ~200, Total: ~1,800** |

---

## 11. Usage Patterns for LLMs

When using these documents with AI assistants:

1. **Quick context**: Load this master index only (~200 lines)
2. **Deep dive**: Load specific bible document (01-04)
3. **Implementation task**: Load 02-IMPLEMENTATION.md + relevant code
4. **Debugging**: Load 03-OPERATIONS.md + 02-IMPLEMENTATION.md §8

Never load all documents simultaneously—use the isomorphic cross-references to navigate.

---

*End of Trinity Technical Bible — Master Navigation Hub v4.0.0*
