use regex::Regex;
use std::{borrow::Cow, collections::HashMap, sync::LazyLock};

static PLACEHOLDER_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{\s*(\w+)\s*\}\}").expect("Failed to compile placeholder regex pattern")
});

#[derive(Debug, Default)]
pub struct Replacements<'a> {
    replacements: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'a> Replacements<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.replacements.insert(key.into(), val.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.replacements.get(key).map(|cow| cow.as_ref())
    }
}

pub fn replace_placeholders<'s>(template: &'s str, rep: &Replacements<'s>) -> Cow<'s, str> {
    if template.is_empty() {
        return Cow::Borrowed(template);
    }
    PLACEHOLDER_PATTERN.replace_all(template, |caps: &regex::Captures| {
        let placeholder = &caps[1];
        rep.get(placeholder).unwrap_or("{{MISSING}}")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_replacement() {
        let mut rep = Replacements::new();
        rep.add("name", "test-package");
        let result = replace_placeholders("Hello {{name}}", &rep);
        assert_eq!(result, "Hello test-package");
    }

    #[test]
    fn test_empty_template() {
        let rep = Replacements::new();
        let result = replace_placeholders("", &rep);
        assert_eq!(result, "");
    }

    #[test]
    fn test_missing_replacement() {
        let rep = Replacements::new();
        let result = replace_placeholders("Hello {{name}}", &rep);
        assert_eq!(result, "Hello {{MISSING}}");
    }
}
