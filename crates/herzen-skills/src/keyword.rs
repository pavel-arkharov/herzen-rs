use std::collections::HashMap;

use crate::definition::Skill;

/// Fast keyword-based slot extraction. No model needed.
///
/// For each slot, builds a reverse index (keyword → canonical value)
/// and scans the input for matches. Multilingual by design —
/// Russian, English, or any language keywords all live in the same index.
pub struct KeywordMatcher {
    /// Per-slot reverse index: slot_name → (keyword_lowercase → canonical_value)
    slot_indices: HashMap<String, HashMap<String, String>>,
}

impl KeywordMatcher {
    /// Build from a skill's slot definitions.
    pub fn from_skill(skill: &Skill) -> Self {
        let mut slot_indices = HashMap::new();
        for (slot_name, slot_def) in &skill.intent.slots {
            slot_indices.insert(slot_name.clone(), slot_def.keyword_index());
        }
        Self { slot_indices }
    }

    /// Extract slots from input text. Returns filled slots as (slot_name → canonical_value).
    pub fn extract_slots(&self, input: &str) -> HashMap<String, String> {
        let input_lower = input.to_lowercase();
        let mut filled = HashMap::new();

        for (slot_name, index) in &self.slot_indices {
            // Try longest keywords first to avoid partial matches
            // e.g., "turn off" should match before "turn"
            let mut keywords: Vec<(&str, &str)> = index
                .iter()
                .map(|(kw, canon)| (kw.as_str(), canon.as_str()))
                .collect();
            keywords.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

            for (keyword, canonical) in keywords {
                if input_lower.contains(keyword) {
                    filled.insert(slot_name.clone(), canonical.to_string());
                    break; // First (longest) match wins for this slot
                }
            }
        }

        filled
    }

    /// Check if all required slots are filled.
    pub fn all_required_filled(
        &self,
        filled: &HashMap<String, String>,
        required: &[String],
    ) -> bool {
        required.iter().all(|r| filled.contains_key(r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::*;

    fn test_skill() -> Skill {
        let toml_str = r#"
            [skill]
            name = "lights"
            description = "Control lights"

            [intent]
            required_slots = ["action", "target"]

            [intent.slots.action]
            canonical = ["turn_on", "turn_off"]

            [intent.slots.action.keywords]
            turn_on = ["turn on", "switch on", "включи", "зажги", "вруби"]
            turn_off = ["turn off", "switch off", "kill", "выключи", "гаси", "потуши"]

            [intent.slots.target.keywords]
            light = ["light", "lights", "lamp", "свет", "лампу", "лампочку", "освещение"]

            [intent.slots.location.keywords]
            bedroom = ["bedroom", "спальня", "спальне"]
            kitchen = ["kitchen", "кухня", "кухне"]
        "#;
        toml::from_str(toml_str).unwrap()
    }

    #[test]
    fn english_turn_off_lights_bedroom() {
        let skill = test_skill();
        let matcher = KeywordMatcher::from_skill(&skill);
        let filled = matcher.extract_slots("turn off the lights in the bedroom");
        assert_eq!(filled.get("action"), Some(&"turn_off".to_string()));
        assert_eq!(filled.get("target"), Some(&"light".to_string()));
        assert_eq!(filled.get("location"), Some(&"bedroom".to_string()));
    }

    #[test]
    fn russian_gasi_svet() {
        let skill = test_skill();
        let matcher = KeywordMatcher::from_skill(&skill);
        let filled = matcher.extract_slots("Ассистент, гаси свет");
        assert_eq!(filled.get("action"), Some(&"turn_off".to_string()));
        assert_eq!(filled.get("target"), Some(&"light".to_string()));
        assert_eq!(filled.get("location"), None); // No room specified
    }

    #[test]
    fn russian_vkluchi_svet_v_spalne() {
        let skill = test_skill();
        let matcher = KeywordMatcher::from_skill(&skill);
        let filled = matcher.extract_slots("включи свет в спальне");
        assert_eq!(filled.get("action"), Some(&"turn_on".to_string()));
        assert_eq!(filled.get("target"), Some(&"light".to_string()));
        assert_eq!(filled.get("location"), Some(&"bedroom".to_string()));
    }

    #[test]
    fn false_positive_bedroom_no_action() {
        let skill = test_skill();
        let matcher = KeywordMatcher::from_skill(&skill);
        let filled =
            matcher.extract_slots("what do I whisper to a woman's ear in bedroom?");
        // "bedroom" matches location, but no action or target → required slots not filled
        assert_eq!(filled.get("action"), None);
        assert_eq!(filled.get("target"), None);
        assert_eq!(filled.get("location"), Some(&"bedroom".to_string()));
        assert!(!matcher.all_required_filled(&filled, &["action".into(), "target".into()]));
    }

    #[test]
    fn longest_keyword_wins() {
        let skill = test_skill();
        let matcher = KeywordMatcher::from_skill(&skill);
        // "turn off" should match as a unit, not "turn" separately
        let filled = matcher.extract_slots("turn off the lamp");
        assert_eq!(filled.get("action"), Some(&"turn_off".to_string()));
    }

    #[test]
    fn required_slots_check() {
        let skill = test_skill();
        let matcher = KeywordMatcher::from_skill(&skill);
        let filled = matcher.extract_slots("гаси свет");
        assert!(matcher.all_required_filled(&filled, &["action".into(), "target".into()]));

        let partial = matcher.extract_slots("что-то в спальне");
        assert!(!matcher.all_required_filled(&partial, &["action".into(), "target".into()]));
    }
}
