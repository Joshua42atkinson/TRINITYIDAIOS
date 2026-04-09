// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        pete_engine.rs
// PURPOSE:     Pete Execution Proxy — routes coding tasks to live inference
//
// CONTEXT:     The Yardmaster coding subagent needs a local LLM for execution.
//              Instead of a hardcoded "Execution Success" stub, this module
//              proxies code execution tasks to the active inference backend
//              (LongCat-Next on :8010 or Yardmaster REAP on :8009).
//
// MATURITY:    L3 — Functional proxy with fallback (was L1 stub)
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct PeteConfig {
    /// Whether to use GPU acceleration (Vulkan/ROCm)
    pub use_gpu_acceleration: bool,
    /// Model identifier for the coding subagent
    pub model_id: String,
    /// API URL for the inference backend
    pub api_url: String,
    /// Max tokens for code generation responses
    pub max_tokens: u32,
}

impl Default for PeteConfig {
    fn default() -> Self {
        Self {
            use_gpu_acceleration: true,
            model_id: "LongCat-Next".into(),
            api_url: std::env::var("PETE_ENGINE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:8010".into()),
            max_tokens: 4096,
        }
    }
}

pub struct PeteEngine {
    config: PeteConfig,
}

impl PeteEngine {
    pub fn new(config: PeteConfig) -> Self {
        info!(
            "👷 Pete Execution Engine initialized (proxy → {})",
            config.api_url
        );
        Self { config }
    }

    /// Execute a coding/analysis task via the inference backend.
    /// Routes to LongCat-Next (:8010) or Yardmaster REAP (:8009) for real inference.
    /// Falls back to an honest error message if no backend is available.
    pub async fn execute_task(&self, instruction: &str) -> Result<String, String> {
        let client = &*crate::http::LONG;

        let payload = serde_json::json!({
            "model": self.config.model_id,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a code execution assistant. Analyze the task and provide a precise, actionable response. Output clean code when requested. Be concise."
                },
                {
                    "role": "user",
                    "content": instruction
                }
            ],
            "max_tokens": self.config.max_tokens,
            "temperature": 0.3,
            "stream": false
        });

        let url = format!("{}/v1/chat/completions", self.config.api_url);

        match client.post(&url).json(&payload).send().await {
            Ok(response) if response.status().is_success() => {
                let json: serde_json::Value = response
                    .json()
                    .await
                    .map_err(|e| format!("Failed to parse response: {}", e))?;

                json["choices"][0]["message"]["content"]
                    .as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| "No content in response".to_string())
            }
            Ok(response) => {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                warn!("Pete Engine API returned {}: {}", status, text);
                Err(format!(
                    "Pete Engine offline ({}): {}. Start LongCat via: distrobox enter sglang-engine -- bash ./longcat_omni_sidecar/launch_engine.sh",
                    status, text
                ))
            }
            Err(e) => {
                warn!("Pete Engine unreachable: {}", e);
                Err(format!(
                    "Pete Engine unreachable: {}. No inference backend found at {}",
                    e, self.config.api_url
                ))
            }
        }
    }

    /// Check if the Pete Engine backend is reachable
    pub async fn is_healthy(&self) -> bool {
        let url = format!("{}/health", self.config.api_url);
        crate::http::check_health(&url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PeteConfig::default();
        assert!(config.use_gpu_acceleration);
        assert!(config.api_url.contains("8010"));
        assert_eq!(config.max_tokens, 4096);
    }

    #[test]
    fn test_pete_engine_creates() {
        let engine = PeteEngine::new(PeteConfig::default());
        assert_eq!(engine.config.model_id, "LongCat-Next");
    }
}
