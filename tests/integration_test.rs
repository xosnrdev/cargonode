use std::collections::HashMap;

use cargonode::{Command, CommandConfig, Config};

fn default_config() -> CommandConfig {
    CommandConfig {
        command: "biome".to_string(),
        args: vec!["check".to_string()],
        pre_checks: vec!["validate".to_string()],
        env_vars: HashMap::from([("ENV".to_string(), "production".to_string())]),
    }
}

#[test]
fn test_command_parsing() {
    let valid_commands = ["format", "check", "build", "test", "release"];
    for cmd in valid_commands {
        assert!(
            Command::map_from_str(cmd).is_some(),
            "Failed to parse: {}",
            cmd
        );
    }
    assert!(
        Command::map_from_str("invalid").is_none(),
        "Invalid command parsed unexpectedly"
    );
}

#[test]
fn test_merge_with_full_file_config() {
    let mut config = Config::default();
    config.commands.insert(
        "build".to_string(),
        CommandConfig {
            command: "custom_command".to_string(),
            args: vec!["custom_arg".to_string()],
            pre_checks: vec!["custom_check".to_string()],
            env_vars: HashMap::from([
                ("LOG_LEVEL".to_string(), "debug".to_string()),
                // Overrides default
                ("ENV".to_string(), "development".to_string()),
            ]),
        },
    );

    let default = default_config();
    let merged = config.merge(&default, "build");

    assert_eq!(merged.command, "custom_command");
    assert_eq!(merged.args, vec!["custom_arg"]);
    assert_eq!(merged.pre_checks, vec!["custom_check"]);
    assert_eq!(
        merged.env_vars,
        HashMap::from([
            ("LOG_LEVEL".to_string(), "debug".to_string()),
            ("ENV".to_string(), "development".to_string()),
        ])
    );
}

#[test]
fn test_merge_with_partial_file_config() {
    let mut config = Config::default();
    config.commands.insert(
        "test".to_string(),
        CommandConfig {
            // Should fallback to default
            command: "".to_string(),
            // Override
            args: vec!["test_arg".to_string()],
            // Fallback to default
            pre_checks: vec![],
            env_vars: HashMap::from([("LOG_LEVEL".to_string(), "debug".to_string())]),
        },
    );

    let default = default_config();
    let merged = config.merge(&default, "test");

    assert_eq!(merged.command, "biome");
    assert_eq!(merged.args, vec!["test_arg"]);
    assert_eq!(merged.pre_checks, vec!["validate"]);
    assert_eq!(
        merged.env_vars,
        HashMap::from([
            // Default retained
            ("ENV".to_string(), "production".to_string()),
            // New added
            ("LOG_LEVEL".to_string(), "debug".to_string()),
        ])
    );
}

#[test]
fn test_merge_with_no_file_config() {
    let config = Config::default();
    let default = default_config();
    let merged = config.merge(&default, "nonexistent");

    assert_eq!(merged, default, "Should default to default_config");
}

#[test]
fn test_merge_with_empty_file_config() {
    let mut config = Config::default();
    config
        .commands
        .insert("build".to_string(), CommandConfig::default());

    let default = default_config();
    let merged = config.merge(&default, "build");

    assert_eq!(
        merged, default,
        "Should fallback entirely to default_config"
    );
}

#[test]
fn test_environment_variable_merging() {
    let default_config = CommandConfig {
        command: "default-cmd".to_string(),
        args: vec![],
        pre_checks: vec![],
        env_vars: HashMap::from([("DEFAULT_VAR".to_string(), "default".to_string())]),
    };

    let mut config = Config::default();
    config.commands.insert(
        "test".to_string(),
        CommandConfig {
            command: "test-cmd".to_string(),
            env_vars: HashMap::from([("TEST_VAR".to_string(), "custom".to_string())]),
            ..Default::default()
        },
    );

    let merged_config = config.merge(&default_config, "test");

    assert_eq!(
        merged_config.env_vars,
        HashMap::from([
            ("DEFAULT_VAR".to_string(), "default".to_string()),
            ("TEST_VAR".to_string(), "custom".to_string()),
        ])
    );
}

#[test]
fn test_default_command_configurations() {
    let default_tests = [
        (Command::Format, "biome", vec!["format"]),
        (Command::Check, "biome", vec!["check"]),
        (Command::Build, "tsup", vec![""]),
        (Command::Test, "vitest", vec![""]),
        (Command::Release, "release-it", vec![""]),
    ];

    for (cmd, expected_cmd, expected_args) in default_tests {
        let default_config = cmd.default_config();
        assert_eq!(default_config.command, expected_cmd);
        assert_eq!(default_config.args, expected_args);
    }
}

#[test]
fn test_pre_check_configurations() {
    let pre_check_tests = [
        (Command::Build, vec!["check"]),
        (Command::Test, vec!["check"]),
        (Command::Release, vec!["build"]),
    ];

    for (cmd, expected_checks) in pre_check_tests {
        let config = cmd.default_config();
        assert_eq!(config.pre_checks, expected_checks);
    }
}

#[test]
fn test_merge_with_empty_or_whitespace_vectors() {
    let default = CommandConfig {
        command: "biome".to_string(),
        args: vec!["check".to_string()],
        pre_checks: vec!["validate".to_string()],
        env_vars: HashMap::from([("ENV".to_string(), "production".to_string())]),
    };

    let mut config = Config::default();
    config.commands.insert(
        "test".to_string(),
        CommandConfig {
            // Should fallback to default
            command: "  ".to_string(),
            // Should fallback to default
            args: vec!["  ".to_string()],
            // Should fallback to default
            pre_checks: vec![],
            env_vars: HashMap::new(),
        },
    );

    let merged = config.merge(&default, "test");

    assert_eq!(merged.command, "biome");
    assert_eq!(merged.args, vec!["check"]);
    assert_eq!(merged.pre_checks, vec!["validate"]);
    assert_eq!(
        merged.env_vars,
        HashMap::from([("ENV".to_string(), "production".to_string())])
    );
}

#[test]
fn test_merge_with_partial_whitespace_vectors() {
    let default = CommandConfig {
        command: "biome".to_string(),
        args: vec!["check".to_string()],
        pre_checks: vec!["validate".to_string()],
        env_vars: HashMap::from([("ENV".to_string(), "production".to_string())]),
    };

    let mut config = Config::default();
    config.commands.insert(
        "build".to_string(),
        CommandConfig {
            command: "custom-cmd".to_string(),
            // Mixed, only valid-arg retained
            args: vec!["valid-arg".to_string(), "  ".to_string()],
            // Mixed, only custom-check retained
            pre_checks: vec!["".to_string(), "custom-check".to_string()],
            env_vars: HashMap::new(),
        },
    );

    let merged = config.merge(&default, "build");

    assert_eq!(merged.command, "custom-cmd");
    assert_eq!(merged.args, vec!["valid-arg"]);
    assert_eq!(merged.pre_checks, vec!["custom-check"]);
    assert_eq!(
        merged.env_vars,
        HashMap::from([("ENV".to_string(), "production".to_string())])
    );
}

#[test]
fn test_merge_with_empty_whitespace_and_non_empty_vectors() {
    let default = CommandConfig {
        command: "default-cmd".to_string(),
        args: vec!["default-arg".to_string()],
        pre_checks: vec!["default-check".to_string()],
        env_vars: HashMap::from([("DEFAULT_ENV".to_string(), "default".to_string())]),
    };

    let mut config = Config::default();
    config.commands.insert(
        "release".to_string(),
        CommandConfig {
            command: "release-cmd".to_string(),
            // Mixed
            args: vec!["   ".to_string(), "release-arg".to_string()],
            // Mixed
            pre_checks: vec!["release-check".to_string(), "  ".to_string()],
            env_vars: HashMap::new(),
        },
    );

    let merged = config.merge(&default, "release");

    assert_eq!(merged.command, "release-cmd");
    assert_eq!(merged.args, vec!["release-arg"]);
    assert_eq!(merged.pre_checks, vec!["release-check"]);
    assert_eq!(
        merged.env_vars,
        HashMap::from([("DEFAULT_ENV".to_string(), "default".to_string())])
    );
}
