use std::{borrow::Cow, path::PathBuf};

use crate::{
    config::Config,
    error::{AppResult, CliError},
};

// Generated at build time by build.rs script
include!(concat!(env!("OUT_DIR"), "/embedding.rs"));

pub trait ConfigSource {
    fn load_config(&self) -> AppResult<Config>;
}

#[derive(Debug)]
pub struct FileConfigSource<'s> {
    pub path: PathBuf,
    pub key: &'s str,
}

impl ConfigSource for FileConfigSource<'_> {
    fn load_config(&self) -> AppResult<Config> {
        Config::from_file(&self.path)
    }
}

pub trait TemplateSource {
    fn load_template(&self) -> Result<Cow<'static, [u8]>, CliError>;
}

#[derive(Debug, Default)]
pub struct EmbeddedTemplateSource;

impl TemplateSource for EmbeddedTemplateSource {
    fn load_template(&self) -> Result<Cow<'static, [u8]>, CliError> {
        Ok(Cow::Borrowed(EMBEDDED_TEMPLATE))
    }
}

#[derive(Debug)]
pub struct RemoteTemplateSource<'s> {
    pub url: &'s str,
    pub path: &'s str,
    pub placeholder: &'s str,
}

impl<'s> RemoteTemplateSource<'s> {
    pub fn new(url: &'s str, path: &'s str, placeholder: &'s str) -> Self {
        Self {
            url,
            path,
            placeholder,
        }
    }
}

impl TemplateSource for RemoteTemplateSource<'_> {
    fn load_template(&self) -> Result<Cow<'static, [u8]>, CliError> {
        unimplemented!("Remote template source not implemented.")
    }
}

#[test]
fn test_embedded_template_source() {
    let source = EmbeddedTemplateSource::default();
    let result = source.load_template();
    assert!(result.is_ok());
    let bytes = result.unwrap();
    assert_eq!(matches!(bytes, Cow::Borrowed(_)), true);
    assert!(!bytes.is_empty(), "Template should not be empty");
}
