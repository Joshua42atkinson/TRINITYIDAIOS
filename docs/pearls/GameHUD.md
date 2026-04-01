# Game HUD (Right Rail)
## PEARL Alignment Document

This document defines the quality gates for the **Game HUD** right-rail sidebar (`GameHUD.jsx`), which aggregates `PearlCard`, `TrainStatus`, `CircuitryCard`, and `BookSection` into the persistent companion panel for the Iron Road.

> **Note:** As of v2, the Game HUD PEARL is subordinate to the unified [PhaseWorkspace PEARL](./PhaseWorkspace.md), which covers the complete Iron Road page — all three columns. This document focuses specifically on **right-rail density and progressive disclosure**.

---

### P — Perspective
*Who is the user, and at what journey stage do they need this sidebar?*

- **Early Journey (Phases 1–3):** The user is learning the system. The right rail should be minimal — PEARL card (showing subject/vision/alignment) + LOCOMOTIVE (Coal + Steam only). No bestiary, no book, no circuitry activation. The ENGINE DIAGNOSTICS toggle collapses Friction/Vulnerability/Shadow behind a disclosure button.
- **Mid Journey (Phases 4–6):** Circuitry activation bars begin to show activity as VAAM detects vocabulary. Bestiary populates when the first Scope Creep is tamed.
- **Late Journey (Phases 7–12):** Full HUD — Bestiary populated, Book chapters accumulating, Circuitry fully activated, ENGINE DIAGNOSTICS auto-expanded if friction rises.

### E — Engineering
*What backend services and their wiring status.*

| Component | API | Status |
|-----------|-----|--------|
| `PearlCard` | `GET/POST /api/pearl`, `PUT /api/pearl/refine` | ✅ Wired — Alignment bars update on refine |
| `TrainStatus` (primary) | `GET /api/quest` → coal/steam/xp/resonance | ✅ Wired |
| `TrainStatus` (advanced) | `GET /api/character` → friction/vulnerability/shadow | ⚠️ Wired but values never updated by backend |
| `CircuitryCard` | `GET /api/quest/circuitry` (polled every 30s) | ✅ Wired to live VAAM state (v2) |
| `BookSection` | `GET /api/book`, `GET /api/narrative/generate` | ✅ Wired |
| Bestiary | `GET /api/bestiary` via `useBestiary` hook | ✅ Wired — render gated on creeps.length > 0 |
| YARDMASTER card | `GET /api/character` | ✅ Wired — Session Zero info displays, ~~duplicate stats removed~~ |

### A — Aesthetic
*Visual density, progressive disclosure, and card hierarchy.*

**v2 Card Hierarchy (Progressive Disclosure):**

| Card | Show Condition | Priority |
|------|---------------|----------|
| PEARL | Always (has subject) | **Primary** |
| LOCOMOTIVE | Always — Coal + Steam only by default | **Primary** |
| ENGINE DIAGNOSTICS | Toggle-expand; auto-expands when values ≠ defaults | **On Demand** |
| YARDMASTER (Identity) | Always — SME + safety badges only | **Secondary** |
| BESTIARY | `creeps.length > 0` | **On Demand** |
| CIRCUITRY | Always — activation bars fill when VAAM detects words | **Secondary** |
| THE BOOK | `chapters.length > 0 || narrative` | **On Demand** |
| INVENTORY | `inventory.length > 0` | **On Demand** |

**Style Normalization (v2):**
- BookSection: All inline `style={{}}` props replaced with CSS classes (`.book-write-btn`, `.book-narrative`, `.book-chapter`, `.book-empty-hint`).
- Mode toggle buttons: Added `.mode-toggle__label` for text labels.

### R — Research
*The Mirror Mechanic.*

The HUD fulfills the "Mirror Mechanic" principle — every backend state change is reflected in the UI as a tangible gauge, bar, or badge. This transparency reinforces user agency by showing exactly how the system interprets their inputs.

**Remaining gap:** VAAM vocabulary detections are ephemeral (narrator messages). There is no persistent "VOCABULARY" card showing recently detected words, their element/mastery status, or proximity to Scope Creep taming threshold.

### L — Layout (C.R.A.P.)

| Principle | v1 Score | v2 Score | Change |
|-----------|----------|----------|--------|
| Contrast | 7/10 | 8/10 | Safety badges distinct from stat gauges |
| Repetition | 5/10 | 7/10 | BookSection normalized to `.card` pattern |
| Alignment | 7/10 | 7/10 | Persistent right-rail, 3-block semantic stack |
| Proximity | 4/10 | 6/10 | Duplicate stats removed, physics metrics grouped under ENGINE DIAGNOSTICS |

---
*Updated 2026-04-01 — PEARL v2 with progressive disclosure audit.*
