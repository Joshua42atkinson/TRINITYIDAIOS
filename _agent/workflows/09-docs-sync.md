---
description: Phase 9 — Sync all docs to reflect the current architecture. Resolve conflicts between context.md, TRINITY_IDENTITY.md, and MASTER_PIVOT_DOCUMENT.md.
---

# Phase 9: Docs Sync

## Objective

Three docs describe Trinity differently. Sync them so they tell one coherent story.

## Prerequisites

- Phases 1-8 complete (or at minimum 1-5)

## Doc Audit

### Current Conflicts

| Doc | Date | Vision | Models | Architecture |
|-----|------|--------|--------|-------------|
| `context.md` | July 3 | Prompting system | Qwythos-9B + DiffusionGemma | Phone/Desktop/XR |
| `TRINITY_IDENTITY.md` | July 5 | Creative studio OS | DiffusionGemma + Janus-Pro/VibeVoice | Core/Middleware/Product |
| `MASTER_PIVOT_DOCUMENT.md` | July 2 | Spatial computing | FACES + LitRPG + Socratic | Three-device |
| `MASTER_WORKFLOW.md` | July 5 | Creative pipeline | DiffusionGemma + ComfyUI | Studio/Solo/Closed |
| `SESSION-HANDOFF.md` | July 3 | Session state | Qwythos + DiffusionGemma | Dual-model |

### The Truth (as of July 5)
- **Vision**: Creative production studio OS (TRINITY_IDENTITY.md is the most current)
- **Models**: DiffusionGemma 26B (P) + ComfyUI/Janus-Pro/VibeVoice (A)
- **Architecture**: 3-tier (Core/Middleware/Product) — enforced by feature gates after Phase 2
- **Hotel**: Studio/Solo/Closed modes (not Team/Solo/Closed)
- **Frontend**: phone.html PWA (not React app)
- **Qwythos-9B**: Removed from resident models (deleted July 5 per MASTER_WORKFLOW.md)

## Steps

1. **Update `context.md`**:
   - Remove Qwythos-9B references (deleted)
   - Update model lineup to DiffusionGemma + Janus-Pro + VibeVoice
   - Update hotel modes to Studio/Solo/Closed
   - Update "What's Working" to reflect Phase 1-8 changes
   - Update "What's Next" to reflect remaining work

2. **Update `SESSION-HANDOFF.md`**:
   - Current system state (post-cleanup)
   - What was done (bug fixes + architecture cleanup)
   - What's next (FACES W7-W10, product creation, deployment)

3. **Archive `MASTER_PIVOT_DOCUMENT.md`**:
   - It's 1,839 lines and describes a vision that's evolved
   - Move to `docs/archive/` or trim to a 1-page summary
   - The key ideas (Socratic, FACES, LitRPG) are captured in TRINITY_IDENTITY.md

4. **Verify `TRINITY_IDENTITY.md`** is still accurate:
   - Update the Core/Middleware/Product tables if files moved in Phase 1
   - Update the Cargo.toml rules if feature gates added in Phase 2

5. **Verify `MASTER_WORKFLOW.md`** pipeline status:
   - Mark E2E pipeline as ✅ if Phase 8 succeeded
   - Update remaining work section

6. **Verify `HOW_TO_USE_TRINITY.md`** is accurate:
   - Startup commands still work
   - API endpoints still exist (check after Phase 3 route split)

7. **Delete or archive stale docs**:
   - `docs/active/SECURITY.md` — update or verify against Phase 5 changes
   - `docs/TRINITY_QUICKSTART.md` — verify startup commands
   - `docs/PARTY_FRAMEWORK.md` — verify still relevant

## Rules

- `TRINITY_IDENTITY.md` is the **authoritative** doc — all others must conform
- `context.md` is the **living** doc — update every session
- `SESSION-HANDOFF.md` is the **bridge** — update at end of every session
- `MASTER_PIVOT_DOCUMENT.md` is the **vision** — archive if too stale, don't rewrite

## Completion Criteria

- All docs reference the same model lineup (DiffusionGemma + ComfyUI)
- All docs reference the same hotel modes (Studio/Solo/Closed)
- All docs reference the same architecture (3-tier with feature gates)
- No doc mentions Qwythos-9B as a resident model
- `context.md` "What's Next" section matches the remaining phases in the master roadmap
- `SESSION-HANDOFF.md` is ready for the next session
