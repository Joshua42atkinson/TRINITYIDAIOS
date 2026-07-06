use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

use crate::inference_router;
use crate::vaam_bridge;
use crate::jobs;
use crate::ChatMessage;


use crate::stubs::trinity_iron_road::book::BookOfTheBible;
use crate::stubs::trinity_iron_road::game_loop::CreepBestiary;
use crate::stubs::trinity_quest::CharacterSheet;
use crate::stubs::trinity_quest;

/// Operating mode — same backend, different UX
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum AppMode {
    /// Full LitRPG gamification (the Iron Road)
    #[default]
    IronRoad,
    /// Skip game mechanics — guided wizard → export
    Express,
    /// IDE/Agent mode (Yardmaster)
    Yardmaster,
    /// Read-only demo — chat and view, no mutation or tool execution
    /// Automatically set when accessed through Cloudflare tunnel (Tier 3)
    Demo,
}

impl std::fmt::Display for AppMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppMode::IronRoad => write!(f, "iron_road"),
            AppMode::Express => write!(f, "express"),
            AppMode::Yardmaster => write!(f, "yardmaster"),
            AppMode::Demo => write!(f, "demo"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// IDENTITY SPLIT — Tier 3.5 Maturation
//
// The Trinity architecture separates state into 3 layers:
//   System  → hardware, AI, database (shared by all users)
//   Player  → identity, preferences, creatures (persists across projects)
//   Project → active PEARL, quest progress, chat, narrative (one per game)
//
// These structs are introduced alongside the existing flat fields.
// Migration: handlers move from state.player.character_sheet → state.player.character_sheet
// one at a time, verified by the compiler. Old fields are removed in Pass 3.
// ═══════════════════════════════════════════════════════════════════════════════

/// Player-level state — persists across projects.
/// This is WHO the educator is, not WHAT they're building.
#[derive(Clone)]
pub struct PlayerContext {
    /// Identity, preferences, skills, competencies, LDT portfolio
    pub character_sheet: Arc<RwLock<CharacterSheet>>,
    /// Vocabulary creature collection — earned through learning, kept forever
    pub bestiary: Arc<RwLock<CreepBestiary>>,
    /// UI preference: IronRoad / Express / Yardmaster
    pub app_mode: Arc<RwLock<AppMode>>,
}

/// Project-level state — one per active PEARL.
/// This is the GAME being built, not the person building it.
#[derive(Clone)]
pub struct ProjectContext {
    /// Quest board, XP, Coal, Steam, phase progress
    pub game_state: trinity_quest::SharedGameState,
    /// Active chat session for this project
    pub conversation_history: Arc<RwLock<Vec<ChatMessage>>>,
    /// Narrative ledger — the story of building this game
    pub book: Arc<RwLock<BookOfTheBible>>,
    /// SSE broadcast for real-time book updates
    pub book_updates: broadcast::Sender<String>,
    /// Session ID for persistence
    pub session_id: Arc<String>,
}

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    // ── System Layer (hardware, AI, database) ──
    pub inference_router: Arc<RwLock<inference_router::InferenceRouter>>,
    pub db_pool: sqlx::SqlitePool,
    pub cow_catcher: Arc<tokio::sync::RwLock<crate::cow_catcher::CowCatcher>>,
    pub vaam_bridge: Arc<vaam_bridge::VaamBridge>,

    // ── Ignition State Machine (server-side, survives tab switches) ──
    /// Tracks LLM boot: idle | launching | daemon_up | server_starting | polling | loading_model | ready | failed
    pub ignition_status: Arc<RwLock<String>>,

    // ── Background Job Queue ──
    pub job_queue: jobs::JobQueue,

    // ── Identity Contexts (Tier 3.5) ──
    /// Player-level state (identity, preferences, creatures)
    pub player: PlayerContext,
    /// Project-level state (active PEARL, quest, chat, narrative)
    pub project: ProjectContext,
    
    // ── Daydream Sidecar Pipeline ──
    /// Channel to send JSON commands to the native Bevy sidecar's STDIN
    pub daydream_tx: Option<tokio::sync::mpsc::Sender<String>>,
    pub telemetry_updates: tokio::sync::broadcast::Sender<serde_json::Value>,
}
