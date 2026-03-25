//! HTTP + WebSocket API server for Herzen.
//!
//! REST endpoints:
//! - `GET  /health`          — liveness check
//! - `GET  /api/config`      — current configuration
//! - `GET  /api/models`      — list loaded models
//! - `POST /api/chat`        — send a chat completion request
//!
//! WebSocket:
//! - `GET  /ws`              — real-time events (future: turn progress, VAD state)

mod routes;
mod state;

pub use state::AppState;

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Build the axum router with all routes.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::health_routes())
        .merge(routes::api_routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Start the server on the given address.
pub async fn serve(state: AppState) -> anyhow::Result<()> {
    let addr: SocketAddr = format!("{}:{}", state.config().server.host, state.config().server.port)
        .parse()?;

    let app = build_router(state);

    tracing::info!("herzen server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
