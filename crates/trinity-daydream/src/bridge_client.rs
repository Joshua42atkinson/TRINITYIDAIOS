use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use serde_json::Value;
use futures::StreamExt;
use reqwest_eventsource::EventSource;
use reqwest_eventsource::Event as SseEvent;

#[derive(Clone)]
pub struct ChatMessage {
    pub speaker: String,
    pub content: String,
}

/// The state of the Socratic Chat Thread shared with the egui HUD.
#[derive(Resource)]
pub struct SocraticThread {
    pub messages: Vec<ChatMessage>,
    pub input_text: String,
    pub is_generating: bool,
}

impl Default for SocraticThread {
    fn default() -> Self {
        Self {
            messages: vec![ChatMessage {
                speaker: "Pete".to_string(),
                content: "The Iron Road awaits. State your intent.".to_string(),
            }],
            input_text: String::new(),
            is_generating: false,
        }
    }
}

/// Receiver for prompts submitted by the HUD.
#[derive(Resource)]
pub struct PromptQueuer(pub Receiver<String>);

/// Sender used by the HUD to submit prompts.
#[derive(Resource)]
pub struct PromptSender(pub Sender<String>);

/// Receiver for SSE chunk streams sent from the background task.
#[derive(Resource)]
pub struct StreamReceiver(pub Receiver<SseChunk>);

pub enum SseChunk {
    Text(String),
    Done,
    Error(String),
}

pub struct BridgeClientPlugin;

impl Plugin for BridgeClientPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = channel::<String>();
        app.init_resource::<SocraticThread>()
            .insert_resource(PromptSender(tx))
            .insert_resource(PromptQueuer(rx))
            .add_systems(Update, (handle_submit_prompt, process_sse_chunks));
    }
}

/// Whenever the user submits a prompt, spawn an async task to POST to localhost:3000
/// and stream the SSE events back via an MPSC channel.
fn handle_submit_prompt(
    mut thread: ResMut<SocraticThread>,
    mut commands: Commands,
    rx: Option<Res<PromptQueuer>>,
) {
    if let Some(prompt_rx) = rx {
        while let Ok(prompt) = prompt_rx.0.try_recv() {
        
        // Push the user's message to the UI
        thread.messages.push(ChatMessage {
            speaker: "User".to_string(),
            content: prompt.clone(),
        });
        
        // Push a placeholder for Pete's incoming response
        thread.messages.push(ChatMessage {
            speaker: "Pete".to_string(),
            content: String::new(),
        });
        
        thread.is_generating = true;
        
        let (tx, rx) = channel::<SseChunk>();
        commands.insert_resource(StreamReceiver(rx));
        
        let thread_pool = IoTaskPool::get();
        thread_pool.spawn(async move {
            let client = reqwest::Client::new();
            
            // Build the payload that matches the expected /api/chat/zen backend schema
            let payload = serde_json::json!({
                "messages": [
                    {"role": "user", "content": prompt}
                ],
                "stream": true,
                "phase": "Analyze"
            });
            
            let req = client.post("http://127.0.0.1:3000/api/chat/zen")
                .json(&payload);
                
            let mut es = match EventSource::new(req) {
                Ok(es) => es,
                Err(_) => {
                    let _ = tx.send(SseChunk::Error("Failed to open EventSource".into()));
                    return;
                }
            };
            
            while let Some(event) = es.next().await {
                match event {
                    Ok(SseEvent::Open) => {}
                    Ok(SseEvent::Message(message)) => {
                        if message.data == "[DONE]" {
                            let _ = tx.send(SseChunk::Done);
                            break;
                        }
                        
                        // Parse JSON chunk from the backend (expects standard OpenAI-compatible chunk)
                        if let Ok(val) = serde_json::from_str::<Value>(&message.data) {
                            if let Some(delta) = val["choices"][0]["delta"]["content"].as_str() {
                                let _ = tx.send(SseChunk::Text(delta.to_string()));
                            }
                        }
                    }
                    Err(_) => {
                        let _ = tx.send(SseChunk::Error("Connection dropped.".to_string()));
                        es.close();
                        break;
                    }
                }
            }
        }).detach();
        }
    }
}

/// Poll the Receiver channel each frame to append new tokens to Pete's active message.
fn process_sse_chunks(
    mut thread: ResMut<SocraticThread>,
    receiver: Option<Res<StreamReceiver>>,
    mut commands: Commands,
    tx_voice: Option<Res<crate::voice_bridge::VoiceRequestSender>>,
) {
    if let Some(rx) = receiver {
        while let Ok(chunk) = rx.0.try_recv() {
            match chunk {
                SseChunk::Text(text) => {
                    if let Some(last) = thread.messages.last_mut() {
                        if last.speaker == "Pete" {
                            last.content.push_str(&text);
                        }
                    }
                }
                SseChunk::Done => {
                    thread.is_generating = false;
                    commands.remove_resource::<StreamReceiver>();
                    
                    // The generation finished. Grab the full text of Pete's message and trigger TTS!
                    if let Some(last) = thread.messages.last() {
                        if last.speaker == "Pete" {
                            bevy::log::info!("🚂 SSE Stream complete. Dispatching text to Voice Engine.");
                            if let Some(tx) = &tx_voice {
                                let _ = tx.0.send((last.content.clone(), "Pete".to_string()));
                            }
                        }
                    }
                }
                SseChunk::Error(_) => {
                    thread.is_generating = false;
                    commands.remove_resource::<StreamReceiver>();
                }
            }
        }
    }
}
