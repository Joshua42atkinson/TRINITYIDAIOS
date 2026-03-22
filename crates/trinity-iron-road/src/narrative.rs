// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-iron-road/src/narrative.rs
// PURPOSE: LitRPG prose generation from quest events and CharacterSheet state
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use trinity_protocol::Genre;

/// Context for narrative generation — captures the game state at the moment of the event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeContext {
    pub genre: Genre,
    pub phase: String,
    pub last_action: String,
    pub coal: f32,
    pub steam: u32,
    pub xp: u64,
    pub resonance_level: u32,
    pub alias: String,
}

/// The narrative engine that transforms quest events into LitRPG prose
pub struct NarrativeEngine {
    /// vLLM inference URL
    inference_url: String,
}

impl NarrativeEngine {
    pub fn new(inference_url: &str) -> Self {
        Self {
            inference_url: inference_url.to_string(),
        }
    }

    /// Generate LitRPG prose for a quest event by calling the Conductor model
    pub async fn generate_prose(
        &self,
        ctx: &NarrativeContext,
        event_description: &str,
    ) -> anyhow::Result<String> {
        let system_prompt = build_narrative_system_prompt(ctx);
        let user_prompt = format!(
            "Write a short LitRPG narrative paragraph (3-5 sentences) for this event:\n\
             Player: {} (Resonance Level {})\n\
             Phase: {}\n\
             Coal: {:.0}, Steam: {}, XP: {}\n\
             Event: {}",
            ctx.alias,
            ctx.resonance_level,
            ctx.phase,
            ctx.coal,
            ctx.steam,
            ctx.xp,
            event_description
        );

        match call_inference(&self.inference_url, &system_prompt, &user_prompt).await {
            Ok(prose) => Ok(prose),
            Err(e) => {
                tracing::warn!("[Narrative] Inference failed, using fallback: {}", e);
                Ok(generate_fallback_prose(ctx, event_description))
            }
        }
    }
}

/// Build a genre-appropriate system prompt for narrative generation
fn build_narrative_system_prompt(ctx: &NarrativeContext) -> String {
    let genre_flavor = match ctx.genre {
        Genre::Cyberpunk => {
            "neon-lit cyberpunk city with holographic interfaces and neural implants"
        }
        Genre::Steampunk => "steam-powered Victorian workshop with brass gears and pneumatic tubes",
        Genre::Solarpunk => {
            "sunlit biodome where organic technology and sustainable systems flourish"
        }
        Genre::DarkFantasy => {
            "shadow-draped tower where ancient runes pulse with forbidden knowledge"
        }
    };

    format!(
        "You are the narrator of an educational LitRPG game called The Iron Road.\n\
         The setting is a {}.\n\
         The player is building an educational game using AI tools.\n\
         Write immersive but concise prose that makes software development feel like an epic quest.\n\
         Use VAAM principles: highlight key technical vocabulary in the narrative.\n\
         Keep paragraphs to 3-5 sentences. Be vivid but not verbose.",
        genre_flavor
    )
}

/// Call the inference endpoint (vLLM OpenAI-compatible API)
async fn call_inference(
    base_url: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> anyhow::Result<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    let body = serde_json::json!({
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": user_prompt }
        ],
        "max_tokens": 256,
        "temperature": 0.8,
        "stream": false
    });

    let response = client
        .post(format!("{}/v1/chat/completions", base_url))
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Inference returned {}", response.status());
    }

    let json: serde_json::Value = response.json().await?;
    json["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("No content in response"))
}

/// Fallback prose when the Conductor model is offline
fn generate_fallback_prose(ctx: &NarrativeContext, event: &str) -> String {
    format!(
        "{} pressed forward through the {} phase, their Coal reserves at {:.0}. \
         The action — {} — sent ripples through the system. \
         Steam gauge read {} units. Another step on the Iron Road.",
        ctx.alias, ctx.phase, ctx.coal, event, ctx.steam
    )
}

/// Generate a d20 critical hit narrative (used when a tool call succeeds spectacularly)
pub fn generate_critical_narrative(ctx: &NarrativeContext) -> String {
    format!(
        "⚡ CRITICAL SUCCESS! {}'s command resonated perfectly through the {} phase. \
         The system hummed with alignment — +50 XP bonus. Coal burned bright at {:.0}.",
        ctx.alias, ctx.phase, ctx.coal
    )
}

/// Generate a d20 fumble narrative (used when a tool call fails)
pub fn generate_fumble_narrative(ctx: &NarrativeContext) -> String {
    format!(
        "💥 FUMBLE! {}'s action misfired during the {} phase. \
         The Cow Catcher absorbed the impact. Coal dropped to {:.0}. \
         Pete's voice echoed: \"A structural collapse. Let's analyze the rubble.\"",
        ctx.alias, ctx.phase, ctx.coal
    )
}
