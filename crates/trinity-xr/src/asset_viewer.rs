// asset_viewer.rs — 3D Asset Preview in XR
// Load glTF files from Trinity's asset directory.
// Display as 3D mesh in XR space with grab/rotate/scale.

use bevy::prelude::*;
use bevy::asset::AssetServer;
use crate::ipc::{IncomingXrMessage, XrBroadcaster, send_message, XrMessage};

pub struct AssetViewerPlugin;

impl Plugin for AssetViewerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PendingAssets>()
            .add_systems(Update, process_asset_notifications)
            .add_systems(Update, load_pending_assets)
            .add_systems(Update, update_asset_positions);
    }
}

#[derive(Resource, Default)]
pub struct PendingAssets {
    pub queue: Vec<AssetRequest>,
}

#[derive(Clone, Debug)]
pub struct AssetRequest {
    pub url: String,
    pub asset_type: String,
    pub name: String,
}

#[derive(Component)]
pub struct AssetModel {
    pub name: String,
    pub asset_type: String,
    pub target_position: Vec3,
}

#[derive(Component)]
pub struct AssetDragState {
    pub is_dragging: bool,
    pub last_scale: f32,
}

fn process_asset_notifications(
    mut events: MessageReader<IncomingXrMessage>,
    mut pending: ResMut<PendingAssets>,
) {
    for event in events.read() {
        if event.0.r#type == "asset_ready" {
            if let Some(data) = &event.0.data {
                let url = data.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let asset_type = data.get("type").and_then(|v| v.as_str()).unwrap_or("model").to_string();
                let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("Asset").to_string();

                if asset_type == "3d" || asset_type == "model" || asset_type == "gltf" {
                    pending.queue.push(AssetRequest { url, asset_type, name });
                }
            }
        }
    }
}

fn load_pending_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pending: ResMut<PendingAssets>,
    existing: Query<&AssetModel>,
) {
    let count = existing.iter().count();
    let queue = std::mem::take(&mut pending.queue);

    for req in queue {
        if req.url.is_empty() {
            continue;
        }

        let scene_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset(req.url.clone()));

        let x = -2.0 + (count as f32) * 1.5;
        let pos = Vec3::new(x, 1.2, -1.0);

        commands.spawn((
            SceneRoot(scene_handle),
            Transform::from_translation(pos),
            AssetModel {
                name: req.name.clone(),
                asset_type: req.asset_type.clone(),
                target_position: pos,
            },
            AssetDragState {
                is_dragging: false,
                last_scale: 1.0,
            },
        ));

        tracing::info!("Loaded 3D asset: {} at {}", req.name, req.url);
    }
}

fn update_asset_positions(
    mut query: Query<(&mut Transform, &AssetModel, &mut AssetDragState)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut transform, model, drag) in query.iter_mut() {
        if !drag.is_dragging {
            let target = model.target_position;
            transform.translation = transform.translation.lerp(target, dt * 3.0);
        }
    }
}

pub fn request_asset_list(broadcaster: &XrBroadcaster) {
    send_message(broadcaster, XrMessage {
        r#type: "list_assets".to_string(),
        data: None,
    });
}
