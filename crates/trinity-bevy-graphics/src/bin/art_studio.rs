// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — ART Studio Desktop Binary
// ═══════════════════════════════════════════════════════════════════════════════
//
// BINARY:      art_studio
// PURPOSE:     Standalone Bevy window with egui-based ADDIECRAPEYE instructional
//              design workspace + live ART sidecar integration
//
// RUN:         cargo run --bin art_studio --features desktop -p trinity-bevy-graphics
//
// ARCHITECTURE:
//   ┌─────────────────────────────────────────────────────────────────┐
//   │  TOP: 12-station ADDIECRAPEYE navigation rail                  │
//   ├──────────┬──────────────────────────────────┬──────────────────┤
//   │  LEFT    │  CENTER                          │  RIGHT           │
//   │  Project │  Phase-specific workspace        │  ART Sidebar     │
//   │  PEARL   │  (changes per active phase)      │  Image/3D/Music  │
//   │          │                                  │  Sidecar Status  │
//   ├──────────┴──────────────────────────────────┴──────────────────┤
//   │  BOTTOM: Connection status │ XP │ Coal │ Steam │ Chapter       │
//   └─────────────────────────────────────────────────────────────────┘
//
// BEVY VERSION: 0.18.1 (workspace)
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use bevy::window::WindowResolution;
use trinity_bevy_graphics::art_panels::ArtPanelsPlugin;
use trinity_bevy_graphics::bridge::BridgePlugin;
use trinity_bevy_graphics::creative_bridge::CreativeBridgePlugin;

fn main() {
    App::new()
        // ── Core Bevy with windowing ────────────────────────────────
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Trinity ART Studio — Instructional Design Workbench".into(),
                resolution: WindowResolution::new(1440, 900),
                ..default()
            }),
            ..default()
        }))
        // ── Trinity plugins ─────────────────────────────────────────
        .add_plugins(BridgePlugin::default())
        .add_plugins(CreativeBridgePlugin)
        .add_plugins(ArtPanelsPlugin)
        // ── Scene setup (minimal — 3D viewport for asset preview) ──
        .add_systems(Startup, setup_viewport)
        .run();
}

/// Set up a minimal viewport for egui panels.
/// A 3D camera for asset preview can be added later.
fn setup_viewport(mut commands: Commands) {
    // 2D camera for egui panel rendering
    commands.spawn(Camera2d);

    info!("🎬 ART Studio viewport ready — egui panels active");
}
