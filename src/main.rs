use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING;
use log::info;

use cargonode::{create_package, init, job::Job, workflow::WorkflowConfig, PackageOptions};

/// `CargoNode`: Cargo-like experience for Node.js
#[derive(Parser)]
#[command(author, version, about, long_about = None, styles = CLAP_STYLING)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[command(flatten)]
    workflow_config: WorkflowConfig,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Node.js package
    New {
        /// Package name
        name: String,
        /// Create a binary package
        #[arg(short, long)]
        bin: bool,
        /// Create a library package
        #[arg(short, long)]
        lib: bool,
        /// Don't initialize a git repository
        #[arg(long)]
        no_vcs: bool,
        /// Use TypeScript
        #[arg(short, long)]
        typescript: bool,
        /// Create a workspace
        #[arg(short, long)]
        workspace: bool,
    },
    /// Initialize a new Node.js package in an existing directory
    Init {
        /// Initialize as a workspace
        #[arg(short, long)]
        workspace: bool,
        /// Create a binary package
        #[arg(short, long)]
        bin: bool,
        /// Create a library package
        #[arg(short, long)]
        lib: bool,
        /// Don't initialize a git repository
        #[arg(long)]
        no_vcs: bool,
        /// Use TypeScript
        #[arg(short, long)]
        typescript: bool,
    },
    /// Run a custom script or command.
    #[command(visible_alias = "r")]
    Run {
        /// Arguments for the runner.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Format code.
    #[command(disable_help_flag = true)]
    Fmt {
        /// Arguments for the formatter.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Check code.
    #[command(disable_help_flag = true, visible_alias = "c")]
    Check {
        /// Arguments for the checker.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Build or bundle.
    #[command(disable_help_flag = true, visible_alias = "b")]
    Build {
        /// Arguments for the builder.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Run tests.
    #[command(disable_help_flag = true, visible_alias = "t")]
    Test {
        /// Arguments for the test runner.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Release project.
    #[command(disable_help_flag = true)]
    Release {
        /// Arguments for the release command.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            bin,
            lib,
            no_vcs,
            typescript,
            workspace,
        } => {
            let mut opts = PackageOptions::new(PathBuf::from(&name));
            opts.set_bin(bin)
                .set_lib(lib)
                .set_vcs(!no_vcs)
                .set_typescript(typescript);
            opts.workspace = workspace;
            create_package(&opts)?;
            info!("Created package `{}`", opts.package_name());
        }
        Commands::Init {
            workspace,
            bin,
            lib,
            no_vcs,
            typescript,
        } => {
            let mut opts = PackageOptions::new(".");
            opts.workspace = workspace;
            opts.set_bin(bin)
                .set_lib(lib)
                .set_vcs(!no_vcs)
                .set_typescript(typescript);
            init(&opts)?;
            info!("Initialized package `{}`", opts.package_name());
        }
        Commands::Run { .. } => {
            let config = cli.workflow_config.from_args(&Job::Run)?;
            Job::Run.call(&config)?
        }
        Commands::Fmt { .. } => {
            let config = cli.workflow_config.from_args(&Job::Fmt)?;
            Job::Fmt.call(&config)?
        }
        Commands::Check { .. } => {
            let config = cli.workflow_config.from_args(&Job::Check)?;
            Job::Check.call(&config)?
        }
        Commands::Build { .. } => {
            let config = cli.workflow_config.from_args(&Job::Build)?;
            Job::Build.call(&config)?
        }
        Commands::Test { .. } => {
            let config = cli.workflow_config.from_args(&Job::Test)?;
            Job::Test.call(&config)?
        }
        Commands::Release { .. } => {
            let config = cli.workflow_config.from_args(&Job::Release)?;
            Job::Release.call(&config)?
        }
    };

    Ok(())
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
