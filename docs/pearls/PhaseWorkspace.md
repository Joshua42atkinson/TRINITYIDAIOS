# Phase Workspace (Iron Road) + Game HUD
## PEARL Alignment Document

This document defines the quality gates for the **Iron Road** UI — the primary page composed of `ChapterRail.jsx`, `PhaseWorkspace.jsx`, `GameHUD.jsx`, and their child components (`PearlCard`, `TrainStatus`, `CircuitryCard`, `BookSection`, `PerspectiveSidebar`).

---

### P — Perspective
*Who is the user, and what is their cognitive state at each journey stage?*

**Pre-Journey (SubjectPicker):**
- **Cognitive Load: Low.** The user is a first-time visitor choosing a subject, medium, and vision. The interface is clean, centered, and focused. No game mechanics are visible. This is correct — no changes needed.

**Early Journey (Phase 1–3: Analysis → Design → Development):**
- **Cognitive Load: Medium — Conversational.** The user is riding the Iron Road, conversing with The Great Recycler (Socratic AI) and Programmer Pete. They are in "Inhale" mode — planning, reflecting, and learning.
- **Goal:** Converse, tame Scope Creep, unlock VAAM vocabulary, clear objectives, advance stations.
- **Progressive Disclosure Applied (v2):** The right rail now shows **only PEARL + Coal/Steam** by default. Friction/Vulnerability/Shadow are collapsed behind "ENGINE DIAGNOSTICS" until their values change from defaults. Bestiary, Circuitry activation bars, and Book sections auto-reveal when they have content. This reduces first-impression cognitive overload from 7 cards to 3.

**Late Journey (Phase 6+: CRAP → EYE):**
- The Circuitry grid activates with live VAAM data. Book chapters accumulate. Bestiary populates as words are tamed. The interface naturally fills in as the user progresses.

---

### E — Engineering (Rust Backend Mapping)
*What backend services drive this view, and what is their wiring status?*

**Component Architecture (3-Column Grid):**

| Column | Component | Backend API | Status |
|--------|-----------|-------------|--------|
| Left | `ChapterRail` | Client-side constants | ✅ Fully static |
| Center | `PhaseWorkspace` | `/api/chat/stream`, `/api/quest/*`, `/api/tts`, `/api/eye/export` | ✅ Wired |
| Center | `PerspectiveSidebar` | SSE `perspective` events from `agent.rs` Ring 6 | ⚠️ Events don't fire during normal Iron Road chat |
| Right | `PearlCard` | `GET/POST /api/pearl`, `PUT /api/pearl/refine` | ✅ Wired |
| Right | `TrainStatus` | `GET /api/quest` (coal/steam/xp), `GET /api/character` (friction/vuln/shadow) | ✅ Gauges wired, ⚠️ Friction/Shadow never updated by backend |
| Right | `CircuitryCard` | `GET /api/quest/circuitry` | ✅ Wired to live VAAM state (v2) |
| Right | `BookSection` | `GET /api/book`, `GET /api/narrative/generate` | ✅ Wired |
| Overlay | `ScopeCreepModal` | SSE `creep_tameable` events | ⚠️ Detection threshold too high; never fires |
| Overlay | `PhaseTransition` | Client-side ceremony | ✅ Wired |

**Known Backend Gaps:**
1. `track_friction` and `vulnerability` are never written to during Iron Road chat flow — the values remain at defaults (0% and 50%).
2. Coal -2 / Steam +5 messages are cosmetic — no backend economy call occurs.
3. Scope Creep VAAM threshold too high for casual conversation — modal never triggers.
4. Quest objectives are generic ("Speak with the Great Recycler") — need per-phase objectives from `quests/` YAML.

---

### A — Aesthetic
*Visual language, density, and mood.*

**Palette:** High-fantasy industrial steam. Phase colors map to Bloom's Taxonomy (Gold = Remember/Analyze, Blue = Understand/Apply, Purple = Evaluate, Green = Create).

**Density Improvements (v2):**
- Duplicate XP/Coal/Steam rows removed from YARDMASTER card (was showing same 3 numbers in both LOCOMOTIVE and YARDMASTER).
- BookSection inline styles replaced with CSS classes (`.book-write-btn`, `.book-narrative`, `.book-chapter`, `.book-empty-hint`).
- Footer mode buttons now show text labels ("Iron Road", "Express", "Workshop") next to emoji icons, removing guesswork.

**Key Visual Elements:**
- Narrator text: serif, italic, gold — distinct from Pete's sans-serif white.
- Phase banners use per-phase Bloom's color.
- Handoff banner pulsing gradient signals Recycler → Pete transition.
- ENGINE DIAGNOSTICS toggle uses a green dot indicator when values are non-default.

---

### R — Research
*VAAM usability and framework alignment.*

**Iron Road Duality:** Enforces "Inhale" (planning/reflection) vs "Exhale" (building/executing) from `TRINITY_FANCY_BIBLE.md`. The `conductor_leader.rs` restricts the user to planning by injecting Great Recycler preambles.

**VAAM Matrix Gaps:**
- VAAM vocabulary detections appear as transient narrator messages but have **no persistent UI panel**. Users cannot see their vocabulary matrix, word weights, or mastery progress.
- Recommendation: Add a VOCABULARY card to GameHUD (future work item P1).
- Recommendation: Connect Circuitry activation bars to live VAAM word counts per quadrant (done in v2).

**ADDIECRAPEYE Coverage:**
- All 12 phases render correctly in ChapterRail with Hero's Journey titles.
- Bloom's taxonomy levels display on each station.
- Group labels (EXTRACT—ADDIE, PLACE—CRAP, REFINE—EYE) correctly segment the rail.

---

### L — Layout (C.R.A.P. Methodology)
*How does this view score on Contrast, Repetition, Alignment, and Proximity?*

| Principle | Score | Notes |
|-----------|-------|-------|
| **Contrast** | 8/10 | Strong differentiation between narrator (serif gold), Pete (sans white), user (right-aligned gold-bordered), and system messages. Phase colors are effective. |
| **Repetition** | 7/10 | (v2) BookSection now uses consistent `.card` pattern classes instead of inline styles. Scope Creep always uses same red-shifted modal. Safety badges use consistent pill styling. |
| **Alignment** | 7/10 | 3-column grid is structurally sound. Center column dominates correctly. Narrative scroll caps at max-width for readability. |
| **Proximity** | 6/10 | (v2) Duplicate stats removed. MicButton + textarea form a unified input zone. ENGINE DIAGNOSTICS groups physics metrics separately from primary gauges. Remaining issue: Session Zero info in YARDMASTER could merge into PEARL. |

---

### Remaining Work Items (Ordered by Priority)

1. **C2** — Lower Scope Creep VAAM threshold (backend: `vaam_bridge.rs`)
2. **C1** — Wire Coal/Steam to real backend economy endpoint
3. **C4** — Populate real per-phase quest objectives from `quests/` YAML
4. **P1** — Add VAAM vocabulary activity feed to GameHUD
5. **P3** — Wire Perspective Engine SSE events from Iron Road chat
6. **P4** — Wire Friction/Vulnerability to conversation analysis

---
*Updated 2026-04-01 — PEARL evaluation v2 with progressive disclosure and circuitry wiring.*


### HOW IT WORKS (User Action)
*The Presentation \How\: What the user actually does.*
- **Action:** This is the core chat interface. Select one of the 12 ADDIECRAPEYE phases on the left, read the 3 checkboxes at the top, and chat with Pete to complete each objective.
- **Why:** This demystifies the theoretical \why\ into a direct, clickable interaction that drives the system forward.
