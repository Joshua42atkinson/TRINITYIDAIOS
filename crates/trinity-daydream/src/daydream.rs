// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — DAYDREAM Plugin (3D LitRPG World)
// ═══════════════════════════════════════════════════════════════════════════════
//
// THE PEARL OF TRINITY
//
// DAYDREAM is a canvas that Pete (Mistral MS4) constructs for each user,
// driven by their PEARL and ADDIECRAPEYE progression. We don't build content —
// we build the FRAMEWORK that Pete uses to build each user's unique world.
//
// HOW IT WORKS:
//   1. User creates a PEARL (subject + medium + vision)
//   2. Pete receives the PEARL in his system prompt
//   3. As user progresses through ADDIECRAPEYE stations (1-12),
//      Pete emits DaydreamBlueprints that shape the 3D world
//   4. The world evolves with the user's learning:
//      - Extracting (ADDIE 1-5): Misty, emerging terrain — wisdom forming
//      - Placing (CRAP 6-9):     Structured paths, landmarks — design taking shape
//      - Refining (EYE 10-12):   Vibrant, alive world — the PEARL manifest
//      - Polished:               The DAYDREAM is complete — user explores freely
//
// ISOLATION:
//   This module is self-contained. Pete can work on DAYDREAM without touching
//   the rest of Trinity. The only interface is DaydreamCommandQueue, fed by
//   the `daydream_command` agent tool via JSON.
//
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
#[cfg(feature = "desktop")]
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use avian3d::prelude::*;
// NOTE: `noise` crate imported at Cargo.toml level for future Perlin terrain.
// Blocked on bevy_mesh re-exports being private in 0.18.1 — using Plane3d for now.

use trinity_protocol::daydream_commands::*;
use trinity_protocol::pearl::PearlPhase;

// ─── Queue Resource ──────────────────────────────────────────────────────────

#[derive(Resource, Default)]
pub struct DaydreamCommandQueue {
    blueprints: Vec<DaydreamBlueprint>,
    commands: Vec<DaydreamCommand>,
}

impl DaydreamCommandQueue {
    pub fn push_blueprint(&mut self, bp: DaydreamBlueprint) {
        self.blueprints.push(bp);
    }
    pub fn push(&mut self, cmd: DaydreamCommand) {
        self.commands.push(cmd);
    }
    pub fn drain_blueprints(&mut self) -> Vec<DaydreamBlueprint> {
        self.blueprints.drain(..).collect()
    }
    pub fn drain_commands(&mut self) -> Vec<DaydreamCommand> {
        self.commands.drain(..).collect()
    }
}

// ─── Components ──────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct DaydreamEntity {
    pub id: DaydreamEntityId,
    pub station: Option<u8>,
}

#[derive(Component)]
pub struct MoveTo {
    pub target: Vec3,
    pub speed: f32,
}

#[derive(Component)]
pub struct QuestWaypoint {
    pub objective_index: Option<u32>,
    pub completed: bool,
}

#[derive(Component)]
pub struct PlayerAvatar;

#[derive(Component)]
pub struct PeteNpc {
    pub persona: String,
}

#[derive(Component)]
pub struct SubjectNpc {
    pub role: String,
    pub vocabulary_domain: Option<String>,
}

// ─── Plugin ──────────────────────────────────────────────────────────────────

/// The DAYDREAM plugin — registers physics, camera, and command processing.
/// All world content is created via DaydreamCommands from Pete, not hardcoded.
pub struct DaydreamPlugin;

impl Plugin for DaydreamPlugin {
    fn build(&self, app: &mut App) {
        // ── Physics engine (shared across all targets) ───────────────
        app.add_plugins(PhysicsPlugins::default());

        // ── Orbit camera controls (desktop only) ─────────────────────
        #[cfg(feature = "desktop")]
        app.add_plugins(PanOrbitCameraPlugin);

        // ── DAYDREAM world state ─────────────────────────────────────
        app.insert_resource(DaydreamCommandQueue::default())
            .insert_resource(DaydreamWorldState::default());

        // ── Startup systems (platform-specific shell) ────────────────
        #[cfg(feature = "desktop")]
        app.add_systems(Startup, setup_daydream_shell);

        #[cfg(feature = "xr")]
        app.add_systems(Startup, crate::xr_shell::setup_xr_shell);

        // ── Frame-by-frame processing (shared across all targets) ────
        app.add_systems(
            Update,
            (process_blueprints, process_commands, animate_move_to),
        );
        app.add_systems(Update, (animate_world_mood, pulse_waypoints));

        // ── Cross-platform plugins (desktop + XR) ────────────────────
        // Spatial UI — native Bevy dashboard panels (no egui)
        app.add_plugins(crate::spatial_ui::SpatialUiPlugin);
        // P-ART-Y Bridge — connects to the AI fleet via HTTP
        app.add_plugins(crate::party_bridge::PartyBridgePlugin::default());

        // ── Desktop-only UI plugins ──────────────────────────────────
        #[cfg(feature = "desktop")]
        {
            app.add_plugins(crate::python_bridge::PythonPlugin)
                .add_plugins(crate::train_car::IsomorphicTrainPlugin)
                .add_plugins(crate::yardmaster_ui::YardmasterUiPlugin)
                .add_plugins(crate::iron_road_ui::IronRoadUiPlugin)
                .add_plugins(crate::sao_menu::SaoMenuPlugin)
                .add_plugins(crate::voice_bridge::VoiceBridgePlugin);
        }
    }
}


// ─── World State ─────────────────────────────────────────────────────────────

/// Tracks the current DAYDREAM world state — synced with PEARL progression.
#[derive(Resource)]
pub struct DaydreamWorldState {
    /// Current PEARL phase — drives atmosphere and world mood
    pub pearl_phase: PearlPhase,
    /// Current world mood (derived from pearl_phase)
    pub mood: WorldMood,
    /// Current terrain theme
    pub terrain_theme: TerrainTheme,
    /// Time of day (0.0=midnight, 0.5=noon) — shifts with mood
    pub time_of_day: f32,
    /// Fog density (0.0=clear, 1.0=dense) — thick during Extracting, clear when Polished
    pub fog_density: f32,
    /// Whether avatar has been spawned
    pub avatar_spawned: bool,
    /// Whether terrain has been generated
    pub terrain_spawned: bool,
    /// Count of concept entities in world
    pub concept_count: u32,
    /// Count of waypoints placed
    pub waypoint_count: u32,
}

impl Default for DaydreamWorldState {
    fn default() -> Self {
        Self {
            pearl_phase: PearlPhase::Extracting,
            mood: WorldMood::Emergence,
            terrain_theme: TerrainTheme::Meadow,
            time_of_day: 0.3,    // Dawn — the beginning
            fog_density: 0.7,    // Dense fog — wisdom not yet formed
            avatar_spawned: false,
            terrain_spawned: false,
            concept_count: 0,
            waypoint_count: 0,
        }
    }
}

// ─── Markers ─────────────────────────────────────────────────────────────────

/// Marker for terrain entities.
#[derive(Component)]
struct TerrainMesh;

/// Marker for the directional sun light.
#[derive(Component)]
struct DaydreamSun;

/// Marker for ambient fog volume.
#[allow(dead_code)]
#[derive(Component)]
struct DaydreamFog;

// ─── Constants ───────────────────────────────────────────────────────────────

/// Purdue Old Gold — the PEARL's color
const OLD_GOLD: Color = Color::srgb(0.812, 0.725, 0.569);
/// Trinity cyan accent
const CYAN_ACCENT: Color = Color::srgb(0.0, 0.8, 1.0);

// ─── Startup: The Empty Canvas ───────────────────────────────────────────────

/// Set up the DAYDREAM shell — just camera and lighting.
/// NO terrain, NO entities, NO content. That's Pete's job.
/// The user enters a misty, empty world. As their PEARL forms,
/// Pete shapes the landscape around their subject.
#[cfg(feature = "desktop")]
fn setup_daydream_shell(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── 3D Camera with orbit controls ────────────────────────────
    // The user can look around the empty world while Pete builds it
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(20.0, 15.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera {
            focus: Vec3::new(0.0, 2.0, 0.0),
            radius: Some(30.0),
            pitch: Some(-0.4),
            yaw: Some(0.8),
            ..default()
        },
    ));

    // ── Directional sunlight (dim — it's dawn, the Extracting phase) ─
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,  // Dim — will brighten as PEARL evolves
            color: Color::srgb(1.0, 0.85, 0.7), // Warm dawn light
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ, -0.8, 0.4, 0.0,
        )),
        DaydreamSun,
    ));

    // ── Ambient fill light (cool, mysterious — Extracting mood) ──────
    commands.spawn((
        PointLight {
            color: Color::srgb(0.4, 0.45, 0.6),
            intensity: 5_000.0,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(0.0, 30.0, 0.0),
    ));

    // ── The Dream Void Floor ──
    // A vast, reflective, dark cyan obsidian surface to catch reflections
    let floor_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.01, 0.03, 0.06),
        perceptual_roughness: 0.1, // very smooth/reflective
        metallic: 0.8,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2000.0, 2000.0))),
        MeshMaterial3d(floor_mat),
        Transform::from_xyz(0.0, -0.5, 0.0), // slightly below the origin to not clip standard entities
    ));

    // ── The Astral Rings (Dream aesthetics) ──
    // Giant floating geometric rings that give a sci-fi / LitRPG "Dream" vibe
    let ring_mesh1 = meshes.add(Torus::new(80.0, 1.0));
    let ring_mat1 = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: CYAN_ACCENT.into(),
        ..default()
    });
    commands.spawn((
        Mesh3d(ring_mesh1),
        MeshMaterial3d(ring_mat1),
        Transform::from_xyz(0.0, 50.0, -150.0).with_rotation(Quat::from_rotation_x(1.2)),
    ));
    
    let ring_mesh2 = meshes.add(Torus::new(60.0, 2.0));
    let ring_mat2 = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: OLD_GOLD.into(),
        ..default()
    });
    commands.spawn((
        Mesh3d(ring_mesh2),
        MeshMaterial3d(ring_mat2),
        Transform::from_xyz(0.0, 50.0, -150.0).with_rotation(Quat::from_rotation_z(0.5).mul_quat(Quat::from_rotation_x(1.2))),
    ));

    // ── Dream Orbs (Floating Wisps) ──
    // Scatter a field of gently glowing spheres to give the void some scale
    let orb_mesh = meshes.add(Sphere::new(0.4));
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
    
    for i in 0..100 {
        let x = (i as f32 * 23.0 % 400.0) - 200.0;
        let y = (i as f32 * 11.0 % 80.0) + 5.0;
        let z = (i as f32 * 37.0 % 400.0) - 200.0;
        
        commands.spawn((
            Mesh3d(orb_mesh.clone()),
            MeshMaterial3d(if i % 3 == 0 { orb_mat_gold.clone() } else { orb_mat_white.clone() }),
            Transform::from_xyz(x, y, z),
        ));
    }

    info!("🌙 DAYDREAM shell ready — Dream Aesthetic Initialized");
    info!("🌙 The background is vast. As the user's PEARL forms, Pete will construct the core scenario.");
}

// ─── Blueprint Processing ────────────────────────────────────────────────────

/// Process DAYDREAM blueprints — these come from Pete at phase transitions.
/// A blueprint is a batch of commands that represent a world evolution step.
fn process_blueprints(
    mut queue: ResMut<DaydreamCommandQueue>,
    mut world_state: ResMut<DaydreamWorldState>,
) {
    let blueprints = queue.drain_blueprints();
    for bp in blueprints {
        info!(
            "🌙 DAYDREAM: Processing blueprint for station {} ({:?} phase, subject: {})",
            bp.station, bp.pearl_phase, bp.subject
        );

        // Update world state from blueprint
        world_state.pearl_phase = bp.pearl_phase;
        world_state.mood = WorldMood::from(bp.pearl_phase);

        // Adjust atmosphere based on phase
        match bp.pearl_phase {
            PearlPhase::Extracting => {
                world_state.fog_density = 0.6;
                world_state.time_of_day = 0.3; // Dawn
            }
            PearlPhase::Placing => {
                world_state.fog_density = 0.3;
                world_state.time_of_day = 0.45; // Late morning
            }
            PearlPhase::Refining => {
                world_state.fog_density = 0.1;
                world_state.time_of_day = 0.5; // High noon
            }
            PearlPhase::Polished => {
                world_state.fog_density = 0.0;
                world_state.time_of_day = 0.55; // Golden afternoon
            }
        }

        // Enqueue all blueprint commands for processing
        for cmd in bp.commands {
            queue.push(cmd);
        }
    }
}

/// Process individual DAYDREAM commands each frame.
fn process_commands(
    mut commands: Commands,
    mut queue: ResMut<DaydreamCommandQueue>,
    mut world_state: ResMut<DaydreamWorldState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    entity_query: Query<(Entity, &DaydreamEntity)>,
    mut waypoint_query: Query<(&mut QuestWaypoint, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let pending = queue.drain_commands();
    for cmd in pending {
        match cmd {
            // ── TERRAIN ──────────────────────────────────────────
            DaydreamCommand::SetTerrain { theme, seed } => {
                if world_state.terrain_spawned {
                    // TODO: Despawn old terrain, regenerate
                    info!("🌙 DAYDREAM: Terrain already exists, would regenerate");
                    world_state.terrain_theme = theme;
                    continue;
                }

                let (color, height_scale) = terrain_style(&theme);
                let terrain_mesh = generate_terrain_mesh(64, 100.0, height_scale, seed);
                let terrain_handle = meshes.add(terrain_mesh);
                let terrain_material = materials.add(StandardMaterial {
                    base_color: color,
                    perceptual_roughness: 0.85,
                    metallic: 0.0,
                    double_sided: true,
                    ..default()
                });

                commands.spawn((
                    Mesh3d(terrain_handle),
                    MeshMaterial3d(terrain_material),
                    Transform::from_xyz(-50.0, 0.0, -50.0),
                    TerrainMesh,
                    RigidBody::Static,
                    Collider::cuboid(100.0, 0.2, 100.0),
                ));

                // Ground water plane
                let water_material = materials.add(StandardMaterial {
                    base_color: Color::srgba(0.05, 0.15, 0.3, 0.5),
                    alpha_mode: AlphaMode::Blend,
                    perceptual_roughness: 0.1,
                    metallic: 0.8,
                    ..default()
                });
                commands.spawn((
                    Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100.0)))),
                    MeshMaterial3d(water_material),
                    Transform::from_xyz(0.0, -0.5, 0.0),
                ));

                world_state.terrain_spawned = true;
                world_state.terrain_theme = theme;
                info!("🌙 DAYDREAM: Terrain spawned (seed: {})", seed);
            }

            // ── ATMOSPHERE ───────────────────────────────────────
            DaydreamCommand::SetAtmosphere { time_of_day, fog_density, mood } => {
                world_state.time_of_day = time_of_day.clamp(0.0, 1.0);
                world_state.fog_density = fog_density.clamp(0.0, 1.0);
                world_state.mood = mood;
                info!("🌙 DAYDREAM: Atmosphere updated");
            }

            // ── CONCEPT ENTITIES ─────────────────────────────────
            DaydreamCommand::SpawnConcept { id, label, position, mesh_type, station, python_script } => {
                let mesh = match mesh_type {
                    MeshType::Cube => meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                    MeshType::Sphere => meshes.add(Sphere::new(0.5)),
                    MeshType::Capsule => meshes.add(Capsule3d::new(0.3, 1.0)),
                    MeshType::Cylinder => meshes.add(Cylinder::new(0.5, 1.0)),
                    MeshType::Plane => meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(5.0))),
                    MeshType::Gltf { .. } => {
                        info!("🌙 DAYDREAM: glTF mesh pending — using sphere placeholder");
                        meshes.add(Sphere::new(0.5))
                    }
                };

                let material = materials.add(StandardMaterial {
                    base_color: CYAN_ACCENT,
                    metallic: 0.5,
                    perceptual_roughness: 0.4,
                    emissive: LinearRgba::new(0.0, 0.3, 0.5, 1.0),
                    ..default()
                });

                let mut spawn_cmd = commands.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(material),
                    Transform::from_xyz(position[0], position[1], position[2]),
                    DaydreamEntity { id: id.clone(), station },
                    RigidBody::Dynamic,
                    LinearVelocity::default(),
                    Collider::cuboid(1.0, 1.0, 1.0),
                    Name::new(label.clone()),
                ));

                // If Pete generated a Python script for this entity, attach the interpreter bridge
                // (desktop only — PyO3 not available on Android XR)
                #[cfg(feature = "desktop")]
                if let Some(script_code) = python_script {
                    spawn_cmd.insert(crate::python_bridge::PythonScript {
                        source: script_code,
                    });
                    info!("🐍 DAYDREAM: Python script attached to '{}'", label);
                }
                #[cfg(not(feature = "desktop"))]
                let _ = python_script; // Suppress unused variable warning in XR build

                world_state.concept_count += 1;
                info!("🌙 DAYDREAM: Concept '{}' spawned (total: {})", label, world_state.concept_count);
            }

            // ── DESPAWN ──────────────────────────────────────────
            DaydreamCommand::DespawnEntity { id } => {
                for (entity, de) in entity_query.iter() {
                    if de.id == id {
                        commands.entity(entity).despawn();
                        info!("🌙 DAYDREAM: Despawned {:?}", id);
                        break;
                    }
                }
            }

            // ── MOVE ─────────────────────────────────────────────
            DaydreamCommand::MoveEntity { id, target, speed } => {
                for (entity, de) in entity_query.iter() {
                    if de.id == id {
                        commands.entity(entity).insert(MoveTo {
                            target: Vec3::new(target[0], target[1], target[2]),
                            speed,
                        });
                        break;
                    }
                }
            }

            // ── AVATAR ───────────────────────────────────────────
            DaydreamCommand::SpawnAvatar { position } => {
                if world_state.avatar_spawned {
                    info!("🌙 DAYDREAM: Avatar already exists");
                    continue;
                }

                let material = materials.add(StandardMaterial {
                    base_color: OLD_GOLD,
                    metallic: 0.7,
                    perceptual_roughness: 0.3,
                    emissive: LinearRgba::new(0.812, 0.725, 0.569, 1.0) * 0.5,
                    ..default()
                });

                commands.spawn((
                    Mesh3d(meshes.add(Capsule3d::new(0.4, 1.2))),
                    MeshMaterial3d(material),
                    Transform::from_xyz(position[0], position[1] + 1.0, position[2]),
                    PlayerAvatar,
                    RigidBody::Dynamic,
                    Collider::capsule(0.4, 1.2),
                    Name::new("Yardmaster"),
                ));

                // Accent light on avatar
                commands.spawn((
                    PointLight {
                        color: OLD_GOLD,
                        intensity: 15_000.0,
                        range: 15.0,
                        ..default()
                    },
                    Transform::from_xyz(position[0], position[1] + 3.0, position[2]),
                ));

                world_state.avatar_spawned = true;
                info!("🌙 DAYDREAM: Yardmaster avatar spawned");
            }

            // ── PETE NPC ─────────────────────────────────────────
            DaydreamCommand::SpawnPeteNpc { position, persona } => {
                let color = if persona.contains("Recycler") {
                    Color::srgb(0.7, 0.5, 0.9) // Purple for Great Recycler
                } else {
                    Color::srgb(0.3, 0.7, 0.4) // Green for Programmer Pete
                };

                let material = materials.add(StandardMaterial {
                    base_color: color,
                    metallic: 0.4,
                    perceptual_roughness: 0.5,
                    emissive: LinearRgba::new(0.3, 0.2, 0.5, 1.0),
                    ..default()
                });

                commands.spawn((
                    Mesh3d(meshes.add(Capsule3d::new(0.35, 1.0))),
                    MeshMaterial3d(material),
                    Transform::from_xyz(position[0], position[1] + 0.85, position[2]),
                    PeteNpc { persona: persona.clone() },
                    Name::new(format!("Pete ({})", persona)),
                ));

                info!("🌙 DAYDREAM: Pete NPC spawned as {}", persona);
            }

            // ── SUBJECT NPC ──────────────────────────────────────
            DaydreamCommand::SpawnSubjectNpc { id, name, role, position, vocabulary_domain } => {
                let material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.4, 0.6, 0.8),
                    metallic: 0.3,
                    perceptual_roughness: 0.5,
                    ..default()
                });

                commands.spawn((
                    Mesh3d(meshes.add(Capsule3d::new(0.35, 1.0))),
                    MeshMaterial3d(material),
                    Transform::from_xyz(position[0], position[1] + 0.85, position[2]),
                    DaydreamEntity { id, station: None },
                    SubjectNpc { role, vocabulary_domain },
                    Name::new(name.clone()),
                ));

                info!("🌙 DAYDREAM: Subject NPC '{}' spawned", name);
            }

            // ── NPC SPEECH ───────────────────────────────────────
            DaydreamCommand::NpcSpeak { id, text, emotion } => {
                info!("🌙 DAYDREAM: NPC {:?} speaks: \"{}\" (emotion: {:?})", id, text, emotion);
                // TODO: Wire to TTS (Supertonic-2) and floating 3D text
            }

            // ── QUEST WAYPOINTS ──────────────────────────────────
            DaydreamCommand::PlaceWaypoint { id, label, position, objective_index } => {
                let material = materials.add(StandardMaterial {
                    base_color: OLD_GOLD,
                    emissive: LinearRgba::new(0.812, 0.725, 0.569, 1.0) * 1.5,
                    metallic: 0.9,
                    perceptual_roughness: 0.2,
                    ..default()
                });

                commands.spawn((
                    Mesh3d(meshes.add(Cylinder::new(0.2, 4.0))),
                    MeshMaterial3d(material),
                    Transform::from_xyz(position[0], position[1] + 2.0, position[2]),
                    DaydreamEntity { id, station: None },
                    QuestWaypoint { objective_index, completed: false },
                    Name::new(label.clone()),
                ));

                // Waypoint light
                commands.spawn((
                    PointLight {
                        color: OLD_GOLD,
                        intensity: 30_000.0,
                        range: 20.0,
                        ..default()
                    },
                    Transform::from_xyz(position[0], position[1] + 5.0, position[2]),
                ));

                world_state.waypoint_count += 1;
                info!("🌙 DAYDREAM: Waypoint '{}' placed (objective: {:?})", label, objective_index);
            }

            DaydreamCommand::CompleteWaypoint { id } => {
                for (entity, de) in entity_query.iter() {
                    if de.id == id {
                        // Mark as completed — visual change handled by pulse_waypoints
                        if let Ok((mut wp, _mat)) = waypoint_query.get_mut(entity) {
                            wp.completed = true;
                            info!("🌙 DAYDREAM: Waypoint {:?} completed!", id);
                        }
                        break;
                    }
                }
            }

            // ── CAMERA ───────────────────────────────────────────
            DaydreamCommand::FocusCamera { target: _ } => {
                info!("🌙 DAYDREAM: Camera focus requested");
                // TODO: Animate PanOrbitCamera to entity
            }

            // ── TIER 3 SENSORY & UI ──────────────────────────────
            DaydreamCommand::PlaySound { src, spatial_entity } => {
                info!("🌙 DAYDREAM: PlaySound requested: {} (entity: {:?})", src, spatial_entity);
            }
            DaydreamCommand::AnimateEntity { id, clip, loop_anim } => {
                info!("🌙 DAYDREAM: AnimateEntity requested: {:?} with {} (loop: {})", id, clip, loop_anim);
            }
            DaydreamCommand::SpawnUiButton { id, text, position, event_trigger } => {
                info!("🌙 DAYDREAM: SpawnUiButton requested: {:?} ({}) at {:?} triggering {}", id, text, position, event_trigger);
            }
            DaydreamCommand::SpawnDialogueTree { id, npc_id, nodes } => {
                info!("🌙 DAYDREAM: SpawnDialogueTree requested: {:?} on {:?} with {} nodes", id, npc_id, nodes.len());
            }
            // ── TCG HOOK DECK ────────────────────────────────────
            // Removed CastHook: Pete now completely generates Bevy mechanics rather than relying on hardcoded logic.
            DaydreamCommand::LogMessage { msg } => {
                info!("🌙 DAYDREAM Pete: {}", msg);
            }
        }
    }
}

// ─── Animation Systems ──────────────────────────────────────────────────────

/// Smoothly move entities toward their MoveTo target.
fn animate_move_to(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &MoveTo)>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, move_to) in query.iter_mut() {
        let direction = move_to.target - transform.translation;
        let distance = direction.length();

        if distance < 0.1 {
            transform.translation = move_to.target;
            commands.entity(entity).remove::<MoveTo>();
        } else {
            let step = direction.normalize() * move_to.speed * dt;
            if step.length() > distance {
                transform.translation = move_to.target;
                commands.entity(entity).remove::<MoveTo>();
            } else {
                transform.translation += step;
            }
        }
    }
}

/// Update world atmosphere based on PEARL phase — lighting and mood evolve.
fn animate_world_mood(
    world_state: Res<DaydreamWorldState>,
    mut sun_query: Query<&mut DirectionalLight, With<DaydreamSun>>,
) {
    let t = world_state.time_of_day;

    // Sun intensity: brighter as PEARL evolves
    let sun_intensity = ((t * std::f32::consts::TAU).sin() * 0.5 + 0.5) * 10_000.0 + 500.0;

    // Fog reduces light — world clears as PEARL advances
    let fog_factor = 1.0 - (world_state.fog_density * 0.5);

    for mut light in sun_query.iter_mut() {
        light.illuminance = sun_intensity * fog_factor;
    }
}

/// Pulse quest waypoints — uncompleted glow gold, completed glow green.
fn pulse_waypoints(
    time: Res<Time>,
    mut query: Query<(&QuestWaypoint, &mut Transform)>,
) {
    let t = time.elapsed_secs();
    for (wp, mut transform) in query.iter_mut() {
        if !wp.completed {
            // Gentle pulse for active waypoints
            let scale = 1.0 + (t * 1.5).sin() * 0.1;
            transform.scale = Vec3::splat(scale);
        }
    }
}

// ─── Terrain Generation ──────────────────────────────────────────────────────

/// Get terrain color and height scale based on theme (driven by PEARL subject).
fn terrain_style(theme: &TerrainTheme) -> (Color, f32) {
    match theme {
        TerrainTheme::Meadow => (Color::srgb(0.25, 0.55, 0.2), 6.0),
        TerrainTheme::Crystal => (Color::srgb(0.3, 0.5, 0.7), 10.0),
        TerrainTheme::Ruins => (Color::srgb(0.6, 0.5, 0.35), 5.0),
        TerrainTheme::Geometric => (Color::srgb(0.5, 0.5, 0.6), 8.0),
        TerrainTheme::Garden => (Color::srgb(0.2, 0.6, 0.3), 4.0),
        TerrainTheme::Ethereal => (Color::srgb(0.6, 0.4, 0.8), 12.0),
        TerrainTheme::Custom { .. } => (Color::srgb(0.4, 0.4, 0.5), 7.0),
    }
}

/// Generate terrain mesh for the DAYDREAM world.
///
/// Phase 1: Simple flat plane (Bevy 0.18.1 mesh internals are private).
/// Phase 2: Full Perlin noise heightmap when `bevy_mesh` is added as
///          an explicit workspace dependency, unlocking `Indices` and
///          `PrimitiveTopology` for custom mesh construction.
fn generate_terrain_mesh(_resolution: u32, size: f32, _height_scale: f32, _seed: u32) -> Mesh {
    Plane3d::new(Vec3::Y, Vec2::splat(size / 2.0)).into()
}
