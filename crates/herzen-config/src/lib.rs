//! Centralized typed configuration for Herzen.
//!
//! All configuration lives in a single `herzen.toml` file with typed sections.
//! Each section has sensible defaults. The config can be loaded from a file path
//! or from a default location (`~/.herzen/herzen.toml`).
//!
//! Provides JSON Schema export via `schemars` for dynamic UI form generation.

mod model;

pub use model::{
    AudioConfig, HaConfig, HerzenConfig, LlmConfig, ModelConfig, ModelRole, ServerConfig,
    SttConfig, SttLanguage, TtsConfig,
};

use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("config file not found: {0}")]
    NotFound(PathBuf),
    #[error("failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("failed to parse config: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("validation error: {0}")]
    ValidationError(String),
}

/// Load configuration from the given path, or fall back to default location.
pub fn load(path: Option<&Path>) -> Result<HerzenConfig, ConfigError> {
    let path = match path {
        Some(p) => p.to_path_buf(),
        None => default_config_path(),
    };

    if !path.exists() {
        // Return defaults if no config file exists
        tracing::info!("no config file at {}, using defaults", path.display());
        return Ok(HerzenConfig::default());
    }

    let contents = std::fs::read_to_string(&path)?;
    let config: HerzenConfig = toml::from_str(&contents)?;
    config.validate()?;

    tracing::info!("loaded config from {}", path.display());
    Ok(config)
}

/// Default config file location: ~/.herzen/herzen.toml
pub fn default_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".herzen")
        .join("herzen.toml")
}

/// Default data directory: ~/.herzen/data/
pub fn default_data_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".herzen")
        .join("data")
}

/// Default models directory: ~/.herzen/models/
pub fn default_models_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".herzen")
        .join("models")
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!version().is_empty());
    }

    #[test]
    fn default_config_is_valid() {
        let config = HerzenConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn parse_minimal_toml() {
        let toml_str = r#"
            [server]
            port = 4000
        "#;
        let config: HerzenConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.server.port, 4000);
        assert_eq!(config.server.host, "127.0.0.1"); // default
    }

    #[test]
    fn parse_full_toml() {
        let toml_str = r#"
            [server]
            host = "0.0.0.0"
            port = 5000

            [audio]
            sample_rate = 44100
            channels = 2

            [stt]
            model_path = "/path/to/model.bin"
            language = "en"

            [tts]
            default_provider = "piper"

            [llm]
            default_model = "dialog"

            [[llm.models]]
            name = "dialog"
            path = "/path/to/model.gguf"
            role = "dialog"
            gpu_layers = 99
            context_size = 4096
            temperature = 0.7
            top_p = 0.9
            max_tokens = 512
        "#;
        let config: HerzenConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.audio.sample_rate, 44100);
        assert_eq!(config.stt.language, SttLanguage::En);
        assert_eq!(config.llm.models.len(), 1);
        assert_eq!(config.llm.models[0].name, "dialog");
        assert_eq!(config.llm.models[0].gpu_layers, 99);
    }

    #[test]
    fn invalid_port_zero() {
        let config = HerzenConfig {
            server: ServerConfig {
                port: 0,
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn load_nonexistent_returns_defaults() {
        let config = load(Some(Path::new("/tmp/herzen-test-nonexistent.toml"))).unwrap();
        assert_eq!(config.server.port, 3100);
    }

    #[test]
    fn json_schema_generation() {
        let schema = schemars::schema_for!(HerzenConfig);
        let json = serde_json::to_string_pretty(&schema).unwrap();
        assert!(json.contains("server"));
        assert!(json.contains("audio"));
        assert!(json.contains("llm"));
    }
}
