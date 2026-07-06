---
description: Phase 4 — Remove all dead code, stale files, and orphaned artifacts from the workspace.
---

# Phase 4: Dead Code Removal

## Objective

Clean up dead code, stale files, and orphaned artifacts that clutter the workspace and confuse navigation.

## Prerequisites

- Phase 3 complete (main.rs split)
- `cargo check` passes

## Targets

### Dead Code in Core
1. **`conductor_leader.rs`** (1,361 lines) — Already `#![allow(dead_code)]`, not wired into routes. Either:
   - Delete entirely if ADDIECRAPEYE orchestration is dead, OR
   - Move to `trinity-iron-road` if it's still conceptually needed
   - **Decision**: Check if `conductor_leader` is referenced anywhere in route registrations. If not referenced, delete.

2. **Unused variables in `agent.rs`** — 4 warnings:
   - `content_json` (line 1071)
   - `tool_status` (line 1163)
   - `tool_json` (line 1234)
   - `result_json` (line 1310)
   - These are SSE events prepared but never sent. Either wire them up or delete them.

3. **`pete_engine.rs`** (137 lines) — Check if referenced. If not, delete.

4. **`voice_loop.rs`** (29 lines) — Check if referenced. If not, delete.

5. **`music_streamer.rs`** (123 lines) — Check if referenced. If not, delete.

6. **`trinity_api.rs`** (254 lines) — Check if referenced. If not, delete.

### Stale Files
7. **`-.o`** (920 bytes) — Accidental file with dash name. Delete.
```bash
rm /home/joshua/Workflow/TRINITYIDAIOS/-.o
```

8. **`crates/trinity/daydream-x86_64-unknown-linux-gnu`** (1.4GB) — Binary blob in source tree. Delete and add to `.gitignore`.
```bash
rm /home/joshua/Workflow/TRINITYIDAIOS/crates/trinity/daydream-x86_64-unknown-linux-gnu
echo "crates/trinity/daydream-*" >> /home/joshua/Workflow/TRINITYIDAIOS/.gitignore
```

9. **`crates/trinity/crates/trinity_bevy_ui_test/`** — Nested test crate. Check if used. If not, delete.

10. **`LDTAtkinson/`** — Check contents. If product-specific, move to product repo. If empty/stale, delete.

### Old Workflows
11. **`_agent/workflows/`** — These have stale paths (`desktop_trinity/trinity-genesis`). Either update paths or delete and rely on `.windsurf/workflows/` only.

## Steps

1. **Audit each file**: For each target above, grep for references:
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && grep -r "conductor_leader\|pete_engine\|voice_loop\|music_streamer\|trinity_api" crates/trinity/src/ --include="*.rs" -l
```

2. **Delete confirmed dead code** one file at a time:
   - Remove the `mod` declaration from `main.rs` or `lib.rs`
   - Delete the `.rs` file
   - Run `cargo check`
   - Commit

3. **Fix unused variables**: Either prefix with `_` or delete the unused code block.

4. **Delete stale files** (binary blobs, accidental files).

5. **Run clippy to find more dead code**:
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo clippy -p trinity 2>&1 | grep "dead_code\|unused" | head -20
```

## Testing

```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check -p trinity 2>&1 | tail -5
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo clippy -p trinity 2>&1 | grep "^warning" | wc -l
```

## Completion Criteria

- Zero `#![allow(dead_code)]` in Core
- Zero unused variable warnings
- No binary blobs in source tree
- No accidental files (`-.o`, etc.)
- Clippy warnings reduced to under 5
