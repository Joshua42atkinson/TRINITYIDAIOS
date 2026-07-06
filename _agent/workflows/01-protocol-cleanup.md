---
description: Phase 1 — Clean trinity-protocol into a pure shared-types crate. Move product/middleware logic to its proper crate.
---

# Phase 1: Protocol Cleanup

## Objective

`trinity-protocol` is supposed to be shared types only. It currently contains 29 files / ~14,000 lines, much of which is product or middleware logic. Move non-shared types to their proper crates.

## Prerequisites

- Read `docs/TRINITY_IDENTITY.md` for the 3-tier architecture rules
- `cargo check` passes (baseline)

## Analysis — What Belongs Where

### STAYS in trinity-protocol (true shared types)
- `types.rs` — Core Trinity types (AgentEvent, AgentRequest, etc.)
- `state.rs` — GameState, QuestState shared structs
- `lib.rs` — Crate root
- `memory.rs` — Memory entry types
- `bridge.rs` — Bridge trait definitions
- `id_contract.rs` — Identity contract types
- `sidecars.rs` — Sidecar type definitions
- `stream.rs` — Streaming types
- `production.rs` — Production pipeline types
- `ontology.rs` — Ontology definitions
- `tutorial_events.rs` — Tutorial event types
- `diffusion.rs` — Diffusion model types
- `brain.rs` — Brain/config types
- `task.rs` — Task types (if used by Core)

### MOVES to trinity-quest (quest/middleware logic)
- `character_sheet.rs` (1,151 lines) — User profile, portfolio, artifacts
- `pearl.rs` (611 lines) — PEARL evaluation framework
- `qm_rubric.rs` (404 lines) — Quality measurement rubric

### MOVES to trinity-iron-road (ADDIECRAPEYE/middleware logic)
- `sacred_circuitry.rs` (904 lines) — ADDIECRAPEYE circuit definitions
- `semantic_creep.rs` (937 lines) — Scope creep detection
- `vaam_profile.rs` (515 lines) — VAAM vocabulary profiles
- `vocabulary.rs` (682 lines) — Vocabulary word types

### MOVES to trinity-daydream (XR/middleware logic)
- `daydream_commands.rs` (501 lines) — Daydream command types

### MOVES to trinity-mcp-server (MCP-specific logic)
- `trinity_mcp_server.rs` (855 lines) — MCP server types
- `yardmaster_generator.rs` (668 lines) — Quest generator

### REVIEW NEEDED (may stay or move)
- `asset_generation.rs` (876 lines) — Asset generation prompts (Core or Product?)
- `artifact.rs` (328 lines) — Artifact types (shared or product?)
- `profile.rs` (553 lines) — Profile types (shared or middleware?)
- `crate_manual.rs` — Crate documentation (delete or keep?)
- `memory_bridge.rs` (360 lines) — Memory bridge types

## Steps

1. **Audit imports**: For each file listed above, grep all crates to find who imports it:
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && grep -r "trinity_protocol::character_sheet\|trinity_protocol::pearl\|trinity_protocol::sacred_circuitry\|trinity_protocol::semantic_creep\|trinity_protocol::vaam_profile\|trinity_protocol::vocabulary\|trinity_protocol::daydream_commands\|trinity_protocol::trinity_mcp_server\|trinity_protocol::yardmaster_generator" crates/ --include="*.rs" -l
```

2. **Move one file at a time** (start with the clearest cases):
   - Move `character_sheet.rs` to `trinity-quest/src/`
   - Add it to `trinity-quest/src/lib.rs` as `pub mod character_sheet;`
   - Update all imports from `trinity_protocol::character_sheet` to `trinity_quest::character_sheet`
   - Run `cargo check` — fix any errors
   - Commit this single move

3. **Repeat for each file** in the MOVES lists above, one at a time:
   - Move file to target crate's `src/` directory
   - Add module declaration in target crate's `lib.rs`
   - Update all imports across the workspace
   - `cargo check` must pass before moving to next file

4. **For REVIEW NEEDED files**: Check if Core (`crates/trinity`) imports them. If yes, they stay in protocol. If only middleware imports them, move them.

5. **Delete `crate_manual.rs` files** if they're just documentation comments with no code.

## Testing

```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check 2>&1 | grep "^error" | head -20
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo test --workspace 2>&1 | tail -10
```

## Completion Criteria

- `trinity-protocol/src/` contains only shared types (~12-15 files, ~5,000 lines)
- All product/middleware logic moved to appropriate crates
- `cargo check` passes with zero errors
- All tests pass
- `trinity-protocol/Cargo.toml` has no dependencies on other Trinity crates
