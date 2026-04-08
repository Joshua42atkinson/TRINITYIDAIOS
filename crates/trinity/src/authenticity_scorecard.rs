// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        authenticity_scorecard.rs
// PURPOSE:     Subsystem Logic
//
// 🪟 THE LIVING CODE TEXTBOOK:
// This file is part of the Trinity ID AI OS. It is designed to be read, 
// modified, and authored by YOU. As you transition from LEARNING to WORK, 
// this is where the logic lives. 
//
// 📖 THE HOOK BOOK CONNECTION:
// For a full catalogue of system capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// MATURITY:     L5 → Shippable
// QUEST_PHASE:  Integration
//
// CHANGES:
//   2026-04-08  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use trinity_protocol::CharacterSheet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticityScorecard {
    pub document_id: String,
    pub slop_penalty: f32, // 0.0 means no slop, 1.0 means pure slop
    pub vaam_resonance: f32, // 0.0-1.0
    pub player_pov_empathy: f32, // 0.0-1.0
    pub overall_authenticity: f32, // 0.0-1.0
    pub summary: String,
    pub detected_slop: Vec<String>,
}

const AI_SLOP_BLACKLIST: &[&str] = &[
    "delve", "tapestry", "seamlessly", "dive deeply",
    "a testament to", "in today's digital age", "fosters", "supercharge",
    "transformative", "crucial", "important to note", "multifaceted", 
    "in conclusion", "furthermore", "it is worth noting", "vital"
];

const PLAYER_POV_MARKERS: &[&str] = &[
    "you will", "your journey", "explore", "discover", "your goal",
    "imagine", "you", "your", "we will", "let's"
];

pub fn score_authenticity(text: &str, document_id: &str, sheet: &CharacterSheet) -> AuthenticityScorecard {
    let lower = text.to_lowercase();
    let word_count = text.split_whitespace().count().max(1);
    
    // 1. Detect Slop (Negative Heuristic)
    let mut detected_slop = Vec::new();
    let mut slop_hits = 0;
    for &slop in AI_SLOP_BLACKLIST {
        let count = lower.matches(slop).count();
        if count > 0 {
            detected_slop.push(format!("'{}' ({}x)", slop, count));
            slop_hits += count;
        }
    }
    // Very harsh penalty for slop. 4 hits is enough to be 1.0 (100% slop)
    let slop_penalty = (slop_hits as f32 / 4.0).clamp(0.0, 1.0);
    
    // 2. VAAM Resonance (Positive Heuristic)
    let mut vaam_hits = 0;
    let expected_words = sheet.vaam_profile.word_weights.len().max(1);
    for word in sheet.vaam_profile.word_weights.keys() {
        if lower.contains(&word.to_lowercase()) {
            vaam_hits += 1;
        }
    }
    
    // If we hit at least 3 vaam words, or 100% of small lists, we resonate.
    let resonance_target = (expected_words as f32 * 0.3).min(3.0).max(1.0); 
    let vaam_resonance = if sheet.vaam_profile.word_weights.is_empty() {
        1.0 // Skip penalty if empty profile
    } else {
        (vaam_hits as f32 / resonance_target).clamp(0.0, 1.0)
    };
    
    // 3. Player POV Empathy (Positive Heuristic)
    let mut pov_hits = 0;
    for &marker in PLAYER_POV_MARKERS {
        pov_hits += lower.matches(marker).count();
    }
    // We expect roughly 1 player POV marker per 50 words
    let pov_expected = (word_count as f32 / 50.0).max(2.0);
    let player_pov_empathy = (pov_hits as f32 / pov_expected).clamp(0.0, 1.0);
    
    // Overall = 1.0 - Slop, weighted with VAAM and Empathy.
    // Slop is toxic, so it destroys the score heavily.
    let base_positive = (vaam_resonance * 0.5) + (player_pov_empathy * 0.5);
    let overall_authenticity = (base_positive * (1.0 - (slop_penalty * 0.8))).clamp(0.0, 1.0);
    
    let summary = if slop_penalty > 0.6 {
        "High machine homogenization detected. Product lacks human authenticity and feels generated.".to_string()
    } else if overall_authenticity > 0.8 {
        "Strong SME voice. Good player empathy and clean vocabulary.".to_string()
    } else {
        "Acceptable, but consider elevating direct player framing and using specific domain vocabulary.".to_string()
    };
    
    AuthenticityScorecard {
        document_id: document_id.to_string(),
        slop_penalty,
        vaam_resonance,
        player_pov_empathy,
        overall_authenticity,
        summary,
        detected_slop
    }
}

pub fn evaluate_and_trigger_if_needed(text: &str, path: &str) {
    // Only evaluate substantial newly written content
    if text.len() < 500 || (!path.ends_with(".md") && !path.ends_with(".rs") && !path.ends_with(".txt")) {
        return;
    }

    let text_clone = text.to_string();
    let path_clone = path.to_string();

    tokio::spawn(async move {
        let sheet = crate::character_sheet::load_character_sheet();
        let scorecard = score_authenticity(&text_clone, &path_clone, &sheet);
        
        // If it's too sloppy or disconnected, page Pete on the background queue.
        if scorecard.slop_penalty > 0.6 || (scorecard.overall_authenticity < 0.3 && scorecard.player_pov_empathy < 0.2) {
            tracing::warn!("User Autopoiesis triggered for {}: {}", path_clone, scorecard.summary);
            
            let client = reqwest::Client::new();
            let payload = serde_json::json!({
                "message": format!(
                    "USER AUTOPOIESIS ALERT: The artifact '{}' was just written, but it lacks the Operator's authentic voice.\n\
                     Slop Penalty: {:.2}\n\
                     Detected Slop: {:?}\n\
                     Player POV Empathy: {:.2}\n\
                     Overall Authenticity: {:.2}\n\n\
                     Your task: Review the file you just wrote. Strip out the generic AI terms (delve, tapestry, seamlessly, etc) \
                     and rewrite it using the user's authentic Subject Matter Expert tone and directly engage the Player POV.",
                    path_clone, scorecard.slop_penalty, scorecard.detected_slop, scorecard.player_pov_empathy, scorecard.overall_authenticity
                ),
                "mode": "engineer",
                "max_turns": 3
            });
            
            let _ = client.post("http://127.0.0.1:3000/api/jobs")
                .json(&payload)
                .send()
                .await;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use trinity_protocol::UserClass;

    fn test_sheet() -> CharacterSheet {
        let mut sheet = CharacterSheet::new("Tester", UserClass::InstructionalDesigner);
        sheet.vaam_profile.record_word_usage("sandbox", true);
        sheet.vaam_profile.record_word_usage("heuristic", true);
        sheet
    }

    #[test]
    fn test_slop_detection() {
        let text = "In today's digital age, this tapestry of learning seamlessly integrates multifaceted ideas to delve into crucial matters. Furthermore, it is a testament to progress.";
        let scorecard = score_authenticity(text, "slop.md", &test_sheet());
        
        // This is pure slop.
        assert!(scorecard.slop_penalty > 0.8);
        assert!(!scorecard.detected_slop.is_empty());
        assert!(scorecard.overall_authenticity < 0.3);
    }

    #[test]
    fn test_authentic_voice() {
        let text = "You will explore the mechanical sandbox, testing your heuristic limits as you progress on your journey.";
        let scorecard = score_authenticity(text, "authentic.md", &test_sheet());
        
        // No slop
        assert_eq!(scorecard.slop_penalty, 0.0);
        // Player POV matches "you will", "explore", "your journey", "your"
        assert!(scorecard.player_pov_empathy > 0.8);
        // VAAM used "sandbox", "heuristic"
        assert!(scorecard.vaam_resonance > 0.9);
        // High overall
        assert!(scorecard.overall_authenticity > 0.8);
    }
}
