use std::{borrow::Cow, collections::HashMap, sync::LazyLock};

use regex::Regex;

static PLACEHOLDER_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{\s*(\w+)\s*\}\}").expect("Failed to compile placeholder pattern")
});

#[derive(Debug, Default)]
pub struct Replacer<'a> {
    replacement: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'a> Replacer<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.replacement.insert(key.into(), val.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.replacement.get(key).map(|cow| cow.as_ref())
    }

    pub fn with_haystack(&self, haystack: &'a str) -> Cow<'a, str> {
        if haystack.is_empty() {
            return Cow::Borrowed(haystack);
        }
        PLACEHOLDER_PATTERN.replace_all(haystack, |caps: &regex::Captures| {
            let placeholder = &caps[1];
            self.get(placeholder).unwrap_or("{{MISSING}}")
        })
    }
}

#[test]
fn test_replace_placeholders() {
    let mut rep = Replacer::new();
    rep.add("name", "cargonode");
    rep.add("version", "0.1.0");

    let haystack = "name: {{ name }}\nversion: {{ version }}";
    let expected = "name: cargonode\nversion: 0.1.0";
    assert_eq!(rep.with_haystack(haystack), expected);
}

#[test]
fn test_replace_placeholders_missing() {
    let rep = Replacer::new();
    let haystack = "name: {{ name }}\nversion: {{ version }}";
    let expected = "name: {{MISSING}}\nversion: {{MISSING}}";
    assert_eq!(rep.with_haystack(haystack), expected);
}

#[test]
fn test_replace_placeholders_empty() {
    let rep = Replacer::new();
    let haystack = "";
    let expected = "";
    assert_eq!(rep.with_haystack(haystack), expected);
}
