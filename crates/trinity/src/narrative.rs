// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        narrative.rs
// PURPOSE:     Great Recycler narrative generation for Iron Road Book
//
// ARCHITECTURE:
//   • Genre-aware LitRPG prose generation
//   • Uses LLM via inference module for generation
//   • Streams narrative updates via SSE to /api/book/stream
//   • Per IRON_ROAD_LITRPG_FRAMEWORK.md lines 52-59 (30-second loop phase 5)
//
// DEPENDENCIES:
//   - crate::inference — LLM inference backend
//   - trinity_protocol — Genre enum
//   - trinity_quest — HeroStage, Phase
//   - serde — Serialization
//
// CHANGES:
//   2026-03-16  Cascade  Created for 30-second core loop
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use trinity_protocol::Genre;
use trinity_quest::{HeroStage, Phase};

/// Narrative context for generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeContext {
    /// User's selected genre
    pub genre: Genre,
    /// Current Hero's Journey stage
    pub hero_stage: HeroStage,
    /// Current ADDIE phase
    pub phase: Phase,
    /// Last player action description
    pub last_action: String,
    /// Current coal reserves
    pub coal: f32,
    /// Current steam generated
    pub steam: f32,
    /// Current XP earned
    pub xp: u32,
    /// Player alias
    pub alias: String,
}

impl Default for NarrativeContext {
    fn default() -> Self {
        Self {
            genre: Genre::Cyberpunk,
            hero_stage: HeroStage::OrdinaryWorld,
            phase: Phase::Analysis,
            last_action: String::new(),
            coal: 87.0,
            steam: 0.0,
            xp: 0,
            alias: "Architect".to_string(),
        }
    }
}

/// Generated narrative entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // Fields populated via serde deserialization
pub struct NarrativeEntry {
    /// Unique identifier
    pub id: String,
    /// Timestamp
    pub timestamp: String,
    /// Entry type
    pub entry_type: String,
    /// Narrative content
    pub content: String,
    /// Genre
    pub genre: Genre,
    /// Station number (1-12)
    pub station: u8,
}

/// Genre-specific style guide for narrative generation
pub fn genre_style_guide(genre: Genre) -> &'static str {
    match genre {
        Genre::Cyberpunk =>
            "Neon lights flicker through rain-slicked streets. Chrome and circuitry pulse with data. \
             Holographic ads cast ghostly shadows. The air hums with digital noise.",
        Genre::Steampunk =>
            "Brass gears click and steam hisses from copper pipes. Victorian elegance meets industrial grit. \
             Clockwork mechanisms whir beneath polished mahogany. The scent of coal smoke lingers.",
        Genre::Solarpunk =>
            "Living architecture breathes with green vines. Solar glass catches the golden light. \
             Wind turbines spin lazily against azure skies. Abundance flows through crystalline channels.",
        Genre::DarkFantasy =>
            "Gothic stone towers pierce the mist. Flickering candles cast dancing shadows. \
             Ancient tomes whisper forgotten secrets. Eldritch horrors stir in the darkness.",
    }
}

/// Station description for narrative context
pub fn station_description(stage: HeroStage) -> &'static str {
    match stage {
        HeroStage::OrdinaryWorld => {
            "Station 1: The Ordinary World — The Architect awakens in the Junkyard Peak, \
             surrounded by the detritus of abandoned projects and half-formed dreams."
        }
        HeroStage::CallToAdventure => {
            "Station 2: The Call to Adventure — A signal from the Blueprint Mesa beckons. \
             The Iron Road awaits its first traveler."
        }
        HeroStage::RefusalOfTheCall => {
            "Station 3: Refusal of the Call — Doubt creeps in at the Code Forges. \
             Scope Creeps whisper tempting distractions."
        }
        HeroStage::MeetingTheMentor => {
            "Station 4: Meeting the Mentor — The Great Recycler emerges from the digital mist, \
             offering wisdom and the first tools of the trade."
        }
        HeroStage::CrossingTheThreshold => {
            "Station 5: Crossing the Threshold — The Engineer's Gate looms ahead. \
             Beyond lies the true Iron Road."
        }
        HeroStage::TestsAlliesEnemies => {
            "Station 6: Tests, Allies, & Enemies — The party assembles as challenges mount. \
             Each member brings unique strengths to the journey."
        }
        HeroStage::ApproachToInmostCave => {
            "Station 7: Approach to the Inmost Cave — The core loop takes shape. \
             Germane Load must be balanced against the weight of Cargo."
        }
        HeroStage::TheOrdeal => {
            "Station 8: The Ordeal — The great challenge manifests. Code must compile. \
             The Borrow Checker demands perfection."
        }
        HeroStage::TheReward => {
            "Station 9: The Reward — Victory! The first successful build. Steam rises triumphantly \
             from the locomotive's stack."
        }
        HeroStage::TheRoadBack => {
            "Station 10: The Road Back — The pilot test reveals imperfections. \
             The journey is far from over."
        }
        HeroStage::TheResurrection => {
            "Station 11: The Resurrection — Final polish. WCAG compliance. Quality Matters review. \
             The Golem begins to breathe."
        }
        HeroStage::ReturnWithElixir => {
            "Station 12: Return with the Elixir — The WASM build succeeds. The Golem lives. \
             +5 Resonance. The Iron Road is conquered."
        }
    }
}

/// Build the Great Recycler system prompt
pub fn build_narrative_system_prompt(context: &NarrativeContext) -> String {
    let style = genre_style_guide(context.genre);
    let station = station_description(context.hero_stage);

    format!(
        r#"You are the Great Recycler, narrator of the Iron Road LitRPG.

GENRE: {:?}
{}

CURRENT LOCATION:
{}

PLAYER STATE:
- Name: {}
- Coal (energy reserves): {:.1}
- Steam (momentum): {:.1}  
- XP earned: {}

Write ONE paragraph (2-4 sentences) of LitRPG prose describing the current moment.
Match the genre's sensory aesthetic exactly.
Include the player's name and reference their resources naturally.
End with a hook or question that prompts the next action.

STYLE: Evocative, immersive, second-person perspective. The player IS the Architect."#,
        context.genre, style, station, context.alias, context.coal, context.steam, context.xp,
    )
}

/// Generate narrative via LLM inference
pub async fn generate_narrative(llm_url: &str, context: &NarrativeContext) -> Option<String> {
    let system_prompt = build_narrative_system_prompt(context);

    // Build messages for chat completion
    let messages = vec![
        crate::ChatMessage {
            role: "system".to_string(),
            content: system_prompt,
            timestamp: None,
            image_base64: None,
        },
        crate::ChatMessage {
            role: "user".to_string(),
            content: format!("What happens next after: {}", context.last_action),
            timestamp: None,
            image_base64: None,
        },
    ];

    // Call inference module
    match crate::inference::chat_completion(llm_url, &messages, 256).await {
        Ok(response) => {
            info!(
                "[Great Recycler] Generated {} chars of narrative",
                response.len()
            );
            Some(response)
        }
        Err(e) => {
            warn!("[Great Recycler] Narrative generation failed: {}", e);
            None
        }
    }
}

/// Create a narrative entry from generated content
pub fn create_entry(content: String, context: &NarrativeContext) -> NarrativeEntry {
    NarrativeEntry {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        entry_type: "narrative_update".to_string(),
        content,
        genre: context.genre,
        station: context.hero_stage.chapter(),
    }
}

/// Generate a failure narrative (Heavilon Protocol)
pub fn generate_failure_narrative(context: &NarrativeContext, failure_reason: &str) -> String {
    let style = genre_style_guide(context.genre);

    format!(
        "The attempt fails. {} The Architect's coal reserves flicker as steam escapes \
         into the void. \"A structural collapse,\" Pete observes. \"Let's analyze the rubble.\" \
         (Heavilon Protocol activated: {} — Must analyze before retry.)",
        style.split('.').next().unwrap_or(""),
        failure_reason
    )
}

/// Generate a success narrative with resource rewards
pub fn generate_success_narrative(
    context: &NarrativeContext,
    coal_burned: f32,
    steam_gained: f32,
    xp_gained: u32,
) -> String {
    let _style = genre_style_guide(context.genre);

    format!(
        "Success ripples through the {}. Coal burns bright in the firebox ({:.1} consumed), \
         transforming into precious steam ({:.1} generated). The locomotive surges forward. \
         +{} XP. {}'s Resonance grows stronger.",
        match context.genre {
            Genre::Cyberpunk => "neon-lit terminal",
            Genre::Steampunk => "brass-engineered machinery",
            Genre::Solarpunk => "crystalline conduits",
            Genre::DarkFantasy => "enchanted forges",
        },
        coal_burned,
        steam_gained,
        xp_gained,
        context.alias
    )
}

/// Generate a critical success narrative (natural 20)
pub fn generate_critical_narrative(context: &NarrativeContext) -> String {
    format!(
        "CRITICAL SUCCESS! The dice roll a natural 20! {}'s action transcends ordinary limits. \
         Steam erupts in a brilliant plume. The Great Recycler pauses, impressed. \
         \"Now THAT,\" Pete grins, \"is how you forge the Iron Road.\" \
         (Double Steam generation, bonus XP awarded.)",
        context.alias
    )
}

/// Generate a fumble narrative (natural 1)
pub fn generate_fumble_narrative(context: &NarrativeContext) -> String {
    format!(
        "FUMBLE! A natural 1! The action goes catastrophically wrong. {} \
         The Engineer winces. \"That's going to need more than a simple fix.\" \
         Coal wasted, no Steam generated. The Heavilon Protocol demands thorough analysis \
         before the Architect can attempt this again.",
        genre_style_guide(context.genre)
            .split('.')
            .next()
            .unwrap_or("")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genre_style_guide() {
        let cyber = genre_style_guide(Genre::Cyberpunk);
        assert!(cyber.to_lowercase().contains("neon"));

        let steam = genre_style_guide(Genre::Steampunk);
        assert!(steam.to_lowercase().contains("brass"));
    }

    #[test]
    fn test_station_description() {
        let desc = station_description(HeroStage::OrdinaryWorld);
        assert!(desc.contains("Station 1"));
        assert!(desc.contains("Junkyard Peak"));
    }

    #[test]
    fn test_build_system_prompt() {
        let context = NarrativeContext {
            genre: Genre::Steampunk,
            hero_stage: HeroStage::MeetingTheMentor,
            phase: Phase::Analysis,
            alias: "TestArchitect".to_string(),
            coal: 50.0,
            steam: 25.0,
            xp: 100,
            ..Default::default()
        };

        let prompt = build_narrative_system_prompt(&context);
        assert!(prompt.contains("Steampunk"));
        assert!(prompt.contains("TestArchitect"));
        assert!(prompt.contains("50.0"));
    }

    #[test]
    fn test_failure_narrative() {
        let context = NarrativeContext::default();
        let narrative = generate_failure_narrative(&context, "DC 15 missed by 5");
        assert!(narrative.contains("Heavilon Protocol"));
    }

    #[test]
    fn test_critical_narrative() {
        let context = NarrativeContext {
            alias: "Alice".to_string(),
            ..Default::default()
        };
        let narrative = generate_critical_narrative(&context);
        assert!(narrative.contains("CRITICAL SUCCESS"));
        assert!(narrative.contains("Alice"));
    }
}
