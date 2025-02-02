use clap::Parser;

use crate::{
    error::CliError,
    job::Job,
    logging::get_logging,
    project,
    workflow::{Workflow, WorkflowConfig},
};

#[derive(Debug, Parser)]
#[command(
    version,
    author,
    about,
    propagate_version = true,
    styles = clap_cargo::style::CLAP_STYLING,
)]
pub struct Cli {
    #[command(subcommand)]
    pub workflow: Workflow,

    #[command(flatten)]
    pub workflow_config: WorkflowConfig,
}

impl Cli {
    pub fn run(self) -> Result<(), CliError> {
        let mut builder = get_logging(self.workflow_config.verbosity);
        builder.init();

        match self.workflow {
            Workflow::New {
                name,
                package_manager,
            } => project::new_pkg(name, package_manager),
            Workflow::Init { package_manager } => project::init_pkg(package_manager),
            Workflow::Run { .. } => {
                let config = self.workflow_config.from_args(&Job::Run)?;
                Job::Run.call(&config)
            }
            Workflow::Fmt { .. } => {
                let config = self.workflow_config.from_args(&Job::Fmt)?;
                Job::Fmt.call(&config)
            }
            Workflow::Check { .. } => {
                let config = self.workflow_config.from_args(&Job::Check)?;
                Job::Check.call(&config)
            }
            Workflow::Build { .. } => {
                let config = self.workflow_config.from_args(&Job::Build)?;
                Job::Build.call(&config)
            }
            Workflow::Test { .. } => {
                let config = self.workflow_config.from_args(&Job::Test)?;
                Job::Test.call(&config)
            }
            Workflow::Release { .. } => {
                let config = self.workflow_config.from_args(&Job::Release)?;
                Job::Release.call(&config)
            }
        }
    }
}
