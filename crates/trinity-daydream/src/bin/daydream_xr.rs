// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — DAYDREAM XR Binary
// ═══════════════════════════════════════════════════════════════════════════════
//
// BINARY:      daydream_xr
// PURPOSE:     XR headset entry point — renders DAYDREAM into OpenXR session
//
// BUILD:
//   Desktop (OpenXR simulator — SteamVR/Monado):
//     cargo run --bin daydream_xr --features xr -p trinity-daydream --release
//
//   Android (Quest 3 APK — via cargo-ndk):
//     cargo ndk -t arm64-v8a -o android/app/src/main/jniLibs \
//       build --release --bin daydream_xr --features xr -p trinity-daydream
//
// ARCHITECTURE:
//   ┌──────────────────────────────────────────────────────────────┐
//   │  🌙 DAYDREAM XR (Full immersive / Mixed Reality)            │
//   │                                                              │
//   │     [TERRAIN — same DaydreamCommand protocol]                │
//   │     [PHYSICS — avian3d XPBD]                                │
//   │     [CAMERA — OpenXR head tracking (no orbit)]              │
//   │     [LIGHTING — Same atmospheric system as desktop]          │
//   │     [UI — Native Bevy spatial panels (no egui)]             │
//   │                                                              │
//   │  ← HTTP → Trinity Axum :3000 (P-ART-Y fleet on desktop)    │
//   └──────────────────────────────────────────────────────────────┘
//
// BEVY VERSION: 0.18.1 (workspace)
// XR DEPS: bevy_mod_openxr 0.5, bevy_mod_xr 0.5
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use bevy_mod_openxr::add_xr_plugins;
use trinity_daydream::daydream::DaydreamPlugin;

fn main() {
    App::new()
        // ── XR Plugin Suite ─────────────────────────────────────────
        // add_xr_plugins wraps DefaultPlugins and replaces the windowing
        // backend with the OpenXR session lifecycle. On Android, this
        // interfaces with libopenxr_loader.so via the Vulkan backend.
        .add_plugins(add_xr_plugins(DefaultPlugins))
        // ── Transparent background for Mixed Reality passthrough ─────
        // Alpha = 0 signals the headset compositor to render the
        // camera feed behind our geometry (Mixed Reality mode).
        // For full VR, change to Color::srgb(0.04, 0.06, 0.12).
        .insert_resource(ClearColor(Color::NONE))
        // ── DAYDREAM world (terrain, physics, commands, animation) ───
        // The DaydreamPlugin detects the `xr` feature at compile time
        // and uses setup_xr_shell instead of setup_daydream_shell.
        .add_plugins(DaydreamPlugin)
        // ── Run ─────────────────────────────────────────────────────
        .run();
}
