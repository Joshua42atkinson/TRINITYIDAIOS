// spatial_ui.rs — Spatial UI Panel System
// Render-to-texture 3D panels that float in XR space.
// Each panel has its own Camera2d rendering to a texture,
// which is mapped onto a 3D quad. Supports pointer drag with inertia.

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use bevy::asset::uuid::Uuid;
use bevy::picking::pointer::PointerId;

pub struct SpatialUiPlugin;

impl Plugin for SpatialUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_panel_inertia);
    }
}

pub const WORLD_SPACE_UI_POINTER: PointerId = PointerId::Custom(Uuid::from_u128(90870987));

#[derive(Component)]
pub struct SpatialPanel {
    pub physical_width: f32,
    pub physical_height: f32,
    pub virtual_width: f32,
    pub virtual_height: f32,
    pub camera_entity: Entity,
    pub image_handle: Handle<Image>,
}

#[derive(Component, Default)]
pub struct UiVelocity(pub Vec3);

pub fn apply_panel_inertia(mut query: Query<(&mut Transform, &mut UiVelocity)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (mut transform, mut velocity) in query.iter_mut() {
        if velocity.0.length_squared() > 0.0001 {
            transform.translation += velocity.0 * dt;
            velocity.0 *= 1.0 - (5.0 * dt).clamp(0.0_f32, 1.0_f32);
        } else {
            velocity.0 = Vec3::ZERO;
        }
    }
}

pub fn create_spatial_panel(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    images: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    physical_size: Vec2,
    virtual_resolution: Vec2,
    transform: Transform,
) -> (Entity, Entity) {
    let resolution = virtual_resolution;

    let mut image = Image::new_fill(
        Extent3d {
            width: resolution.x as u32,
            height: resolution.y as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        bevy::asset::RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage = bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
        | bevy::render::render_resource::TextureUsages::COPY_DST
        | bevy::render::render_resource::TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);

    let texture_camera = commands
        .spawn((
            Name::new("UiTargetCamera"),
            Camera2d,
            Camera {
                order: -1,
                clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                ..default()
            },
            bevy::camera::RenderTarget::Image(image_handle.clone().into()),
        ))
        .id();

    let quad_handle = meshes.add(Cuboid::new(physical_size.x, physical_size.y, 0.05));

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        base_color: Color::WHITE,
        unlit: true,
        alpha_mode: bevy::prelude::AlphaMode::Blend,
        ..default()
    });

    let panel_entity = commands
        .spawn((
            Mesh3d(quad_handle),
            MeshMaterial3d(material_handle),
            transform,
            SpatialPanel {
                physical_width: physical_size.x,
                physical_height: physical_size.y,
                virtual_width: virtual_resolution.x,
                virtual_height: virtual_resolution.y,
                camera_entity: texture_camera,
                image_handle,
            },
            UiVelocity::default(),
        ))
        .observe(
            |over: On<Pointer<Over>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             meshes: Query<&MeshMaterial3d<StandardMaterial>>| {
                if let Ok(material_handle) = meshes.get(over.entity) {
                    if let Some(mat) = materials.get_mut(material_handle) {
                        mat.emissive = LinearRgba::new(0.0, 0.4, 0.8, 1.0).into();
                    }
                }
            },
        )
        .observe(
            |out: On<Pointer<Out>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             meshes: Query<&MeshMaterial3d<StandardMaterial>>| {
                if let Ok(material_handle) = meshes.get(out.entity) {
                    if let Some(mat) = materials.get_mut(material_handle) {
                        mat.emissive = LinearRgba::new(0.0, 0.0, 0.0, 0.0).into();
                    }
                }
            },
        )
        .observe(
            |drag: On<Pointer<Drag>>, mut query: Query<(&mut Transform, &mut UiVelocity)>| {
                if let Ok((mut transform, mut velocity)) = query.get_mut(drag.entity) {
                    let delta = drag.event.delta;
                    let right = *transform.right();
                    let up = *transform.up();
                    let move_vec = right * delta.x * 0.005 + up * -delta.y * 0.005;
                    transform.translation += move_vec;
                    velocity.0 = move_vec * 60.0;
                }
            },
        )
        .observe(
            |_end: On<Pointer<DragEnd>>, _query: Query<&mut UiVelocity>| {
            }
        )
        .id();

    (panel_entity, texture_camera)
}
