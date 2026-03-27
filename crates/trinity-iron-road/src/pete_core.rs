// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Iron Road Game Engine
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:         pete_core.rs
// BIBLE CAR:    Car 8 — ALIGNMENT (Pete's Socratic Protocol)
// HOOK SCHOOL:  🏫 Pedagogy — Socratic Interview
// PURPOSE:      Programmer Pete's evaluation and response core. Evaluates user
//               input against their VAAM tier and generates pedagogically
//               calibrated responses via the Conductor model (Mistral Small 4
//               119B MoE). This is the lowest-level Socratic engine — it
//               translates VAAM vocabulary complexity into system prompt
//               constraints that shape Pete's language and scaffolding depth.
//
// ARCHITECTURE:
//   • PeteCore wraps a single HTTP client pointed at vLLM/llama-server
//   • evaluate_and_respond() builds a tier-aware system prompt and calls
//     the OpenAI-compatible /v1/chat/completions endpoint
//   • VAAM tier determines vocabulary complexity in Pete's responses
//   • Called from the agent loop when Iron Road mode needs direct Pete output
//
// DEPENDENCIES:
//   - reqwest   — HTTP client for LLM inference endpoint
//   - serde_json — JSON payload construction
//
// CHANGES:
//   2026-03-16  Joshua Atkinson  Created for Pete evaluation pipeline
//   2026-03-26  Cascade          Added §17 header
//
// ═══════════════════════════════════════════════════════════════════════════════

use reqwest::Client;
use serde_json::json;

pub struct PeteCore {
    vllm_url: String,
    client: Client,
}

impl PeteCore {
    pub fn new(vllm_url: &str) -> Self {
        Self {
            vllm_url: vllm_url.to_string(),
            client: Client::new(),
        }
    }

    /// Evaluates the user's input against their VAAM tier and generates a pedagogical response.
    /// Uses the Conductor model (Mistral Small 4 119B MoE) via vLLM or llama-server.
    pub async fn evaluate_and_respond(
        &self,
        user_input: &str,
        vaam_tier: &str,
    ) -> anyhow::Result<String> {
        let system_prompt = format!(
            "You are Purdue Pete, the Conductor of Trinity ID AI OS.\n\
             You guide users through the ADDIECRAPEYE instructional design process.\n\
             The user is at VAAM Tier: {}. Adjust vocabulary complexity accordingly.\n\
             Use Socratic questioning. Focus on meaning-making, not just answers.",
            vaam_tier
        );

        let payload = json!({
            "model": "default",
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_input}
            ],
            "max_tokens": 1024,
            "temperature": 0.6
        });

        let url = format!(
            "{}/v1/chat/completions",
            self.vllm_url.trim_end_matches('/')
        );

        let response = self.client.post(&url).json(&payload).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("API request failed with status: {}", response.status());
        }

        let json_data: serde_json::Value = response.json().await?;

        if let Some(content) = json_data["choices"][0]["message"]["content"].as_str() {
            Ok(content.to_string())
        } else {
            anyhow::bail!("Failed to extract content from API response")
        }
    }
}
