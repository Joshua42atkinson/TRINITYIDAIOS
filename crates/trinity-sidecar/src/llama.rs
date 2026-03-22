//! llama-server process management and HTTP client
//!
//! Manages two llama-server instances:
//! - Shield (Opus): Dense 27B thinker for planning and review
//! - Sword (REAP): MoE 25B-A3B fast coder for code generation

use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::{Child, Command};
use tracing::info;

/// Configuration for a single llama-server instance
#[derive(Debug, Clone)]
pub struct LlamaConfig {
    pub name: String,
    pub model_path: PathBuf,
    pub port: u16,
    pub context_size: u32,
    pub n_gpu_layers: i32,
    pub threads: u32,
}

/// A running llama-server process
pub struct LlamaProcess {
    pub config: LlamaConfig,
    pub process: Child,
}

impl LlamaProcess {
    /// Start a llama-server with the given config
    pub async fn start(config: LlamaConfig, llama_server_bin: &PathBuf) -> anyhow::Result<Self> {
        if !config.model_path.exists() {
            anyhow::bail!("Model not found: {}", config.model_path.display());
        }
        if !llama_server_bin.exists() {
            anyhow::bail!(
                "llama-server binary not found: {}",
                llama_server_bin.display()
            );
        }

        info!(
            "Starting {} on port {} (ctx={}, ngl={})",
            config.name, config.port, config.context_size, config.n_gpu_layers
        );

        let process = Command::new(llama_server_bin)
            .args([
                "-m",
                &config.model_path.to_string_lossy(),
                "-c",
                &config.context_size.to_string(),
                "--port",
                &config.port.to_string(),
                "--host",
                "127.0.0.1",
                "-ngl",
                &config.n_gpu_layers.to_string(),
                "-t",
                &config.threads.to_string(),
                // AMD Strix Halo optimizations
                "-fa", // Flash Attention for Sliding Windows (Step-3.5-Flash)
                "-ctk",
                "q4_0", // 4-bit KV Cache (Keys)
                "-ctv",
                "q4_0", // 4-bit KV Cache (Values)
                "--mmap",
                "1",           // Force memory mapping for Unified Memory
                "--ctx-shift", // Context Shifting to prevent OOM
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()?;

        info!(
            "Spawned {} (PID {})",
            config.name,
            process.id().unwrap_or(0)
        );

        Ok(Self { config, process })
    }

    /// Kill the process
    pub async fn stop(&mut self) -> anyhow::Result<()> {
        info!("Stopping {} on port {}", self.config.name, self.config.port);
        self.process.kill().await?;
        Ok(())
    }

    #[allow(dead_code)] // Used to compose health/metrics URLs when process is managed externally
    pub fn base_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.config.port)
    }
}

/// HTTP client for talking to llama-server OpenAI-compatible API
pub struct LlamaClient {
    http: reqwest::Client,
    base_url: String,
}

impl LlamaClient {
    pub fn new(base_url: &str) -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to build HTTP client");
        Self {
            http,
            base_url: base_url.to_string(),
        }
    }

    /// Health check
    pub async fn is_healthy(&self) -> bool {
        match self
            .http
            .get(format!("{}/health", self.base_url))
            .timeout(std::time::Duration::from_secs(3))
            .send()
            .await
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Wait for the server to become healthy (up to timeout_secs)
    pub async fn wait_ready(&self, timeout_secs: u64) -> bool {
        for _ in 0..timeout_secs {
            if self.is_healthy().await {
                return true;
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        false
    }

    /// Send a chat completion request and return the full response
    pub async fn chat(
        &self,
        messages: &[ChatMessage],
        max_tokens: u32,
        temperature: f32,
    ) -> anyhow::Result<String> {
        let request = serde_json::json!({
            "messages": messages,
            "max_tokens": max_tokens,
            "temperature": temperature,
            "stream": false,
        });

        let response = self
            .http
            .post(format!("{}/v1/chat/completions", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "llama-server {} returned {}: {}",
                self.base_url,
                status,
                body
            );
        }

        let json: serde_json::Value = response.json().await?;

        // Extract content — handle both standard "content" and "reasoning_content" fields
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .or_else(|| json["choices"][0]["message"]["reasoning_content"].as_str())
            .unwrap_or("")
            .to_string();

        Ok(content)
    }
}

/// Chat message for the OpenAI-compatible API
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: &str) -> Self {
        Self {
            role: "system".to_string(),
            content: content.to_string(),
        }
    }
    pub fn user(content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: content.to_string(),
        }
    }
    #[allow(dead_code)] // Reserved for multi-turn conversations (few-shot, chain-of-thought)
    pub fn assistant(content: &str) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.to_string(),
        }
    }
}
