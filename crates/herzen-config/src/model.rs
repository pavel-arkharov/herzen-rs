use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Root configuration for Herzen.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct HerzenConfig {
    pub server: ServerConfig,
    pub audio: AudioConfig,
    pub stt: SttConfig,
    pub tts: TtsConfig,
    pub llm: LlmConfig,
    pub ha: Option<HaConfig>,
}

impl HerzenConfig {
    pub fn validate(&self) -> Result<(), crate::ConfigError> {
        if self.server.port == 0 {
            return Err(crate::ConfigError::ValidationError(
                "server.port must be > 0".into(),
            ));
        }
        if self.audio.sample_rate == 0 {
            return Err(crate::ConfigError::ValidationError(
                "audio.sample_rate must be > 0".into(),
            ));
        }
        if self.audio.channels == 0 {
            return Err(crate::ConfigError::ValidationError(
                "audio.channels must be > 0".into(),
            ));
        }
        for model in &self.llm.models {
            model.validate()?;
        }
        Ok(())
    }
}

/// HTTP + WebSocket server settings.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct ServerConfig {
    /// Bind address
    pub host: String,
    /// Bind port
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 3100,
        }
    }
}

/// Audio capture and playback settings.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct AudioConfig {
    /// Sample rate in Hz
    pub sample_rate: u32,
    /// Number of audio channels (1 = mono)
    pub channels: u16,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            channels: 1,
        }
    }
}

/// Speech-to-text configuration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct SttConfig {
    /// Path to the whisper model file (.bin)
    pub model_path: Option<String>,
    /// Language for transcription
    pub language: SttLanguage,
}

impl Default for SttConfig {
    fn default() -> Self {
        Self {
            model_path: None,
            language: SttLanguage::Auto,
        }
    }
}

/// Supported STT languages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SttLanguage {
    Auto,
    En,
    Ru,
}

/// Text-to-speech configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct TtsConfig {
    /// Default TTS provider name
    pub default_provider: Option<String>,
}

/// LLM inference configuration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct LlmConfig {
    /// Name of the default model (references a model in the `models` list)
    pub default_model: Option<String>,
    /// Base URL for the OpenAI-compatible inference backend (Ollama, llama-server, etc.)
    pub base_url: String,
    /// Registered models
    pub models: Vec<ModelConfig>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            default_model: None,
            base_url: "http://127.0.0.1:11434".into(),
            models: Vec::new(),
        }
    }
}

/// Configuration for a single LLM model.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModelConfig {
    /// Unique name for this model
    pub name: String,
    /// Path to the GGUF model file
    pub path: String,
    /// What role this model fills
    #[serde(default)]
    pub role: ModelRole,
    /// Number of layers to offload to GPU (0 = CPU only, 99 = all)
    #[serde(default)]
    pub gpu_layers: u32,
    /// Context window size in tokens
    #[serde(default = "default_context_size")]
    pub context_size: u32,
    /// Sampling temperature
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    /// Top-p sampling
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    /// Maximum tokens to generate
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

impl ModelConfig {
    pub fn validate(&self) -> Result<(), crate::ConfigError> {
        if self.name.is_empty() {
            return Err(crate::ConfigError::ValidationError(
                "model name cannot be empty".into(),
            ));
        }
        if self.path.is_empty() {
            return Err(crate::ConfigError::ValidationError(
                format!("model '{}' has empty path", self.name),
            ));
        }
        if self.temperature < 0.0 || self.temperature > 2.0 {
            return Err(crate::ConfigError::ValidationError(
                format!("model '{}' temperature must be 0.0-2.0", self.name),
            ));
        }
        Ok(())
    }
}

/// The role a model fills in the system.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ModelRole {
    #[default]
    Dialog,
    Intent,
    Summarizer,
    Entity,
    Custom,
}

fn default_context_size() -> u32 {
    4096
}

fn default_temperature() -> f32 {
    0.7
}

fn default_top_p() -> f32 {
    0.9
}

fn default_max_tokens() -> u32 {
    512
}

/// Home Assistant integration settings.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HaConfig {
    /// Home Assistant endpoint URL
    pub endpoint: String,
    /// Path to file containing the HA long-lived access token
    pub token_file: String,
}
