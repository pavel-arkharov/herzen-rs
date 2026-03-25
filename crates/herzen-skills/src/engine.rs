use std::collections::HashMap;
use std::sync::Arc;

use crate::definition::Skill;
use crate::embedding::EmbeddingProvider;
use crate::matcher::{MatchMethod, SkillMatcher};

/// The skill engine: loads skills, matches input, returns the best result.
pub struct SkillEngine {
    matchers: Vec<SkillMatcher>,
    embedding_provider: Option<Arc<dyn EmbeddingProvider>>,
}

impl SkillEngine {
    /// Create a new engine with pre-loaded skills.
    pub async fn new(
        skills: Vec<Skill>,
        embedding_provider: Option<Arc<dyn EmbeddingProvider>>,
    ) -> Self {
        let mut matchers = Vec::with_capacity(skills.len());
        for skill in skills {
            let matcher = SkillMatcher::new(skill, embedding_provider.as_ref()).await;
            matchers.push(matcher);
        }

        tracing::info!("skill engine loaded {} skills", matchers.len());

        Self {
            matchers,
            embedding_provider,
        }
    }

    /// Match input against all skills. Returns the best match (if any).
    pub async fn match_input(&self, input: &str) -> Option<MatchResult> {
        // Pre-compute input embedding once (shared across all skill matchers)
        let input_embedding = match &self.embedding_provider {
            Some(provider) => provider.embed(input).await.ok(),
            None => None,
        };

        let mut best: Option<MatchResult> = None;

        for matcher in &self.matchers {
            let attempt = matcher
                .match_input(input, input_embedding.as_deref())
                .await;

            let gates = &matcher.skill.confidence;

            // Skip if below reject threshold
            if attempt.confidence < gates.reject_below {
                continue;
            }

            // Determine execution decision
            let decision = if attempt.method == MatchMethod::Keyword && attempt.confidence >= 0.90 {
                // Keyword match with all slots: always execute
                if gates.destructive {
                    ExecutionDecision::Confirm
                } else {
                    ExecutionDecision::Execute
                }
            } else if attempt.confidence >= gates.auto_execute {
                ExecutionDecision::Execute
            } else if attempt.confidence >= gates.confirm_above {
                ExecutionDecision::Confirm
            } else {
                continue; // Below confirm threshold
            };

            let result = MatchResult {
                skill_name: matcher.skill.skill.name.clone(),
                filled_slots: attempt.filled_slots,
                confidence: attempt.confidence,
                method: format!("{:?}", attempt.method),
                semantic_score: attempt.semantic_score,
                decision,
                skill: matcher.skill.clone(),
            };

            // Keep the highest confidence match
            match &best {
                Some(current) if current.confidence >= result.confidence => {}
                _ => best = Some(result),
            }
        }

        best
    }

    /// Match input and return ALL skill scores (for the Workshop Test Bench).
    pub async fn match_all(&self, input: &str) -> Vec<MatchResult> {
        let input_embedding = match &self.embedding_provider {
            Some(provider) => provider.embed(input).await.ok(),
            None => None,
        };

        let mut results = Vec::new();

        for matcher in &self.matchers {
            let attempt = matcher
                .match_input(input, input_embedding.as_deref())
                .await;

            let gates = &matcher.skill.confidence;
            let decision = if attempt.confidence >= gates.auto_execute {
                ExecutionDecision::Execute
            } else if attempt.confidence >= gates.confirm_above {
                ExecutionDecision::Confirm
            } else {
                ExecutionDecision::Reject
            };

            results.push(MatchResult {
                skill_name: matcher.skill.skill.name.clone(),
                filled_slots: attempt.filled_slots,
                confidence: attempt.confidence,
                method: format!("{:?}", attempt.method),
                semantic_score: attempt.semantic_score,
                decision,
                skill: matcher.skill.clone(),
            });
        }

        // Sort by confidence descending
        results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Number of loaded skills.
    pub fn skill_count(&self) -> usize {
        self.matchers.len()
    }
}

/// A single slot match with its extracted value.
#[derive(Debug, Clone)]
pub struct SlotMatch {
    pub slot_name: String,
    pub canonical_value: String,
}

/// Result of matching input against a skill.
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub skill_name: String,
    pub filled_slots: HashMap<String, String>,
    pub confidence: f32,
    pub method: String,
    pub semantic_score: Option<f32>,
    pub decision: ExecutionDecision,
    pub skill: Skill,
}

/// What the engine recommends doing with this match.
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionDecision {
    /// Confidence high enough to execute without asking.
    Execute,
    /// Confidence in the middle zone: ask user to confirm.
    Confirm,
    /// Below threshold: not a match.
    Reject,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::Skill;
    use crate::embedding::MockEmbeddingProvider;

    fn lights_skill() -> Skill {
        toml::from_str(r#"
            [skill]
            name = "lights"
            description = "Control lights"
            priority = 10

            [intent]
            required_slots = ["action", "target"]

            [intent.slots.action.keywords]
            turn_on = ["turn on", "включи", "зажги", "вруби"]
            turn_off = ["turn off", "выключи", "гаси"]

            [intent.slots.target.keywords]
            light = ["light", "lights", "свет", "лампу"]

            [confidence]
            destructive = true

            [[actions]]
            type = "homeassistant"
            entity = "light.bedroom"
            service = "toggle"

            [response]
            template = "Lights {{action}}."
            confirm_template = "Should I {{action}} the lights?"
        "#).unwrap()
    }

    fn weather_skill() -> Skill {
        toml::from_str(r#"
            [skill]
            name = "weather"
            description = "Check weather"

            [intent]
            required_slots = ["query"]

            [intent.slots.query.keywords]
            weather = ["weather", "forecast", "погода", "прогноз"]

            [intent.semantic]
            examples = ["what's the weather like", "какая сегодня погода"]
            threshold = 0.80

            [response]
            template = "Checking weather..."
        "#).unwrap()
    }

    #[tokio::test]
    async fn engine_matches_best_skill() {
        let provider: Arc<dyn EmbeddingProvider> = Arc::new(MockEmbeddingProvider::new(64));
        let engine = SkillEngine::new(
            vec![lights_skill(), weather_skill()],
            Some(provider),
        ).await;

        assert_eq!(engine.skill_count(), 2);

        let result = engine.match_input("гаси свет").await;
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.skill_name, "lights");
        assert_eq!(result.decision, ExecutionDecision::Confirm); // destructive = true
    }

    #[tokio::test]
    async fn engine_no_match_for_irrelevant() {
        let engine = SkillEngine::new(vec![lights_skill()], None).await;
        let result = engine.match_input("tell me a joke").await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn match_all_returns_all_scores() {
        let engine = SkillEngine::new(
            vec![lights_skill(), weather_skill()],
            None,
        ).await;

        let results = engine.match_all("гаси свет").await;
        assert_eq!(results.len(), 2);
        // Lights should be first (higher confidence)
        assert_eq!(results[0].skill_name, "lights");
    }

    #[tokio::test]
    async fn false_positive_rejected() {
        let engine = SkillEngine::new(vec![lights_skill()], None).await;
        let result = engine
            .match_input("what do I whisper to a woman's ear in bedroom")
            .await;
        assert!(result.is_none());
    }
}
