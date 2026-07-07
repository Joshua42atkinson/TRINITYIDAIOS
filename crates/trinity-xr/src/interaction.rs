// interaction.rs — VR Interaction System
// Laser pointer + hit cursor for VR interaction.
// Uses bevy_picking for raycasting.

use bevy::prelude::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_laser_visuals);
    }
}

#[derive(Component)]
pub struct LaserPointer;

#[derive(Component)]
pub struct HitCursor;

fn update_laser_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    laser_query: Query<Entity, With<LaserPointer>>,
    cursor_query: Query<Entity, With<HitCursor>>,
) {
    if laser_query.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.002, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.6, 1.0),
                emissive: Color::srgb(0.5, 1.0, 3.0).into(),
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::default(),
            LaserPointer,
        ));
    }

    if cursor_query.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.01))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                emissive: Color::srgb(0.5, 0.8, 1.0).into(),
                ..default()
            })),
            Transform::default(),
            HitCursor,
        ));
    }
}
