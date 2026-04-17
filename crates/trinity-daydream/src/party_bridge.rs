// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — P-ART-Y Bridge (AI Fleet Interface for XR)
// ═══════════════════════════════════════════════════════════════════════════════
//
// PURPOSE:  Connect the DAYDREAM XR app to the Trinity P-ART-Y model fleet.
//           Works on BOTH desktop and XR — speaks HTTP to the Axum server,
//           which manages the Hotel Swap Protocol for model loading.
//
// ARCHITECTURE:
//   [DAYDREAM (Desktop/XR)]
//         │
//         ├── GET  /api/health           → Trinity server health
//         ├── GET  /api/quest            → Current quest state
//         ├── GET  /api/daydream/state   → DaydreamBlueprint queue
//         ├── POST /api/chat             → Send prompt to active model
//         ├── POST /api/inference/swap   → Trigger Hotel Swap
//         └── GET  /api/inference/fleet  → Fleet status (T/P/R/A)
//
// PRINCIPLE:
//   The XR app does NOT run any AI models locally.
//   Models live on the GMKtek desktop (128GB RAM).
//   XR app is a thin spatial client that sends requests over WiFi.
//
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use std::sync::{Arc, Mutex};

use trinity_protocol::daydream_commands::DaydreamBlueprint;

use crate::daydream::DaydreamCommandQueue;
use crate::spatial_ui::{SpatialDashboardState, FleetStatus};

// ─── Plugin ──────────────────────────────────────────────────────────────────

/// Connects the DAYDREAM world to the Trinity AI fleet.
/// Polls the server for quest state, blueprint commands, and fleet health.
/// Updates the SpatialDashboardState resource which drives the native UI.
pub struct PartyBridgePlugin {
    pub server_url: String,
}

impl Default for PartyBridgePlugin {
    fn default() -> Self {
        // On XR headset, set TRINITY_SERVER_URL=http://192.168.x.x:3000
        // to point at the GMKtek desktop over WiFi.
        let url = std::env::var("TRINITY_SERVER_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
        Self { server_url: url }
    }
}

impl Plugin for PartyBridgePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PartyBridgeState {
            server_url: self.server_url.clone(),
            connected: false,
            last_poll: 0.0,
            pending_blueprints: Arc::new(Mutex::new(Vec::new())),
            pending_fleet: Arc::new(Mutex::new(None)),
            pending_quest: Arc::new(Mutex::new(None)),
        })
        .add_systems(Update, (
            poll_server_state,
            drain_pending_blueprints,
            sync_dashboard_state,
        ));
    }
}

// ─── State ───────────────────────────────────────────────────────────────────

/// Connection state for the P-ART-Y bridge.
#[derive(Resource)]
pub struct PartyBridgeState {
    pub server_url: String,
    pub connected: bool,
    pub last_poll: f64,

    /// Pending blueprints received from the server
    pub pending_blueprints: Arc<Mutex<Vec<DaydreamBlueprint>>>,

    /// Pending fleet status update
    pub pending_fleet: Arc<Mutex<Option<FleetSnapshot>>>,

    /// Pending quest state
    pub pending_quest: Arc<Mutex<Option<QuestSnapshot>>>,
}

/// Snapshot of the P-ART-Y fleet status.
#[derive(Debug, Clone, Default)]
pub struct FleetSnapshot {
    pub tempo: String,
    pub programming: String,
    pub reasoning: String,
    pub aesthetics: String,
    pub server_healthy: bool,
}

/// Snapshot of current quest state.
#[derive(Debug, Clone, Default)]
pub struct QuestSnapshot {
    pub station: u8,
    pub station_name: String,
    pub phase_label: String,
    pub pearl_subject: String,
    pub pearl_medium: String,
    pub pearl_vision: String,
    pub pearl_phase: String,
    pub pearl_progress: f32,
    pub coal: f32,
    pub steam: f32,
    pub xp: u32,
    pub xp_max: u32,
    pub traction: f32,
    pub creeps_slain: u32,
    pub objectives: Vec<(String, bool)>,
}

// ─── Polling System ──────────────────────────────────────────────────────────

const POLL_INTERVAL: f64 = 2.0; // Poll every 2 seconds

/// Periodic HTTP polling for server state.
/// Runs on a background thread to avoid blocking the render loop.
fn poll_server_state(
    time: Res<Time>,
    mut bridge: ResMut<PartyBridgeState>,
) {
    let elapsed = time.elapsed_secs_f64();
    if elapsed - bridge.last_poll < POLL_INTERVAL {
        return;
    }
    bridge.last_poll = elapsed;

    let url = bridge.server_url.clone();
    let pending_fleet = bridge.pending_fleet.clone();
    let pending_quest = bridge.pending_quest.clone();
    let pending_blueprints = bridge.pending_blueprints.clone();

    // Fire non-blocking HTTP requests on a background thread
    std::thread::spawn(move || {
        let client = match reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(3))
            .build()
        {
            Ok(c) => c,
            Err(_) => return,
        };

        // ── 1. Health check ──────────────────────────────────────
        let healthy = client
            .get(format!("{}/api/health", url))
            .send()
            .map(|r| r.status().is_success())
            .unwrap_or(false);

        // ── 2. Quest state ───────────────────────────────────────
        if healthy {
            if let Ok(resp) = client.get(format!("{}/api/quest", url)).send() {
                if let Ok(json) = resp.json::<serde_json::Value>() {
                    let quest = QuestSnapshot {
                        station: json["station"].as_u64().unwrap_or(1) as u8,
                        station_name: json["station_name"]
                            .as_str()
                            .unwrap_or("Station 1: Analysis")
                            .to_string(),
                        phase_label: json["phase"]
                            .as_str()
                            .unwrap_or("Analyze")
                            .to_string(),
                        pearl_subject: json["pearl"]["subject"]
                            .as_str()
                            .unwrap_or("Awaiting PEARL...")
                            .to_string(),
                        pearl_medium: json["pearl"]["medium"]
                            .as_str()
                            .unwrap_or("—")
                            .to_string(),
                        pearl_vision: json["pearl"]["vision"]
                            .as_str()
                            .unwrap_or("—")
                            .to_string(),
                        pearl_phase: json["pearl"]["phase"]
                            .as_str()
                            .unwrap_or("Extracting")
                            .to_string(),
                        pearl_progress: json["pearl"]["progress"].as_f64().unwrap_or(0.0) as f32,
                        coal: json["coal"].as_f64().unwrap_or(100.0) as f32,
                        steam: json["steam"].as_f64().unwrap_or(0.0) as f32,
                        xp: json["xp"].as_u64().unwrap_or(0) as u32,
                        xp_max: json["xp_max"].as_u64().unwrap_or(500) as u32,
                        traction: json["traction"].as_f64().unwrap_or(0.0) as f32,
                        creeps_slain: json["creeps_slain"].as_u64().unwrap_or(0) as u32,
                        objectives: json["objectives"]
                            .as_array()
                            .map(|arr| {
                                arr.iter()
                                    .map(|obj| {
                                        let label = obj["label"]
                                            .as_str()
                                            .unwrap_or("")
                                            .to_string();
                                        let done = obj["completed"].as_bool().unwrap_or(false);
                                        (label, done)
                                    })
                                    .collect()
                            })
                            .unwrap_or_default(),
                    };

                    if let Ok(mut lock) = pending_quest.lock() {
                        *lock = Some(quest);
                    }
                }
            }

            // ── 3. Fleet status ──────────────────────────────────
            if let Ok(resp) = client.get(format!("{}/api/inference/fleet", url)).send() {
                if let Ok(json) = resp.json::<serde_json::Value>() {
                    let fleet = FleetSnapshot {
                        tempo: json["tempo"]["status"]
                            .as_str()
                            .unwrap_or("offline")
                            .to_string(),
                        programming: json["programming"]["status"]
                            .as_str()
                            .unwrap_or("offline")
                            .to_string(),
                        reasoning: json["reasoning"]["status"]
                            .as_str()
                            .unwrap_or("offline")
                            .to_string(),
                        aesthetics: json["aesthetics"]["status"]
                            .as_str()
                            .unwrap_or("offline")
                            .to_string(),
                        server_healthy: true,
                    };

                    if let Ok(mut lock) = pending_fleet.lock() {
                        *lock = Some(fleet);
                    }
                }
            }

            // ── 4. Pending Daydream Blueprints ───────────────────
            if let Ok(resp) = client
                .get(format!("{}/api/daydream/blueprints", url))
                .send()
            {
                if let Ok(blueprints) = resp.json::<Vec<DaydreamBlueprint>>() {
                    if !blueprints.is_empty() {
                        if let Ok(mut lock) = pending_blueprints.lock() {
                            lock.extend(blueprints);
                        }
                    }
                }
            }
        }

        // Store health status
        if let Ok(mut lock) = pending_fleet.lock() {
            if lock.is_none() {
                *lock = Some(FleetSnapshot {
                    server_healthy: healthy,
                    ..Default::default()
                });
            }
        }
    });
}

// ─── Blueprint Drain ─────────────────────────────────────────────────────────

/// Drain pending blueprints into the DaydreamCommandQueue for ECS processing.
fn drain_pending_blueprints(
    bridge: Res<PartyBridgeState>,
    mut queue: ResMut<DaydreamCommandQueue>,
) {
    if let Ok(mut lock) = bridge.pending_blueprints.try_lock() {
        for blueprint in lock.drain(..) {
            info!(
                "🌙 PARTY Bridge: Received blueprint for station {} ({} commands)",
                blueprint.station,
                blueprint.commands.len()
            );
            queue.push_blueprint(blueprint);
        }
    }
}

// ─── Dashboard Sync ──────────────────────────────────────────────────────────

/// Sync polling results into the SpatialDashboardState resource.
/// This drives the native Bevy UI update systems.
fn sync_dashboard_state(
    bridge: Res<PartyBridgeState>,
    mut dashboard: ResMut<SpatialDashboardState>,
) {
    // Sync quest state
    if let Ok(mut lock) = bridge.pending_quest.try_lock() {
        if let Some(quest) = lock.take() {
            dashboard.station = quest.station;
            dashboard.station_name = quest.station_name;
            dashboard.addiecrapeye_label = quest.phase_label;
            dashboard.pearl_subject = quest.pearl_subject;
            dashboard.pearl_medium = quest.pearl_medium;
            dashboard.pearl_vision = quest.pearl_vision;
            dashboard.pearl_phase = quest.pearl_phase;
            dashboard.pearl_progress = quest.pearl_progress;
            dashboard.coal = quest.coal;
            dashboard.steam = quest.steam;
            dashboard.xp = quest.xp;
            dashboard.xp_max = quest.xp_max;
            dashboard.traction = quest.traction;
            dashboard.creeps_slain = quest.creeps_slain;
            if !quest.objectives.is_empty() {
                dashboard.objectives = quest.objectives;
            }
        }
    }

    // Sync fleet status
    if let Ok(mut lock) = bridge.pending_fleet.try_lock() {
        if let Some(fleet) = lock.take() {
            dashboard.tempo_status = parse_fleet_status(&fleet.tempo);
            dashboard.programming_status = parse_fleet_status(&fleet.programming);
            dashboard.reasoning_status = parse_fleet_status(&fleet.reasoning);
            dashboard.aesthetics_status = parse_fleet_status(&fleet.aesthetics);
        }
    }
}

fn parse_fleet_status(status: &str) -> FleetStatus {
    match status.to_lowercase().as_str() {
        "online" | "healthy" | "active" => FleetStatus::Online,
        "standby" | "ready" => FleetStatus::Standby,
        "swapping" | "loading" => FleetStatus::HotelSwapping,
        _ => FleetStatus::Offline,
    }
}
