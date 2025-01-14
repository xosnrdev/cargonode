use std::path::PathBuf;

use crate::{config::Config, error::AppResult};

// Access our embedded template module.
// Generated at build time by `build.rs` script.
include!(concat!(env!("OUT_DIR"), "/embedded_template.rs"));

//----------------------------------------------------------------------
// Traits
//----------------------------------------------------------------------

/// Defines a source for loading configuration data.
pub trait ConfigSource {
    /// Loads configuration from the source
    fn load_config(&self) -> AppResult<Config>;
}

/// Defines a source for loading template data.
pub trait TemplateSource {
    /// Loads the template data from the source
    fn load_template(&self) -> AppResult<Vec<u8>>;
}

//----------------------------------------------------------------------
// Types
//----------------------------------------------------------------------

/// Configuration source that loads from a file.
#[derive(Debug)]
pub struct FileConfigSource<'s> {
    /// Path to the configuration file.
    pub path: PathBuf,
    /// Key to identify the configuration section.
    pub key: &'s str,
}

/// Template source that loads from embedded data.
#[derive(Debug, Default)]
pub struct EmbeddedTemplateSource;

/// Template source that loads from a remote location.
#[derive(Debug)]
pub struct RemoteTemplateSource<'s> {
    /// URL of the remote template.
    pub url: &'s str,
    /// Local path for caching.
    pub path: &'s str,
    /// Placeholder for template customization.
    pub placeholder: &'s str,
}

//----------------------------------------------------------------------
// Implementations
//----------------------------------------------------------------------

impl ConfigSource for FileConfigSource<'_> {
    fn load_config(&self) -> AppResult<Config> {
        Config::from_config_file(&self.path, self.key)
    }
}

impl TemplateSource for EmbeddedTemplateSource {
    fn load_template(&self) -> AppResult<Vec<u8>> {
        Ok(EMBEDDED_TEMPLATE.to_vec())
    }
}

impl<'s> RemoteTemplateSource<'s> {
    /// Creates a new RemoteTemplateSource
    pub fn new(url: &'s str, path: &'s str, placeholder: &'s str) -> Self {
        Self {
            url,
            path,
            placeholder,
        }
    }
}

impl TemplateSource for RemoteTemplateSource<'_> {
    fn load_template(&self) -> AppResult<Vec<u8>> {
        // TODO: Flesh out remote fetching logic here.
        unimplemented!("Remote template source not implemented.")
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[test]
fn test_embedded_template_source() {
    // Arrange
    let source = EmbeddedTemplateSource::default();

    // Act
    let result = source.load_template();

    // Assert
    assert!(result.is_ok(), "Should load embedded template");
    assert!(!result.unwrap().is_empty(), "Template should not be empty");
}
