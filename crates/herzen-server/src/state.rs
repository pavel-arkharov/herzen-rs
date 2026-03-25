use std::sync::Arc;

use herzen_config::HerzenConfig;
use herzen_llm::ModelPool;

/// Shared application state, accessible from all route handlers.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    config: HerzenConfig,
    model_pool: ModelPool,
}

impl AppState {
    pub fn new(config: HerzenConfig, model_pool: ModelPool) -> Self {
        Self {
            inner: Arc::new(Inner {
                config,
                model_pool,
            }),
        }
    }

    pub fn config(&self) -> &HerzenConfig {
        &self.inner.config
    }

    pub fn model_pool(&self) -> &ModelPool {
        &self.inner.model_pool
    }
}
