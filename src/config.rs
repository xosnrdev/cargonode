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
            cmd::from_default("npx", "tsup", &["src/main.js"], None, vec![Job::Check]),
        );
        cargonode.insert(
            Job::Check,
            cmd::from_default("npx", "biome", &["check"], None, Vec::new()),
        );
        cargonode.insert(
            Job::Fmt,
            cmd::from_default("npx", "biome", &["format"], None, Vec::new()),
        );
        cargonode.insert(
            Job::Release,
            cmd::from_default("npx", "release-it", &[], None, vec![Job::Build]),
        );
        cargonode.insert(
            Job::Run,
            cmd::from_default("node", "main.js", &[], Some("dist"), vec![Job::Build]),
        );
        cargonode.insert(
            Job::Test,
            cmd::from_default("npx", "vitest", &[], None, vec![Job::Check]),
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
        let mut contents = String::with_capacity(1024);
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
    use std::{
        env,
        io::{Cursor, Write},
        path::PathBuf,
    };
    use tempfile::NamedTempFile;

    use super::*;

    fn create_temp_json_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temporary file");
        write!(file, "{}", content).expect("Failed to write to temporary file");
        file
    }

    #[test]
    fn test_config_from_default() {
        // Arrange
        let config = Config::from_default();
        // Assert
        assert!(config.cargonode.contains_key(&Job::Build));
        assert!(config.cargonode.contains_key(&Job::Check));
        assert!(config.cargonode.contains_key(&Job::Fmt));
        assert!(config.cargonode.contains_key(&Job::Release));
        assert!(config.cargonode.contains_key(&Job::Run));
        assert!(config.cargonode.contains_key(&Job::Test));
        // Arrange
        let build_context = config.cargonode.get(&Job::Build).unwrap();
        // Assert
        assert_eq!(build_context.executable, PathBuf::from("npx"));
        assert_eq!(build_context.subcommand, "tsup");
        assert_eq!(build_context.args, vec!["src/main.js"]);
        assert_eq!(build_context.working_dir, env::current_dir().unwrap());
        assert_eq!(build_context.steps, vec![Job::Check]);
    }

    #[test]
    fn test_config_merge() {
        // Arrange
        let mut config1 = Config::from_default();
        let mut config2 = Config::default();
        config2.cargonode.insert(
            Job::Check,
            cmd::from_default("npx", "eslint", &["src"], None, Vec::new()),
        );
        // Act
        config1.merge(config2);
        // Assert
        assert!(config1.cargonode.contains_key(&Job::Check));
        assert!(config1.cargonode.contains_key(&Job::Build));
    }

    #[test]
    fn test_config_with_file_valid() {
        // Arrange
        let json_content = r#"
        {
            "cargonode": {
                "Build": {
                    "executable": "npx",
                    "subcommand": "tsup",
                    "args": ["src/main.js"],
                    "working-dir": ".",
                    "steps": ["Check"]
                }
            }
        }"#;
        let file = create_temp_json_file(json_content);
        // Act
        let config = Config::with_file(file.path()).expect("Failed to load config from file");
        // Assert
        assert!(config.cargonode.contains_key(&Job::Build));
        let build_context = config.cargonode.get(&Job::Build).unwrap();
        assert_eq!(build_context.executable, PathBuf::from("npx"));
        assert_eq!(build_context.subcommand, "tsup");
        assert_eq!(build_context.args, vec!["src/main.js"]);
    }

    #[test]
    fn test_config_with_file_invalid_path() {
        // Arrange
        let invalid_path = Path::new("nonexistent_file.json");
        // Act
        let result = Config::with_file(invalid_path);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_config_with_file_malformed_config() {
        // Arrange
        let malformed_json = r#"
        {
            "cargonode": {
                "Build": {
                    "executable": "npx",
                    "subcommand": "tsup",
                    "args": ["src/main.js"],
                    "working_dir": ".",
                    "steps": ["check"]
                }
            }
        }
        "#;
        let file = create_temp_json_file(malformed_json);
        // Act
        let result = Config::with_file(file.path());
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_config_with_reader_valid() {
        // Arrange
        let json_content = r#"
        {
            "cargonode": {
                "build": {
                    "executable": "npx",
                    "subcommand": "tsup",
                    "args": ["src/main.js"],
                    "working-dir": ".",
                    "steps": ["check"]
                }
            }
        }"#;
        let reader = Cursor::new(json_content);
        // Act
        let config = Config::with_reader(reader).expect("Failed to load config from reader");
        // Assert
        assert!(config.cargonode.contains_key(&Job::Build));
        let build_context = config.cargonode.get(&Job::Build).unwrap();
        assert_eq!(build_context.executable, PathBuf::from("npx"));
        assert_eq!(build_context.subcommand, "tsup");
        assert_eq!(build_context.args, vec!["src/main.js"]);
    }

    #[test]
    fn test_config_with_reader_malformed_config() {
        // Arrange
        let malformed_json = r#"
        {
            "cargonode": {
                "Build": {
                    "executable": "npx",
                    "subcommand": "tsup",
                    "args": ["src/main.js"],
                    "working_dir": ".",
                    "steps": ["check"]
                }
            }
        }
        "#;
        let reader = Cursor::new(malformed_json);
        // Act
        let result = Config::with_reader(reader);
        // Assert
        assert!(result.is_err());
    }
}
