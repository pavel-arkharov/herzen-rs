use std::path::Path;

use crate::definition::Skill;

/// Load all skill TOML files from a directory.
pub fn load_skills_from_dir(dir: &Path) -> anyhow::Result<Vec<Skill>> {
    let pattern = dir.join("*.toml").to_string_lossy().to_string();
    let mut skills = Vec::new();

    for entry in glob::glob(&pattern)? {
        let path = entry?;
        tracing::debug!("loading skill from {}", path.display());

        let content = std::fs::read_to_string(&path)?;
        match toml::from_str::<Skill>(&content) {
            Ok(mut skill) => {
                if skill.skill.enabled {
                    tracing::info!("loaded skill: {} (priority: {})", skill.skill.name, skill.skill.priority);
                    // Ensure name matches filename if not explicitly set differently
                    if skill.skill.name.is_empty() {
                        skill.skill.name = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("unnamed")
                            .to_string();
                    }
                    skills.push(skill);
                } else {
                    tracing::debug!("skipping disabled skill: {}", skill.skill.name);
                }
            }
            Err(e) => {
                tracing::warn!("failed to parse {}: {}", path.display(), e);
            }
        }
    }

    // Sort by priority (higher first)
    skills.sort_by(|a, b| b.skill.priority.cmp(&a.skill.priority));

    Ok(skills)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn load_from_dir() {
        let dir = TempDir::new().unwrap();
        let skill_toml = r#"
            [skill]
            name = "test"
            description = "A test skill"

            [intent]
            required_slots = ["action"]

            [intent.slots.action.keywords]
            greet = ["hello", "hi"]
        "#;

        fs::write(dir.path().join("test.toml"), skill_toml).unwrap();
        fs::write(dir.path().join("not_toml.txt"), "garbage").unwrap();

        let skills = load_skills_from_dir(dir.path()).unwrap();
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].skill.name, "test");
    }

    #[test]
    fn disabled_skills_skipped() {
        let dir = TempDir::new().unwrap();
        let skill_toml = r#"
            [skill]
            name = "disabled"
            description = "Should be skipped"
            enabled = false

            [intent]
        "#;
        fs::write(dir.path().join("disabled.toml"), skill_toml).unwrap();

        let skills = load_skills_from_dir(dir.path()).unwrap();
        assert_eq!(skills.len(), 0);
    }

    #[test]
    fn skills_sorted_by_priority() {
        let dir = TempDir::new().unwrap();

        let low = r#"
            [skill]
            name = "low"
            description = "Low priority"
            priority = 1
            [intent]
        "#;
        let high = r#"
            [skill]
            name = "high"
            description = "High priority"
            priority = 100
            [intent]
        "#;

        fs::write(dir.path().join("low.toml"), low).unwrap();
        fs::write(dir.path().join("high.toml"), high).unwrap();

        let skills = load_skills_from_dir(dir.path()).unwrap();
        assert_eq!(skills[0].skill.name, "high");
        assert_eq!(skills[1].skill.name, "low");
    }
}
