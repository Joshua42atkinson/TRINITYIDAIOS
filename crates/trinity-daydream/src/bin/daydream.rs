// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — ART Canvas (Bevy)
// ═══════════════════════════════════════════════════════════════════════════════
//
// BINARY:      art_studio
// PURPOSE:     The PEARL Presentation Layer — DAYDREAM 3D LitRPG world
//
// RUN:         cargo run --bin art_studio --features desktop -p trinity-bevy-graphics
//
// ARCHITECTURE:
//   ┌──────────────────────────────────────────────────────────────┐
//   │  🌙 DAYDREAM 3D World (full-screen immersive)               │
//   │                                                              │
//   │     [PROCEDURAL TERRAIN — Perlin noise heightmap]            │
//   │     [PHYSICS — Rapier3D rigid bodies + colliders]            │
//   │     [CAMERA — PanOrbit with orbit/zoom/rotate]              │
//   │     [LIGHTING — Directional sun + ambient + point accents]  │
//   │     [BEACON — Gold pillar at origin (Trinity waypoint)]     │
//   │                                                              │
//   │  ┌──────────────────────────────────────────────────────────┐│
//   │  │  🎚️ CONTROL RAIL (egui overlay — bottom UI)              ││
//   │  │  [🖼️] [🎵] [🎲] [🎬] [🗣️]  Prompt: [____] [Generate]   ││
//   │  └──────────────────────────────────────────────────────────┘│
//   └──────────────────────────────────────────────────────────────┘
//
// BEVY VERSION: 0.18.1 (workspace)
// DAYDREAM DEPS: bevy_rapier3d 0.33, bevy_panorbit_camera 0.34, noise 0.9
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use bevy::window::WindowResolution;
use trinity_daydream::art_panels::ArtPanelsPlugin;
use trinity_daydream::creative_bridge::CreativeBridgePlugin;
use trinity_daydream::daydream::DaydreamPlugin;
use trinity_daydream::hud::DaydreamHudPlugin;
use trinity_daydream::bridge_client::BridgeClientPlugin;

fn main() {
    App::new()
        // ── Core Bevy with windowing ────────────────────────────────
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Trinity DAYDREAM — LitRPG World".into(),
                    resolution: WindowResolution::new(1440, 900),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: {
                    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                    format!("{}/.local/share/trinity/workspace/assets", home)
                },
                watch_for_changes_override: Some(true),
                ..default()
            })
        )
        // Deep navy background — DAYDREAM sky
        .insert_resource(ClearColor(Color::srgb(0.04, 0.06, 0.12)))
        // ── DAYDREAM 3D world (terrain, physics, camera, commands) ──
        .add_plugins(DaydreamPlugin)
        // ── Trinity plugins ─────────────────────────────────────────
        .add_plugins(CreativeBridgePlugin)
        .add_plugins(ArtPanelsPlugin)
        .add_plugins(DaydreamHudPlugin)
        .add_plugins(BridgeClientPlugin)
        .run();
}
