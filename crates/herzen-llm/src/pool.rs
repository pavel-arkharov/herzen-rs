use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use herzen_config::ModelConfig;

use crate::provider::{LlmProvider, OpenAiProvider};
use crate::types::LlmError;

/// Tracks loaded models and routes requests to the appropriate provider.
pub struct ModelPool {
    providers: RwLock<HashMap<String, Arc<dyn LlmProvider>>>,
    configs: RwLock<HashMap<String, ModelConfig>>,
    default_model: RwLock<Option<String>>,
}

impl ModelPool {
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
            configs: RwLock::new(HashMap::new()),
            default_model: RwLock::new(None),
        }
    }

    /// Register a model config and create its provider.
    /// For now, all models use the OpenAI-compatible HTTP provider.
    pub async fn register_model(&self, config: ModelConfig, base_url: &str) {
        let provider = OpenAiProvider::new(base_url).with_default_model(&config.name);
        let name = config.name.clone();

        self.providers
            .write()
            .await
            .insert(name.clone(), Arc::new(provider));
        self.configs.write().await.insert(name, config);
    }

    /// Set the default model name.
    pub async fn set_default(&self, name: impl Into<String>) {
        *self.default_model.write().await = Some(name.into());
    }

    /// Get a provider by model name, or the default.
    pub async fn get(&self, name: Option<&str>) -> Result<Arc<dyn LlmProvider>, LlmError> {
        let providers = self.providers.read().await;

        if providers.is_empty() {
            return Err(LlmError::NoModelsLoaded);
        }

        let key = match name {
            Some(n) => n.to_string(),
            None => {
                let default = self.default_model.read().await;
                default
                    .clone()
                    .or_else(|| providers.keys().next().cloned())
                    .ok_or(LlmError::NoModelsLoaded)?
            }
        };

        providers
            .get(&key)
            .cloned()
            .ok_or_else(|| LlmError::ModelNotFound(key))
    }

    /// List all registered model names.
    pub async fn list(&self) -> Vec<String> {
        self.providers.read().await.keys().cloned().collect()
    }

    /// List all registered model configs (for rich API responses).
    pub async fn list_with_configs(&self) -> Vec<ModelConfig> {
        self.configs.read().await.values().cloned().collect()
    }

    /// Remove a model from the pool.
    pub async fn unload(&self, name: &str) {
        self.providers.write().await.remove(name);
        self.configs.write().await.remove(name);
        tracing::info!("unloaded model: {}", name);
    }

    /// Number of loaded models.
    pub async fn count(&self) -> usize {
        self.providers.read().await.len()
    }
}

impl Default for ModelPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn empty_pool_returns_error() {
        let pool = ModelPool::new();
        assert!(pool.get(None).await.is_err());
    }

    #[tokio::test]
    async fn register_and_get() {
        let pool = ModelPool::new();
        let config = ModelConfig {
            name: "test-model".into(),
            path: "/fake/path.gguf".into(),
            role: herzen_config::ModelRole::Dialog,
            gpu_layers: 0,
            context_size: 2048,
            temperature: 0.7,
            top_p: 0.9,
            max_tokens: 256,
        };
        pool.register_model(config, "http://localhost:11434").await;
        pool.set_default("test-model").await;

        let provider = pool.get(None).await.unwrap();
        assert_eq!(provider.name(), "openai-compatible");
        assert_eq!(pool.count().await, 1);
    }

    #[tokio::test]
    async fn unload_removes_model() {
        let pool = ModelPool::new();
        let config = ModelConfig {
            name: "ephemeral".into(),
            path: "/fake/path.gguf".into(),
            role: herzen_config::ModelRole::Dialog,
            gpu_layers: 0,
            context_size: 2048,
            temperature: 0.7,
            top_p: 0.9,
            max_tokens: 256,
        };
        pool.register_model(config, "http://localhost:11434").await;
        assert_eq!(pool.count().await, 1);

        pool.unload("ephemeral").await;
        assert_eq!(pool.count().await, 0);
    }
}
