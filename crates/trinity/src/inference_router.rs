// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        inference_router.rs
// PURPOSE:     Multi-backend inference router — auto-detect, health-check,
//              and manage multiple LLM inference servers
//
// ARCHITECTURE:
//   • Primary: vLLM Omni sidecar (ROCm GPU) — unified LLM + vision + audio + images
//   • Fallback: llama-server, LM Studio, Ollama, or any OpenAI-compatible server
//   • Auto-detection probes known ports on startup
//   • Health monitoring with automatic failover
//   • Config-file driven (configs/runtime/default.toml) + env var override
//   • All callers use router.active_url() instead of raw strings
//
// CHANGES:
//   2026-04-04  vLLM Omni  vLLM Omni as primary — unified backbone for all modalities
//   2026-03-28  Embedded   Removed vLLM/SGLang — embedded llama-cpp-2 is primary
//   2026-03-22  Phase 3    Initial implementation — multi-backend inference router
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use tracing::{info, warn};

// ═══════════════════════════════════════════════════
// Backend Types
// ═══════════════════════════════════════════════════

/// Known inference backend types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BackendKind {
    VllmOmni,
    LlamaServer,
    Ollama,
    LmStudio,
    LongCat,
    Custom,
}

impl BackendKind {
    /// Default port for each backend kind
    pub fn default_port(&self) -> u16 {
        match self {
            BackendKind::VllmOmni => 8000,
            BackendKind::LlamaServer => 8080,
            BackendKind::Ollama => 11434,
            BackendKind::LmStudio => 1234,
            BackendKind::LongCat => 8010,
            BackendKind::Custom => 8080,
        }
    }

    /// Human-readable display name
    pub fn display_name(&self) -> &str {
        match self {
            BackendKind::VllmOmni => "vLLM Omni",
            BackendKind::LlamaServer => "llama-server",
            BackendKind::Ollama => "Ollama",
            BackendKind::LmStudio => "LM Studio",
            BackendKind::LongCat => "LongCat-Next",
            BackendKind::Custom => "Custom",
        }
    }

    /// Health check endpoint path for each backend
    pub fn health_path(&self) -> &str {
        match self {
            BackendKind::VllmOmni => "/health",    // vLLM uses standard /health
            BackendKind::Ollama => "/api/tags",     // Ollama uses /api/tags as health-ish
            BackendKind::LmStudio => "/v1/models",  // LM Studio uses /v1/models
            _ => "/health",                         // OpenAI-compatible servers use /health
        }
    }

    /// Chat completions endpoint path
    pub fn completions_path(&self) -> &str {
        match self {
            BackendKind::Ollama => "/v1/chat/completions", // Ollama has OpenAI compat layer
            _ => "/v1/chat/completions",
        }
    }
}

/// A single inference backend with its connection details and capabilities
#[derive(Debug, Clone, Serialize)]
pub struct InferenceBackend {
    /// Unique name for this backend (e.g., "llama-server", "vllm", "ollama")
    pub name: String,
    /// Backend type
    pub kind: BackendKind,
    /// Base URL (e.g., "http://127.0.0.1:8080")
    pub base_url: String,
    /// Whether this backend supports OpenAI-compatible tool calling
    pub supports_tools: bool,
    /// Whether this backend supports vision/multimodal inputs
    pub supports_vision: bool,
    /// Model name hint (populated from /v1/models if available)
    pub model_name: Option<String>,
    /// Whether this backend is currently reachable
    pub healthy: bool,
    /// Last health check timestamp (Unix seconds)
    pub last_checked: u64,
}

// ═══════════════════════════════════════════════════
// Config File Parsing
// ═══════════════════════════════════════════════════

/// TOML config structure for [inference] section
#[derive(Debug, Deserialize)]
pub struct InferenceConfig {
    /// Name of the preferred primary backend
    #[serde(default = "default_primary")]
    pub primary: String,
    /// Whether to auto-detect backends on startup
    #[serde(default = "default_true")]
    pub auto_detect: bool,
    /// Context window size
    #[serde(default = "default_ctx_size")]
    pub ctx_size: u32,
    /// Max response tokens
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    /// Backend definitions
    #[serde(default)]
    pub backends: std::collections::HashMap<String, BackendConfig>,
}

#[derive(Debug, Deserialize)]
pub struct BackendConfig {
    pub url: String,
    #[serde(default)]
    pub supports_tools: bool,
    #[serde(default)]
    pub supports_vision: bool,
}

fn default_primary() -> String {
    "vllm-omni".to_string()
}
fn default_true() -> bool {
    true
}
fn default_ctx_size() -> u32 {
    262144
}
fn default_max_tokens() -> u32 {
    16384
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            primary: default_primary(),
            auto_detect: true,
            ctx_size: default_ctx_size(),
            max_tokens: default_max_tokens(),
            backends: std::collections::HashMap::new(),
        }
    }
}

// ═══════════════════════════════════════════════════
// Top-level TOML wrapper (for parsing the full file)
// ═══════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
struct TomlRoot {
    #[serde(default)]
    inference: Option<InferenceConfig>,
}

// ═══════════════════════════════════════════════════
// Inference Router
// ═══════════════════════════════════════════════════

/// The multi-backend inference router. Manages discovery, health-checking,
/// and selection of the active LLM inference backend.
#[derive(Debug)]
pub struct InferenceRouter {
    /// All known backends (from config + auto-detect)
    pub backends: Vec<InferenceBackend>,
    /// Index of the currently active backend
    active: usize,
    /// Configuration
    pub config: InferenceConfig,
}

/// Serializable status snapshot for the API
#[derive(Debug, Serialize)]
pub struct RouterStatus {
    pub active_backend: String,
    pub active_url: String,
    pub backends: Vec<InferenceBackend>,
    pub ctx_size: u32,
    pub max_tokens: u32,
}

impl InferenceRouter {
    // ── Construction ──

    /// Create a new router from the config file + env var overrides.
    /// Does NOT probe backends yet — call `auto_detect()` after construction.
    pub fn from_config(config_path: Option<&str>) -> Self {
        let config = Self::load_config(config_path);
        let mut backends = Self::build_backend_list(&config);

        // ENV override: LLM_URL takes highest priority
        if let Ok(env_url) = std::env::var("LLM_URL") {
            // Check if any existing backend already uses this URL
            let already_exists = backends.iter().any(|b| b.base_url == env_url);
            if !already_exists {
                info!("🔧 Adding env-override backend: {}", env_url);
                backends.insert(
                    0,
                    InferenceBackend {
                        name: "env-override".to_string(),
                        kind: BackendKind::Custom,
                        base_url: env_url,
                        supports_tools: true,
                        supports_vision: true,
                        model_name: None,
                        healthy: false,
                        last_checked: 0,
                    },
                );
            }
        }

        // If no backends at all, add defaults
        if backends.is_empty() {
            backends = Self::default_backends();
        }

        // Set active to the configured primary, or first backend
        let active = backends
            .iter()
            .position(|b| b.name == config.primary)
            .unwrap_or(0);

        Self {
            backends,
            active,
            config,
        }
    }

    /// Load InferenceConfig from TOML file, with graceful fallback
    fn load_config(config_path: Option<&str>) -> InferenceConfig {
        let default_path = "configs/runtime/default.toml";
        let path = config_path.unwrap_or(default_path);

        match std::fs::read_to_string(path) {
            Ok(content) => match toml::from_str::<TomlRoot>(&content) {
                Ok(root) => {
                    if let Some(inference) = root.inference {
                        info!("📋 Loaded inference config from {}", path);
                        inference
                    } else {
                        info!(
                            "📋 Config file {} has no [inference] section, using defaults",
                            path
                        );
                        InferenceConfig::default()
                    }
                }
                Err(e) => {
                    warn!("⚠️ Failed to parse {}: {}. Using defaults.", path, e);
                    InferenceConfig::default()
                }
            },
            Err(_) => {
                info!("📋 No config file at {}, using defaults", path);
                InferenceConfig::default()
            }
        }
    }

    /// Build backend list from config file entries
    fn build_backend_list(config: &InferenceConfig) -> Vec<InferenceBackend> {
        config
            .backends
            .iter()
            .map(|(name, bc)| {
                let kind = match name.as_str() {
                    "vllm-omni" | "vllm" => BackendKind::VllmOmni,
                    "llama-server" => BackendKind::LlamaServer,
                    "ollama" => BackendKind::Ollama,
                    "lm-studio" => BackendKind::LmStudio,
                    _ => BackendKind::Custom,
                };
                InferenceBackend {
                    name: name.clone(),
                    kind,
                    base_url: bc.url.clone(),
                    supports_tools: bc.supports_tools,
                    supports_vision: bc.supports_vision,
                    model_name: None,
                    healthy: false,
                    last_checked: 0,
                }
            })
            .collect()
    }

    /// Default backend list when no config is found
    fn default_backends() -> Vec<InferenceBackend> {
        vec![
            // ── vLLM Omni P.A.R.T.Y. Hotel (individual model ports) ──
            // Great Recycler — Gemma-4 31B Dense AWQ (Socratic reasoning)
            InferenceBackend {
                name: "vllm-recycler".to_string(),
                kind: BackendKind::VllmOmni,
                base_url: "http://127.0.0.1:8001".to_string(),
                supports_tools: true,
                supports_vision: true,
                model_name: None,
                healthy: false,
                last_checked: 0,
            },
            // Programmer Pete — Gemma-4 26B MoE AWQ (action-oriented)
            InferenceBackend {
                name: "vllm-pete".to_string(),
                kind: BackendKind::VllmOmni,
                base_url: "http://127.0.0.1:8002".to_string(),
                supports_tools: true,
                supports_vision: true,
                model_name: None,
                healthy: false,
                last_checked: 0,
            },
            // Tempo Engine — Gemma-4 E4B AWQ (ASR + lightweight tasks)
            InferenceBackend {
                name: "vllm-tempo".to_string(),
                kind: BackendKind::VllmOmni,
                base_url: "http://127.0.0.1:8003".to_string(),
                supports_tools: false,
                supports_vision: true,
                model_name: None,
                healthy: false,
                last_checked: 0,
            },
            // vLLM Omni proxy (if running a unified proxy on 8000)
            InferenceBackend {
                name: "vllm-omni".to_string(),
                kind: BackendKind::VllmOmni,
                base_url: "http://127.0.0.1:8000".to_string(),
                supports_tools: true,
                supports_vision: true,
                model_name: None,
                healthy: false,
                last_checked: 0,
            },
            InferenceBackend {
                name: "llama-server".to_string(),
                kind: BackendKind::LlamaServer,
                base_url: "http://127.0.0.1:8080".to_string(),
                supports_tools: true,
                supports_vision: true,
                model_name: None,
                healthy: false,
                last_checked: 0,
            },
            InferenceBackend {
                name: "lm-studio".to_string(),
                kind: BackendKind::LmStudio,
                base_url: "http://127.0.0.1:1234".to_string(),
                supports_tools: true,
                supports_vision: true,
                model_name: None,
                healthy: false,
                last_checked: 0,
            },
            InferenceBackend {
                name: "ollama".to_string(),
                kind: BackendKind::Ollama,
                base_url: "http://127.0.0.1:11434".to_string(),
                supports_tools: true,
                supports_vision: false,
                model_name: None,
                healthy: false,
                last_checked: 0,
            },
        ]
    }


    // ── Auto-Detection ──

    /// Probe all backends for health. Updates `healthy` flags and selects
    /// the first healthy backend if the current one is unhealthy.
    pub async fn auto_detect(&mut self) {
        info!("🔍 Probing {} inference backends...", self.backends.len());

        let client = &*crate::http::QUICK;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        for backend in self.backends.iter_mut() {
            let health_url = format!("{}{}", backend.base_url, backend.kind.health_path());
            let was_healthy = backend.healthy;

            backend.healthy = match client.get(&health_url).send().await {
                Ok(resp) => resp.status().is_success(),
                Err(_) => false,
            };
            backend.last_checked = now;

            if backend.healthy {
                info!(
                    "  ✅ {} at {} — healthy",
                    backend.kind.display_name(),
                    backend.base_url
                );

                // Try to get model name from /v1/models
                if backend.model_name.is_none() {
                    if let Ok(resp) = client
                        .get(format!("{}/v1/models", backend.base_url))
                        .send()
                        .await
                    {
                        if let Ok(body) = resp.json::<serde_json::Value>().await {
                            if let Some(models) = body.get("data").and_then(|d| d.as_array()) {
                                if let Some(first) = models.first() {
                                    if let Some(id) = first.get("id").and_then(|v| v.as_str()) {
                                        backend.model_name = Some(id.to_string());
                                        info!("     Model: {}", id);
                                    }
                                }
                            }
                        }
                    }
                }
            } else if was_healthy {
                warn!(
                    "  ❌ {} at {} — went unhealthy",
                    backend.kind.display_name(),
                    backend.base_url
                );
            } else {
                info!(
                    "  ⬚  {} at {} — not responding",
                    backend.kind.display_name(),
                    backend.base_url
                );
            }
        }

        // If current active backend is unhealthy, failover to first healthy one
        if !self.backends.is_empty() && !self.backends[self.active].healthy {
            if let Some(healthy_idx) = self.backends.iter().position(|b| b.healthy) {
                let old_name = self.backends[self.active].name.clone();
                self.active = healthy_idx;
                let new_name = &self.backends[self.active].name;
                info!("🔄 Failover: {} → {} (auto-detected)", old_name, new_name);
            } else {
                warn!("⚠️ No healthy inference backends found. Trinity will keep checking.");
            }
        }

        let healthy_count = self.backends.iter().filter(|b| b.healthy).count();
        info!(
            "🔍 Inference probe complete: {}/{} backends healthy",
            healthy_count,
            self.backends.len()
        );
    }

    // ── Accessors ──

    /// Get the active backend's base URL. This is the primary method all
    /// callers use instead of the old `llm_url`.
    pub fn active_url(&self) -> &str {
        if self.backends.is_empty() {
            "http://127.0.0.1:8080" // absolute fallback
        } else {
            &self.backends[self.active].base_url
        }
    }

    /// Get the active backend's name
    pub fn active_name(&self) -> &str {
        if self.backends.is_empty() {
            "none"
        } else {
            &self.backends[self.active].name
        }
    }

    /// Get the active backend reference
    pub fn active_backend(&self) -> Option<&InferenceBackend> {
        self.backends.get(self.active)
    }

    /// Check if the active backend supports structured tool calling
    pub fn supports_tools(&self) -> bool {
        self.backends
            .get(self.active)
            .map(|b| b.supports_tools)
            .unwrap_or(false)
    }

    /// Check if the active backend supports vision/multimodal
    pub fn supports_vision(&self) -> bool {
        self.backends
            .get(self.active)
            .map(|b| b.supports_vision)
            .unwrap_or(false)
    }

    /// Whether any backend is currently healthy
    pub fn any_healthy(&self) -> bool {
        self.backends.iter().any(|b| b.healthy)
    }

    /// Whether the active backend is healthy
    pub fn is_healthy(&self) -> bool {
        self.backends
            .get(self.active)
            .map(|b| b.healthy)
            .unwrap_or(false)
    }

    /// Get a backend's base URL by its exact name
    pub fn get_url_by_name(&self, name: &str) -> Option<String> {
        self.backends
            .iter()
            .find(|b| b.name == name)
            .map(|b| b.base_url.clone())
    }

    /// Get a backend's base URL by its kind
    pub fn get_url_by_kind(&self, kind: BackendKind) -> Option<String> {
        self.backends
            .iter()
            .find(|b| b.kind == kind)
            .map(|b| b.base_url.clone())
    }

    // ── Manual Control ──

    /// Switch the active backend by name. Returns true if found.
    pub fn switch_backend(&mut self, name: &str) -> bool {
        if let Some(idx) = self.backends.iter().position(|b| b.name == name) {
            let old = &self.backends[self.active].name;
            info!("🔄 Backend switch: {} → {} (manual)", old, name);
            self.active = idx;
            true
        } else {
            warn!(
                "⚠️ Backend '{}' not found. Available: {:?}",
                name,
                self.backends.iter().map(|b| &b.name).collect::<Vec<_>>()
            );
            false
        }
    }

    /// Manually set the active backend URL (for backward compatibility with
    /// the old model-switch endpoint)
    pub fn set_active_url(&mut self, url: String) {
        if let Some(backend) = self.backends.iter().position(|b| b.base_url == url) {
            self.active = backend;
        } else {
            // Add as a new custom backend and make it active
            self.backends.push(InferenceBackend {
                name: "manual".to_string(),
                kind: BackendKind::Custom,
                base_url: url,
                supports_tools: true,
                supports_vision: true,
                model_name: None,
                healthy: true, // assume healthy if user explicitly set it
                last_checked: 0,
            });
            self.active = self.backends.len() - 1;
        }
    }

    /// Get a serializable status snapshot for the API
    pub fn status(&self) -> RouterStatus {
        RouterStatus {
            active_backend: self.active_name().to_string(),
            active_url: self.active_url().to_string(),
            backends: self.backends.clone(),
            ctx_size: self.config.ctx_size,
            max_tokens: self.config.max_tokens,
        }
    }
}

// ═══════════════════════════════════════════════════
// Unit Tests
// ═══════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_backends_created_when_no_config() {
        let router = InferenceRouter::from_config(Some("/nonexistent/path.toml"));
        assert!(!router.backends.is_empty(), "Should have default backends");
        // Default primary is "vllm-omni", so active should select it
        assert_eq!(router.active_name(), "vllm-omni");
        assert_eq!(router.active_url(), "http://127.0.0.1:8000");
    }

    #[test]
    fn test_parse_toml_config() {
        let toml_content = r#"
[inference]
primary = "ollama"
auto_detect = true
ctx_size = 131072
max_tokens = 8192

[inference.backends.llama-server]
url = "http://127.0.0.1:8080"
supports_tools = true
supports_vision = true

[inference.backends.ollama]
url = "http://127.0.0.1:11434"
supports_tools = true
supports_vision = false
"#;
        let root: TomlRoot = toml::from_str(toml_content).unwrap();
        let config = root.inference.unwrap();

        assert_eq!(config.primary, "ollama");
        assert_eq!(config.ctx_size, 131072);
        assert_eq!(config.max_tokens, 8192);
        assert_eq!(config.backends.len(), 2);
        assert!(config.backends.contains_key("llama-server"));
        assert!(config.backends.contains_key("ollama"));
        assert!(config.backends["llama-server"].supports_tools);
        assert!(config.backends["llama-server"].supports_vision);
        assert!(!config.backends["ollama"].supports_vision);
    }

    #[test]
    fn test_primary_backend_selection() {
        let toml_content = r#"
[inference]
primary = "lm-studio"

[inference.backends.llama-server]
url = "http://127.0.0.1:8080"
supports_tools = true

[inference.backends.lm-studio]
url = "http://127.0.0.1:1234"
supports_tools = true
"#;
        // Write to temp file
        let tmp = std::env::temp_dir().join("trinity_test_primary.toml");
        std::fs::write(&tmp, toml_content).unwrap();

        let router = InferenceRouter::from_config(Some(tmp.to_str().unwrap()));
        assert_eq!(router.active_name(), "lm-studio");
        assert_eq!(router.active_url(), "http://127.0.0.1:1234");

        std::fs::remove_file(tmp).ok();
    }

    #[test]
    fn test_switch_backend() {
        let toml_content = r#"
[inference]
primary = "llama-server"

[inference.backends.llama-server]
url = "http://127.0.0.1:8080"
supports_tools = true
supports_vision = true

[inference.backends.ollama]
url = "http://127.0.0.1:11434"
supports_tools = true
supports_vision = false
"#;
        let tmp = std::env::temp_dir().join("trinity_test_switch.toml");
        std::fs::write(&tmp, toml_content).unwrap();

        let mut router = InferenceRouter::from_config(Some(tmp.to_str().unwrap()));
        assert_eq!(router.active_name(), "llama-server");
        assert!(router.supports_tools());
        assert!(router.supports_vision());

        // Switch to ollama
        assert!(router.switch_backend("ollama"));
        assert_eq!(router.active_name(), "ollama");
        assert_eq!(router.active_url(), "http://127.0.0.1:11434");
        assert!(!router.supports_vision());

        // Switch to nonexistent
        assert!(!router.switch_backend("nonexistent"));
        assert_eq!(router.active_name(), "ollama"); // unchanged

        std::fs::remove_file(tmp).ok();
    }

    #[test]
    fn test_set_active_url_existing() {
        let mut router = InferenceRouter::from_config(Some("/nonexistent.toml"));
        let original_count = router.backends.len();

        // Set to an existing backend URL
        router.set_active_url("http://127.0.0.1:11434".to_string());
        assert_eq!(router.active_url(), "http://127.0.0.1:11434");
        assert_eq!(router.backends.len(), original_count); // no new backend added
    }

    #[test]
    fn test_set_active_url_new() {
        let mut router = InferenceRouter::from_config(Some("/nonexistent.toml"));
        let original_count = router.backends.len();

        // Set to a brand new URL
        router.set_active_url("http://192.168.1.50:9999".to_string());
        assert_eq!(router.active_url(), "http://192.168.1.50:9999");
        assert_eq!(router.backends.len(), original_count + 1);
        assert_eq!(router.active_name(), "manual");
    }

    #[test]
    fn test_status_serialization() {
        let router = InferenceRouter::from_config(Some("/nonexistent.toml"));
        let status = router.status();
        assert_eq!(status.active_backend, "vllm-omni");
        assert!(!status.backends.is_empty());

        // Should serialize to JSON without panic
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("vllm-omni"));
    }

    #[test]
    fn test_backend_kind_properties() {
        assert_eq!(BackendKind::VllmOmni.default_port(), 8000);
        assert_eq!(BackendKind::LlamaServer.default_port(), 8080);
        assert_eq!(BackendKind::Ollama.default_port(), 11434);
        assert_eq!(BackendKind::LmStudio.default_port(), 1234);

        assert_eq!(BackendKind::VllmOmni.health_path(), "/health");
        assert_eq!(BackendKind::Ollama.health_path(), "/api/tags");
        assert_eq!(BackendKind::LlamaServer.health_path(), "/health");
    }

    #[test]
    fn test_config_with_no_inference_section() {
        let toml_content = r#"
[node]
type = "Auto"

[model]
model_dir = "~/trinity-models/gguf"
"#;
        let tmp = std::env::temp_dir().join("trinity_test_no_inference.toml");
        std::fs::write(&tmp, toml_content).unwrap();

        let router = InferenceRouter::from_config(Some(tmp.to_str().unwrap()));
        // Should use defaults when [inference] section is missing
        assert!(!router.backends.is_empty());
        assert_eq!(router.config.primary, "vllm-omni");
        assert_eq!(router.config.ctx_size, 262144);

        std::fs::remove_file(tmp).ok();
    }
}
