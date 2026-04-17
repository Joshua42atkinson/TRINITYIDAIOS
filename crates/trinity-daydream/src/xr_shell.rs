// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — DAYDREAM XR Shell
// ═══════════════════════════════════════════════════════════════════════════════
//
// PURPOSE:  XR-specific world setup — replaces the desktop windowed shell.
//           No Window, no PanOrbitCamera. OpenXR manages the camera rig via
//           head tracking. We only set up the world geometry and lighting.
//
// SCALE:    1 Bevy unit = 1 meter (OpenXR standard). The user stands inside
//           the DAYDREAM world at human scale. Terrain, NPCs, and quest
//           waypoints are all scaled for room-scale VR/MR.
//
// CAMERA:   bevy_mod_openxr provides the stereo camera rig automatically.
//           Do NOT spawn a Camera3d here — that conflicts with the XR session.
//
// PASSTHROUGH MR:
//           ClearColor is set to Color::NONE (transparent alpha) in the
//           binary entry point. The headset compositor renders the physical
//           camera feed behind our geometry, enabling Mixed Reality.
//
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;

/// Purdue Old Gold — the PEARL's color
const OLD_GOLD: Color = Color::srgb(0.812, 0.725, 0.569);
/// Trinity cyan accent
const CYAN_ACCENT: Color = Color::srgb(0.0, 0.8, 1.0);

/// XR-specific startup: sets up the DAYDREAM world for head-tracked rendering.
///
/// Key differences from the desktop shell:
///   - No Camera3d (OpenXR provides that)
///   - No PanOrbitCamera (head tracking replaces orbit controls)
///   - Scale tuned for room-scale (1 unit = 1 meter)
///   - Floor at Y=0 (user's physical floor plane)
pub fn setup_xr_shell(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── Directional sunlight ─────────────────────────────────────────
    // Dim at dawn (Extracting phase) — will brighten as PEARL evolves
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            color: Color::srgb(1.0, 0.85, 0.7), // Warm dawn light
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));

    // ── Ambient fill light ───────────────────────────────────────────
    commands.spawn((
        PointLight {
            color: Color::srgb(0.4, 0.45, 0.6),
            intensity: 5_000.0,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(0.0, 30.0, 0.0),
    ));

    // ── The Dream Void Floor ─────────────────────────────────────────
    // Dark reflective obsidian — the user stands on this
    let floor_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.01, 0.03, 0.06),
        perceptual_roughness: 0.1,
        metallic: 0.8,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(200.0, 200.0))),
        MeshMaterial3d(floor_mat),
        // Floor at Y=0 — aligns with OpenXR floor reference space
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // ── The Astral Rings ─────────────────────────────────────────────
    // Giant floating rings — sci-fi LitRPG aesthetic
    // Scaled down for room-scale (user is inside the world)
    let ring_mesh1 = meshes.add(Torus::new(40.0, 0.5));
    let ring_mat1 = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: CYAN_ACCENT.into(),
        ..default()
    });
    commands.spawn((
        Mesh3d(ring_mesh1),
        MeshMaterial3d(ring_mat1),
        Transform::from_xyz(0.0, 25.0, -80.0)
            .with_rotation(Quat::from_rotation_x(1.2)),
    ));

    let ring_mesh2 = meshes.add(Torus::new(30.0, 1.0));
    let ring_mat2 = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: OLD_GOLD.into(),
        ..default()
    });
    commands.spawn((
        Mesh3d(ring_mesh2),
        MeshMaterial3d(ring_mat2),
        Transform::from_xyz(0.0, 25.0, -80.0)
            .with_rotation(
                Quat::from_rotation_z(0.5).mul_quat(Quat::from_rotation_x(1.2)),
            ),
    ));

    // ── Dream Orbs (Floating Wisps) ──────────────────────────────────
    // Scattered glowing spheres for depth perception in XR
    let orb_mesh = meshes.add(Sphere::new(0.15));
    let orb_mat_white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::rgb(1.0, 1.5, 3.0),
        ..default()
    });
    let orb_mat_gold = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::rgb(2.5, 2.0, 0.5),
        ..default()
    });

    // Fewer orbs than desktop (performance on mobile XR hardware)
    for i in 0..50 {
        let x = (i as f32 * 23.0 % 100.0) - 50.0;
        let y = (i as f32 * 11.0 % 30.0) + 3.0;
        let z = (i as f32 * 37.0 % 100.0) - 50.0;

        commands.spawn((
            Mesh3d(orb_mesh.clone()),
            MeshMaterial3d(if i % 3 == 0 {
                orb_mat_gold.clone()
            } else {
                orb_mat_white.clone()
            }),
            Transform::from_xyz(x, y, z),
        ));
    }

    info!("🌙 DAYDREAM XR shell ready — room-scale Dream Aesthetic initialized");
    info!("🌙 Head tracking active. Pete will construct the scenario as the PEARL forms.");
}
