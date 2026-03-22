# ADDIECRAPEYE — Canonical Design Document
## The 12 Stations of Manifestation
## SOURCE OF TRUTH — Do Not Modify Without Joshua's Approval

> **WARNING:** This document is the authoritative reference for the ADDIECRAPEYE system.
> Previous AI sessions have **incorrectly renamed 6 of the 12 phases**.
> All code and UI must conform to THIS document.

---

## THE ACRONYM

**ADDIE** = Purdue's Instructional Design framework
**CRAP** = Robin Williams' design philosophy (Contrast, Repetition, Alignment, Proximity)
**EYE** = The user/meta-awareness (Envision, **Yoke**, Evolve)

```
A - Analyze
D - Design
D - Develop
I - Implement
E - Evaluate
C - Contrast       ← NOT "Correction"
R - Repetition     ← NOT "Review"
A - Alignment      ← NOT "Assessment"
P - Proximity      ← NOT "Planning"
E - Envision       ← NOT "Extension"
Y - Yoke           ← CORRECT (preserved)
E - Evolve         ← NOT "Execution"
```

---

## THREE ACTS

### ACT I: THE DEPARTURE (ADD) — Building the Blueprint & Bones

| # | Phase | Hero's Journey | Body Part | Location | Party Member |
|---|-------|---------------|-----------|----------|-------------|
| 1 | **Analyze** | The Ordinary World | 👀 Eyes / Sensory Organs | The Junkyard Peak | Ask Pete & The Evaluator |
| 2 | **Design** | The Call to Adventure | 🧠 The Brain | Blueprint Mesa | The Artist & The Evaluator |
| 3 | **Develop** | Refusal of the Call | 🦴 The Skeleton | The Code Forges | The Engineer (Sword & Shield) |

### ACT II: THE INITIATION (IECRAP) — Fleshing out the World

| # | Phase | Hero's Journey | Body Part | Location | Party Member |
|---|-------|---------------|-----------|----------|-------------|
| 4 | **Implement** | Meeting the Mentor | 🥩 The Muscles | The Proving Grounds | The Engineer |
| 5 | **Evaluate** | Crossing the Threshold | ⚡ The Nervous System | The Friction Wastes | The Brakeman |
| 6 | **Contrast** | Tests, Allies, Enemies | 🧥 The Skin / Hide | The Neon Chasm | The Visionary |
| 7 | **Repetition** | Approach to Inmost Cave | ❤️ The Heart / Circulatory | The Loop Engine | The Engineer |
| 8 | **Alignment** | The Ordeal | 🦴 The Spine | The Great Chokepoint | The Evaluator |
| 9 | **Proximity** | The Reward | ✋ The Hands / Digits | The Optimization Yards | The Visionary & The Artist |

### ACT III: THE RETURN (EYE) — Meta-Awareness & Release

| # | Phase | Hero's Journey | Body Part | Location | Party Member |
|---|-------|---------------|-----------|----------|-------------|
| 10 | **Envision** | The Road Back | 👁️ The Third Eye | The Overlook | Ask Pete |
| 11 | **Yoke** | The Resurrection | 🔗 Connective Tissue / Joints | The Grand Coupling | The Entire Party |
| 12 | **Evolve** | Return with the Elixir | 🫁 Breath / Lungs | Conscious Framework Terminal | The Great Recycler |

---

## KNOWN ERRORS IN THE CODEBASE (as of March 21, 2026)

### `index.html` line 1124 — WRONG:
```javascript
const PHASES = ['Analysis', 'Design', 'Development', 'Implementation', 'Evaluation',
  'Correction', 'Review', 'Assessment', 'Planning', 'Extension', 'Yield', 'Execution'];
```

### CORRECT:
```javascript
const PHASES = ['Analyze', 'Design', 'Develop', 'Implement', 'Evaluate',
  'Contrast', 'Repetition', 'Alignment', 'Proximity', 'Envision', 'Yoke', 'Evolve'];
```

### `conductor_leader.rs` — WRONG enum variants:
```rust
// These 6 are WRONG:
Correction,   // Should be: Contrast
Review,       // Should be: Repetition
Assessment,   // Should be: Alignment
Planning,     // Should be: Proximity
Extension,    // Should be: Envision
Execution,    // Should be: Evolve
```

### What's Missing Entirely from Code:
- Hero's Journey stage per phase
- Body part / Digital Golem progression
- Location names (Junkyard Peak, Blueprint Mesa, etc.)
- Party member assignments per phase
- The Great Recycler system entity
- The Dumpster Universe setting
- Coal → Steam thermodynamic metaphor (partially implemented via VAAM)
- Bloom's Taxonomy alignment per phase (Analyze = Remember/Understand, etc.)

---

## THE MEANING LOOP

```
User Message
  → VAAM Bridge (Style/Vocab detect → mine Coal)
    → Pete Orchestration (VAAM-aligned, phase-aware)
      → Quest Objective Complete
        → Station Advance (body part builds on Golem)
          → Character Skill Boost
            → Next phase prompt (Hero's Journey progression)
```

The isomorphism: Learning ID ≅ Programming a game ≅ Building a body.

---

*Created from Joshua's original design document. This is the canonical source of truth.*
