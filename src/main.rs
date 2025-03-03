use std::{path::PathBuf, process};

use clap::{Parser, Subcommand, ValueEnum};
use clap_cargo::style::CLAP_STYLING;

use cargonode::{commands, progress, utils};

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum Vcs {
    Git,
    None,
}

impl From<Vcs> for utils::VcsType {
    fn from(vcs: Vcs) -> Self {
        match vcs {
            Vcs::Git => utils::VcsType::Git,
            Vcs::None => utils::VcsType::None,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, styles = CLAP_STYLING)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Node.js project at PATH
    New {
        /// The path to create the project in
        path: PathBuf,
        /// Create a library package
        #[arg(long)]
        lib: bool,
        /// Initialize a new repository of the given type
        #[arg(long, value_enum, default_value_t = Vcs::Git)]
        vcs: Vcs,
    },
    /// Create a new Node.js project in an existing directory
    Init {
        /// Create a library package
        #[arg(long)]
        lib: bool,
        /// Initialize a new repository of the given type
        #[arg(long, value_enum, default_value_t = Vcs::Git)]
        vcs: Vcs,
    },
    /// Run a specific tool
    Run {
        /// The tool to run
        tool: String,
        /// Arguments to pass to the tool
        _args: Vec<String>,
        /// Force execution even if cached
        #[arg(long)]
        force: bool,
        /// Print verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Check files for errors
    Check {
        /// Paths to check (defaults to all files)
        paths: Vec<PathBuf>,
        /// Force execution even if cached
        #[arg(long)]
        force: bool,
        /// Print verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Build the project
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,
        /// Force execution even if cached
        #[arg(long)]
        force: bool,
        /// Print verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Run tests
    Test {
        /// Test pattern to run
        #[arg(default_value = "")]
        pattern: String,
        /// Force execution even if cached
        #[arg(long)]
        force: bool,
        /// Print verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Show command execution history
    History {
        /// Filter by tool name
        #[arg(long)]
        tool: Option<String>,
        /// Maximum number of entries to show
        #[arg(long, default_value_t = 10)]
        limit: usize,
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },
    /// Clear the cache
    ClearCache {
        /// Clear cache for a specific tool
        #[arg(long)]
        tool: Option<String>,
        /// Print verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { path, lib, vcs } => {
            let config = utils::VcsConfig {
                vcs_type: vcs.into(),
                ..Default::default()
            };
            commands::create_new_project(&path, lib, Some(config))
        }
        Commands::Init { lib, vcs } => {
            let config = utils::VcsConfig {
                vcs_type: vcs.into(),
                ..Default::default()
            };
            commands::init_project(lib, Some(config))
        }
        Commands::Run {
            tool,
            _args,
            force,
            verbose,
        } => {
            // Get current directory
            let current_dir = std::env::current_dir().map_err(cargonode::Error::Io)?;

            // Create cache and journal directories
            let cache_dir = current_dir.join(".cargonode").join("cache");
            std::fs::create_dir_all(&cache_dir).map_err(cargonode::Error::Io)?;

            let journal_dir = current_dir.join(".cargonode").join("journal");
            std::fs::create_dir_all(&journal_dir).map_err(cargonode::Error::Io)?;

            // Create run options
            let options = commands::RunOptions {
                project_dir: current_dir.clone(),
                cache_dir,
                journal_dir,
                force,
                verbose,
                max_journal_entries: 100,
            };

            // Load configuration
            let config = cargonode::config::load_config(&current_dir)?;

            // Run the tool
            let result = commands::run_tool(&tool, &config, &options)?;

            // Check if the command was successful
            if !result.status.success() {
                return Err(Box::new(cargonode::Error::CommandFailed {
                    command: tool,
                    status: result.status,
                }));
            }

            Ok(())
        }
        Commands::Check {
            paths,
            force,
            verbose,
        } => {
            // Get current directory
            let current_dir = std::env::current_dir().map_err(cargonode::Error::Io)?;

            // Create cache and journal directories
            let cache_dir = current_dir.join(".cargonode").join("cache");
            std::fs::create_dir_all(&cache_dir).map_err(cargonode::Error::Io)?;

            let journal_dir = current_dir.join(".cargonode").join("journal");
            std::fs::create_dir_all(&journal_dir).map_err(cargonode::Error::Io)?;

            // Run check command
            let _result = commands::check(
                &paths,
                &current_dir,
                &cache_dir,
                &journal_dir,
                force,
                verbose,
            )?;

            Ok(())
        }
        Commands::Build {
            release,
            force,
            verbose,
        } => {
            // Get current directory
            let current_dir = std::env::current_dir().map_err(cargonode::Error::Io)?;

            // Create cache and journal directories
            let cache_dir = current_dir.join(".cargonode").join("cache");
            std::fs::create_dir_all(&cache_dir).map_err(cargonode::Error::Io)?;

            let journal_dir = current_dir.join(".cargonode").join("journal");
            std::fs::create_dir_all(&journal_dir).map_err(cargonode::Error::Io)?;

            // Run build command
            let _result = commands::build(
                release,
                &current_dir,
                &cache_dir,
                &journal_dir,
                force,
                verbose,
            )?;

            Ok(())
        }
        Commands::Test {
            pattern,
            force,
            verbose,
        } => {
            // Get current directory
            let current_dir = std::env::current_dir().map_err(cargonode::Error::Io)?;

            // Create cache and journal directories
            let cache_dir = current_dir.join(".cargonode").join("cache");
            std::fs::create_dir_all(&cache_dir).map_err(cargonode::Error::Io)?;

            let journal_dir = current_dir.join(".cargonode").join("journal");
            std::fs::create_dir_all(&journal_dir).map_err(cargonode::Error::Io)?;

            // Run test command
            let _result = commands::test(
                &pattern,
                &current_dir,
                &cache_dir,
                &journal_dir,
                force,
                verbose,
            )?;

            Ok(())
        }
        Commands::History {
            tool,
            limit,
            verbose,
        } => {
            // Get current directory
            let current_dir = std::env::current_dir().map_err(cargonode::Error::Io)?;

            // Create journal directory
            let journal_dir = current_dir.join(".cargonode").join("journal");
            std::fs::create_dir_all(&journal_dir).map_err(cargonode::Error::Io)?;

            // Show history
            commands::show_history(tool.as_deref(), limit, &journal_dir, verbose)?;

            Ok(())
        }
        Commands::ClearCache { tool, verbose } => {
            // Get current directory
            let current_dir = std::env::current_dir().map_err(cargonode::Error::Io)?;

            // Create cache directory
            let cache_dir = current_dir.join(".cargonode").join("cache");
            std::fs::create_dir_all(&cache_dir).map_err(cargonode::Error::Io)?;

            // Clear cache
            commands::clear_cache(tool.as_deref(), &cache_dir, verbose)?;

            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("{}", progress::format_error(&err.to_string()));
        process::exit(1);
    }

    Ok(())
}
