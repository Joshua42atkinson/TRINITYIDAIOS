// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-quest
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        lib.rs
// PURPOSE:     Hero's Journey Quest Engine — ADDIE phase progression system
//
// ARCHITECTURE:
//   • Maps Joseph Campbell's 12 Hero's Journey stages to ADDIE cycles
//   • Each chapter = one stage with full ADDIE cycle (Analysis → Evaluation)
//   • Party system: AI agents accompany user on quests (P-ART-Y roles)
//   • Quest state persisted to PostgreSQL
//   • Completion triggers LitRPG chapter generation
//
// CHANGES:
//   2026-03-16  Cascade  Restored Hero's Journey quest system from archive
//
// ═══════════════════════════════════════════════════════════════════════════════

use std::sync::Arc;
use tokio::sync::RwLock;

pub mod hero;
pub mod party;
pub mod quest_system;
pub mod state;

// Canonical types from state/hero/party - NOT from quest_system (which has duplicates)
pub use hero::*;
pub use party::*;
pub use state::*;
// Only import quest_system functions, not types
pub use quest_system::{ensure_quest_tables, load_game_state, save_game_state};

/// Shared game state (wrapped in Arc<RwLock> for thread safety)
pub type SharedGameState = Arc<RwLock<GameState>>;
