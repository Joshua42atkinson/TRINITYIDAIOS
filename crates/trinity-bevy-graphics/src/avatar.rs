// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-bevy-graphics/src/avatar.rs
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        avatar.rs
// PURPOSE:     Pete Spirit Crystal avatar — 3D Bevy representation
// ORIGIN:      Ported from crates/archive/trinity-body/src/avatar.rs (Bevy 0.14→0.18)
//
// The Spirit Crystal is Pete's 3D form: an icosphere with
// glowing core, three orbital torus rings, and state-aware animations.
// Colors drawn from the Pete character sheet:
//   Primary:   #CFB991 (Purdue Old Gold)
//   Accent:    #00FFFF (Cyan energy veins)
//   Secondary: #000000 (Black)
//
// ARCHITECTURE:
//   This is a pure Bevy Plugin. It knows nothing about HTTP or servers.
//   The `AvatarState` component is set by external systems (bridge polls).
//
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;

/// Avatar plugin — adds spawn + animation systems
pub struct AvatarPlugin;

impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pete_spirit_crystal)
            .add_systems(
                Update,
                (
                    animate_avatar,
                    animate_orbital_rings,
                    animate_particles,
                    update_avatar_materials,
                ),
            );
    }
}

// ─── Components ──────────────────────────────────────────────────────────────

/// Avatar activity state — set externally by the bridge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Component)]
pub enum AvatarState {
    #[default]
    Idle,
    Thinking,
    Coding,
    Speaking,
    Sleeping,
}

/// Marker: the main Pete Spirit Crystal entity
#[derive(Component)]
pub struct PeteAvatar;

/// Orbital ring decoration
#[derive(Component)]
pub struct OrbitalRing {
    pub speed: f32,
}

/// Floating particle effect
#[derive(Component)]
pub struct AvatarParticle {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub velocity: Vec3,
}

/// Per-frame animation bookkeeping
#[derive(Component)]
pub struct AvatarAnimation {
    pub time: f32,
    pub base_y: f32,
    pub state_transition: f32,
    pub previous_state: AvatarState,
}

/// Handle to the crystal's material for live colour updates
#[derive(Component)]
pub struct AvatarMaterialHandle(pub Handle<StandardMaterial>);

// ─── Pete's colour palette (character_sheet.json) ────────────────────────────

/// Purdue Old Gold  #CFB991
fn gold() -> Color {
    Color::srgb(0.812, 0.725, 0.569)
}

/// Cyan energy veins  #00FFFF
fn cyan() -> Color {
    Color::srgb(0.0, 1.0, 1.0)
}

/// Emissive gold glow (lower intensity for default)
fn gold_emissive() -> Color {
    Color::srgb(0.4, 0.35, 0.25)
}

/// Ring material — translucent cyan
fn ring_color() -> Color {
    Color::srgba(0.0, 0.8, 1.0, 0.45)
}

// ─── Startup system ──────────────────────────────────────────────────────────

fn spawn_pete_spirit_crystal(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let base_y: f32 = 1.5;

    // ── Main crystal ────────────────────────────────────────────────
    let main_mat = materials.add(StandardMaterial {
        base_color: gold(),
        emissive: gold_emissive().into(),
        metallic: 0.9,
        perceptual_roughness: 0.1,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(4).unwrap())),
        MeshMaterial3d(main_mat.clone()),
        Transform::from_xyz(0.0, base_y, 0.0),
        PeteAvatar,
        AvatarState::Idle,
        AvatarAnimation {
            time: 0.0,
            base_y,
            state_transition: 1.0,
            previous_state: AvatarState::Idle,
        },
        AvatarMaterialHandle(main_mat),
    ));

    // ── Inner glow core ─────────────────────────────────────────────
    let core_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: cyan().into(),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.2).mesh().ico(2).unwrap())),
        MeshMaterial3d(core_mat),
        Transform::from_xyz(0.0, base_y, 0.0),
    ));

    // ── Three orbital rings ─────────────────────────────────────────
    let ring_mat = materials.add(StandardMaterial {
        base_color: ring_color(),
        emissive: Color::srgb(0.0, 0.4, 0.6).into(),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    for (i, (radius, speed)) in [(0.7, 1.0f32), (0.9, -0.7), (1.1, 0.5)]
        .iter()
        .enumerate()
    {
        commands.spawn((
            Mesh3d(meshes.add(Torus::new(0.02, *radius))),
            MeshMaterial3d(ring_mat.clone()),
            Transform::from_xyz(0.0, base_y, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.3 * i as f32,
                0.0,
                0.5 * i as f32,
            )),
            OrbitalRing { speed: *speed },
        ));
    }

    info!("🔮 Pete Spirit Crystal spawned — Old Gold + Cyan rings");
}

// ─── Update systems ──────────────────────────────────────────────────────────

fn animate_avatar(
    time: Res<Time>,
    mut query: Query<
        (&mut Transform, &mut AvatarAnimation, &AvatarState),
        With<PeteAvatar>,
    >,
) {
    for (mut tf, mut anim, state) in query.iter_mut() {
        anim.time += time.delta_secs();

        // Smooth state transitions
        if *state != anim.previous_state {
            anim.state_transition = 0.0;
            anim.previous_state = *state;
        }
        anim.state_transition = (anim.state_transition + time.delta_secs() * 2.0).min(1.0);
        let t = ease_out_cubic(anim.state_transition);

        match state {
            AvatarState::Idle => {
                let y_off = (anim.time * 0.5).sin() * 0.1 * t;
                tf.translation.y = anim.base_y + y_off;
                tf.translation.x = lerp(tf.translation.x, 0.0, 0.1);
                tf.rotate_y(time.delta_secs() * 0.2);
                tf.scale = Vec3::lerp(tf.scale, Vec3::ONE, 0.1);
            }
            AvatarState::Thinking => {
                let pulse = 1.0 + (anim.time * 3.0).sin() * 0.08 * t;
                tf.translation.y = anim.base_y + 0.2 * t;
                tf.scale = Vec3::splat(pulse);
                tf.rotate_y(time.delta_secs() * 1.5);
            }
            AvatarState::Coding => {
                let x_off = (anim.time * 25.0).sin() * 0.015 * t;
                let y_off = (anim.time * 30.0).cos() * 0.01 * t;
                tf.translation.x = x_off;
                tf.translation.y = anim.base_y + y_off;
                tf.rotate_y(time.delta_secs() * 3.0);
                tf.scale = Vec3::lerp(tf.scale, Vec3::splat(1.1), 0.1);
            }
            AvatarState::Speaking => {
                let bounce = (anim.time * 6.0).sin().abs() * 0.15 * t;
                tf.translation.y = anim.base_y + bounce;
                tf.translation.x = lerp(tf.translation.x, 0.0, 0.1);
                tf.rotate_y(time.delta_secs() * 0.5);
                let s = 1.0 + (anim.time * 4.0).sin() * 0.05 * t;
                tf.scale = Vec3::splat(s);
            }
            AvatarState::Sleeping => {
                let s = 0.9 + (anim.time * 0.3).sin() * 0.03 * t;
                tf.translation.y = anim.base_y - 0.1 * t;
                tf.scale = Vec3::splat(s);
                tf.rotate_y(time.delta_secs() * 0.05);
            }
        }
    }
}

fn animate_orbital_rings(
    time: Res<Time>,
    avatars: Query<(&Transform, &AvatarState), With<PeteAvatar>>,
    mut rings: Query<(&mut Transform, &OrbitalRing), Without<PeteAvatar>>,
) {
    let (avatar_tf, avatar_state) = match avatars.single() {
        Ok(a) => a,
        Err(_) => return,
    };

    let speed_mult = match avatar_state {
        AvatarState::Idle => 1.0,
        AvatarState::Thinking => 2.5,
        AvatarState::Coding => 4.0,
        AvatarState::Speaking => 1.5,
        AvatarState::Sleeping => 0.2,
    };

    for (mut tf, ring) in rings.iter_mut() {
        tf.translation.y = avatar_tf.translation.y;
        let rot = ring.speed * speed_mult * time.delta_secs();
        tf.rotate_y(rot);
        tf.rotate_x(rot * 0.3);
    }
}

fn animate_particles(
    time: Res<Time>,
    mut particles: Query<(&mut Transform, &mut AvatarParticle)>,
) {
    for (mut tf, mut p) in particles.iter_mut() {
        p.lifetime += time.delta_secs();
        tf.translation += p.velocity * time.delta_secs();
        let alpha = 1.0 - (p.lifetime / p.max_lifetime);
        tf.scale = Vec3::splat(alpha.max(0.0));
    }
}

fn update_avatar_materials(
    avatars: Query<(&AvatarState, &AvatarMaterialHandle), Changed<AvatarState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (state, handle) in avatars.iter() {
        if let Some(mat) = materials.get_mut(&handle.0) {
            let intensity = match state {
                AvatarState::Idle => 1.0f32,
                AvatarState::Thinking => 2.0,
                AvatarState::Coding => 2.5,
                AvatarState::Speaking => 1.8,
                AvatarState::Sleeping => 0.3,
            };
            let base = gold_emissive();
            let c = base.to_srgba();
            mat.emissive = Color::srgb(
                c.red * intensity,
                c.green * intensity,
                c.blue * intensity,
            )
            .into();
        }
    }
}

// ─── Utilities ───────────────────────────────────────────────────────────────

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
