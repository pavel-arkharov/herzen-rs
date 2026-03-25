use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::types::{ChatRequest, ChatResponse, LlmError};

/// Trait for LLM inference providers.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Generate a chat completion.
    async fn chat(&self, request: &ChatRequest) -> Result<ChatResponse, LlmError>;

    /// List available models from this provider.
    async fn list_models(&self) -> Result<Vec<String>, LlmError>;

    /// Provider name for logging/UI.
    fn name(&self) -> &str;
}

/// OpenAI-compatible HTTP provider.
///
/// Works with Ollama, llama-server, vLLM, LM Studio, and any server
/// that implements the `/v1/chat/completions` endpoint.
pub struct OpenAiProvider {
    client: reqwest::Client,
    base_url: String,
    default_model: Option<String>,
}

impl OpenAiProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            default_model: None,
        }
    }

    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = Some(model.into());
        self
    }

    /// Build an Ollama provider (default port 11434).
    pub fn ollama() -> Self {
        Self::new("http://127.0.0.1:11434")
    }

    /// Build a llama-server provider (default port 8080).
    pub fn llama_server() -> Self {
        Self::new("http://127.0.0.1:8080")
    }
}

// OpenAI API request/response types
#[derive(Serialize)]
struct ApiChatRequest {
    model: String,
    messages: Vec<ApiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct ApiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ApiChatResponse {
    choices: Vec<ApiChoice>,
    model: String,
    usage: Option<ApiUsage>,
}

#[derive(Deserialize)]
struct ApiChoice {
    message: ApiMessage,
}

#[derive(Deserialize)]
struct ApiUsage {
    prompt_tokens: Option<u32>,
    completion_tokens: Option<u32>,
}

#[derive(Deserialize)]
struct ApiModelList {
    models: Option<Vec<ApiModel>>,
    data: Option<Vec<ApiModel>>,
}

#[derive(Deserialize)]
struct ApiModel {
    #[serde(alias = "name")]
    id: String,
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    async fn chat(&self, request: &ChatRequest) -> Result<ChatResponse, LlmError> {
        let model = request
            .model
            .as_deref()
            .or(self.default_model.as_deref())
            .unwrap_or("default")
            .to_string();

        let api_request = ApiChatRequest {
            model: model.clone(),
            messages: request
                .messages
                .iter()
                .map(|m| ApiMessage {
                    role: format!("{:?}", m.role).to_lowercase(),
                    content: m.content.clone(),
                })
                .collect(),
            temperature: request.temperature,
            top_p: request.top_p,
            max_tokens: request.max_tokens,
        };

        let url = format!("{}/v1/chat/completions", self.base_url);
        tracing::debug!("POST {} model={}", url, model);

        let response = self
            .client
            .post(&url)
            .json(&api_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(LlmError::ProviderError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        let api_response: ApiChatResponse = response.json().await?;

        let choice = api_response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| LlmError::ProviderError("empty choices in response".into()))?;

        Ok(ChatResponse {
            content: choice.message.content,
            model: api_response.model,
            prompt_tokens: api_response.usage.as_ref().and_then(|u| u.prompt_tokens),
            completion_tokens: api_response
                .usage
                .as_ref()
                .and_then(|u| u.completion_tokens),
        })
    }

    async fn list_models(&self) -> Result<Vec<String>, LlmError> {
        // Try OpenAI-style /v1/models first, fall back to Ollama /api/tags
        let urls = [
            format!("{}/v1/models", self.base_url),
            format!("{}/api/tags", self.base_url),
        ];

        for url in &urls {
            let response = match self.client.get(url).send().await {
                Ok(r) if r.status().is_success() => r,
                _ => continue,
            };

            if let Ok(list) = response.json::<ApiModelList>().await {
                let models = list
                    .data
                    .or(list.models)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|m| m.id)
                    .collect();
                return Ok(models);
            }
        }

        Ok(Vec::new())
    }

    fn name(&self) -> &str {
        "openai-compatible"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ollama_default_url() {
        let provider = OpenAiProvider::ollama();
        assert_eq!(provider.base_url, "http://127.0.0.1:11434");
    }

    #[test]
    fn llama_server_default_url() {
        let provider = OpenAiProvider::llama_server();
        assert_eq!(provider.base_url, "http://127.0.0.1:8080");
    }

    #[test]
    fn custom_url_trailing_slash_stripped() {
        let provider = OpenAiProvider::new("http://localhost:9999/");
        assert_eq!(provider.base_url, "http://localhost:9999");
    }
}
