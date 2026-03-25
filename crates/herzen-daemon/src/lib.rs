//! Process supervisor, resource monitor, and graceful degradation.
//!
//! - Manages Python TTS sidecars (start/stop/restart/health)
//! - Memory pressure monitoring with graceful degradation
//! - Model eviction when memory is tight
//! - Startup sequencing
