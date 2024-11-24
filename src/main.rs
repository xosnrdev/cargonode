pub mod cargo_node;
pub mod integration;

use std::{env, path::PathBuf};

use cargo_node::package::{self, Package};
use clap::{Parser, Subcommand};
use integration::{biome, tsup};

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
    /// Format files of the current package using biomejs
    #[command(disable_help_flag = true)]
    Fmt {
        /// Flag arguments to pass to biomejs
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Check files of the current package using biomejs
    #[command(disable_help_flag = true)]
    Check {
        /// Flag arguments to pass to biomejs
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Build and bundle the current package using tsup
    #[command(disable_help_flag = true)]
    Build {
        /// Flag arguments to pass to tsup
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { package_name } => {
            let config = package::Config {
                package_name,
                current_dir: get_current_dir(),
                template: package::Template::NodeTypeScript,
            };
            let package = Package::new(config);
            match package.create() {
                Ok(res) => println!("{}", res),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Init => {
            let config = package::Config {
                package_name: get_current_dir()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                current_dir: get_current_dir(),
                template: package::Template::NodeTypeScript,
            };
            let package = Package::new(config);
            match package.create_as_init() {
                Ok(res) => println!("{:?}", res),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Fmt { args } => match biome::format(get_current_dir(), args) {
            Ok(res) => println!("{}", res),
            Err(err) => eprintln!("Error: {}", err),
        },
        Commands::Check { args } => match biome::check(get_current_dir(), args) {
            Ok(res) => println!("{}", res),
            Err(err) => eprintln!("Error: {}", err),
        },
        Commands::Build { args } => match tsup::build(get_current_dir(), args) {
            Ok(res) => println!("{}", res),
            Err(err) => eprintln!("Error: {}", err),
        },
    }
}

fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap()
}
