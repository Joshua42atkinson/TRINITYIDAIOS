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
                "Scope Creep ({})",
                feature_request.chars().take(20).collect::<String>()
            ),
            description: format!(
                "A wild Scope Creep appears! Trying to add {} during {:?}",
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
    let triggers = [
        "also",
        "add",
        "what if",
        "let's do",
        "can we",
        "include",
        "new feature",
        "while we're at it",
    ];
    let prompt_lower = prompt.to_lowercase();
    let contains_trigger = triggers.iter().any(|t| prompt_lower.contains(t));
    if contains_trigger {
        return Some(ScopeCreep::new(prompt, phase));
    }
    None
}
