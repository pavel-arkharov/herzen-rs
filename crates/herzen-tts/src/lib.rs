//! TTS provider registry with self-describing settings schemas.
//!
//! Each provider declares its own capabilities and JSON Schema for settings,
//! enabling the UI to render dynamic controls per provider.
//!
//! Built-in providers: macOS `say`, Piper (ONNX), espeak.
//! Sidecar providers: XTTS, Bark, StyleTTS2 (Python via HTTP).
