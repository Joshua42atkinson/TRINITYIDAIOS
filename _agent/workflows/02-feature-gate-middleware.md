---
description: Phase 2 — Feature-gate Middleware dependencies in Core's Cargo.toml so Core builds without quest/iron-road/voice crates.
---

# Phase 2: Feature-Gate Middleware Dependencies

## Objective

Core's `Cargo.toml` currently depends on `trinity-quest`, `trinity-iron-road`, and `trinity-voice`. Per the 3-tier architecture, Core must NEVER depend on Middleware. We feature-gate these deps so Core builds clean without them, and they're only included when the `ironroad` feature is enabled.

## Prerequisites

- Phase 1 complete (protocol cleanup done)
- `cargo check` passes

## Steps

1. **Add feature flags to `crates/trinity/Cargo.toml`**:
```toml
[features]
default = ["ironroad"]
ironroad = ["dep:trinity-quest", "dep:trinity-iron-road", "dep:trinity-voice"]
export = ["dep:docx-rs", "dep:zip"]
```

2. **Change the dependency declarations to optional**:
```toml
trinity-quest = { path = "../trinity-quest", optional = true }
trinity-iron-road = { path = "../trinity-iron-road", optional = true }
trinity-voice = { path = "../trinity-voice", optional = true }
docx-rs = { version = "0.4", optional = true }
zip = { version = "0.6", optional = true }
```

3. **Gate all Middleware imports in Core source files**:
   - In `main.rs`, wrap middleware imports:
     ```rust
     #[cfg(feature = "ironroad")]
     mod quests;
     #[cfg(feature = "ironroad")]
     mod conductor_leader;
     #[cfg(feature = "ironroad")]
     mod narrative;
     #[cfg(feature = "ironroad")]
     mod vaam;
     #[cfg(feature = "ironroad")]
     mod vaam_bridge;
     #[cfg(feature = "ironroad")]
     mod skills;
     #[cfg(feature = "ironroad")]
     mod journal;
     #[cfg(feature = "ironroad")]
     mod character_sheet;
     #[cfg(feature = "ironroad")]
     mod character_api;
     ```
   - In `agent.rs`, gate quest/skill/vaam imports
   - In `tools.rs`, gate quest-related tools
   - In `export.rs` and `eye_container.rs`, gate with `#[cfg(feature = "export")]`

4. **Gate all Middleware route registrations in `main.rs`**:
   - Wrap quest routes, character routes, journal routes with `#[cfg(feature = "ironroad")]`
   - Wrap export routes with `#[cfg(feature = "export")]`

5. **Build without features to verify Core is clean**:
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check -p trinity --no-default-features 2>&1 | grep "^error" | head -20
```

6. **Build with default features to verify everything still works**:
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check -p trinity 2>&1 | grep "^error" | head -20
```

## Files to Edit

- `crates/trinity/Cargo.toml` — Add features, make deps optional
- `crates/trinity/src/main.rs` — Gate module declarations and route registrations
- `crates/trinity/src/agent.rs` — Gate middleware imports
- `crates/trinity/src/tools.rs` — Gate quest-related tools
- `crates/trinity/src/export.rs` — Gate with export feature
- `crates/trinity/src/eye_container.rs` — Gate with export feature

## Testing

```bash
# Core builds without middleware
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check -p trinity --no-default-features 2>&1 | tail -5

# Core builds with middleware (default)
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check -p trinity 2>&1 | tail -5

# Full workspace
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo test --workspace 2>&1 | tail -10
```

## Completion Criteria

- `cargo check -p trinity --no-default-features` passes (Core has zero Middleware deps)
- `cargo check -p trinity` passes (with `ironroad` feature, everything works)
- `cargo check -p trinity --no-default-features --features export` passes
- No `#[allow(dead_code)]` needed for the gated-out code
