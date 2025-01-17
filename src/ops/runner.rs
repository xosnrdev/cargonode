use crate::error::CliError;

pub trait Runner {
    fn run(&self) -> Result<(), CliError>;
}
