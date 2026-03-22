//! ═══════════════════════════════════════════════════════════════════════════════
//! P-ART-Y PARTY SYSTEM — MODEL CONFIGURATION (FUTURE MULTI-MODEL)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! *** CURRENT: Single brain (Mistral Small 4 119B, static RAM load) ***
//! *** FUTURE: These roles map to dedicated model slots for hot-swapping ***
//!
//! P-ART-Y roles:
//!   pete        — "P" Socratic Mirror: The only AI personality
//!   aesthetics  — "A" The Visionary: CRAP visual design mode
//!   research    — "R" The Brakeman: QM audits, tests, CI/CD mode
//!   tempo       — "T" The Engineer: Code gen, momentum mode
//!   You         — "Y" The Yardmaster: Executive core (the user)
//!
//! ARCHITECTURE:
//!   Currently Lone Wolf: single Mistral Small 4 119B handles all roles.
//!   The model configs below are preserved for future multi-model support.
//!
//! ═══════════════════════════════════════════════════════════════════════════════

use std::path::PathBuf;

/// A model to load via llama-server
#[derive(Debug, Clone)]
pub struct ModelSlot {
    pub name: &'static str,
    pub filename: &'static str,
    pub context_size: u32,
    pub port: u16,
}

/// Complete role configuration
#[derive(Debug, Clone)]
pub struct RoleConfig {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
    pub primary: ModelSlot,
    pub secondary: Option<ModelSlot>,
    pub tertiary: Option<ModelSlot>,
    #[allow(dead_code)] // Used for future role-based quest matching in sidecar
    pub quest_specialties: &'static [&'static str],
    pub addie_phases: &'static [&'static str],
    pub party_skill: &'static str,
}

// ═══════════════════════════════════════════════════════════════════════════════
// FINAL MODEL FILES — LOCKED IN PER USER REQUEST (Mar 16, 2026)
// ═══════════════════════════════════════════════════════════════════════════════

/// *** FINAL: Nemotron-3-Super-120B-A12B — The Conductor's brain ***
/// Location: ~/ai_models/gguf/conductor/ (3-part split, ~74GB total Q4_K_M)
/// HuggingFace: nvidia/NVIDIA-Nemotron-3-Super-120B-A12B-BF16
/// Doc: docs/model_specs/Nemotron.md
const NEMOTRON_FILE: &str = "NVIDIA-Nemotron-3-Super-120B-A12B-BF16-Q4_K_M-00001-of-00003.gguf";

/// *** FINAL: Step-Flash-121B — The Engineer's code forge ***
/// Location: ~/trinity-models/gguf/ (83GB Q4_K_S)
/// Doc: docs/model_specs/Step-Flash.md
const STEP_FLASH_FILE: &str = "Step-3.5-Flash-REAP-121B-A11B.Q4_K_S.gguf";

/// *** FINAL: Crow-9B-Opus — Artist Brain ***
/// Location: ~/trinity-models/gguf/ (5.3GB Q4_K_M)
/// HuggingFace: Crownelius/Crow-9B-HERETIC
/// Doc: docs/model_specs/Crow.md
const CROW_OPUS_FILE: &str = "Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf";

/// *** FINAL: OmniCoder-9B — Artist Evaluator ***
/// Location: ~/trinity-models/gguf/ (5.4GB Q4_K_M)
/// HuggingFace: Tesslate/OmniCoder-9B
/// Doc: docs/model_specs/OmniCoder.md
const OMNICODER_FILE: &str = "OmniCoder-9B-Q4_K_M.gguf";

/// *** FINAL: Qwen3-Coder-REAP-25B-Rust — Artist Builder ***
/// Location: ~/trinity-models/gguf/ (15GB Q4_K_M)
/// HuggingFace: cerebras/Qwen3-Coder-REAP-25B-A3B
/// Doc: docs/model_specs/Qwen3-REAP-Rust.md
const REAP_RUST_FILE: &str = "Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf";

/// Port assignments (shared — only one sidecar at a time)
const PRIMARY_PORT: u16 = 8081;
const SECONDARY_PORT: u16 = 8082;
const TERTIARY_PORT: u16 = 8083;

pub fn get_role(role_id: &str) -> Option<RoleConfig> {
    match role_id {
        "pete" | "conductor" | "p" => Some(pete()),
        "aesthetics" | "artist" | "art" | "a" => Some(aesthetics()),
        "tempo" | "engineer" | "t" => Some(tempo()),
        "research" | "brakeman" | "evaluator" | "r" => Some(research()),
        _ => None,
    }
}

#[allow(dead_code)] // Reserved for GET /api/party/roles endpoint
pub fn list_roles() -> Vec<RoleConfig> {
    vec![pete(), aesthetics(), research(), tempo()]
}

// ═══════════════════════════════════════════════════════════════════════════════
// P — PETE (The Only Personality)
// ═══════════════════════════════════════════════════════════════════════════════

/// P — Pete (Socratic Mirror)
/// The only AI personality. Currently: Mistral Small 4 119B (static RAM load).
/// Future multi-model: Nemotron-3-Super-120B (~74GB Q4_K_M)
fn pete() -> RoleConfig {
    RoleConfig {
        id: "pete",
        name: "Ask Pete (Socratic Mirror)",
        icon: "🎓",
        description: "P — The only AI personality. Socratic dialogue, Bloom's alignment, ADDIE orchestration.",
        primary: ModelSlot {
            name: "Pete (Nemotron-3-Super-120B)",
            filename: NEMOTRON_FILE,
            context_size: 131072,
            port: PRIMARY_PORT,
        },
        secondary: None,
        tertiary: None,
        quest_specialties: &["orchestration", "planning", "strategy", "socratic_dialogue"],
        addie_phases: &["Analysis", "Design", "Evaluation", "Alignment", "Envision"],
        party_skill: "Socratic Mirror — asks WHY not WHAT, forces active recall via Bloom's Taxonomy.",
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// A — AESTHETICS (The Visionary — CRAP Design Mode)
// ═══════════════════════════════════════════════════════════════════════════════

/// A — Aesthetics (The Visionary)
/// CRAP design enforcer. Currently: same Mistral brain, different system prompt.
/// Future multi-model: Crow 9B + OmniCoder 9B + Qwen 25B Rust triad swarm.
fn aesthetics() -> RoleConfig {
    RoleConfig {
        id: "aesthetics",
        name: "The Visionary (Aesthetics)",
        icon: "🎨",
        description: "A — CRAP visual design mode: Contrast, Repetition, Alignment, Proximity. ComfyUI assets.",
        primary: ModelSlot {
            name: "Aesthetics Brain (Crow 9B Opus)",
            filename: CROW_OPUS_FILE,
            context_size: 32768,
            port: PRIMARY_PORT,
        },
        secondary: Some(ModelSlot {
            name: "Aesthetics Evaluator (OmniCoder 9B)",
            filename: OMNICODER_FILE,
            context_size: 16384,
            port: SECONDARY_PORT,
        }),
        tertiary: Some(ModelSlot {
            name: "Aesthetics Builder (Qwen 25B Rust)",
            filename: REAP_RUST_FILE,
            context_size: 16384,
            port: TERTIARY_PORT,
        }),
        quest_specialties: &["design", "assets", "ui_ux", "comfyui"],
        addie_phases: &["Design", "Contrast", "Proximity"],
        party_skill: "CRAP Enforcer — visual hierarchy, ComfyUI orchestration, UI boundary design.",
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// R — RESEARCH (The Brakeman — QM Audits Mode)
// ═══════════════════════════════════════════════════════════════════════════════

/// R — Research (The Brakeman)
/// QM rubric enforcer. Currently: same Mistral brain, different system prompt.
fn research() -> RoleConfig {
    RoleConfig {
        id: "research",
        name: "The Brakeman (Research)",
        icon: "🛡️",
        description: "R — Research mode: QM rubric audits, test generation, security, cargo clippy/test.",
        primary: ModelSlot {
            name: "Research Brain (Crow 9B Opus)",
            filename: CROW_OPUS_FILE,
            context_size: 32768,
            port: PRIMARY_PORT,
        },
        secondary: None,
        tertiary: None,
        quest_specialties: &["evaluation", "testing", "security", "quality"],
        addie_phases: &["Evaluation", "Alignment", "Repetition"],
        party_skill: "The Cow Catcher — catches bugs, enforces QM rubrics, runs CI/CD audits.",
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// T — TEMPO (The Engineer — Momentum Mode)
// ═══════════════════════════════════════════════════════════════════════════════

/// T — Tempo (The Engineer)
/// Momentum keeper. Currently: same Mistral brain, different system prompt.
/// Future multi-model: Step-Flash-121B (83GB) for raw code gen power.
fn tempo() -> RoleConfig {
    RoleConfig {
        id: "tempo",
        name: "The Engineer (Tempo)",
        icon: "⚙️",
        description: "T — Tempo mode: code gen, Bevy scaffolding, 30-second loop momentum.",
        primary: ModelSlot {
            name: "Tempo Code Forge (Step-Flash 121B)",
            filename: STEP_FLASH_FILE,
            context_size: 16384,
            port: PRIMARY_PORT,
        },
        secondary: None,
        tertiary: None,
        quest_specialties: &["bugfix", "feature", "refactor", "code_generation"],
        addie_phases: &["Development", "Implementation", "Repetition"],
        party_skill: "Momentum Forge — code gen, boilerplate scaffolding, 30-second productivity loop.",
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MODEL DIRECTORY HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Get the model directory for Artist and Engineer sidecars
/// ~/trinity-models/gguf/
pub fn model_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/joshua".to_string());
    PathBuf::from(home).join("trinity-models/gguf")
}

/// Get the model directory for the Conductor (Nemotron 3-part split)
/// ~/ai_models/gguf/conductor/
pub fn conductor_model_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/joshua".to_string());
    PathBuf::from(home).join("ai_models/gguf/conductor")
}

/// Check if a role's models exist
pub fn validate_models(role: &RoleConfig) -> Result<(), String> {
    // Pete uses special directory for 3-part split (Nemotron)
    let dir = if role.id == "pete" {
        conductor_model_dir()
    } else {
        model_dir()
    };

    let primary_path = dir.join(role.primary.filename);
    if !primary_path.exists() {
        return Err(format!(
            "Primary model not found: {}",
            primary_path.display()
        ));
    }

    if let Some(ref secondary) = role.secondary {
        let secondary_path = dir.join(secondary.filename);
        if !secondary_path.exists() {
            return Err(format!(
                "Secondary model not found: {}",
                secondary_path.display()
            ));
        }
    }

    if let Some(ref tertiary) = role.tertiary {
        let tertiary_path = dir.join(tertiary.filename);
        if !tertiary_path.exists() {
            return Err(format!(
                "Tertiary model not found: {}",
                tertiary_path.display()
            ));
        }
    }

    Ok(())
}
