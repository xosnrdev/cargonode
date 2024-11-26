pub mod exec;
mod integration;
mod package;

use clap::{Parser, Subcommand};
use integration::{build, check, format, release, test};
use package::{get_current_dir, get_current_dir_name, Package};

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
    /// Initialize package in current directory
    Init,
    /// Format files using biomejs
    #[command(disable_help_flag = true)]
    Fmt {
        /// Flag arguments to pass to biomejs
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Check files using biomejs
    #[command(disable_help_flag = true)]
    Check {
        /// Flag arguments to pass to biomejs
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Build and bundle using tsup
    #[command(disable_help_flag = true)]
    Build {
        /// Flag arguments to pass to tsup
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Run tests using vitest
    #[command(disable_help_flag = true)]
    Test {
        /// Flag arguments to pass to vitest
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Automate package release using release-it
    #[command(disable_help_flag = true)]
    Release {
        /// Flag arguments to pass to release-it
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { package_name } => {
            let config = package::Config {
                package_name,
                current_dir: get_current_dir(),
                template: package::Template::NodeTypeScript,
            };
            let package = Package::new(config);
            match package.create().await {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Init => {
            let config = package::Config {
                package_name: get_current_dir_name(),
                current_dir: get_current_dir(),
                template: package::Template::NodeTypeScript,
            };
            let package = Package::new(config);
            match package.create_as_init().await {
                Ok(output) => println!("{}", output.unwrap_or_default()),
                Err(err) => eprintln!("Error: {}", err),
            }
        }
        Commands::Fmt { args } => match format(&get_current_dir(), args).await {
            Ok(output) => println!("{}", output),
            Err(err) => eprintln!("Error: {}", err),
        },
        Commands::Check { args } => match check(&get_current_dir(), args).await {
            Ok(output) => println!("{}", output),
            Err(err) => eprintln!("Error: {}", err),
        },
        Commands::Build { args } => match build(&get_current_dir(), args).await {
            Ok(output) => println!("{}", output),
            Err(err) => eprintln!("Error: {}", err),
        },
        Commands::Test { args } => match test(&get_current_dir(), args).await {
            Ok(output) => println!("{}", output),
            Err(err) => eprintln!("Error: {}", err),
        },
        Commands::Release { args } => match release(&get_current_dir(), args).await {
            Ok(output) => println!("{}", output),
            Err(err) => eprintln!("Error: {}", err),
        },
    }
}
