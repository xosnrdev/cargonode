use std::path::PathBuf;

use crate::{config::Config, error::AppResult};

//----------------------------------------------------------------------
// Traits
//----------------------------------------------------------------------

pub trait ConfigSource {
    fn load_config(&self) -> AppResult<Config>;
}

//----------------------------------------------------------------------
// Types
//----------------------------------------------------------------------

pub struct FileConfigSource<'s> {
    pub path: PathBuf,
    pub key: &'s str,
}

/// Future implementation for a remote configuration source.
pub struct RemoteConfigSource {
    pub url: String,
}

//----------------------------------------------------------------------
// Implementations
//----------------------------------------------------------------------

impl ConfigSource for FileConfigSource<'_> {
    fn load_config(&self) -> AppResult<Config> {
        Config::from_config_file(&self.path, self.key)
    }
}

impl ConfigSource for RemoteConfigSource {
    fn load_config(&self) -> AppResult<Config> {
        // Flesh out remote fetching logic here.
        // For now, we'll return an unimplemented error.
        anyhow::bail!("Remote configuration source not implemented.")
    }
}
