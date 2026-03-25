//! Centralized typed configuration for Herzen.
//!
//! Replaces environment variables with a single `herzen.toml` file.
//! Provides JSON Schema export for dynamic UI form generation.

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!version().is_empty());
    }
}
