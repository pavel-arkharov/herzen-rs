use std::collections::HashMap;
use std::sync::Arc;

use crate::definition::Skill;
use crate::embedding::{self, EmbeddingProvider};
use crate::keyword::KeywordMatcher;

/// Combined matcher for a single skill: keyword fast path + semantic embedding.
pub struct SkillMatcher {
    pub skill: Skill,
    keyword_matcher: KeywordMatcher,
    /// Pre-computed embeddings of semantic examples (computed at load time).
    example_embeddings: Vec<Vec<f32>>,
}

impl SkillMatcher {
    /// Create a matcher for a skill. Pre-computes semantic embeddings if
    /// an embedding provider is available and the skill has semantic examples.
    pub async fn new(
        skill: Skill,
        embedding_provider: Option<&Arc<dyn EmbeddingProvider>>,
    ) -> Self {
        let keyword_matcher = KeywordMatcher::from_skill(&skill);

        let example_embeddings = match (&skill.intent.semantic, embedding_provider) {
            (Some(semantic), Some(provider)) if !semantic.examples.is_empty() => {
                match provider.embed_batch(&semantic.examples).await {
                    Ok(embeddings) => {
                        tracing::debug!(
                            "precomputed {} embeddings for skill '{}'",
                            embeddings.len(),
                            skill.skill.name
                        );
                        embeddings
                    }
                    Err(e) => {
                        tracing::warn!(
                            "failed to embed examples for skill '{}': {}",
                            skill.skill.name,
                            e
                        );
                        Vec::new()
                    }
                }
            }
            _ => Vec::new(),
        };

        Self {
            skill,
            keyword_matcher,
            example_embeddings,
        }
    }

    /// Match input using the two-layer approach:
    /// 1. Keyword extraction (fast, deterministic)
    /// 2. Semantic similarity (if keywords didn't fill all required slots)
    ///
    /// Returns (filled_slots, confidence, match_method).
    pub async fn match_input(
        &self,
        input: &str,
        input_embedding: Option<&[f32]>,
    ) -> MatchAttempt {
        // Layer 1: Keyword extraction
        let filled_slots = self.keyword_matcher.extract_slots(input);
        let required = &self.skill.intent.required_slots;
        let all_filled = self.keyword_matcher.all_required_filled(&filled_slots, required);

        if all_filled {
            return MatchAttempt {
                filled_slots,
                confidence: 0.95, // Keywords are high confidence
                method: MatchMethod::Keyword,
                semantic_score: None,
            };
        }

        // Layer 2: Semantic matching
        let semantic_score = match (input_embedding, self.example_embeddings.is_empty()) {
            (Some(embedding), false) => {
                let (_best_idx, score) =
                    embedding::best_match(embedding, &self.example_embeddings);
                Some(score)
            }
            _ => None,
        };

        let threshold = self
            .skill
            .intent
            .semantic
            .as_ref()
            .map(|s| s.threshold)
            .unwrap_or(0.85);

        let confidence = match semantic_score {
            Some(score) if score >= threshold => {
                // Semantic match found, boost if some keyword slots also filled
                let keyword_bonus = if filled_slots.is_empty() { 0.0 } else { 0.05 };
                (score + keyword_bonus).min(1.0)
            }
            Some(score) => score, // Below threshold, but still report the score
            None => {
                // No embedding available — report keyword-only confidence
                if filled_slots.is_empty() {
                    0.0
                } else {
                    // Some slots filled but not all required
                    let filled_ratio =
                        filled_slots.len() as f32 / required.len().max(1) as f32;
                    filled_ratio * 0.6 // Partial keyword match, low confidence
                }
            }
        };

        let method = if semantic_score.is_some() {
            MatchMethod::Semantic
        } else if !filled_slots.is_empty() {
            MatchMethod::PartialKeyword
        } else {
            MatchMethod::None
        };

        MatchAttempt {
            filled_slots,
            confidence,
            method,
            semantic_score,
        }
    }
}

/// Result of a match attempt against a single skill.
#[derive(Debug, Clone)]
pub struct MatchAttempt {
    pub filled_slots: HashMap<String, String>,
    pub confidence: f32,
    pub method: MatchMethod,
    pub semantic_score: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchMethod {
    /// All required slots filled via keyword aliases.
    Keyword,
    /// Matched via semantic embedding similarity.
    Semantic,
    /// Some keyword slots filled but not all required.
    PartialKeyword,
    /// No match at all.
    None,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedding::MockEmbeddingProvider;

    fn test_skill_toml() -> Skill {
        let toml_str = r#"
            [skill]
            name = "lights"
            description = "Control lights"

            [intent]
            required_slots = ["action", "target"]

            [intent.slots.action.keywords]
            turn_on = ["turn on", "включи", "зажги"]
            turn_off = ["turn off", "выключи", "гаси"]

            [intent.slots.target.keywords]
            light = ["light", "lights", "свет", "лампу"]

            [intent.slots.location.keywords]
            bedroom = ["bedroom", "спальне"]

            [intent.semantic]
            examples = ["turn off the lights", "выключи свет", "kill the lights"]
            threshold = 0.80
        "#;
        toml::from_str(toml_str).unwrap()
    }

    #[tokio::test]
    async fn keyword_match_russian() {
        let skill = test_skill_toml();
        let provider: Arc<dyn EmbeddingProvider> = Arc::new(MockEmbeddingProvider::new(64));
        let matcher = SkillMatcher::new(skill, Some(&provider)).await;

        let result = matcher.match_input("гаси свет", None).await;
        assert_eq!(result.method, MatchMethod::Keyword);
        assert!(result.confidence >= 0.90);
        assert_eq!(result.filled_slots.get("action"), Some(&"turn_off".into()));
        assert_eq!(result.filled_slots.get("target"), Some(&"light".into()));
    }

    #[tokio::test]
    async fn no_match_irrelevant_input() {
        let skill = test_skill_toml();
        let matcher = SkillMatcher::new(skill, None).await;

        let result = matcher
            .match_input("what is the meaning of life", None)
            .await;
        assert_eq!(result.method, MatchMethod::None);
        assert!(result.confidence < 0.1);
    }

    #[tokio::test]
    async fn partial_match_location_only() {
        let skill = test_skill_toml();
        let matcher = SkillMatcher::new(skill, None).await;

        let result = matcher
            .match_input("what do I whisper in the bedroom", None)
            .await;
        assert_eq!(result.method, MatchMethod::PartialKeyword);
        // Location filled but not action/target → low confidence
        assert!(result.confidence < 0.70);
    }
}
