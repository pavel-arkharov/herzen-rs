use anyhow::Result;
use tracing_subscriber::EnvFilter;

use herzen_llm::ModelPool;
use herzen_server::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("herzen daemon starting");

    // Load configuration
    let config_path = std::env::args().nth(1).map(std::path::PathBuf::from);
    let config = herzen_config::load(config_path.as_deref())?;

    tracing::info!(
        "server will listen on {}:{}",
        config.server.host,
        config.server.port
    );

    // Initialize model pool
    let model_pool = ModelPool::new();

    // Register models from config
    for model_config in &config.llm.models {
        model_pool
            .register_model(model_config.clone(), &config.llm.base_url)
            .await;
        tracing::info!("registered model: {} (role: {:?})", model_config.name, model_config.role);
    }

    if let Some(default) = &config.llm.default_model {
        model_pool.set_default(default).await;
    }

    tracing::info!(
        "{} model(s) registered",
        model_pool.count().await
    );

    // Start HTTP server
    let state = AppState::new(config, model_pool);
    herzen_server::serve(state).await?;

    Ok(())
}
