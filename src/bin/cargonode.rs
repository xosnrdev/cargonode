use cargonode::{
    error::{self, CliError},
    ops::parser::Cli,
};
use clap::Parser;

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();
    let result = cli.run();

    exit(result)
}

/// Report any error message and exit.
fn exit(result: Result<(), error::CliError>) -> ! {
    let code = error::report(result);
    std::process::exit(code)
}

// #[test]
// fn verify_app() {
//     use clap::CommandFactory;

//     Cli::command().debug_assert();
// }
