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

/// Set up a minimal 3D viewport for future asset preview.
/// The egui panels overlay this viewport.
fn setup_viewport(mut commands: Commands) {
    // Camera (looking at origin — will be used for 3D asset preview later)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ambient light
    commands.spawn(AmbientLight {
        color: Color::srgb(0.12, 0.12, 0.18),
        brightness: 150.0,
        ..default()
    });

    info!("🎬 ART Studio viewport ready");
}
