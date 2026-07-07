// chat_panel.rs — Chat interface as floating 3D panel
// WebSocket connection to Trinity :3000 /api/xr/connect
// Renders chat messages on spatial UI panel (render-to-texture)
// Sends user input back to Trinity agent

use crate::spatial_ui::create_spatial_panel;
use crate::widgets::{COLOR_TEXT_HIGHLIGHT, COLOR_TEXT_PRIMARY, spawn_glass_panel, spawn_holographic_button};
use crate::ipc::{IncomingXrMessage, XrMessage, XrBroadcaster, send_message};
use bevy::prelude::*;

pub struct ChatPanelPlugin;

impl Plugin for ChatPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_chat_panel)
            .add_systems(Update, process_chat_messages)
            .add_systems(Update, update_chat_display);
    }
}

#[derive(Component)]
pub struct ChatPanel;

#[derive(Component)]
pub struct ChatLogText;

#[derive(Component)]
pub struct ChatInputText;

#[derive(Component)]
pub struct ChatSendButton;

#[derive(Resource, Default)]
pub struct ChatState {
    pub messages: Vec<ChatEntry>,
    pub pending_input: String,
}

#[derive(Clone)]
pub struct ChatEntry {
    pub role: String,
    pub content: String,
}

fn spawn_chat_panel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
) {
    let panel_size = Vec2::new(800.0, 600.0);

    let transform = if let Some(cam) = camera_query.iter().next() {
        let pos = cam.translation() + cam.right() * 0.8 + cam.forward() * 0.6;
        Transform::from_translation(pos).looking_at(cam.translation(), Vec3::Y)
    } else {
        Transform::from_xyz(1.0, 1.5, -0.5).looking_at(Vec3::new(0.0, 1.5, 0.0), Vec3::Y)
    };

    let (panel_entity, camera_entity) = create_spatial_panel(
        &mut commands,
        &mut meshes,
        &mut images,
        &mut materials,
        Vec2::new(panel_size.x / 1000.0, panel_size.y / 1000.0),
        panel_size,
        transform,
    );

    commands.entity(panel_entity).insert(ChatPanel);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            bevy::ui::UiTargetCamera(camera_entity),
        ))
        .with_children(|root| {
            spawn_glass_panel(root, panel_size, 20.0, |panel| {
                panel.spawn((
                    Text::new("TRINITY CHAT"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(COLOR_TEXT_HIGHLIGHT),
                    Node {
                        margin: UiRect::bottom(Val::Px(12.0)),
                        ..default()
                    },
                ));

                panel.spawn((
                    Text::new("Connecting to Trinity...\n"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(COLOR_TEXT_PRIMARY),
                    Node {
                        flex_grow: 1.0,
                        ..default()
                    },
                    ChatLogText,
                ));

                panel.spawn((
                    Text::new("Tap to type..."),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.5, 0.55)),
                    Node {
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                    ChatInputText,
                ));

                spawn_holographic_button(
                    panel,
                    "Send",
                    200.0,
                    40.0,
                    ChatSendButton,
                );
            });
        });

    commands.insert_resource(ChatState::default());
}

fn process_chat_messages(
    mut events: MessageReader<IncomingXrMessage>,
    mut chat_state: ResMut<ChatState>,
) {
    for event in events.read() {
        match event.0.r#type.as_str() {
            "chat_message" => {
                if let Some(data) = &event.0.data {
                    let role = data.get("role").and_then(|v| v.as_str()).unwrap_or("assistant").to_string();
                    let content = data.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    chat_state.messages.push(ChatEntry { role, content });
                }
            }
            "chat_stream" => {
                if let Some(data) = &event.0.data {
                    if let Some(chunk) = data.get("chunk").and_then(|v| v.as_str()) {
                        if let Some(last) = chat_state.messages.last_mut() {
                            if last.role == "assistant" {
                                last.content.push_str(chunk);
                            }
                        } else {
                            chat_state.messages.push(ChatEntry {
                                role: "assistant".to_string(),
                                content: chunk.to_string(),
                            });
                        }
                    }
                }
            }
            "tool_progress" => {
                if let Some(data) = &event.0.data {
                    let tool = data.get("tool").and_then(|v| v.as_str()).unwrap_or("tool");
                    let status = data.get("status").and_then(|v| v.as_str()).unwrap_or("running");
                    chat_state.messages.push(ChatEntry {
                        role: "system".to_string(),
                        content: format!("[{}] {}...", tool, status),
                    });
                }
            }
            "asset_ready" => {
                if let Some(data) = &event.0.data {
                    let asset_type = data.get("type").and_then(|v| v.as_str()).unwrap_or("asset");
                    let url = data.get("url").and_then(|v| v.as_str()).unwrap_or("");
                    chat_state.messages.push(ChatEntry {
                        role: "system".to_string(),
                        content: format!("Asset ready: {} ({})", asset_type, url),
                    });
                }
            }
            _ => {}
        }
    }
}

fn update_chat_display(
    chat_state: Res<ChatState>,
    mut text_query: Query<&mut Text, With<ChatLogText>>,
) {
    if !chat_state.is_changed() {
        return;
    }

    let display: String = chat_state.messages.iter().map(|entry| {
        match entry.role.as_str() {
            "user" => format!("You: {}\n", entry.content),
            "assistant" => format!("Trinity: {}\n", entry.content),
            "system" => format!("  > {}\n", entry.content),
            _ => format!("{}\n", entry.content),
        }
    }).collect();

    let text_value = if display.is_empty() {
        "Connected. Say hello to Trinity!\n".to_string()
    } else {
        display
    };

    for mut text in text_query.iter_mut() {
        text.0 = text_value.clone();
    }
}

pub fn send_chat_message(
    broadcaster: &XrBroadcaster,
    content: &str,
) {
    send_message(broadcaster, XrMessage {
        r#type: "chat_message".to_string(),
        data: Some(serde_json::json!({
            "role": "user",
            "content": content,
        })),
    });
}
