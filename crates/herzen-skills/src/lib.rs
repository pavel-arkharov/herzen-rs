//! Skill engine with multilingual intent matching.
//!
//! Skills are declarative units: when input matches an intent (via multilingual
//! embeddings or keyword aliases), execute an action pipeline, return a response.
//!
//! Matching layers (in order):
//! 1. Keyword alias scan (fast, deterministic, multilingual)
//! 2. Multilingual semantic embedding (cosine similarity)
//! 3. Fall through to general dialog if no skill matches

mod definition;
mod embedding;
mod engine;
mod keyword;
mod loader;
mod matcher;

pub use definition::{
    Action, ActionType, ConfidenceGates, Intent, Skill, SkillResponse, SlotDefinition, SlotKeywords,
};
pub use embedding::EmbeddingProvider;
pub use engine::{MatchResult, SkillEngine, SlotMatch};
pub use loader::load_skills_from_dir;
pub use matcher::SkillMatcher;
