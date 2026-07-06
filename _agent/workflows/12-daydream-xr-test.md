---
description: Phase 12 — Test trinity-daydream (Bevy 3D/XR) with OpenXR. Verify build, test VR scene, check FACES integration.
---

# Phase 12: Daydream XR Test

## Objective

`trinity-daydream` (7,200 lines, 22 files) is a Bevy 0.18.1 3D/XR studio. It builds but hasn't been tested with OpenXR. Verify it works, test the VR scene, and check FACES integration.

## Prerequisites

- Phase 3 complete (main.rs split — clean Core)
- `cargo check` passes
- Godot 4.4.1 installed (for asset pipeline)
- XREAL Aura headset (optional — can test desktop mode without it)

## Steps

1. **Verify the build**:
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo check -p trinity-daydream 2>&1 | tail -10
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo build -p trinity-daydream --release 2>&1 | tail -5
```

2. **Run in desktop mode** (no XR headset needed):
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo run -p trinity-daydream --release 2>&1 &
```
- Verify: window opens, 3D scene renders, camera controls work
- Test: can you orbit/pan/zoom?
- Test: does the FACES state display render?

3. **Test Trinity API integration**:
   - Start Trinity server on :3000
   - Verify daydream connects to Trinity API (check `bridge.rs` / `bridge_client.rs`)
   - Test: can daydream send commands to Trinity?
   - Test: can daydream receive FACES states from Trinity?

4. **Test OpenXR mode** (if headset available):
```bash
cd /home/joshua/Workflow/TRINITYIDAIOS && cargo run -p trinity-daydream --release --features xr 2>&1 &
```
- Verify: XR session starts
- Verify: 3D models render in space
- Verify: hand tracking / pinch gestures work
- Verify: FACES panels anchor in physical space

5. **Test asset pipeline**:
   - Generate a 3D model with TRELLIS via ComfyUI
   - Export as glTF
   - Import into daydream
   - Verify it renders

6. **Document what works and what doesn't**:
   - Update `SESSION-HANDOFF.md` with test results
   - Note any OpenXR issues for future fixing

## Files to Review

- `crates/trinity-daydream/src/daydream.rs` (774 lines) — Main app
- `crates/trinity-daydream/src/xr_shell.rs` (6,360 bytes) — OpenXR setup
- `crates/trinity-daydream/src/bridge_client.rs` (6,280 bytes) — Trinity API client
- `crates/trinity-daydream/src/spatial_ui.rs` (24,390 bytes) — Spatial UI panels
- `crates/trinity-daydream/src/vision.rs` (26,051 bytes) — Vision/rendering

## Completion Criteria

- Daydream builds in desktop mode ✅
- Daydream runs and renders 3D scene in desktop mode ✅
- Trinity API bridge works (commands up, FACES down) ✅
- OpenXR mode tested (if headset available) — documented results
- Any bugs found are filed as issues or fixed
