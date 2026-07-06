# ADDIECRAPEYE — Middleware Design Framework
## The 12 Stations of Creative Manifestation
## MIDDLEWARE — Not Core. Used by educational products.

> **Classification:** Middleware (`trinity-iron-road` crate)
> **Used by:** Educational games, learning modules, gamified products
> **Core dependency:** None — Trinity Core does not need this to function
> **Product dependency:** Products that want structured instructional design import this crate

---

## What This Is

ADDIECRAPEYE is a 12-phase creative design framework that maps instructional design principles (ADDIE + CRAP + EYE) to the Hero's Journey. It is **middleware** — a reusable framework that products can import to structure their creative process.

Trinity Core's creative pipeline (Intent → Story → Art → Voice → Video → 3D → Music → Assemble → Review → Vault) is the core process. ADDIECRAPEYE is one way to structure that process for educational products.

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

## Mapping to the Creative Pipeline

Each ADDIECRAPEYE phase maps to a step in Trinity's core creative pipeline:

| # | Phase | Pipeline Step | What You Produce |
|---|-------|--------------|-----------------|
| 1 | **Analyze** | INTENT + STORY | Concept doc, target audience, creative brief |
| 2 | **Design** | STORY | Design doc, story outline, art direction |
| 3 | **Develop** | ART + VOICE | Character art, backgrounds, voice prototypes |
| 4 | **Implement** | VIDEO + 3D | Animated scenes, 3D assets, game prototype |
| 5 | **Evaluate** | REVIEW | Playtest feedback, quality check, iteration plan |
| 6 | **Contrast** | ART | Visual identity, differentiation from similar works |
| 7 | **Repetition** | STORY + ART | Consistency: character sheets, style guides, motifs |
| 8 | **Alignment** | REVIEW | Cohesion check: do story, art, gameplay serve same goal? |
| 9 | **Proximity** | ASSEMBLE | Pacing: scene layout, level design, video edit rhythm |
| 10 | **Envision** | STORY | Vision statement, emotional arc, series potential |
| 11 | **Yoke** | VAULT | Connect to other works: series, universe, portfolio |
| 12 | **Evolve** | VAULT | Reflect on process, document learnings, plan next |

---

## THREE ACTS

### ACT I: THE DEPARTURE (ADD) — Building the Blueprint & Bones

| # | Phase | Hero's Journey | Body Part | Location | Pipeline |
|---|-------|---------------|-----------|----------|---------|
| 1 | **Analyze** | The Ordinary World | Eyes / Sensory Organs | The Junkyard Peak | INTENT + STORY |
| 2 | **Design** | The Call to Adventure | The Brain | Blueprint Mesa | STORY |
| 3 | **Develop** | Refusal of the Call | The Skeleton | The DAYDREAM Workshop | ART + VOICE |

### ACT II: THE INITIATION (IECRAP) — Fleshing out the World

| # | Phase | Hero's Journey | Body Part | Location | Pipeline |
|---|-------|---------------|-----------|----------|---------|
| 4 | **Implement** | Meeting the Mentor | The Muscles | The Proving Grounds | VIDEO + 3D |
| 5 | **Evaluate** | Crossing the Threshold | The Nervous System | The Friction Wastes | REVIEW |
| 6 | **Contrast** | Tests, Allies, Enemies | The Skin / Hide | The Neon Chasm | ART |
| 7 | **Repetition** | Approach to Inmost Cave | The Heart / Circulatory | The Loop Engine | STORY + ART |
| 8 | **Alignment** | The Ordeal | The Spine | The Great Chokepoint | REVIEW |
| 9 | **Proximity** | The Reward | The Hands / Digits | The Optimization Yards | ASSEMBLE |

### ACT III: THE RETURN (EYE) — Meta-Awareness & Release

| # | Phase | Hero's Journey | Body Part | Location | Pipeline |
|---|-------|---------------|-----------|----------|---------|
| 10 | **Envision** | The Road Back | The Third Eye | The Overlook | STORY |
| 11 | **Yoke** | The Resurrection | Connective Tissue / Joints | The Grand Coupling | VAULT |
| 12 | **Evolve** | Return with the Elixir | Breath / Lungs | Conscious Framework Terminal | VAULT |

---

## THE MEANING LOOP

```
User Message
  → Trinity API (P: DiffusionGemma processes intent)
    → Creative Pipeline (Story → Art → Voice → Video → 3D)
      → Quest Objective Complete
        → Station Advance (body part builds on Golem)
          → Character Skill Boost
            → Next phase prompt (Hero's Journey progression)
```

The isomorphism: Learning ID ≅ Programming a game ≅ Building a body ≅ Creating a product.

---

## How Products Use This

A product (e.g., an educational game) imports `trinity-iron-road` and uses the 12-phase structure to gamify its creative process. The product talks to Trinity Core via HTTP (:3000) for story/code generation and ComfyUI (:8188) for art/voice/video. The ADDIECRAPEYE framework provides the quest structure, objectives, and progression — Trinity Core provides the creative generation.

**Trinity Core does not know about ADDIECRAPEYE.** It just receives API calls and generates assets. The framework lives in the product, using Trinity as the engine.

---

*Created from Joshua's original design document. This is the canonical source of truth for the ADDIECRAPEYE middleware.*
*Reclassified as Middleware on 2026-07-05 — see TRINITY_IDENTITY.md for the boundary definition.*
