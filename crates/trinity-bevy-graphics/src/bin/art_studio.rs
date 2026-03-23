// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — ART Canvas (Bevy)
// ═══════════════════════════════════════════════════════════════════════════════
//
// BINARY:      art_studio
// PURPOSE:     The PEARL Presentation Layer — an immersive aesthetic canvas
//              for creative content display + generation
//
// RUN:         cargo run --bin art_studio --features desktop -p trinity-bevy-graphics
//
// PHILOSOPHY:
//   ART is the HEART of Trinity. It doesn't think or plan — it BREATHES.
//   It inhales experiences (images, music, 3D, video, voice) and exhales
//   the aesthetic vibe that makes the Iron Road feel REAL.
//
//   The canvas takes 90% of the window. Controls take 10%.
//   Previous build was 90% controls, 10% canvas. INVERTED.
//
// ARCHITECTURE:
//   ┌──────────────────────────────────────────────────────────────┐
//   │  🎨 ART CANVAS (full-screen immersive)                      │
//   │                                                              │
//   │     [AMBIENT PARTICLES — steam wisps, gold sparks]           │
//   │                                                              │
//   │          ┌──────────────────────────────┐                    │
//   │          │   GENERATED ART / TEXTURE    │                    │
//   │          │   (image, 3D preview, etc.)  │                    │
//   │          └──────────────────────────────┘                    │
//   │                                                              │
//   │  ┌──────────────────────────────────────────────────────────┐│
//   │  │  🎚️ CONTROL RAIL (egui overlay — bottom 10%)             ││
//   │  │  [🖼️] [🎵] [🎲] [🎬] [🗣️]  Prompt: [____] [Generate]   ││
//   │  └──────────────────────────────────────────────────────────┘│
//   └──────────────────────────────────────────────────────────────┘
//
// BEVY VERSION: 0.18.1 (workspace)
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use bevy::window::WindowResolution;
use trinity_bevy_graphics::art_panels::ArtPanelsPlugin;
use trinity_bevy_graphics::creative_bridge::CreativeBridgePlugin;

fn main() {
    App::new()
        // ── Core Bevy with windowing ────────────────────────────────
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Trinity ART — Aesthetic Research Tempo".into(),
                resolution: WindowResolution::new(1440, 900),
                ..default()
            }),
            ..default()
        }))
        // Deep navy background — particles pop against this
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.08)))
        // ── Trinity plugins ─────────────────────────────────────────
        .add_plugins(CreativeBridgePlugin)
        .add_plugins(ArtPanelsPlugin)
        // ── Scene setup: ambient canvas with particles ──────────────
        .add_systems(Startup, setup_canvas)
        .add_systems(Update, (animate_particles, pulse_glow, drift_title))
        .run();
}

/// Marker for the floating title text.
#[derive(Component)]
struct CanvasTitle;

// ─── Components ──────────────────────────────────────────────────────────────

/// Marker for ambient particle entities.
#[derive(Component)]
struct AmbientParticle {
    velocity: Vec2,
    lifetime: f32,
    max_lifetime: f32,
}

/// Marker for the central art display sprite.
#[derive(Component)]
struct ArtDisplay;

/// Marker for the pulsing glow ring.
#[derive(Component)]
struct GlowRing {
    phase: f32,
}

// ─── Constants ───────────────────────────────────────────────────────────────

/// Purdue Old Gold
#[allow(dead_code)]
const OLD_GOLD: Color = Color::srgb(0.812, 0.725, 0.569);
/// Cyan accent
const CYAN_ACCENT: Color = Color::srgba(0.0, 1.0, 1.0, 0.5);
/// Dark background
#[allow(dead_code)]
const CANVAS_BG: Color = Color::srgb(0.04, 0.04, 0.08);
/// Number of ambient particles
const PARTICLE_COUNT: usize = 80;

// ─── Setup ───────────────────────────────────────────────────────────────────

/// Create the immersive canvas: camera, background, particles, art display.
fn setup_canvas(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2d, Transform::default()));

    // ── Centered title text ──────────────────────────────────────
    commands.spawn((
        Text2d::new("TRINITY ART"),
        TextFont {
            font_size: 72.0,
            ..default()
        },
        TextColor(Color::srgba(0.812, 0.725, 0.569, 0.6)),
        Transform::from_xyz(0.0, 100.0, 2.0),
        CanvasTitle,
    ));

    // Subtitle
    commands.spawn((
        Text2d::new("Aesthetic Research Tempo"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgba(0.0, 0.9, 1.0, 0.4)),
        Transform::from_xyz(0.0, 40.0, 2.0),
    ));

    // Tagline
    commands.spawn((
        Text2d::new("— The PEARL Presentation Layer —"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgba(0.6, 0.6, 0.7, 0.35)),
        Transform::from_xyz(0.0, 5.0, 2.0),
    ));

    // ── Ambient particles (steam wisps + golden sparks) ──────────
    for i in 0..PARTICLE_COUNT {
        let angle = (i as f32 / PARTICLE_COUNT as f32) * std::f32::consts::TAU;
        let radius = 80.0 + (i as f32 * 8.5) % 500.0;
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        // 2/3 gold sparks, 1/3 cyan wisps
        let is_gold = i % 3 != 0;
        let color = if is_gold {
            Color::srgba(0.85, 0.75, 0.55, 0.8) // Bright Old Gold
        } else {
            Color::srgba(0.0, 0.9, 1.0, 0.5) // Cyan wisp, visible
        };

        let size = if is_gold {
            4.0 + (i as f32 * 0.31) % 4.0  // 4–8px gold sparks
        } else {
            8.0 + (i as f32 * 0.47) % 8.0  // 8–16px cyan wisps
        };
        let speed = 8.0 + (i as f32 * 3.7) % 25.0;
        let vx = ((i as f32 * 1.618) % 2.0 - 1.0) * speed;
        let vy = ((i as f32 * 2.718) % 2.0 - 1.0) * speed * 0.4 + 8.0; // drift upward

        let lifetime = 4.0 + (i as f32 * 0.37) % 6.0;

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_xyz(x, y, 1.0),
            AmbientParticle {
                velocity: Vec2::new(vx, vy),
                lifetime,
                max_lifetime: lifetime,
            },
        ));
    }

    // ── Central glow ring (pulsing orbital) ───────────────────────
    commands.spawn((
        Sprite {
            color: CYAN_ACCENT,
            custom_size: Some(Vec2::new(350.0, 350.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 60.0, 0.5),
        GlowRing { phase: 0.0 },
    ));

    // ── Placeholder art display frame ─────────────────────────────
    commands.spawn((
        Sprite {
            color: Color::srgba(0.12, 0.12, 0.18, 0.6),
            custom_size: Some(Vec2::new(520.0, 520.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 60.0, 0.0),
        ArtDisplay,
    ));

    info!("🎨 ART Canvas ready — Aesthetic Research Tempo active");
}

// ─── Animation Systems ──────────────────────────────────────────────────────

/// Animate ambient particles — drift, fade, respawn.
fn animate_particles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut AmbientParticle, &mut Sprite)>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut particle, mut sprite) in query.iter_mut() {
        // Move
        transform.translation.x += particle.velocity.x * dt;
        transform.translation.y += particle.velocity.y * dt;

        // Age
        particle.lifetime -= dt;

        // Fade based on lifetime (fade in first 20%, fade out last 30%)
        let life_ratio = particle.lifetime / particle.max_lifetime;
        let alpha = if life_ratio > 0.8 {
            (1.0 - life_ratio) * 5.0 // fade in
        } else if life_ratio < 0.3 {
            life_ratio / 0.3 // fade out
        } else {
            1.0
        };

        let base_alpha = if sprite.color.to_srgba().green > 0.5 {
            0.5 // cyan wisps visible
        } else {
            0.8 // gold sparks bright
        };
        sprite.color = sprite.color.with_alpha(base_alpha * alpha);

        // Respawn when dead
        if particle.lifetime <= 0.0 {
            particle.lifetime = particle.max_lifetime;
            // Reset position near center with some spread
            let hash = (transform.translation.x * 100.0) as i32;
            let angle = (hash as f32 * 0.618) % std::f32::consts::TAU;
            let radius = 50.0 + (hash.unsigned_abs() as f32 % 200.0);
            transform.translation.x = angle.cos() * radius;
            transform.translation.y = angle.sin() * radius;
        }
    }
}

/// Pulse the glow ring — breathing effect.
fn pulse_glow(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut GlowRing, &mut Sprite)>,
) {
    for (mut transform, mut glow, mut sprite) in query.iter_mut() {
        glow.phase += time.delta_secs() * 0.8; // slow breathe
        let scale = 1.0 + (glow.phase.sin() * 0.15); // ±15% scale
        transform.scale = Vec3::splat(scale);

        let alpha = 0.15 + (glow.phase.sin() * 0.5 + 0.5) * 0.25; // 0.15 to 0.40
        sprite.color = CYAN_ACCENT.with_alpha(alpha);
    }
}

/// Gentle floating motion for the title text.
fn drift_title(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<CanvasTitle>>,
) {
    let t = time.elapsed_secs();
    for mut transform in query.iter_mut() {
        // Gentle sine bob — 3px up/down over 4 seconds
        transform.translation.y = 100.0 + (t * 0.5).sin() * 3.0;
    }
}
