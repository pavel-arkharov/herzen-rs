//! LLM inference and model pool management.
//!
//! Provides a trait-based provider system for LLM inference. Currently supports
//! OpenAI-compatible HTTP APIs (works with Ollama, llama-server, vLLM, etc.).
//! Direct llama.cpp bindings planned as a future provider.

mod pool;
mod provider;
mod types;

pub use pool::ModelPool;
pub use provider::{LlmProvider, OpenAiProvider};
pub use types::{ChatMessage, ChatRequest, ChatResponse, LlmError, Role};
