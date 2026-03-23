// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-bevy-graphics/src/bridge.rs
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        bridge.rs
// PURPOSE:     Bevy ↔ Axum HTTP bridge — polls Trinity server for live state
// ORIGIN:      Ported from crates/archive/trinity-body/src/bridge.rs
//              REPLACED: crossbeam channels + tarpc + ureq
//              WITH:     reqwest HTTP client hitting localhost:3000
//
// ARCHITECTURE:
//   A Bevy Resource (`TrinityBridge`) holds the server URL and a tokio
//   runtime handle. A periodic Bevy system (`poll_trinity_state`) fires
//   every 2 seconds, fetches /api/health, /api/quest, /api/bevy/state,
//   and updates local Bevy resources that other systems can read.
//
// ═══════════════════════════════════════════════════════════════════════════════

use bevy::prelude::*;
use std::sync::Arc;

/// Bridge plugin — sets up resources and polling system
pub struct BridgePlugin {
    pub server_url: String,
}

impl Default for BridgePlugin {
    fn default() -> Self {
        Self {
            server_url: "http://127.0.0.1:3000".to_string(),
        }
    }
}

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut App) {
        // Build a tokio runtime for async HTTP inside Bevy's sync world
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime for bridge");
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .expect("Failed to create reqwest client");

        app.insert_resource(TrinityBridge {
            server_url: self.server_url.clone(),
            client,
            rt: Arc::new(rt),
            connected: false,
        })
        .insert_resource(TrinityServerState::default())
        .add_systems(Update, poll_trinity_state);
    }
}

// ─── Resources ───────────────────────────────────────────────────────────────

/// Connection handle to the Trinity Axum server
#[derive(Resource)]
pub struct TrinityBridge {
    pub server_url: String,
    pub client: reqwest::Client,
    pub rt: Arc<tokio::runtime::Runtime>,
    pub connected: bool,
}

/// Cached snapshot of server state — updated every poll cycle
#[derive(Resource, Default, Debug, Clone)]
pub struct TrinityServerState {
    /// Is the server reachable?
    pub healthy: bool,
    /// Current quest phase label (e.g. "Analysis")
    pub phase: String,
    /// Current quest chapter
    pub chapter: u8,
    /// Quest subject
    pub subject: String,
    /// XP earned
    pub xp: u32,
    /// Coal remaining (0–100)
    pub coal: f32,
    /// Steam generated
    pub steam: f32,
    /// Pete's reported state from /api/bevy/state
    pub pete_state: String,
    /// Last poll timestamp (seconds since app start)
    pub last_poll: f64,
    /// Raw quest JSON (for advanced consumers)
    pub quest_json: Option<serde_json::Value>,
}

// ─── Polling system ──────────────────────────────────────────────────────────

/// Polls the Trinity server every 2 seconds and caches the result
fn poll_trinity_state(
    time: Res<Time>,
    mut bridge: ResMut<TrinityBridge>,
    mut state: ResMut<TrinityServerState>,
) {
    const POLL_INTERVAL: f64 = 2.0;

    let elapsed = time.elapsed_secs_f64();
    if elapsed - state.last_poll < POLL_INTERVAL {
        return;
    }
    state.last_poll = elapsed;

    let url = bridge.server_url.clone();
    let client = bridge.client.clone();

    // Fire the async requests on the bridge's tokio runtime
    // Use block_on briefly — these are localhost requests, <5ms typical
    let result = bridge.rt.block_on(async {
        // Health check
        let health_ok = client
            .get(format!("{}/api/health", url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false);

        if !health_ok {
            return Err("Server unreachable");
        }

        // Quest state
        let quest_json: Option<serde_json::Value> = match client
            .get(format!("{}/api/quest", url))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => resp.json().await.ok(),
            _ => None,
        };

        // Bevy state (Pete's DM HUD)
        let bevy_json: Option<serde_json::Value> = match client
            .get(format!("{}/api/bevy/state", url))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => resp.json().await.ok(),
            _ => None,
        };

        Ok((quest_json, bevy_json))
    });

    match result {
        Ok((quest_opt, bevy_opt)) => {
            bridge.connected = true;
            state.healthy = true;

            if let Some(q) = &quest_opt {
                state.phase = q["phase"].as_str().unwrap_or("Unknown").to_string();
                state.chapter = q["chapter"].as_u64().unwrap_or(1) as u8;
                state.subject = q["subject"].as_str().unwrap_or("").to_string();
                state.xp = q["xp"].as_u64().unwrap_or(0) as u32;
                state.coal = q["coal"].as_f64().unwrap_or(100.0) as f32;
                state.steam = q["steam"].as_f64().unwrap_or(0.0) as f32;
            }
            state.quest_json = quest_opt;

            if let Some(b) = &bevy_opt {
                state.pete_state = b["entities"]
                    .as_array()
                    .and_then(|entities| {
                        entities.iter().find(|e| {
                            e["id"].as_str() == Some("Dungeon_Master_Pete")
                        })
                    })
                    .and_then(|pete| pete["components"]["State"].as_str())
                    .unwrap_or("Idle")
                    .to_string();
            }
        }
        Err(_) => {
            bridge.connected = false;
            state.healthy = false;
        }
    }
}
