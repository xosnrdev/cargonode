use std::{env, path::PathBuf, process};

use clap::{Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING;

use cargonode::{commands, config, progress, utils};

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
        #[arg(long, value_enum, default_value_t = utils::Vcs::default())]
        vcs: utils::Vcs,
    },
    /// Create a new Node.js project in an existing directory
    Init {
        /// Create a library package
        #[arg(long)]
        lib: bool,
        /// Initialize a new repository of the given type
        #[arg(long, value_enum, default_value_t = utils::Vcs::default())]
        vcs: utils::Vcs,
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { path, lib, vcs } => {
            let config = utils::VcsConfig {
                vcs,
                ..Default::default()
            };
            commands::create_new_project(&path, lib, Some(config))
        }
        Commands::Init { lib, vcs } => {
            let config = utils::VcsConfig {
                vcs,
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
            let current_dir = env::current_dir().map_err(cargonode::Error::Io)?;
            let options = commands::RunOptions {
                project_dir: current_dir.clone(),
                force,
                verbose,
            };

            let config = config::load_config(&current_dir)?;
            let result = commands::run_tool(&tool, &config, &options)?;
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
            let current_dir = env::current_dir().map_err(cargonode::Error::Io)?;
            commands::check(&paths, &current_dir, force, verbose)?;
            Ok(())
        }
        Commands::Build {
            release,
            force,
            verbose,
        } => {
            let current_dir = env::current_dir().map_err(cargonode::Error::Io)?;
            commands::build(release, &current_dir, force, verbose)?;
            Ok(())
        }
        Commands::Test {
            pattern,
            force,
            verbose,
        } => {
            let current_dir = env::current_dir().map_err(cargonode::Error::Io)?;
            commands::test(&pattern, &current_dir, force, verbose)?;
            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("{}", progress::format_error(&err.to_string()));
        process::exit(1);
    }

    Ok(())
}
