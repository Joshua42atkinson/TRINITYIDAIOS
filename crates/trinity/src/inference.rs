// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        inference.rs
// PURPOSE:     Multi-engine inference client — OpenAI-compatible HTTP API
//
// ARCHITECTURE:
//   • Engine-agnostic: works with ANY OpenAI-compatible server
//   • vLLM (:8001) — Great Recycler (primary brain, served-model-name)
//   • Any OpenAI-compatible server
//   • FastFlowLM (NPU)   — ONNX models via AMD XDNA 2
//   • All engines share /v1/chat/completions protocol
//   • PersonaPlex audio-to-audio uses separate ONNX path (see voice.rs)
//   • Streaming SSE support for real-time response generation
//   • Handles GPT-OSS reasoning_content field for CoT models
//
// DEPENDENCIES:
//   - serde — JSON serialization
//   - reqwest — HTTP client
//
// CHANGES:
//   2026-03-18  Cascade  Multi-engine architecture (llama.cpp + any OpenAI-compatible)
//   2026-03-16  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

use crate::ChatMessage;

#[derive(Serialize)]
struct CompletionRequest {
    model: String,
    messages: Vec<ApiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    temperature: f32,
    stream: bool,
    /// Reasoning effort: "high" for deep thinking, "none" for fast responses
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_effort: Option<String>,
    /// OpenAI-compatible tool definitions (vLLM supports natively)
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<ToolDefinition>>,
}

// ═══════════════════════════════════════════════════
// Tool Calling Structs (OpenAI-compatible)
// ═══════════════════════════════════════════════════

/// Tool definition sent in the request (OpenAI format)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String, // always "function"
    pub function: ToolFunctionDef,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolFunctionDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema
}

/// Tool call returned by the model in a response
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String, // always "function"
    pub function: FunctionCall,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String, // JSON string
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum ApiMessageContent {
    Text(String),
    Multimodal(Vec<ApiContentPart>),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
enum ApiContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ApiImageUrl },
}

#[derive(Serialize, Deserialize, Clone)]
struct ApiImageUrl {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct ApiMessage {
    role: String,
    content: ApiMessageContent,
    /// GPT-OSS models put reasoning in this field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reasoning_content: Option<String>,
}

#[derive(Deserialize)]
struct CompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ApiMessageResponse,
}

#[derive(Deserialize)]
#[allow(dead_code)] // role deserialized but only content/tool_calls read
struct ApiMessageResponse {
    role: String,
    #[serde(default)]
    content: Option<String>,
    /// GPT-OSS models put reasoning in this field
    #[serde(default)]
    reasoning_content: Option<String>,
    /// Structured tool calls from the model
    #[serde(default)]
    tool_calls: Option<Vec<ToolCall>>,
}

/// Check if the LLM server is healthy
pub async fn check_health(base_url: &str) -> bool {
    crate::http::check_health(base_url).await
}

/// Send a streaming chat completion request, yielding tokens as they arrive
pub async fn chat_completion_stream(
    base_url: &str,
    messages: &[ChatMessage],
    _max_tokens: u32,
    tx: tokio::sync::mpsc::Sender<String>,
    reasoning_effort: Option<&str>,
) -> anyhow::Result<()> {
    let client = &*crate::http::LONG;
    let no_reasoning = reasoning_effort.as_ref().map(|e| *e == "none").unwrap_or(false);

    let api_messages: Vec<ApiMessage> = messages
        .iter()
        .map(|m| {
            let content = if let Some(base64) = &m.image_base64 {
                ApiMessageContent::Multimodal(vec![
                    ApiContentPart::Text {
                        text: m.content.clone(),
                    },
                    ApiContentPart::ImageUrl {
                        image_url: ApiImageUrl {
                            url: base64.clone(),
                        },
                    },
                ])
            } else {
                ApiMessageContent::Text(m.content.clone())
            };

            ApiMessage {
                role: m.role.clone(),
                content,
                reasoning_content: None,
            }
        })
        .collect();

    let request = CompletionRequest {
        model: "Great_Recycler".to_string(), // Default proxy name for local LLM routing
        messages: api_messages,
        max_tokens: None, // Let inference backend auto-calculate to prevent context length errors
        temperature: 0.7,
        stream: true,
        reasoning_effort: None, // Omitted — some backends reject unknown fields
        tools: None,
    };

    let mut response = client
        .post(format!("{}/v1/chat/completions", base_url))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("LLM returned {}: {}", status, body);
    }

    // Read SSE stream from LLM server
    while let Some(chunk) = response.chunk().await? {
        let text = String::from_utf8_lossy(&chunk);
        for line in text.lines() {
            if let Some(data) = line.strip_prefix("data: ") {
                if data.trim() == "[DONE]" {
                    break;
                }
                if let Ok(parsed) = serde_json::from_str::<StreamChunk>(data) {
                    if let Some(choice) = parsed.choices.first() {
                        // When reasoning is disabled (zen mode), ONLY stream content tokens.
                        // When reasoning is enabled, fall back to reasoning_content if content is empty
                        // (GPT-OSS-20B puts actual content in reasoning_content).
                        let token = if no_reasoning {
                            choice.delta.content.as_ref()
                        } else {
                            choice
                                .delta
                                .content
                                .as_ref()
                                .or(choice.delta.reasoning_content.as_ref())
                        };
                        if let Some(content) = token {
                            if tx.send(content.clone()).await.is_err() {
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

#[derive(Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
}

#[derive(Deserialize)]
struct StreamDelta {
    content: Option<String>,
    reasoning_content: Option<String>,
}

/// Send a chat completion request to LLM server
pub async fn chat_completion(
    base_url: &str,
    messages: &[ChatMessage],
    max_tokens: u32,
) -> anyhow::Result<String> {
    chat_completion_with_effort(base_url, messages, max_tokens, None).await
}

/// Send a chat completion request with configurable reasoning effort
/// reasoning_effort: Some("high") for deep thinking, Some("none") for fast, None for default
pub async fn chat_completion_with_effort(
    base_url: &str,
    messages: &[ChatMessage],
    _max_tokens: u32,
    _reasoning_effort: Option<&str>,
) -> anyhow::Result<String> {
    let client = &*crate::http::LONG;

    let api_messages: Vec<ApiMessage> = messages
        .iter()
        .map(|m| {
            let content = if let Some(base64) = &m.image_base64 {
                ApiMessageContent::Multimodal(vec![
                    ApiContentPart::Text {
                        text: m.content.clone(),
                    },
                    ApiContentPart::ImageUrl {
                        image_url: ApiImageUrl {
                            url: base64.clone(),
                        },
                    },
                ])
            } else {
                ApiMessageContent::Text(m.content.clone())
            };

            ApiMessage {
                role: m.role.clone(),
                content,
                reasoning_content: None,
            }
        })
        .collect();

    let request = CompletionRequest {
        model: "Great_Recycler".to_string(),
        messages: api_messages,
        max_tokens: None, // Let inference backend auto-calculate to prevent context length errors
        temperature: 0.7,
        stream: false,
        reasoning_effort: None, // Omitted — some backends reject unknown fields
        tools: None,
    };

    let response = client
        .post(format!("{}/v1/chat/completions", base_url))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("LLM returned {}: {}", status, body);
    }

    let completion: CompletionResponse = response.json().await?;

    completion
        .choices
        .first()
        .map(|c| {
            // GPT-OSS models put actual content in reasoning_content, content is truncated
            // Use reasoning_content if available and longer, otherwise use content
            let content = c.message.content.clone().unwrap_or_default();
            let reasoning = c.message.reasoning_content.clone().unwrap_or_default();

            // Prefer reasoning_content if it's longer (GPT-OSS behavior)
            if reasoning.len() > content.len() {
                reasoning
            } else {
                content
            }
        })
        .ok_or_else(|| anyhow::anyhow!("No completion choices returned"))
}

// ═══════════════════════════════════════════════════
// Structured Tool Calling (Phase 2)
// ═══════════════════════════════════════════════════

/// Result from a tool-aware completion: either text content, tool calls, or both
#[derive(Debug, Clone)]
pub struct ToolAwareResponse {
    pub content: Option<String>,
    pub tool_calls: Vec<ToolCall>,
}

/// Build ToolDefinition array from our ToolInfo registry
pub fn build_tool_definitions(tools: &[crate::tools::ToolInfo]) -> Vec<ToolDefinition> {
    tools
        .iter()
        .map(|t| {
            // Build JSON Schema properties from param names
            let mut properties = serde_json::Map::new();
            for param in &t.params {
                properties.insert(
                    param.clone(),
                    serde_json::json!({ "type": "string", "description": param }),
                );
            }

            ToolDefinition {
                tool_type: "function".to_string(),
                function: ToolFunctionDef {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": properties,
                        "required": t.params, // Strict schema enforcement needs required field
                    }),
                },
            }
        })
        .collect()
}

/// Send a chat completion with tool definitions — returns structured tool calls
pub async fn chat_completion_with_tools(
    base_url: &str,
    messages: &[ChatMessage],
    _max_tokens: u32,
    tools: &[ToolDefinition],
    _reasoning_effort: Option<&str>,
) -> anyhow::Result<ToolAwareResponse> {
    let client = &*crate::http::LONG;

    let api_messages: Vec<ApiMessage> = messages
        .iter()
        .map(|m| {
            let content = if let Some(base64) = &m.image_base64 {
                ApiMessageContent::Multimodal(vec![
                    ApiContentPart::Text {
                        text: m.content.clone(),
                    },
                    ApiContentPart::ImageUrl {
                        image_url: ApiImageUrl {
                            url: base64.clone(),
                        },
                    },
                ])
            } else {
                ApiMessageContent::Text(m.content.clone())
            };

            ApiMessage {
                role: m.role.clone(),
                content,
                reasoning_content: None,
            }
        })
        .collect();

    let request = CompletionRequest {
        model: "Great_Recycler".to_string(),
        messages: api_messages,
        max_tokens: None, // Let inference backend auto-calculate to prevent context length errors
        temperature: 0.7,
        stream: false,
        reasoning_effort: None, // Omitted — some backends reject unknown fields
        tools: if tools.is_empty() {
            None
        } else {
            Some(tools.to_vec())
        },
    };

    let response = client
        .post(format!("{}/v1/chat/completions", base_url))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("LLM returned {}: {}", status, body);
    }

    let completion: CompletionResponse = response.json().await?;

    let choice = completion
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!("No completion choices returned"))?;

    let content = {
        let c = choice.message.content.clone().unwrap_or_default();
        let r = choice.message.reasoning_content.clone().unwrap_or_default();
        let text = if r.len() > c.len() { r } else { c };
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    };

    let tool_calls = choice.message.tool_calls.clone().unwrap_or_default();

    Ok(ToolAwareResponse {
        content,
        tool_calls,
    })
}

