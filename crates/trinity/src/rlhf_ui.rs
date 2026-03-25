// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — RLHF UI Types
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        rlhf_ui.rs
// BIBLE CAR:   Car 4 — IMPLEMENT (Iron Road Game Mechanics)
// HOOK SCHOOL: 🎭 Identity
// PURPOSE:     ResonanceRating + ResonanceFeedbackEvent types for RLHF pipeline
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResonanceRating {
    /// Thumbs up - "This perfectly matched my VAAM tier"
    Positive,
    /// Thumbs down - "Too complex or technically inaccurate"
    Negative,
    /// Neutral / Reroll - "I didn't understand the vocabulary used"
    CognitiveOverload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceFeedbackEvent {
    pub prompt: String,
    pub response: String,
    pub rating: ResonanceRating,
    pub vaam_tier: String,
}
