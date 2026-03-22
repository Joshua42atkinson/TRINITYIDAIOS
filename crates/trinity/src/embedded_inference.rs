// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        embedded_inference.rs
// PURPOSE:     Direct GGUF inference via llama.cpp (Vulkan GPU) — no HTTP needed
//
// ARCHITECTURE:
//   • Loads a GGUF model directly into GPU memory via llama-cpp-2 crate
//   • Vulkan backend for portable GPU acceleration (AMD, NVIDIA, Intel)
//   • Replaces the HTTP hop to llama-server / vLLM / LM Studio
//   • Thread-safe: model is Arc-wrapped, context created per-request
//
// BEFORE: TRINITY (Rust) ──HTTP──▶ llama-server (C++) ──ROCm──▶ GPU
// AFTER:  TRINITY (Rust) ──direct──▶ llama.cpp (C++ FFI) ──Vulkan──▶ GPU
//
// ═══════════════════════════════════════════════════════════════════════════════

use std::num::NonZeroU32;
use std::pin::pin;
use std::sync::Arc;

use anyhow::{Context, Result};
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::LogOptions;
use tracing::info;

use crate::ChatMessage;

/// Embedded LLM model loaded directly into GPU memory via llama.cpp + Vulkan
pub struct EmbeddedModel {
    backend: LlamaBackend,
    model: LlamaModel,
    model_path: String,
}

// SAFETY: LlamaModel and LlamaBackend are thread-safe C++ objects behind Arc
unsafe impl Send for EmbeddedModel {}
unsafe impl Sync for EmbeddedModel {}

impl EmbeddedModel {
    /// Load a GGUF model from disk with all layers offloaded to Vulkan GPU
    pub fn load(path: &str) -> Result<Self> {
        info!("🧠 Initializing llama.cpp backend...");

        // ── STRIX HALO FIX: RADV_PERFMODE=nogttspill ──
        // AMD Strix Halo APU + RADV Vulkan driver caps each heap budget at ~40GB
        // even though heap size is 83GB. This causes >64GB model loads to hang.
        // RADV_PERFMODE=nogttspill disables GTT spilling, allowing full heap usage.
        // (From archived trinity-sidecar-llama-cpp)
        std::env::set_var("RADV_PERFMODE", "nogttspill");
        info!("   RADV_PERFMODE=nogttspill (Strix Halo >64GB fix)");

        let backend = LlamaBackend::init().context("Failed to initialize llama.cpp backend")?;

        // Log available devices
        let devices = llama_cpp_2::list_llama_ggml_backend_devices();
        for dev in &devices {
            info!(
                "   GPU device: {} ({:?}) — {}MB free",
                dev.name,
                dev.device_type,
                dev.memory_free / 1024 / 1024
            );
        }

        // Enable llama.cpp logs for diagnostics
        llama_cpp_2::send_logs_to_tracing(LogOptions::default().with_logs_enabled(true));

        // Offload all layers to GPU (Vulkan)
        // CRITICAL: disable mmap for AMD Strix Halo unified memory (>64GB models hang with mmap)
        let model_params = LlamaModelParams::default()
            .with_n_gpu_layers(999)
            .with_use_mmap(false);
        let model_params = pin!(model_params);

        info!("🧠 Loading GGUF: {}...", path);
        let model = LlamaModel::load_from_file(&backend, path, &model_params)
            .map_err(|e| anyhow::anyhow!("Failed to load GGUF model: {:?}", e))?;

        info!("✅ Model loaded — {} tokens in vocab", model.n_vocab());

        Ok(Self {
            backend,
            model,
            model_path: path.to_string(),
        })
    }

    /// Format chat messages into a prompt string using the model's built-in chat template
    fn format_prompt(&self, messages: &[ChatMessage]) -> String {
        // Build a simple ChatML-style prompt
        let mut prompt = String::new();
        for msg in messages {
            match msg.role.as_str() {
                "system" => {
                    prompt.push_str(&format!("<|im_start|>system\n{}<|im_end|>\n", msg.content));
                }
                "user" => {
                    prompt.push_str(&format!("<|im_start|>user\n{}<|im_end|>\n", msg.content));
                }
                "assistant" => {
                    prompt.push_str(&format!(
                        "<|im_start|>assistant\n{}<|im_end|>\n",
                        msg.content
                    ));
                }
                _ => {
                    prompt.push_str(&format!(
                        "<|im_start|>{}\n{}<|im_end|>\n",
                        msg.role, msg.content
                    ));
                }
            }
        }
        prompt.push_str("<|im_start|>assistant\n");
        prompt
    }

    /// Run chat completion — returns the full response text
    pub async fn chat_completion(
        self: &Arc<Self>,
        messages: &[ChatMessage],
        max_tokens: u32,
    ) -> Result<String> {
        let model = self.clone();
        let messages = messages.to_vec();

        // Run inference on a blocking thread (llama.cpp is synchronous C++)
        tokio::task::spawn_blocking(move || model.generate_sync(&messages, max_tokens, None))
            .await?
    }

    /// Run streaming chat completion — sends tokens through the channel as they're generated
    pub async fn chat_completion_stream(
        self: &Arc<Self>,
        messages: &[ChatMessage],
        max_tokens: u32,
        tx: tokio::sync::mpsc::Sender<String>,
    ) -> Result<()> {
        let model = self.clone();
        let messages = messages.to_vec();

        tokio::task::spawn_blocking(move || {
            model
                .generate_sync(&messages, max_tokens, Some(tx))
                .map(|_| ())
        })
        .await?
    }

    /// Synchronous generation — called from spawn_blocking
    fn generate_sync(
        &self,
        messages: &[ChatMessage],
        max_tokens: u32,
        tx: Option<tokio::sync::mpsc::Sender<String>>,
    ) -> Result<String> {
        let prompt = self.format_prompt(messages);

        // Create a fresh context for this request
        let ctx_params = LlamaContextParams::default().with_n_ctx(NonZeroU32::new(262144));

        let mut ctx = self
            .model
            .new_context(&self.backend, ctx_params)
            .map_err(|e| anyhow::anyhow!("Failed to create context: {:?}", e))?;

        // Tokenize the prompt
        let tokens = self
            .model
            .str_to_token(&prompt, AddBos::Always)
            .map_err(|e| anyhow::anyhow!("Tokenization failed: {:?}", e))?;

        let n_prompt = tokens.len() as i32;
        let n_len = n_prompt + max_tokens as i32;

        // Create batch and fill with prompt tokens
        let mut batch = LlamaBatch::new(512, 1);
        let last_idx = tokens.len() - 1;
        for (i, token) in tokens.into_iter().enumerate() {
            batch
                .add(token, i as i32, &[0], i == last_idx)
                .map_err(|e| anyhow::anyhow!("Batch add failed: {:?}", e))?;
        }

        // Decode the prompt
        ctx.decode(&mut batch)
            .map_err(|e| anyhow::anyhow!("Prompt decode failed: {:?}", e))?;

        // Sample tokens
        let mut n_cur = batch.n_tokens();
        let mut output = String::new();
        let mut decoder = encoding_rs::UTF_8.new_decoder();

        let mut sampler =
            LlamaSampler::chain_simple([LlamaSampler::dist(1234), LlamaSampler::greedy()]);

        while n_cur < n_len {
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            sampler.accept(token);

            // Check for end of generation
            if self.model.is_eog_token(token) {
                break;
            }

            // Decode token to text
            let piece = self
                .model
                .token_to_piece(token, &mut decoder, true, None)
                .map_err(|e| anyhow::anyhow!("Token decode failed: {:?}", e))?;

            // Stop on ChatML end markers
            if piece.contains("<|im_end|>") || piece.contains("<|im_start|>") {
                break;
            }

            output.push_str(&piece);

            // Stream token if channel provided
            if let Some(ref tx) = tx {
                if tx.blocking_send(piece).is_err() {
                    break; // Client disconnected
                }
            }

            batch.clear();
            batch
                .add(token, n_cur, &[0], true)
                .map_err(|e| anyhow::anyhow!("Batch add failed: {:?}", e))?;

            ctx.decode(&mut batch)
                .map_err(|e| anyhow::anyhow!("Decode failed: {:?}", e))?;

            n_cur += 1;
        }

        Ok(output)
    }

    /// Get the model file path (for status/logging)
    pub fn model_path(&self) -> &str {
        &self.model_path
    }
}
