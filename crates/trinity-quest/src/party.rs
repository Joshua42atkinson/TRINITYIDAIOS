// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-quest/src/party.rs
// ═══════════════════════════════════════════════════════════════════════════════
//
// PURPOSE:     AI Party System — P-ART-Y members
//
// THE P-ART-Y FRAMEWORK:
//   P = Pete          — The only AI personality (Mistral Small 4 119B, static RAM)
//   A = Aesthetics    — Functional mode: CRAP visual design, ComfyUI/MusicGPT
//   R = Research      — Functional mode: QM audits, tests, CI/CD, security
//   T = Tempo         — Functional mode: code gen, boilerplate, momentum keeper
//   Y = You           — The Yardmaster (executive core — You manage the yard)
//
// ARCHITECTURE:
//   Pete is the ONLY characterized AI — one personality, one static RAM load
//   (~68GB Mistral Small 4 119B). The ART modes are just different system
//   prompts sent to the same brain. No model hot-swapping, no persona mess.
//   You (the Yardmaster) choose which functional mode to engage.
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::hero::Phase;
use serde::{Deserialize, Serialize};

/// An AI party member the player can bring on quests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyMember {
    pub id: String,
    pub name: String,
    pub role: String,
    pub model: String,
    pub memory_gb: f32,
    pub specialty: Vec<Phase>,
    pub icon: String,
    pub available: bool,
    pub active: bool,
}

/// The full P-ART-Y roster
///
/// Pete is the ONLY AI personality — one brain, one static RAM load.
/// Aesthetics, Research, and Tempo are functional modes (system prompts).
/// You are the Yardmaster.
pub fn default_party() -> Vec<PartyMember> {
    vec![
        // ─── P: Pete — The Only Personality ──────────────────────────────
        // Socratic Mirror. The one AI character. Permanent resident.
        // Asks WHY, not WHAT. Forces active recall via Bloom's Taxonomy.
        // All other modes are just system prompts routed through Pete.
        PartyMember {
            id: "pete".into(),
            name: "Ask Pete".into(),
            role: "P — The Socratic Mirror (the only AI personality)".into(),
            model: "Mistral-Small-4-119B".into(),
            memory_gb: 68.0,
            specialty: vec![
                Phase::Analysis,
                Phase::Design,
                Phase::Evaluation,
                Phase::Alignment,
                Phase::Envision,
            ],
            icon: "🎓".into(),
            available: true,
            active: true, // Always active — single static RAM load
        },
        // ─── A: Aesthetics — Functional Mode ─────────────────────────────
        // CRAP design enforcer. Visual hierarchy, ComfyUI image gen.
        // Same brain as Pete, different system prompt.
        PartyMember {
            id: "aesthetics".into(),
            name: "The Visionary".into(),
            role: "A — Aesthetics mode: CRAP visual design, ComfyUI assets".into(),
            model: "Mistral-Small-4-119B".into(),
            memory_gb: 0.0, // No additional RAM — same brain as Pete
            specialty: vec![Phase::Design, Phase::Contrast, Phase::Proximity],
            icon: "🎨".into(),
            available: true,
            active: false,
        },
        // ─── R: Research — Functional Mode ───────────────────────────────
        // QM rubric enforcer. Test gen, security audits, cargo clippy/test.
        // Same brain as Pete, different system prompt.
        PartyMember {
            id: "research".into(),
            name: "The Brakeman".into(),
            role: "R — Research mode: QM audits, test gen, security".into(),
            model: "Mistral-Small-4-119B".into(),
            memory_gb: 0.0, // No additional RAM — same brain as Pete
            specialty: vec![Phase::Evaluation, Phase::Alignment, Phase::Repetition],
            icon: "🛡️".into(),
            available: true,
            active: false,
        },
        // ─── T: Tempo — Functional Mode ──────────────────────────────────
        // Momentum keeper. Code gen, Bevy scaffolding, 30-second loop.
        // Same brain as Pete, different system prompt.
        PartyMember {
            id: "tempo".into(),
            name: "The Engineer".into(),
            role: "T — Tempo mode: code gen, Bevy scaffolding, momentum".into(),
            model: "Mistral-Small-4-119B".into(),
            memory_gb: 0.0, // No additional RAM — same brain as Pete
            specialty: vec![
                Phase::Development,
                Phase::Implementation,
                Phase::Repetition,
                Phase::Yoke,
            ],
            icon: "⚙️".into(),
            available: true,
            active: false,
        },
        // ─── Y: You — The Yardmaster ─────────────────────────────────────
        // You are the Yardmaster. You don't lay every piece of iron —
        // you manage the flow of the yard. The Executive Core.
    ]
}
