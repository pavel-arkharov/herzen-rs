//! Turn pipeline orchestration for Herzen.
//!
//! Manages the flow: Trigger → Record → VAD → STT → Route → LLM → TTS → Playback.
//! Emits typed events via tokio::broadcast for UI streaming.
