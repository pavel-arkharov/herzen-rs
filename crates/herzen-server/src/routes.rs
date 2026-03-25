use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use herzen_llm::{ChatMessage, ChatRequest, Role};

use crate::AppState;

// --- Health ---

pub fn health_routes() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "version": herzen_config::version(),
    }))
}

// --- API ---

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/api/config", get(get_config))
        .route("/api/models", get(list_models))
        .route("/api/chat", post(chat))
}

async fn get_config(State(state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::to_value(state.config()).unwrap_or_default())
}

async fn list_models(State(state): State<AppState>) -> impl IntoResponse {
    let models = state.model_pool().list().await;
    Json(serde_json::json!({ "models": models }))
}

#[derive(Deserialize)]
struct ChatBody {
    messages: Vec<ChatMessageBody>,
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    temperature: Option<f32>,
    #[serde(default)]
    max_tokens: Option<u32>,
}

#[derive(Deserialize)]
struct ChatMessageBody {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatResponseBody {
    content: String,
    model: String,
    prompt_tokens: Option<u32>,
    completion_tokens: Option<u32>,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

async fn chat(
    State(state): State<AppState>,
    Json(body): Json<ChatBody>,
) -> Result<Json<ChatResponseBody>, (StatusCode, Json<ErrorBody>)> {
    let messages: Vec<ChatMessage> = body
        .messages
        .into_iter()
        .map(|m| ChatMessage {
            role: match m.role.as_str() {
                "system" => Role::System,
                "assistant" => Role::Assistant,
                _ => Role::User,
            },
            content: m.content,
        })
        .collect();

    let mut request = ChatRequest::new(messages);
    if let Some(model) = &body.model {
        request = request.with_model(model);
    }
    if let Some(temp) = body.temperature {
        request = request.with_temperature(temp);
    }
    if let Some(tokens) = body.max_tokens {
        request = request.with_max_tokens(tokens);
    }

    let provider = state
        .model_pool()
        .get(body.model.as_deref())
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: e.to_string(),
                }),
            )
        })?;

    let response = provider.chat(&request).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody {
                error: e.to_string(),
            }),
        )
    })?;

    Ok(Json(ChatResponseBody {
        content: response.content,
        model: response.model,
        prompt_tokens: response.prompt_tokens,
        completion_tokens: response.completion_tokens,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use herzen_config::HerzenConfig;
    use herzen_llm::ModelPool;
    use tower::ServiceExt;

    fn test_state() -> AppState {
        AppState::new(HerzenConfig::default(), ModelPool::new())
    }

    #[tokio::test]
    async fn health_endpoint() {
        let app = crate::build_router(test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn config_endpoint() {
        let app = crate::build_router(test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/config")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn models_endpoint_empty() {
        let app = crate::build_router(test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/models")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn chat_no_models_returns_error() {
        let app = crate::build_router(test_state());
        let body = serde_json::json!({
            "messages": [{"role": "user", "content": "hello"}]
        });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/chat")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
