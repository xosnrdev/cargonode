use anyhow::Ok;
use clap::{ArgGroup, Args, Parser, Subcommand};
use log::{debug, trace, warn};
use std::path::PathBuf;

use crate::{
    cmd::do_call,
    config::{Config, DEFAULT_CONFIG_FILE, DEFAULT_CONFIG_KEY},
    error::AppResult,
    source::{ConfigSource, FileConfigSource},
};

//----------------------------------------------------------------------
// Types
//----------------------------------------------------------------------

#[derive(Debug, Parser)]
#[command(
    version,
    author,
    about,
    visible_alias = "cn",
    styles = clap_cargo::style::CLAP_STYLING,
)]
#[command(propagate_version = true)]
pub struct Command {
    #[command(subcommand)]
    pub subs: Option<Subcommands>,

    #[command(flatten)]
    pub args: ConfigArgs,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Bootstrap a new package at <PATH>.
    New {
        #[arg(
            value_name = "PATH",
            help = "Specify the path to create the new package"
        )]
        path: String,
    },
    /// Initialize a new package in current directory.
    Init,
    /// Formats files using biomejs.
    #[command(disable_help_flag = true)]
    Fmt {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Lint files using biomejs
    #[command(disable_help_flag = true, visible_alias = "c")]
    Check {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Build and bundle the current package
    #[command(disable_help_flag = true, visible_alias = "b")]
    Build {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Run the tests
    #[command(disable_help_flag = true, visible_alias = "t")]
    Test {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Automate package release using release-it
    #[command(disable_help_flag = true, visible_alias = "r")]
    Release {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
}

#[derive(Debug, Args)]
#[command(
    group(
        ArgGroup::new("config_source")
            .required(false)
            .multiple(false)
            .args(&["config_file"])
    )
)]
pub struct ConfigArgs {
    /// Path to the configuration file (e.g., package.json)
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Specify the path to the configuration file",
        global = true,
        group = "config_source"
    )]
    pub config_file: Option<PathBuf>,

    /// Executable to run
    #[arg(short = 'x', long, help = "Specify the executable to run")]
    pub executable: Option<String>,

    /// Arguments for the executable
    #[arg(
        short,
        long,
        value_name = "ARGS",
        allow_hyphen_values = true,
        help = "Specify arguments for the executable"
    )]
    pub args: Option<Vec<String>>,

    /// Environment variables (key=value)
    #[arg(short, long, value_name = "KEY=VALUE", num_args = 0.., help = "Specify environment variables in KEY=VALUE format")]
    pub env_vars: Option<Vec<String>>,

    /// Working directory
    #[arg(
        short = 'w',
        long,
        value_name = "DIR",
        help = "Specify the working directory",
        global = true,
        default_value = "."
    )]
    pub working_dir: Option<PathBuf>,

    /// Pre-check commands
    #[arg(short = 'p', long, value_name = "CHECKS", num_args = 0.., help = "Specify pre-check commands", global = true)]
    pub pre_checks: Option<Vec<String>>,

    /// Timeout in seconds
    #[arg(
        short,
        long,
        value_name = "SECONDS",
        help = "Specify timeout in seconds",
        global = true,
        default_value = "60"
    )]
    pub timeout: Option<u64>,

    /// Pass many times for more log output
    ///
    /// By default, it'll report warn. Passing `-v` one time adds debug
    /// logs, `-vv` adds trace logs.
    #[arg(short, long, action = clap::ArgAction::Count, help = "Increase output verbosity", global = true)]
    pub verbose: u8,
}

//----------------------------------------------------------------------
// Implementations
//----------------------------------------------------------------------

impl Command {
    pub fn run(&self) -> AppResult<()> {
        let config = self.args.parse_config()?;

        let mut builder = get_logging(config.verbose);
        builder.init();

        match &self.subs {
            Some(Subcommands::New { path }) => {
                log::trace!("Creating new package at path: {}", path);
                todo!()
            }
            Some(Subcommands::Init) => {
                todo!()
            }
            Some(Subcommands::Fmt { args }) => {
                log::trace!("Formatting files with args: {:?}", args);
                todo!()
            }
            Some(Subcommands::Check { args }) => {
                log::trace!("Linting files with args: {:?}", args);
                todo!()
            }
            Some(Subcommands::Build { args }) => {
                log::trace!("Building and bundling package with args: {:?}", args);
                todo!()
            }
            Some(Subcommands::Test { args }) => {
                log::trace!("Running tests with args: {:?}", args);
                todo!()
            }
            Some(Subcommands::Release { args }) => {
                log::trace!("Releasing package with args: {:?}", args);
                todo!()
            }
            // Do call whatever executable config has
            None => do_call(&config),
        }
    }
}

impl ConfigArgs {
    /// Constructs a new ConfigArgs with only the config_file specified.
    pub const fn new(config_file: Option<PathBuf>) -> Self {
        Self {
            config_file,
            executable: None,
            args: None,
            env_vars: None,
            working_dir: None,
            pre_checks: None,
            timeout: None,
            verbose: 0,
        }
    }

    /// Parses the configuration by combining file and command-line configs.
    /// Command-line configs take precedence over the configuration file.
    pub fn parse_config(&self) -> AppResult<Config> {
        debug!("Starting parsing process.");

        let mut config = Config::default();
        let config_path = self
            .config_file
            .clone()
            .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_FILE));
        trace!("Using configuration file: {:?}", config_path);

        // Check if the configuration file exists
        if config_path.exists() {
            debug!("Configuration file {:?} found. Loading...", config_path);

            let file_source = FileConfigSource {
                path: config_path,
                key: DEFAULT_CONFIG_KEY,
            };

            // Load configuration from file
            let file_config = file_source.load_config()?;
            config.merge(file_config);
            debug!("Loaded configuration from file.");
        } else {
            warn!(
                "Configuration file {:?} not found. Skipping file-based configurations.",
                config_path
            );
        }

        debug!("Converting command-line arguments to configuration.");
        let args_config = config.from_args(self)?;
        config.merge(args_config);
        debug!("Merged configuration from command-line.");

        config.validate()?;
        debug!("Configuration validated successfully.");

        Ok(config)
    }
}

//----------------------------------------------------------------------
// Functions
//----------------------------------------------------------------------

fn get_logging(verbosity: u8) -> env_logger::Builder {
    use log::LevelFilter;

    let level = match verbosity {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);
    builder.format_module_path(false);
    if level == log::LevelFilter::Trace || level == log::LevelFilter::Debug {
        builder.format_timestamp_secs();
    } else {
        builder.format_target(false);
        builder.format_timestamp(None);
    }

    builder
}
