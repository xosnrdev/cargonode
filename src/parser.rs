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
            Workflow::Run { args } => {
                let job = Job::Run;
                self.workflow_config.from_args(&job)?;
                job.call(args)
            }
            Workflow::Fmt { args } => {
                let job = Job::Fmt;
                self.workflow_config.from_args(&job)?;
                job.call(args)
            }
            Workflow::Check { args } => {
                let job = Job::Check;
                self.workflow_config.from_args(&job)?;
                job.call(args)
            }
            Workflow::Build { args } => {
                let job = Job::Build;
                self.workflow_config.from_args(&job)?;
                job.call(args)
            }
            Workflow::Test { args } => {
                let job = Job::Test;
                self.workflow_config.from_args(&job)?;
                job.call(args)
            }
            Workflow::Release { args } => {
                let job = Job::Release;
                self.workflow_config.from_args(&job)?;
                job.call(args)
            }
        }
    }
}
