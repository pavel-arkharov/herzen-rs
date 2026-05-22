# Herzen

**Modular AI-agent & voice pipeline in Rust. Features async-first orchestration (VAD, STT, LLM, TTS), Home Assistant integration, and a real-time Tauri/Svelte desktop UI.**

---

## 🎙️ The Project

Herzen is a high-performance, local-first voice assistant and AI orchestration engine designed with privacy and modularity at its core. Built entirely in Rust, it provides a complete pipeline for ambient intelligence—from voice activity detection and speech recognition to semantic intent matching and LLM-driven responses.

## 🛠️ Technical Highlights

### 1. Modular Pipeline Orchestration
Herzen utilizes a sophisticated workspace-based architecture where every stage of the voice-to-action pipeline is decoupled into specialized, reusable crates:
- **Audio & VAD**: High-performance audio capture and Voice Activity Detection.
- **Speech Processing**: STT (Speech-to-Text) and TTS (Text-to-Speech) modules.
- **Intelligence Layer**: LLM orchestration with support for model pooling and specialized roles.
- **Action & Integration**: A declarative skill engine and Home Assistant integration for deterministic smart-home control.

### 2. Async-First & Event-Driven
Powered by **Tokio**, the core daemon manages complex concurrent workflows without blocking. It leverages `tokio::broadcast` channels to emit real-time telemetry and pipeline events, which are streamed via WebSockets to the desktop interface for a low-latency, "alive" user experience.

### 3. Declarative Skill Engine
Herzen features a multi-layered intent matching system designed for precision and flexibility:
1. **Keyword Aliases**: Fast, deterministic matching for common multilingual commands.
2. **Semantic Embeddings**: Vector-based matching for natural language variations.
3. **LLM Fallback**: Advanced reasoning for complex or ambiguous requests that fall outside predefined skills.

### 4. Cross-Platform Desktop UI
The system includes a modern **Tauri + Svelte** desktop application that acts as a control panel, providing real-time visibility into the agent's internal state, pipeline progress, and model performance.

## 🏗️ Workspace Structure

```text
crates/
├── herzen-audio     # Low-level audio capture and playback
├── herzen-config    # Centralized typed configuration (TOML + JSON Schema)
├── herzen-context   # Global state and conversation context management
├── herzen-core      # Pipeline orchestration logic and shared traits
├── herzen-daemon    # Main service entry point and lifecycle management
├── herzen-ha        # Home Assistant WebSocket integration
├── herzen-llm       # LLM provider abstraction and model pool
├── herzen-router    # Intent routing and skill dispatch
├── herzen-server    # Axum-based API and WebSocket broadcast server
├── herzen-skills    # Semantic skill engine and intent matching
├── herzen-stt       # Speech-to-Text (STT) module
├── herzen-tts       # Text-to-Speech (TTS) module
└── herzen-vad       # Voice Activity Detection (VAD)
```

## 🚀 Technical Stack

- **Backend**: Rust (Tokio, Axum, Serde, Tracing, Anyhow)
- **Frontend**: Svelte, TypeScript, Vite, Tauri
- **AI/ML**: Local LLM Inference (GGUF), STT/TTS (Whisper/Piper variants), Semantic Embeddings
- **Infrastructure**: Home Assistant (WebSockets), WebSocket streaming, TOML/JSON Schema export

## 🧠 Design Philosophy

Herzen is built on the principle of **Local-First AI**. By keeping the entire pipeline—from speech recognition to large language model inference—within the local network, Herzen ensures maximum privacy and minimal latency. The architecture is deliberately modular, allowing for the seamless swapping of components (e.g., STT engines or LLM providers) without modifying the core orchestration logic.
