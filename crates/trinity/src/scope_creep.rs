// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Scope Creep Detection
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        scope_creep.rs
// BIBLE CAR:   Car 4 — IMPLEMENT (Iron Road Game Mechanics)
// HOOK SCHOOL: 🏫 Pedagogy
// PURPOSE:     Detect out-of-scope feature requests, spawn ScopeCreep creatures
//
// ═══════════════════════════════════════════════════════════════════════════════

use trinity_quest::hero::Phase;

/// A wild Scope Creep that threatens to derail the project
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields serialized to JSON for frontend
pub struct ScopeCreep {
    pub name: String,
    pub description: String,
    pub threat_level: u32,    // 1-10
    pub steam_penalty: f32,   // How much momentum it drains
    pub coal_penalty: f32,    // How much compute budget it drains
    pub required_rolls: u32,  // Successful skill checks needed to defeat
    pub backlog_item: String, // The feature to be added if defeated
}

impl ScopeCreep {
    pub fn new(feature_request: &str, phase: &Phase) -> Self {
        let (threat, steam, coal) = match phase {
            Phase::Analysis => (2, 5.0, 2.0),
            Phase::Design => (4, 10.0, 5.0),
            Phase::Development => (7, 25.0, 15.0),
            Phase::Implementation => (9, 40.0, 30.0),
            Phase::Evaluation => (10, 50.0, 50.0),
            _ => (5, 15.0, 10.0), // Default for remaining phases
        };

        Self {
            name: format!(
                "Scope Anomaly ({})",
                feature_request.chars().take(20).collect::<String>()
            ),
            description: format!(
                "Scope Anomaly Detected. The SME suggested '{}' during {:?}. Pending PEARL Evaluation...",
                feature_request, phase
            ),
            threat_level: threat,
            steam_penalty: steam,
            coal_penalty: coal,
            required_rolls: threat / 2 + 1,
            backlog_item: feature_request.to_string(),
        }
    }
}

pub fn detect_scope_creep(
    prompt: &str,
    _current_objectives: &[String],
    phase: &Phase,
) -> Option<ScopeCreep> {
    let prompt_lower = prompt.to_lowercase();
    let word_count = prompt.split_whitespace().count();

    // Don't trigger on extremely short messages (< 5 words)
    if word_count < 5 {
        return None;
    }

    // Strong phrase-level triggers (high confidence scope creep)
    let strong_triggers = [
        "new feature",
        "while we're at it",
        "we should also",
        "let's also add",
        "can we also",
        "what if we added",
        "wouldn't it be cool if",
        "one more thing",
        "before we move on, let's add",
    ];

    let has_strong = strong_triggers.iter().any(|t| prompt_lower.contains(t));
    if has_strong {
        return Some(ScopeCreep::new(prompt, phase));
    }

    // Weak triggers need at least 1 present + message > 7 words
    let weak_triggers = ["also", "add", "include", "what if", "let's do", "can we", "how about"];
    let weak_count = weak_triggers
        .iter()
        .filter(|t| prompt_lower.contains(**t))
        .count();
        
    if weak_count >= 1 && word_count >= 7 {
        return Some(ScopeCreep::new(prompt, phase));
    }

    None
}

