# Trinity Production Roadmap — Purdue Submission
## Created: March 21, 2026 — Next Session Plan

---

## THE GOAL

Trinity must demonstrate that it is a **real instructional design system**, not a tech demo. Purdue professors need to see:
1. A teacher can describe a lesson → Pete coaches through ADDIECRAPEYE → Trinity produces a structured Game Design Document
2. The system applies real pedagogy (Bloom's Taxonomy, backwards design, formative assessment)
3. Output is actionable — vocabulary lists, learning objectives, assessment rubrics, game scaffolds

**What matters:** Productivity. Real ID output. Not animations or badges.

---

## SESSION CHECKLIST (copy-paste to start)

```bash
# 1. Kill stale terminals from previous sessions
pkill -f "cargo run" 2>/dev/null; pkill -f "cargo test" 2>/dev/null

# 2. Start longcat-sglang (Mistral Small 4 119B — the brain)
longcat-sglang \
  -m ~/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
  --host 0.0.0.0 --port 8080 -ngl 99 -c 8192

# 3. In Antigravity IDE: /build-and-test
# Or manually:
cargo test --workspace && cargo run -p trinity --release

# 4. Open browser: http://localhost:3000
```

---

## PHASE 5: STUBS → REAL SYSTEMS

### Priority 1: Yardmaster SSE Streaming (Quick Win)

**Problem:** `agent.rs` uses `chat_completion()` (waits for full response) instead of `chat_completion_stream()` (tokens appear as they're generated). This makes the chat feel dead while waiting.

**Fix:** Wire `chat_completion_stream` into the agent loop. `inference.rs` already has the streaming function — `agent.rs` just doesn't call it.

**Files:** `agent.rs` (~20 lines changed)

---

### Priority 2: Knowledge Tracing (Core Pedagogy)

**Problem:** Static SVG curve with hardcoded BKT values. No actual tracking of what the user knows.

**What It Should Do:**
- Bayesian Knowledge Tracing per vocabulary word and per ADDIECRAPEYE skill
- As the user interacts, BKT updates P(know) for each concept
- Drive adaptive difficulty — Pete asks harder questions when mastery is high
- Feed the Character Sheet's skill boosts with real data

**Why It Matters:** This is the difference between "gamified chat" and "intelligent tutoring system." Purdue ID professors will look for this.

**Files:** `trinity-iron-road/src/vaam/` (new `bkt.rs`), `trinity-protocol/` (BKT types), UI `index.html` (replace static SVG with real curve)

---

### Priority 3: GDD Quality (The Deliverable)

**Problem:** The GDD compilation (`/api/quest/compile`) collects chat notes but may not structure them into pedagogically rigorous output.

**What It Should Produce:**
- Learning Objectives aligned to Bloom's Taxonomy verbs (Remember → Create)
- Assessment strategy per objective (formative + summative)
- Vocabulary with tier classification (Basic/Intermediate/Advanced)
- Game mechanics mapped to learning outcomes (not random gamification)
- Standards alignment (Common Core, ISTE, or instructor-defined)

**Why It Matters:** This IS the product. The GDD is what professors will evaluate.

**Files:** `quests.rs` (compile handler), `trinity-protocol/` (GDD struct), `conductor_leader.rs` (orchestration prompts)

---

### Priority 4: Video/Media Pipeline

**Problem:** Video generation returns "coming soon." Image generation works (vLLM Omni) but needs to be part of the GDD workflow.

**Realistic Approach:**
- Wire vLLM Omni image generation into GDD compilation — auto-generate concept art for each game scene
- For video: add screen recording of the Bevy scaffold running as a "preview" — don't need AI video gen for MVP
- Alternative: Generate animated GIFs from sprite sheets

**Files:** `creative.rs`, `quests.rs` (GDD compile step), UI integration

---

### Priority 5: Forge Terminal (Real Code Gen)

**Problem:** Decorative terminal that shows tool execution logs but doesn't actually generate code.

**What It Should Do:**
- When Pete reaches the Development phase → Forge runs `scaffold_bevy_game` automatically
- Shows real build output as the game compiles
- The user can see their game being built in real-time

**Files:** `agent.rs` (auto-scaffold trigger), UI `index.html` (Forge display)

---

### Priority 6: RLHF (Feedback Loop)

**Problem:** Thumbs up/down logs to console but doesn't influence anything.

**What It Should Do:**
- Store feedback in PostgreSQL (message_id, score, phase)
- Aggregate per-phase satisfaction scores
- Feed back into Pete's system prompt: "User found Phase 3 confusing, provide more scaffolding"

**Files:** `rlhf_api.rs`, `persistence.rs`, `agent.rs` (prompt injection)

---

## WHAT I CAN DO VS. WHAT NEEDS YOUR INPUT

### I Can Build Right Now (code changes)
- ✅ SSE streaming for Yardmaster
- ✅ BKT knowledge tracing engine
- ✅ RLHF persistence + prompt feedback loop
- ✅ Forge auto-scaffold trigger
- ✅ GDD struct improvements
- ✅ Image generation into GDD pipeline

### Needs Your Pedagogical Expertise
- What Bloom's verbs should Pete enforce per ADDIECRAPEYE phase?
- What assessment types map to which game mechanics?
- What does a "good" GDD look like for Purdue? (Do you have a template from LDT?)
- Which standards framework should Trinity align to? (Common Core? ISTE? Custom?)
- What subjects should the demo showcase? (What would impress the professors?)

---

## PACKAGING FOR PURDUE

When the system works end-to-end:

1. **Professor README** — One page: install PostgreSQL, download GGUF model, run Trinity, open browser
2. **Demo Video** — Screen recording of a teacher building a vocabulary game from scratch
3. **Technical Summary** — Architecture diagram, test results, feature matrix
4. **Sample GDD Output** — A completed Game Design Document showing real pedagogical structure

---

## NEXT SESSION ORDER OF OPERATIONS

1. Clean workspace (kill stale processes)
2. Start longcat-sglang → verify chat works end-to-end
3. Wire SSE streaming into Yardmaster
4. Implement BKT knowledge tracing
5. Improve GDD compilation with real Bloom's alignment
6. End-to-end test: full ADDIECRAPEYE → GDD → scaffold
7. Update docs, commit

---

*This document supersedes CONTEXT.md Section 9 for session planning.*
