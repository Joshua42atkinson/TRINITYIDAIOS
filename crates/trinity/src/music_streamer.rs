// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Music Streamer
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        music_streamer.rs
// BIBLE CAR:   Car 11 — YOKE (ART Pipeline & Creative Tools)
// HOOK SCHOOL: 🎨 Creation
// PURPOSE:     Background music playback via rodio, genre-aware from CharacterSheet
//
// ═══════════════════════════════════════════════════════════════════════════════

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use trinity_protocol::CharacterSheet;

pub fn start_music_streamer(character_sheet: Arc<RwLock<CharacterSheet>>) {
    info!("Starting background music streamer...");

    // Channel to send genre updates from async world to sync audio world
    let (tx, rx) = std::sync::mpsc::channel::<(bool, String)>();

    // Async task to monitor CharacterSheet and send updates
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            let (enabled, genre) = {
                let sheet = character_sheet.read().await;
                let prefs = &sheet.audio_preferences;
                let desired_genre = prefs
                    .bg_music_genre
                    .clone()
                    .or_else(|| prefs.genre.clone())
                    .unwrap_or_else(|| "ambient".to_string());
                (prefs.music_flow_enabled, desired_genre)
            };

            if tx.send((enabled, genre)).is_err() {
                break;
            }
        }
    });

    // Sync thread for rodio audio playback
    std::thread::spawn(move || {
        let (_stream, stream_handle) = match rodio::OutputStream::try_default() {
            Ok(res) => res,
            Err(e) => {
                warn!("Could not initialize audio output for music: {}", e);
                return;
            }
        };

        let sink = match rodio::Sink::try_new(&stream_handle) {
            Ok(res) => res,
            Err(e) => {
                warn!("Could not create audio sink for music: {}", e);
                return;
            }
        };

        let mut current_genre = String::new();
        let mut is_playing = false;

        loop {
            match rx.recv_timeout(std::time::Duration::from_secs(1)) {
                Ok((enabled, genre)) => {
                    if !enabled {
                        if is_playing {
                            debug!("Music flow disabled, stopping playback.");
                            sink.stop();
                            is_playing = false;
                        }
                    } else {
                        // Re-trigger if not playing OR genre changed
                        if !is_playing || current_genre != genre {
                            if is_playing {
                                sink.stop();
                            }
                            info!("Music flow enabled, streaming genre: {}", genre);
                            current_genre = genre.clone();
                            is_playing = true;

                            let music_dir = dirs::home_dir()
                                .unwrap_or_default()
                                .join(format!("models/music/{}", genre.to_lowercase()));

                            if music_dir.exists() {
                                if let Ok(entries) = std::fs::read_dir(&music_dir) {
                                    for entry in entries.flatten() {
                                        let path = entry.path();
                                        if let Some(ext) = path.extension() {
                                            if ext == "wav" || ext == "mp3" {
                                                if let Ok(file) = std::fs::File::open(&path) {
                                                    if let Ok(source) = rodio::Decoder::new(
                                                        std::io::BufReader::new(file),
                                                    ) {
                                                        info!("Playing track: {:?}", path);
                                                        sink.append(source);
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    if is_playing && sink.empty() {
                        debug!("Music track finished, queuing next...");
                        is_playing = false; // Will trigger replay on next loop
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }
    });
}
