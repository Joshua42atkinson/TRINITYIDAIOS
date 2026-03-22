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
