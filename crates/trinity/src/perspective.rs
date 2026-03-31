// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        perspective.rs
// PURPOSE:     Ring 6 — Perspective Engine: Multi-perspective AI output evaluation
//
// ARCHITECTURE:
//   The Perspective Engine evaluates Pete's responses through multiple lenses
//   before the user sees them. Each lens is a short, focused LLM call that
//   annotates — never modifies — Pete's output.
//
//   Three default lenses:
//     A. Bloom's Check — Does Pete's response match the current phase's Bloom's verb?
//     B. Practitioner — Would an experienced teacher in this subject agree?
//     C. Devil's Advocate — What assumption is Pete making that could be wrong?
//
//   Lenses fire in parallel (tokio::join!) with a 100-token budget each.
//
// INTEGRATION:
//   - Called from chat_stream after Pete's full response is collected
//   - Results sent as SSE "perspective" events to the frontend
//   - Does NOT modify Pete's response — annotation only
//
// RING INTERACTIONS:
//   - Ring 2 (Persona): Perspectives respect persona slot
//   - Ring 3 (Context): Lenses do NOT receive full conversation — only Pete's response
//   - Ring 5 (Rate Limit): Perspective calls count toward 60/min global limit
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use tracing::{info, warn};

// ============================================================================
// FEEDBACK PERSISTENCE
// ============================================================================

/// User feedback on a perspective evaluation (thumbs up/down).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerspectiveFeedback {
    /// Which lens was rated
    pub lens_id: String,
    /// "up" or "down"
    pub direction: String,
    /// What ADDIECRAPEYE phase was active
    pub phase: Option<String>,
    /// ISO 8601 timestamp
    pub timestamp: String,
}

/// Save feedback to ~/.local/share/trinity/perspective_feedback.jsonl
pub fn save_feedback(feedback: &PerspectiveFeedback) {
    let dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("trinity");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("perspective_feedback.jsonl");

    if let Ok(json) = serde_json::to_string(feedback) {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            let _ = writeln!(f, "{}", json);
            info!(
                "[Ring 6] Feedback saved: {} → {}",
                feedback.lens_id, feedback.direction
            );
        }
    }
}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// A perspective lens evaluates Pete's response through a specific angle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lens {
    /// Short name: "blooms", "practitioner", "advocate"
    pub id: String,
    /// Display label for the UI
    pub label: String,
    /// Emoji icon
    pub icon: String,
    /// System prompt for this lens (kept very short — <200 tokens)
    pub system_prompt: String,
    /// Maximum output tokens (2-3 sentences)
    pub max_tokens: u32,
}

/// The result of evaluating Pete's response through a lens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perspective {
    /// Which lens produced this
    pub lens_id: String,
    /// The lens label
    pub label: String,
    /// Emoji icon
    pub icon: String,
    /// The evaluation content (2-3 sentences max)
    pub content: String,
    /// Relevance score (0.0-1.0) — how useful was this perspective?
    pub relevance: f32,
    /// Generation latency in ms
    pub latency_ms: u64,
}

/// The full set of perspectives for a single Pete response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerspectiveSet {
    pub perspectives: Vec<Perspective>,
    pub total_latency_ms: u64,
}

/// Classification of the user's message type for lens selection.
#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    /// First message in a phase (greeting/orientation)
    PhaseEntry,
    /// Substantive exchange (answering objectives, discussing content)
    Substantive,
    /// Short/casual reply
    Brief,
}

// ============================================================================
// DEFAULT LENSES
// ============================================================================

/// Bloom's Check lens — validates pedagogical alignment.
fn blooms_lens(phase: &str, blooms_level: &str) -> Lens {
    Lens {
        id: "blooms".to_string(),
        label: "BLOOM'S CHECK".to_string(),
        icon: "🔍".to_string(),
        system_prompt: format!(
            r#"You are a Bloom's Taxonomy validator. The current ADDIECRAPEYE phase is "{phase}" (Bloom's: {blooms_level}).

Evaluate whether the AI instructor's response matches the expected Bloom's level.
- If it matches: say "✓ Matches phase" and note which verb aligns.
- If it overshoots (e.g., asking to "Create" when phase is "Remember"): flag the mismatch and suggest a better approach.
- If it undershoots: note the missed opportunity.

Be extremely concise — 1-2 sentences max. No preamble."#,
            phase = phase,
            blooms_level = blooms_level,
        ),
        max_tokens: 80,
    }
}

/// Practitioner lens — cross-references with teaching experience.
fn practitioner_lens(experience: Option<&str>, audience: Option<&str>) -> Lens {
    let context = match (experience, audience) {
        (Some(exp), Some(aud)) => {
            format!("The teacher has {} experience and teaches {}.", exp, aud)
        }
        (Some(exp), None) => format!("The teacher has {} experience.", exp),
        (None, Some(aud)) => format!("The teacher teaches {}.", aud),
        (None, None) => "No teacher background info available.".to_string(),
    };

    Lens {
        id: "practitioner".to_string(),
        label: "PRACTITIONER".to_string(),
        icon: "👤".to_string(),
        system_prompt: format!(
            r#"You are an experienced teaching consultant. {context}

Would you adjust the AI instructor's advice for this teacher's level?
- If the advice is too basic for their experience, suggest going deeper.
- If it assumes too much expertise, suggest scaffolding.
- If it's well-calibrated, say so briefly.

1-2 sentences max. No preamble."#,
            context = context,
        ),
        max_tokens: 80,
    }
}

/// Devil's Advocate lens — challenges assumptions.
fn advocate_lens() -> Lens {
    Lens {
        id: "advocate".to_string(),
        label: "DEVIL'S ADVOCATE".to_string(),
        icon: "😈".to_string(),
        system_prompt: r#"You are a critical thinking partner. Find the strongest counterargument to the AI instructor's response.

- What assumption is being made that could be wrong?
- Is there an alternative approach the teacher should consider?
- What might a skeptical colleague say about this advice?

1-2 sentences max. Be constructive, not dismissive. No preamble."#.to_string(),
        max_tokens: 80,
    }
}

// ============================================================================
// LENS SELECTION
// ============================================================================

/// Decide which lenses to activate based on context.
///
/// Rules:
///   - Bloom's Check: ALWAYS active (it's the pedagogical spine)
///   - Practitioner: Only when Session Zero data is available
///   - Devil's Advocate: Only on substantive exchanges (not greetings/transitions)
pub fn select_lenses(
    phase: &str,
    blooms_level: &str,
    message_type: &MessageType,
    experience: Option<&str>,
    audience: Option<&str>,
) -> Vec<Lens> {
    let mut lenses = vec![blooms_lens(phase, blooms_level)];

    // Add Practitioner lens when the user's experience level is known
    if experience.is_some() || audience.is_some() {
        lenses.push(practitioner_lens(experience, audience));
    }

    // Add Devil's Advocate only on substantive responses
    if *message_type == MessageType::Substantive {
        lenses.push(advocate_lens());
    }

    lenses
}

/// Classify a message to determine which lenses are relevant.
pub fn classify_message(message: &str, is_first_in_phase: bool) -> MessageType {
    if is_first_in_phase {
        return MessageType::PhaseEntry;
    }

    // Brief messages: less than 20 words or very short
    let word_count = message.split_whitespace().count();
    if word_count < 8 {
        return MessageType::Brief;
    }

    MessageType::Substantive
}

// ============================================================================
// EVALUATION ENGINE
// ============================================================================

/// Evaluate Pete's response through all selected lenses in parallel.
///
/// Each lens makes an independent LLM call with:
///   - Its own system prompt (short, focused)
///   - Pete's response as the user message
///   - 80-token output budget
///
/// All lenses fire concurrently via tokio::join!.
pub async fn evaluate(llm_url: &str, pete_response: &str, lenses: &[Lens]) -> PerspectiveSet {
    let start = std::time::Instant::now();

    // Truncate Pete's response to avoid blowing up lens context
    let truncated_response = if pete_response.len() > 800 {
        format!("{}...", &pete_response[..800])
    } else {
        pete_response.to_string()
    };

    // Fire all lenses in parallel
    let mut handles = Vec::new();
    for lens in lenses {
        let url = llm_url.to_string();
        let response = truncated_response.clone();
        let lens = lens.clone();

        handles.push(tokio::spawn(async move {
            evaluate_single_lens(&url, &response, &lens).await
        }));
    }

    let mut perspectives = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(Some(perspective)) => perspectives.push(perspective),
            Ok(None) => {} // Lens returned nothing (timeout or error)
            Err(e) => warn!("[Ring 6] Lens task panicked: {}", e),
        }
    }

    let total_latency = start.elapsed().as_millis() as u64;
    info!(
        "[Ring 6] Perspectives: {} lenses evaluated in {}ms",
        perspectives.len(),
        total_latency
    );

    PerspectiveSet {
        perspectives,
        total_latency_ms: total_latency,
    }
}

/// Evaluate a single lens against Pete's response.
async fn evaluate_single_lens(
    llm_url: &str,
    pete_response: &str,
    lens: &Lens,
) -> Option<Perspective> {
    let start = std::time::Instant::now();

    let messages = vec![
        crate::ChatMessage {
            role: "system".to_string(),
            content: lens.system_prompt.clone(),
            timestamp: None,
            image_base64: None,
        },
        crate::ChatMessage {
            role: "user".to_string(),
            content: format!("Evaluate this AI instructor response:\n\n{}", pete_response),
            timestamp: None,
            image_base64: None,
        },
    ];

    // Use a short timeout — lenses must be fast
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        crate::inference::chat_completion_with_effort(
            llm_url,
            &messages,
            lens.max_tokens,
            Some("low"), // Low effort — these should be quick evaluations
        ),
    )
    .await;

    let latency_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(Ok(content)) => {
            let content = content.trim().to_string();
            if content.is_empty() {
                return None;
            }

            // Simple relevance heuristic: longer responses with specific verbs score higher
            let relevance = calculate_relevance(&content);

            Some(Perspective {
                lens_id: lens.id.clone(),
                label: lens.label.clone(),
                icon: lens.icon.clone(),
                content,
                relevance,
                latency_ms,
            })
        }
        Ok(Err(e)) => {
            warn!("[Ring 6] Lens '{}' failed: {}", lens.id, e);
            None
        }
        Err(_) => {
            warn!("[Ring 6] Lens '{}' timed out after 5s", lens.id);
            None
        }
    }
}

/// Simple relevance heuristic for perspective content.
/// Higher score = more actionable perspective.
fn calculate_relevance(content: &str) -> f32 {
    let mut score = 0.5_f32; // Base relevance

    // Boost for actionable language
    let action_words = [
        "consider",
        "suggest",
        "try",
        "instead",
        "alternative",
        "missing",
        "add",
        "adjust",
    ];
    for word in &action_words {
        if content.to_lowercase().contains(word) {
            score += 0.1;
        }
    }

    // Boost for specific Bloom's verbs
    let blooms_verbs = [
        "remember",
        "understand",
        "apply",
        "analyze",
        "evaluate",
        "create",
    ];
    for verb in &blooms_verbs {
        if content.to_lowercase().contains(verb) {
            score += 0.05;
        }
    }

    // Penalize generic responses
    if content.len() < 20 {
        score -= 0.2;
    }

    // Cap at 1.0
    score.clamp(0.0, 1.0)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_message_brief() {
        assert_eq!(classify_message("ok", false), MessageType::Brief);
        assert_eq!(classify_message("sounds good", false), MessageType::Brief);
        assert_eq!(classify_message("yes", false), MessageType::Brief);
    }

    #[test]
    fn test_classify_message_substantive() {
        let msg = "I teach 10th grade biology and my students really struggle with understanding photosynthesis. I've tried diagrams but they don't seem to engage with the concept.";
        assert_eq!(classify_message(msg, false), MessageType::Substantive);
    }

    #[test]
    fn test_classify_message_phase_entry() {
        assert_eq!(classify_message("anything", true), MessageType::PhaseEntry);
        assert_eq!(
            classify_message("I teach 10th grade biology", true),
            MessageType::PhaseEntry
        );
    }

    #[test]
    fn test_select_lenses_minimal() {
        let lenses = select_lenses("Analysis", "Remember", &MessageType::Brief, None, None);
        // Only Bloom's Check for brief messages with no session data
        assert_eq!(lenses.len(), 1);
        assert_eq!(lenses[0].id, "blooms");
    }

    #[test]
    fn test_select_lenses_full() {
        let lenses = select_lenses(
            "Analysis",
            "Remember",
            &MessageType::Substantive,
            Some("10 years K-12"),
            Some("high school students"),
        );
        // All three lenses for substantive messages with session data
        assert_eq!(lenses.len(), 3);
        assert_eq!(lenses[0].id, "blooms");
        assert_eq!(lenses[1].id, "practitioner");
        assert_eq!(lenses[2].id, "advocate");
    }

    #[test]
    fn test_select_lenses_no_advocate_on_entry() {
        let lenses = select_lenses(
            "Analysis",
            "Remember",
            &MessageType::PhaseEntry,
            Some("first year"),
            None,
        );
        // Bloom's + Practitioner, but no Advocate on phase entry
        assert_eq!(lenses.len(), 2);
        assert_eq!(lenses[0].id, "blooms");
        assert_eq!(lenses[1].id, "practitioner");
    }

    #[test]
    fn test_relevance_scoring() {
        // Generic should score low
        let generic = calculate_relevance("ok");
        assert!(generic < 0.5);

        // Actionable should score higher
        let actionable = calculate_relevance(
            "Consider adding a practice activity to help students apply the concept before moving to analysis."
        );
        assert!(actionable > 0.5);

        // Specific with Bloom's verbs should score highest
        let specific = calculate_relevance(
            "The response asks students to evaluate, but the current phase targets remember. Consider adjusting to a listing or identification activity instead."
        );
        assert!(specific > actionable);
    }

    #[test]
    fn test_blooms_lens_prompt_contains_phase() {
        let lens = blooms_lens("Design", "Understand/Apply");
        assert!(lens.system_prompt.contains("Design"));
        assert!(lens.system_prompt.contains("Understand/Apply"));
    }

    #[test]
    fn test_practitioner_lens_with_experience() {
        let lens = practitioner_lens(Some("5 years K-12"), Some("middle school"));
        assert!(lens.system_prompt.contains("5 years K-12"));
        assert!(lens.system_prompt.contains("middle school"));
    }

    #[test]
    fn test_practitioner_lens_no_data() {
        let lens = practitioner_lens(None, None);
        assert!(lens.system_prompt.contains("No teacher background"));
    }
}
