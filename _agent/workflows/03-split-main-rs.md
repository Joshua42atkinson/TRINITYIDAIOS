---
description: Phase 3 — Split main.rs (5,162 lines) into routes.rs + handlers/ directory. Each handler file under ~1,000 lines.
---

# Phase 3: Split main.rs

## Objective

`main.rs` is 5,162 lines containing routes, handlers, startup logic, system management, and business logic. Split it into a clean structure where `main.rs` is under 500 lines (startup only), routes are in `routes.rs`, and handlers are in a `handlers/` directory.

## Prerequisites

- Phase 2 complete (feature-gated middleware)
- `cargo check` passes

## Target Structure

```
crates/trinity/src/
├── main.rs              (~300 lines — AppState, startup, server bind)
├── routes.rs            (~200 lines — all Router::new() chain)
├── handlers/
│   ├── mod.rs           (~20 lines — re-exports)
│   ├── chat.rs          (~400 lines — /api/chat, SSE stream, yardmaster)
│   ├── creative.rs      (~300 lines — /api/creative/* endpoints)
│   ├── inference.rs     (~200 lines — /api/inference/* hotel/status)
│   ├── quest.rs         (~300 lines — /api/quest/* endpoints, cfg-gated)
│   ├── rag.rs           (~200 lines — /api/rag/* endpoints)
│   ├── memory.rs        (~150 lines — /api/memory/* endpoints)
│   ├── health.rs        (~100 lines — /api/health endpoint)
│   ├── jobs.rs          (~200 lines — /api/jobs/* endpoints)
│   ├── voice.rs         (~150 lines — /api/voice/* endpoints)
│   ├── projects.rs      (~200 lines — /api/projects/* endpoints)
│   ├── export.rs        (~150 lines — /api/eye/export endpoint, cfg-gated)
│   ├── system.rs        (~200 lines — system management, sidecar control)
│   └── daydream.rs      (~100 lines — daydream command forwarding)
```

## Steps

1. **Create `handlers/` directory and `handlers/mod.rs`**:
```bash
mkdir -p /home/joshua/Workflow/TRINITYIDAIOS/crates/trinity/src/handlers
```

2. **Identify handler functions in main.rs**:
   - Search for `async fn` functions that are route handlers
   - Group them by domain (chat, creative, inference, quest, rag, etc.)
   - Note which ones are `#[cfg(feature = "ironroad")]` gated

3. **Move one group at a time** (start with the simplest — health):
   - Cut the handler function from `main.rs`
   - Paste into new file (e.g., `handlers/health.rs`)
   - Add `pub mod health;` to `handlers/mod.rs`
   - Add `mod handlers;` to `main.rs`
   - Update route registration to use `handlers::health::health_check`
   - Run `cargo check` — fix import errors
   - Commit this single move

4. **Repeat for each handler group**:
   - chat handlers (largest — may need to split further)
   - creative handlers
   - inference handlers
   - quest handlers (cfg-gated)
   - rag handlers
   - memory handlers
   - jobs handlers
   - voice handlers
   - project handlers
   - export handlers (cfg-gated)
   - system handlers

5. **Create `routes.rs`**:
   - Move the entire `Router::new().route(...)` chain from `main.rs`
   - This file should only contain route definitions, no handler logic
   - Reference handlers via `handlers::module::function`

6. **Slim down `main.rs`**:
   - Should contain only: AppState struct, main(), server startup, signal handling
   - Target: under 300 lines

7. **Verify the full build**:
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check -p trinity 2>&1 | tail -5
```

## Rules

- **Don't rewrite logic** — move functions as-is, only fix imports
- **One handler group per commit** — easy to revert if something breaks
- **`cargo check` after every move** — never leave the build broken
- **Keep `#[cfg(feature = ...)]` gates** on the moved handlers

## Testing

```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check -p trinity 2>&1 | tail -5
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check -p trinity --no-default-features 2>&1 | tail -5
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo test --workspace 2>&1 | tail -10
```

## Completion Criteria

- `main.rs` is under 500 lines
- `routes.rs` contains all route definitions
- `handlers/` directory contains all handler functions, each file under ~1,000 lines
- `cargo check` passes with and without `ironroad` feature
- No logic changed — only file organization
