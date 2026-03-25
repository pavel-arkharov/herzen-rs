use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete skill definition, loaded from TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub skill: SkillMeta,
    pub intent: Intent,
    #[serde(default)]
    pub confidence: ConfidenceGates,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default)]
    pub response: Option<SkillResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMeta {
    pub name: String,
    pub description: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub priority: i32,
}

/// Intent definition with required slots and semantic examples.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    #[serde(default)]
    pub required_slots: Vec<String>,
    #[serde(default)]
    pub slots: HashMap<String, SlotDefinition>,
    #[serde(default)]
    pub semantic: Option<SemanticConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotDefinition {
    /// Canonical values this slot can take
    #[serde(default)]
    pub canonical: Vec<String>,
    /// Keyword aliases per canonical value, multilingual
    #[serde(default)]
    pub keywords: HashMap<String, Vec<String>>,
    #[serde(default = "default_true")]
    pub required: bool,
    /// Default value if slot is not filled but not required
    pub default: Option<String>,
}

impl SlotDefinition {
    /// Build a reverse lookup: keyword → canonical value (lowercased).
    pub fn keyword_index(&self) -> HashMap<String, String> {
        let mut index = HashMap::new();
        for (canonical, keywords) in &self.keywords {
            // The canonical value itself is also a keyword
            index.insert(canonical.to_lowercase(), canonical.clone());
            for kw in keywords {
                index.insert(kw.to_lowercase(), canonical.clone());
            }
        }
        index
    }
}

/// Semantic matching configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticConfig {
    /// Example utterances in any language — embedded at load time.
    pub examples: Vec<String>,
    /// Cosine similarity threshold.
    #[serde(default = "default_threshold")]
    pub threshold: f32,
    /// Embedding model to use (default: multilingual-e5-small).
    #[serde(default = "default_embedding_model")]
    pub model: String,
}

/// Confidence thresholds for execution gating.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceGates {
    /// Above this: execute immediately without confirmation.
    #[serde(default = "default_auto_execute")]
    pub auto_execute: f32,
    /// Between confirm_above and auto_execute: ask user to confirm.
    #[serde(default = "default_confirm_above")]
    pub confirm_above: f32,
    /// Below this: not a match, fall through.
    #[serde(default = "default_reject_below")]
    pub reject_below: f32,
    /// Whether this skill modifies external state (HA, shell, etc.)
    #[serde(default)]
    pub destructive: bool,
}

impl Default for ConfidenceGates {
    fn default() -> Self {
        Self {
            auto_execute: default_auto_execute(),
            confirm_above: default_confirm_above(),
            reject_below: default_reject_below(),
            destructive: false,
        }
    }
}

/// An action in the skill's execution pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: ActionType,
    /// Action-specific parameters (entity, service, model, url, command, etc.)
    #[serde(flatten)]
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    HomeAssistant,
    Llm,
    Http,
    Shell,
    Tts,
    Chain,
}

/// Response template for the skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillResponse {
    /// Template with `{{slot_name}}` placeholders.
    pub template: String,
    /// Template for confirmation prompt.
    pub confirm_template: Option<String>,
    /// Override TTS provider for this response.
    pub tts_provider: Option<String>,
    /// Override TTS style for this response.
    pub tts_style: Option<String>,
}

// --- Serde defaults ---

fn default_true() -> bool {
    true
}

fn default_threshold() -> f32 {
    0.85
}

fn default_embedding_model() -> String {
    "multilingual-e5-small".into()
}

fn default_auto_execute() -> f32 {
    0.90
}

fn default_confirm_above() -> f32 {
    0.70
}

fn default_reject_below() -> f32 {
    0.70
}

/// Convenience type alias for slot keyword mappings.
pub type SlotKeywords = HashMap<String, Vec<String>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_skill_toml() {
        let toml_str = r#"
            [skill]
            name = "lights"
            description = "Control lights"
            priority = 10

            [intent]
            required_slots = ["action", "target"]

            [intent.slots.action]
            canonical = ["turn_on", "turn_off", "dim"]
            required = true

            [intent.slots.action.keywords]
            turn_on = ["turn on", "switch on", "включи", "зажги", "вруби"]
            turn_off = ["turn off", "switch off", "выключи", "гаси", "потуши"]
            dim = ["dim", "приглуши"]

            [intent.slots.target.keywords]
            light = ["light", "lights", "lamp", "свет", "лампу", "лампочку"]

            [intent.semantic]
            examples = [
                "turn off the lights",
                "выключи свет",
                "kill the lights",
            ]
            threshold = 0.85

            [confidence]
            auto_execute = 0.92
            destructive = true

            [[actions]]
            type = "homeassistant"
            entity = "light.bedroom"
            service = "toggle"

            [response]
            template = "{{action}} the lights."
            confirm_template = "Should I {{action}} the lights?"
        "#;

        let skill: Skill = toml::from_str(toml_str).unwrap();
        assert_eq!(skill.skill.name, "lights");
        assert_eq!(skill.intent.required_slots, vec!["action", "target"]);
        assert_eq!(skill.intent.slots.len(), 2);
        assert!(skill.confidence.destructive);
        assert_eq!(skill.actions.len(), 1);
        assert_eq!(skill.actions[0].action_type, ActionType::HomeAssistant);

        // Test keyword index
        let action_slot = &skill.intent.slots["action"];
        let index = action_slot.keyword_index();
        assert_eq!(index.get("гаси"), Some(&"turn_off".to_string()));
        assert_eq!(index.get("включи"), Some(&"turn_on".to_string()));
        assert_eq!(index.get("dim"), Some(&"dim".to_string()));
    }

    #[test]
    fn default_confidence_gates() {
        let gates = ConfidenceGates::default();
        assert_eq!(gates.auto_execute, 0.90);
        assert_eq!(gates.confirm_above, 0.70);
        assert!(!gates.destructive);
    }
}
