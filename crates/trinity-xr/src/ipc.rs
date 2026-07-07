// ipc.rs — Trinity XR WebSocket Client
// Connects to Trinity server :3000 via WebSocket for chat, asset notifications, and scene updates.
// Replaces Bertrand's IPC with Trinity API integration.

use bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender as CrossbeamSender};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::thread;
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XrMessage {
    pub r#type: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Message, Debug, Clone)]
pub struct IncomingXrMessage(pub XrMessage);

#[derive(Message, Debug, Clone)]
pub struct OutgoingXrMessage(pub XrMessage);

#[derive(Resource)]
struct XrReceiver(Receiver<XrMessage>);

#[derive(Resource)]
pub struct XrBroadcaster(broadcast::Sender<String>);

#[derive(Resource)]
pub struct XrConnectionState {
    pub connected: bool,
    pub server_url: String,
}

impl Default for XrConnectionState {
    fn default() -> Self {
        Self {
            connected: false,
            server_url: std::env::var("TRINITY_URL").unwrap_or_else(|_| "ws://127.0.0.1:3000".to_string()),
        }
    }
}

pub struct IpcPlugin;

impl Plugin for IpcPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = unbounded();
        let (b_tx, _b_rx) = broadcast::channel::<String>(100);
        let b_tx_clone = b_tx.clone();

        let server_url = std::env::var("TRINITY_URL").unwrap_or_else(|_| "ws://127.0.0.1:3000".to_string());

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(async {
                run_ws_client(tx, b_tx_clone, &server_url).await;
            });
        });

        app.add_message::<IncomingXrMessage>()
            .add_message::<OutgoingXrMessage>()
            .insert_resource(XrReceiver(rx))
            .insert_resource(XrBroadcaster(b_tx))
            .insert_resource(XrConnectionState::default())
            .add_systems(Update, process_incoming_messages)
            .add_systems(Update, broadcast_outgoing_messages);
    }
}

async fn run_ws_client(tx: CrossbeamSender<XrMessage>, b_tx: broadcast::Sender<String>, server_url: &str) {
    let ws_url = format!("{}/api/xr/connect", server_url.replace("http://", "ws://").replace("https://", "wss://"));

    loop {
        tracing::info!("Connecting to Trinity WebSocket: {}", ws_url);

        match tokio_tungstenite::connect_async(&ws_url).await {
            Ok((ws_stream, _)) => {
                tracing::info!("Connected to Trinity server");
                let (mut write, mut read) = ws_stream.split();

                let b_tx_clone = b_tx.clone();
                let send_task = tokio::spawn(async move {
                    let mut rx = b_tx_clone.subscribe();
                    while let Ok(msg) = rx.recv().await {
                        if write.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await.is_err() {
                            break;
                        }
                    }
                });

                while let Some(msg_result) = read.next().await {
                    match msg_result {
                        Ok(msg) => {
                            if msg.is_text() {
                                if let Ok(text) = msg.to_text() {
                                    if let Ok(xr_msg) = serde_json::from_str::<XrMessage>(text) {
                                        let _ = tx.send(xr_msg);
                                    } else {
                                        tracing::warn!("Failed to parse XR message: {}", text);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("WebSocket error: {}", e);
                            break;
                        }
                    }
                }

                send_task.abort();
                tracing::warn!("WebSocket disconnected, reconnecting in 3s...");
            }
            Err(e) => {
                tracing::error!("Failed to connect to Trinity: {}", e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}

fn process_incoming_messages(receiver: Res<XrReceiver>, mut events: MessageWriter<IncomingXrMessage>) {
    for payload in receiver.0.try_iter() {
        events.write(IncomingXrMessage(payload));
    }
}

fn broadcast_outgoing_messages(
    mut events: MessageReader<OutgoingXrMessage>,
    broadcaster: Res<XrBroadcaster>,
) {
    for event in events.read() {
        if let Ok(json) = serde_json::to_string(&event.0) {
            let _ = broadcaster.0.send(json);
        }
    }
}

pub fn send_message(broadcaster: &XrBroadcaster, msg: XrMessage) {
    if let Ok(json) = serde_json::to_string(&msg) {
        let _ = broadcaster.0.send(json);
    }
}
