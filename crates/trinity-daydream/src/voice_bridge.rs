// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Voice Bridge Plugin
// ═══════════════════════════════════════════════════════════════════════════════
//
// Bridges the Socratic loop output to the native OS Audio server.
// Fetches `.wav` bytes from /api/tts and injects them directly into Bevy's
// native AssetServer for high-quality, non-web spatial playback.
//
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use std::sync::Arc;
pub struct AudioBytes(pub Vec<u8>);

#[derive(Resource)]
pub struct VoiceRequestSender(pub Sender<(String, String)>);

#[derive(Resource)]
pub struct VoiceRequestReceiver(pub Receiver<(String, String)>);

#[derive(Resource)]
pub struct VoiceAudioReceiver(pub Receiver<AudioBytes>);

#[derive(Resource)]
pub struct VoiceAudioSender(pub Sender<AudioBytes>);

// ─── Plugin ──────────────────────────────────────────────────────────────────

pub struct VoiceBridgePlugin;

impl Plugin for VoiceBridgePlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = channel::<AudioBytes>();
        let (tx_req, rx_req) = channel::<(String, String)>();
        
        app.insert_resource(VoiceAudioSender(tx))
           .insert_resource(VoiceAudioReceiver(rx))
           .insert_resource(VoiceRequestSender(tx_req))
           .insert_resource(VoiceRequestReceiver(rx_req))
           .add_systems(Update, (handle_speak_events, process_audio_queue));
    }
}

// ─── Systems ─────────────────────────────────────────────────────────────────

/// Listens for requests, then spawns an async task to fetch the .wav
fn handle_speak_events(
    req_res: Option<Res<VoiceRequestReceiver>>,
    sender: Option<Res<VoiceAudioSender>>,
) {
    let Some(rx_req) = req_res else { return };
    let Some(tx_res) = sender else { return };
    
    while let Ok((text, voice)) = rx_req.0.try_recv() {
        let tx = tx_res.0.clone();
        
        bevy::log::info!("🎙️ Requesting Voice Pipeline for: {}", text.chars().take(30).collect::<String>());

        let thread_pool = IoTaskPool::get();
        thread_pool.spawn(async move {
            let client = reqwest::Client::new();
            
            let payload = serde_json::json!({
                "text": text,
                "voice": voice,
                "format": "wav"
            });
            
            match client.post("http://127.0.0.1:3000/api/tts")
                        .timeout(std::time::Duration::from_secs(30))
                        .json(&payload)
                        .send()
                        .await 
            {
                Ok(response) => {
                    if response.status().is_success() {
                        if let Ok(bytes) = response.bytes().await {
                            let _ = tx.send(AudioBytes(bytes.to_vec()));
                        }
                    } else {
                        bevy::log::error!("🎙️ TTS API failed with status: {}", response.status());
                    }
                }
                Err(e) => {
                    bevy::log::error!("🎙️ TTS Connection failed: {}", e);
                }
            }
        }).detach();
    }
}

/// Polls the crossbeam channel for finished web-request byte buffers, wrapping them in an AudioSource
fn process_audio_queue(
    mut commands: Commands,
    receiver: Option<Res<VoiceAudioReceiver>>,
    audio_assets_opt: Option<ResMut<Assets<AudioSource>>>,
) {
    let Some(rx) = receiver else { return };
    let Some(mut audio_assets) = audio_assets_opt else { return };

    while let Ok(audio_data) = rx.0.try_recv() {
        bevy::log::info!("🔊 TTS bytes retrieved ({} bytes), wrapping AudioSource...", audio_data.0.len());

        let source = AudioSource {
            bytes: std::sync::Arc::from(audio_data.0),
        };
        
        let handle = audio_assets.add(source);
        
        commands.spawn((
            AudioPlayer(handle),
            PlaybackSettings::DESPAWN,
        ));
    }
}
