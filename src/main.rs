//! Unified tooling for Node.js
use cargonode::{build, check, format, package::*, release, test, ui};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about, version, long_about = None, styles = ui::styles())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new package with <package_name>
    New { package_name: String },
    /// Initialize package in current directory
    Init,
    /// Format files using biomejs
    #[command(disable_help_flag = true)]
    Fmt {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Check files using biomejs
    #[command(disable_help_flag = true)]
    Check {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Build and bundle using tsup
    #[command(disable_help_flag = true)]
    Build {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Run tests using vitest
    #[command(disable_help_flag = true)]
    Test {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Automate package release using release-it
    #[command(disable_help_flag = true)]
    Release {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
}

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    let cli = Cli::parse();
    let current_dir = get_current_dir();

    /// Macro for creating a new package with the specified name.
    macro_rules! create_package {
        ($package_name:expr, $current_dir:expr, $create_method:ident) => {{
            let config = Config {
                package_name: $package_name,
                current_dir: $current_dir.clone(),
                template: Template::NodeTypeScript,
            };
            let package = Package::new(config);
            match package.$create_method() {
                Ok(output) => println!("{}", output),
                Err(err) => eprintln!("Error: {}", err),
            }
        }};
    }

    macro_rules! handle_command {
        ($cmd:expr, $func:expr) => {
            match $func(&current_dir, $cmd) {
                Ok(output) => println!("{}", output),
                Err(err) => eprintln!("Error: {}", err),
            }
        };
    }

    match cli.command {
        Commands::New { package_name } => {
            create_package!(package_name, current_dir, create_package)
        }
        Commands::Init => {
            create_package!(get_current_dir_name(), current_dir, init_package)
        }
        Commands::Fmt { args } => handle_command!(args, format),
        Commands::Check { args } => handle_command!(args, check),
        Commands::Build { args } => handle_command!(args, build),
        Commands::Test { args } => handle_command!(args, test),
        Commands::Release { args } => handle_command!(args, release),
    }
}
