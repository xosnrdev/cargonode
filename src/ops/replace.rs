use regex::Regex;
use std::{borrow::Cow, collections::HashMap, sync::LazyLock};

//----------------------------------------------------------------------
// Constants
//----------------------------------------------------------------------

static PLACEHOLDER_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{\s*(\w+)\s*\}\}").expect("Failed to compile placeholder regex pattern")
});

//----------------------------------------------------------------------
// Types
//----------------------------------------------------------------------

/// Holds key-value pairs for placeholder replacements.
#[derive(Debug, Default)]
pub struct Replacements<'s> {
    /// Map of placeholder names to their replacement values
    replacements: HashMap<&'s str, &'s str>,
}

//----------------------------------------------------------------------
// Implementations
//----------------------------------------------------------------------

impl<'s> Replacements<'s> {
    /// Creates a new empty Replacements instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a replacement pair
    pub fn add(&mut self, k: &'s str, v: &'s str) -> &mut Self {
        self.replacements.insert(k, v);
        self
    }

    /// Gets a replacement value by key
    pub fn get(&self, k: &str) -> Option<&'s str> {
        self.replacements.get(k).copied()
    }
}

//----------------------------------------------------------------------
// Functions
//----------------------------------------------------------------------

/// Replaces placeholders in the template with the given replacements.
/// Logs a warning if a placeholder is not found in the replacements.
pub fn replace_placeholders<'s>(template: &'s str, rep: &Replacements) -> Cow<'s, str> {
    if template.is_empty() {
        return Cow::Borrowed(template);
    }

    PLACEHOLDER_PATTERN.replace_all(template, |caps: &regex::Captures| {
        let placeholder = &caps[1];
        rep.get(placeholder).unwrap_or_else(|| {
            log::warn!("No replacement found for placeholder '{}'", placeholder);
            "{{MISSING}}"
        })
    })
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_replacement() {
        // Arrange
        let mut rep = Replacements::new();
        rep.add("name", "test-package");

        // Act
        let result = replace_placeholders("Hello {{name}}", &rep);

        // Assert
        assert_eq!(result, "Hello test-package");
    }

    #[test]
    fn test_empty_template() {
        // Arrange
        let rep = Replacements::new();

        // Act
        let result = replace_placeholders("", &rep);

        // Assert
        assert_eq!(result, "");
    }

    #[test]
    fn test_missing_replacement() {
        // Arrange
        let rep = Replacements::new();

        // Act
        let result = replace_placeholders("Hello {{name}}", &rep);

        // Assert
        assert_eq!(result, "Hello {{MISSING}}");
    }
}
