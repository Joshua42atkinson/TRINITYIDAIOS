// environment_manager.rs — Trinity XR Scene State Machine
// Lesson environments for instructional design:
// - Classroom: Traditional classroom layout
// - Lab: Science lab with equipment
// - FieldTrip: Outdoor exploration space

use bevy::prelude::*;

pub struct EnvironmentManagerPlugin;

impl Plugin for EnvironmentManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SceneState>();
        app.add_systems(OnEnter(SceneState::Classroom), spawn_classroom);
        app.add_systems(OnExit(SceneState::Classroom), despawn_scene);
        app.add_systems(OnEnter(SceneState::Lab), spawn_lab);
        app.add_systems(OnExit(SceneState::Lab), despawn_scene);
        app.add_systems(OnEnter(SceneState::FieldTrip), spawn_field_trip);
        app.add_systems(OnExit(SceneState::FieldTrip), despawn_scene);
        app.add_systems(Startup, set_initial_scene);
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneState {
    #[default]
    Classroom,
    Lab,
    FieldTrip,
}

#[derive(Component)]
pub struct SceneRoot;

fn set_initial_scene(mut next_state: ResMut<NextState<SceneState>>) {
    next_state.set(SceneState::Classroom);
}

fn despawn_scene(mut commands: Commands, query: Query<Entity, With<SceneRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn spawn_classroom(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.18),
        perceptual_roughness: 0.5,
        ..default()
    });

    let root = commands
        .spawn((SceneRoot, Transform::default(), Visibility::default()))
        .id();

    let floor = commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
            MeshMaterial3d(floor_mat),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
    commands.entity(root).add_child(floor);

    let sun = commands
        .spawn((
            DirectionalLight {
                illuminance: 8000.0,
                shadows_enabled: true,
                color: Color::srgb(1.0, 0.95, 0.9),
                ..default()
            },
            Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
        ))
        .id();
    commands.entity(root).add_child(sun);

    let fill = commands
        .spawn((
            PointLight {
                color: Color::srgb(0.5, 0.6, 1.0),
                intensity: 5_000.0,
                range: 50.0,
                ..default()
            },
            Transform::from_xyz(0.0, 5.0, 0.0),
        ))
        .id();
    commands.entity(root).add_child(fill);

    let desk_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.2, 0.1),
        perceptual_roughness: 0.7,
        ..default()
    });

    for i in 0..4 {
        let x = -3.0 + (i as f32) * 2.0;
        let desk = commands
            .spawn((
                Mesh3d(meshes.add(Cuboid::new(1.2, 0.05, 0.6))),
                MeshMaterial3d(desk_mat.clone()),
                Transform::from_xyz(x, 0.75, -1.0),
            ))
            .id();
        commands.entity(root).add_child(desk);
    }
}

fn spawn_lab(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.1, 0.12),
        perceptual_roughness: 0.3,
        metallic: 0.5,
        ..default()
    });

    let root = commands
        .spawn((SceneRoot, Transform::default(), Visibility::default()))
        .id();

    let floor = commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
            MeshMaterial3d(floor_mat),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
    commands.entity(root).add_child(floor);

    let spot = commands
        .spawn((
            SpotLight {
                intensity: 50_000.0,
                color: Color::srgb(0.9, 0.95, 1.0),
                shadows_enabled: true,
                inner_angle: 0.6,
                outer_angle: 1.0,
                range: 30.0,
                ..default()
            },
            Transform::from_xyz(0.0, 5.0, 2.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ))
        .id();
    commands.entity(root).add_child(spot);

    let ambient = commands
        .spawn((
            PointLight {
                color: Color::srgb(0.3, 0.35, 0.5),
                intensity: 3_000.0,
                range: 50.0,
                ..default()
            },
            Transform::from_xyz(0.0, 3.0, 0.0),
        ))
        .id();
    commands.entity(root).add_child(ambient);
}

fn spawn_field_trip(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grass_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.4, 0.12),
        perceptual_roughness: 0.9,
        ..default()
    });

    let trunk_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.15, 0.05),
        ..default()
    });

    let leaves_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.5, 0.18),
        perceptual_roughness: 0.6,
        ..default()
    });

    let root = commands
        .spawn((SceneRoot, Transform::default(), Visibility::default()))
        .id();

    let floor = commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
            MeshMaterial3d(grass_mat),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();
    commands.entity(root).add_child(floor);

    let sun = commands
        .spawn((
            DirectionalLight {
                illuminance: 10000.0,
                shadows_enabled: true,
                color: Color::srgb(1.0, 0.9, 0.8),
                ..default()
            },
            Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
        ))
        .id();
    commands.entity(root).add_child(sun);

    let positions = vec![
        Vec3::new(3.0, 0.0, -4.0),
        Vec3::new(-5.0, 0.0, -2.0),
        Vec3::new(4.0, 0.0, 5.0),
        Vec3::new(-3.0, 0.0, 4.0),
    ];

    for pos in positions {
        let trunk = commands
            .spawn((
                Mesh3d(meshes.add(Cylinder::new(0.2, 2.0))),
                MeshMaterial3d(trunk_mat.clone()),
                Transform::from_translation(pos + Vec3::new(0.0, 1.0, 0.0)),
            ))
            .id();
        commands.entity(root).add_child(trunk);

        let leaves = commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(1.5))),
                MeshMaterial3d(leaves_mat.clone()),
                Transform::from_translation(pos + Vec3::new(0.0, 2.5, 0.0)),
            ))
            .id();
        commands.entity(root).add_child(leaves);
    }
}
