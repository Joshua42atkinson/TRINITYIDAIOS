// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        npu_ort_engine.rs
// PURPOSE:     NPU Coprocessor Proxy — routes embeddings to available backend
//
// CONTEXT:     The Ryzen AI NPU isn't bound via ONNX Runtime yet.
//              Instead of returning hardcoded zeros, this module proxies
//              embedding requests to the A.R.T.Y. Hub (nomic-embed on :8005)
//              or falls back to a simple hash-based embedding for offline use.
//
// MATURITY:    L3 — Functional proxy with honest fallback (was L1 stub)
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrtConfig {
    /// Whether to attempt NPU acceleration (future — ONNX VitisAI)
    pub use_npu: bool,
    /// Path to embedding model ONNX file (future use)
    pub embedding_model_path: String,
    /// URL of the embedding API (nomic-embed via A.R.T.Y. Hub)
    pub embedding_api_url: String,
}

impl Default for OrtConfig {
    fn default() -> Self {
        Self {
            use_npu: false, // NPU not yet bound — honest about this
            embedding_model_path: "models/nomic-embed-text-v1.5.onnx".into(),
            embedding_api_url: "http://127.0.0.1:8005".into(),
        }
    }
}

pub struct NpuEngine {
    config: OrtConfig,
}

impl NpuEngine {
    pub fn new(config: OrtConfig) -> Self {
        info!("🧠 NPU Engine initialized (proxy mode → nomic-embed API)");
        if config.use_npu {
            warn!("⚠️ NPU VitisAI execution not yet bound — using API proxy");
        }
        Self { config }
    }

    /// Embed text via the A.R.T.Y. Hub nomic-embed endpoint.
    /// Falls back to a deterministic hash-based embedding if the API is unreachable.
    pub async fn embed_text(&self, text: &str) -> Result<Vec<f32>, String> {
        let client = &*crate::http::QUICK;

        // Try nomic-embed via A.R.T.Y. Hub first
        let payload = serde_json::json!({
            "input": text,
            "model": "nomic-embed-text-v1.5"
        });

        match client
            .post(&format!("{}/v1/embeddings", self.config.embedding_api_url))
            .json(&payload)
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                let json: serde_json::Value = response
                    .json()
                    .await
                    .map_err(|e| format!("Failed to parse embedding response: {}", e))?;

                if let Some(embedding) = json["data"][0]["embedding"].as_array() {
                    let vec: Vec<f32> = embedding
                        .iter()
                        .filter_map(|v| v.as_f64().map(|f| f as f32))
                        .collect();
                    if !vec.is_empty() {
                        return Ok(vec);
                    }
                }
                Err("Invalid embedding format in response".to_string())
            }
            Ok(response) => {
                warn!(
                    "Embedding API returned {}: falling back to hash embedding",
                    response.status()
                );
                Ok(hash_embedding(text))
            }
            Err(e) => {
                warn!(
                    "Embedding API unreachable ({}): falling back to hash embedding",
                    e
                );
                Ok(hash_embedding(text))
            }
        }
    }
}

/// Deterministic hash-based embedding fallback.
/// Produces a 768-dimensional vector from the text content.
/// Not semantically meaningful, but provides consistent results for
/// deduplication and basic similarity when the embedding API is offline.
fn hash_embedding(text: &str) -> Vec<f32> {
    use std::hash::{Hash, Hasher};
    let mut embedding = vec![0.0f32; 768];

    for (i, chunk) in text.as_bytes().chunks(4).enumerate() {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        chunk.hash(&mut hasher);
        i.hash(&mut hasher);
        let hash = hasher.finish();

        let idx = i % 768;
        // Map hash to [-1.0, 1.0] range
        embedding[idx] = ((hash as f64 / u64::MAX as f64) * 2.0 - 1.0) as f32;
    }

    // L2 normalize
    let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in &mut embedding {
            *v /= norm;
        }
    }

    embedding
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_embedding_deterministic() {
        let a = hash_embedding("hello world");
        let b = hash_embedding("hello world");
        assert_eq!(a, b);
    }

    #[test]
    fn test_hash_embedding_different_inputs() {
        let a = hash_embedding("hello");
        let b = hash_embedding("world");
        assert_ne!(a, b);
    }

    #[test]
    fn test_hash_embedding_normalized() {
        let emb = hash_embedding("test input for normalization");
        let norm: f32 = emb.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01, "Embedding should be L2-normalized, got norm={}", norm);
    }

    #[test]
    fn test_hash_embedding_correct_dimension() {
        let emb = hash_embedding("any text");
        assert_eq!(emb.len(), 768);
    }
}
