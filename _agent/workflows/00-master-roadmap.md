---
description: Master roadmap — the complete "fix Trinity" plan. Read this first to understand all work phases and their order.
---

# Master Roadmap — Trinity Production Readiness

> **Read this workflow first.** It defines all work phases, their dependencies, and execution order.
> Each phase has its own workflow file (e.g., `/01-protocol-cleanup`). Run them in order.
> Update `SESSION-HANDOFF.md` after completing each phase.

## Current State (July 5, 2026 Evening — Verified)

- **Core engine works**: inference routing, hotel manager, creative pipeline, agent loop, RAG, memory
- **Architecture clean**: `trinity-protocol` is 20 files / 5,577 lines (shared types only). Core depends on it via feature gates.
- **`main.rs` is 508 lines**: Routes only, handlers split into `src/handlers/`
- **No `#![allow(dead_code)]`** anywhere in Core
- **Agent → ComfyUI image generation: WORKING** — Phone PWA chat → DiffusionGemma → `generate_image` tool → ComfyUI LongCat-Image → image saved to workspace. Tested end-to-end July 5.
- **Tool calling fixed for DiffusionGemma**: JSON key normalization, structured tool call validation, 5-min timeout for creative tools
- **FACES moved to Semantic Slime**: Protocol crate + docs + workflows extracted to `/home/joshua/Semantic Slime/`
- **Docs synced (Phase 9 complete)**: All docs updated to reflect P+A+H architecture. Qwythos removed, FACES moved to Semantic Slime, hotel modes corrected. 6 stale docs + 24 stale scripts archived.

## Phase Overview

| Phase | Workflow | Effort | Depends On | Status |
|-------|----------|--------|------------|--------|
| 1 | `/01-protocol-cleanup` | ~2 hr | Nothing | **COMPLETE** — 20 files / 5,577 lines, shared types only |
| 2 | `/02-feature-gate-middleware` | ~2 hr | Phase 1 | **COMPLETE** — `ironroad` is optional feature, middleware gated |
| 3 | `/03-split-main-rs` | ~3 hr | Phase 2 | **COMPLETE** — 508 lines, handlers in `src/handlers/` |
| 4 | `/04-dead-code-removal` | ~1 hr | Phase 3 | **COMPLETE** — 2,861 lines removed, no `#![allow(dead_code)]` |
| 5 | `/05-security-hardening` | ~1 hr | Phase 3 | **COMPLETE** — auth, CORS, rate limiting, body limits, graceful shutdown |
| 6 | `/06-tools-trim` | ~1 hr | Phase 3 | **PARTIAL** — tools exist and work, could still trim further |
| 7 | `/07-frontend-consolidation` | ~2 hr | Phase 3 | **COMPLETE** — phone.html PWA is the single frontend |
| 8 | `/08-e2e-creative-pipeline` | ~2 hr | Phases 3-6 | **IN PROGRESS** — image generation working from PWA via agent. Voice/video/3D pending. |
| 9 | `/09-docs-sync` | ~1 hr | Phases 1-8 | **COMPLETE** — all docs updated to P+A+H architecture, Qwythos removed, FACES moved to Semantic Slime, workspace hygiene done |
| 10 | `/12-daydream-xr-test` | ~3 hr | Phase 3 | **PENDING** — needs Bevy XR work |
| 11 | `/13-deployment-packaging` | ~2 hr | Phases 3-5 | **PENDING** — systemd/Caddy audit, release build |

**Parallel (no dependencies):**

| # | Workflow | Effort | Depends On | Status |
|---|----------|--------|------------|--------|
| S1 | `/strix-halo-optimization` | ~3 hr | Nothing | Pending |

## Critical Security Findings (from prior audit)

These are **above and beyond** the Phase 5 hardening — they are known public-facing vulnerabilities:

1. **`/api/tools/execute` exposes unauthenticated tool execution** — anyone can run arbitrary tools (file read/write, shell commands) without auth. This is the most critical finding.
2. **`/api/models/switch` accepts arbitrary URLs** — `InferenceRouter::set_active_url` can be pointed at attacker-controlled backends, exfiltrating all prompts and responses.
3. **No tenant isolation** — shared global `AppState` with a single session. Not safe for public multi-user deployment. This is acceptable for single-user desktop use but **must be fixed before public exposure**.

**Priority**: Items 1 and 2 must be fixed in Phase 5 (Security Hardening). Item 3 is a longer-term concern — document the single-user limitation in deployment docs.

## Deployment Context

Trinity is hosted publicly from a local edge device (Strix Halo) and served via LDTAtkinson.com for Purdue professors and students. This means:
- Internet exposure is real, not theoretical
- Home-network risk applies
- Educational/stakeholder scrutiny applies
- Red Hat-grade safety standards are the target

## Execution Rules

1. **One phase per session** — don't combine phases unless they're small
2. **`cargo check` must pass** at the end of every phase
3. **Update `SESSION-HANDOFF.md`** after each phase with what changed and what's next
4. **Update `context.md`** if architecture or dependencies change
5. **Commit after each phase** using `/commit-wrap` workflow
6. **Never break the build** — if a step fails, fix it before moving to the next step

## Dependency Graph

```
Phase 1 (Protocol Cleanup)
  └── Phase 2 (Feature-Gate Middleware)
       └── Phase 3 (Split main.rs)
            ├── Phase 4 (Dead Code Removal)
            ├── Phase 5 (Security Hardening)
            ├── Phase 6 (Tools Trim)
            ├── Phase 7 (Frontend Consolidation)
            └── Phase 12 (Daydream XR Test)

Phases 3-7 ──── Phase 8 (E2E Creative Pipeline)
                       └── Phase 9 (Docs Sync)

Phase 3 ──── Phase 11 (Deployment Packaging)
```

## What "Done" Looks Like

### Already Done ✅
- [x] Core `Cargo.toml` has zero mandatory Middleware deps (ironroad/export are optional features)
- [x] `trinity-protocol` contains only shared types (20 files / 5,577 lines)
- [x] `main.rs` is 508 lines (routes only, handlers in separate files)
- [x] No `#![allow(dead_code)]` anywhere in Core
- [x] Creative endpoints protected by auth
- [x] CORS restricted to known origins
- [x] Graceful shutdown on SIGTERM/SIGINT
- [x] FACES protocol moved to Semantic Slime repo
- [x] Phone PWA can trigger image generation via agent (tested July 5)

### Remaining
- [ ] Tools trimmed to ~18 builder-relevant tools (currently works but could be leaner)
- [ ] One creative asset produced end-to-end (story → art → voice) — image works, voice pending VibeVoice
- [x] All docs reflect the current architecture (Phase 9 complete July 5)
- [ ] Systemd services audited and match current model setup
- [ ] Caddyfile audited and matches current architecture
- [ ] Release binary builds clean and starts correctly
