pub mod cargo_node;

use std::{env, path::PathBuf};

use cargo_node::package::{self, Package};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about, author, version, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new package with <package_name>
    #[command(arg_required_else_help = true)]
    New {
        /// The name of the package to create
        package_name: String,
    },
    /// Create a new package in an existing directory
    Init,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { package_name } => {
            let current_dir = get_current_dir();
            let config = package::Config {
                package_name,
                current_dir,
                template: package::Template::NodeTypeScript,
            };
            let package = Package::new(config);
            match package.create() {
                Ok(_) => println!("note: run `npm install` to install dependencies"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Init => {
            let current_dir = get_current_dir();
            let config = package::Config {
                package_name: current_dir
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                current_dir,
                template: package::Template::NodeTypeScript,
            };
            let package = Package::new(config);
            match package.create_as_init() {
                Ok(_) => println!("note: run `npm install` to install dependencies"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}

fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap()
}
