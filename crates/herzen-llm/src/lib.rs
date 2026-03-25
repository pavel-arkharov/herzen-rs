//! LLM inference via llama.cpp and model pool management.
//!
//! - Direct GGUF model loading via llama-cpp-2 bindings
//! - Model pool: load/unload dynamically, track memory per model
//! - Model registry in TOML: path, quantization, role, default parameters
//! - OpenAI-compatible HTTP fallback for external providers (Ollama, etc.)
