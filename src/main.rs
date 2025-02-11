use std::path::PathBuf;

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
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = match cli.command {
        Commands::New { path, lib, vcs } => {
            let msg = format!(
                "Creating new {} project at: {}",
                if lib { "library" } else { "binary" },
                path.display()
            );
            progress::write_message(&msg).ok();

            let config = utils::VcsConfig {
                vcs_type: vcs.into(),
                ..Default::default()
            };
            commands::create_new_project(&path, lib, Some(config))
        }
        Commands::Init { lib, vcs } => {
            let msg = format!(
                "Initializing new {} project in current directory",
                if lib { "library" } else { "binary" }
            );
            progress::write_message(&msg).ok();

            let config = utils::VcsConfig {
                vcs_type: vcs.into(),
                ..Default::default()
            };
            commands::init_project(lib, Some(config))
        }
    } {
        let error_msg = progress::format_error("Command failed", &err.to_string());
        eprintln!("{}", error_msg);
        std::process::exit(1);
    }
}
