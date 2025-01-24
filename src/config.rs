use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    cmd::{self, CommandContext},
    error::AppResult,
    job::Job,
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Config {
    pub cargonode: HashMap<Job, CommandContext>,
}

impl Config {
    pub fn from_default() -> Self {
        let mut cargonode = HashMap::with_capacity(6);
        cargonode.insert(
            Job::Build,
            cmd::from_default("npx", "tsup", &["src/main.js"], ".", &["check"]),
        );
        cargonode.insert(
            Job::Check,
            cmd::from_default("npx", "biome", &["check"], ".", &[]),
        );
        cargonode.insert(
            Job::Fmt,
            cmd::from_default("npx", "biome", &["format"], ".", &[]),
        );
        cargonode.insert(
            Job::Release,
            cmd::from_default("npx", "release-it", &[""], ".", &["build"]),
        );
        cargonode.insert(
            Job::Run,
            cmd::from_default("node", "main.js", &[""], "dist", &["build"]),
        );
        cargonode.insert(
            Job::Test,
            cmd::from_default("npx", "vitest", &[""], ".", &["check"]),
        );
        Self { cargonode }
    }

    pub fn merge(&mut self, other: Config) {
        for (job, ctx) in other.cargonode {
            self.cargonode.insert(job, ctx);
        }
    }

    pub fn with_file(path: &Path) -> AppResult<Config> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open configuration file: {}", path.display()))?;
        let reader = BufReader::new(file);
        Self::with_reader(reader)
    }

    fn with_reader<R: Read>(mut reader: R) -> AppResult<Config> {
        let mut contents = String::new();
        reader
            .read_to_string(&mut contents)
            .context("Failed to read configuraton file")?;
        let config =
            serde_json::from_str(&contents).context("Failed to parse configuration file")?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_with_default_config() {
        let config = Config::from_default();
        assert_eq!(config.cargonode.len(), 6);
    }

    #[test]
    fn test_with_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("cargonode.json");
        let config = Config::from_default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&file_path, json).unwrap();

        let loaded_config = Config::with_file(&file_path).unwrap();
        assert_eq!(loaded_config.cargonode.len(), 6);
    }

    #[test]
    fn test_with_invalid_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("cargonode.json");
        fs::write(&file_path, "invalid json").unwrap();

        let result = Config::with_file(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_merge() {
        let mut config = Config::from_default();
        let mut other = Config::from_default();
        other.cargonode.remove(&Job::Build);
        other.cargonode.insert(
            Job::Build,
            cmd::from_default("npx", "tsup", &["src/main.ts"], ".", &["check"]),
        );

        config.merge(other);
        assert_eq!(config.cargonode.len(), 6);
        assert_eq!(
            config.cargonode.get(&Job::Build).unwrap().args,
            vec!["src/main.ts"]
        );
    }
}
