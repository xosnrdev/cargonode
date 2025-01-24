use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{
    cmd::{do_call, CommandContext},
    config::Config,
    error::CliError,
};

#[derive(Debug, PartialEq)]
pub struct ParseError(pub String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, clap::ValueEnum, Clone, Copy)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub enum Job {
    Build,
    Check,
    Fmt,
    Release,
    Run,
    Test,
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Job::Build => "build",
                Job::Check => "check",
                Job::Fmt => "fmt",
                Job::Release => "release",
                Job::Run => "run",
                Job::Test => "test",
            }
        )
    }
}

impl FromStr for Job {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "build" => Ok(Job::Build),
            "check" => Ok(Job::Check),
            "fmt" => Ok(Job::Fmt),
            "release" => Ok(Job::Release),
            "run" => Ok(Job::Run),
            "test" => Ok(Job::Test),
            _ => Err(ParseError(format!("Unsupported job: {}", s))),
        }
    }
}

pub fn call_with_job(job: &Job, args: Vec<String>) -> Result<(), CliError> {
    let mut config = Config::from_default();
    let mut default_ctx = CommandContext::default();
    let ctx = if let Some(ctx) = config.cargonode.get_mut(job) {
        ctx.args.extend(args);
        ctx
    } else {
        default_ctx.args.extend(args);
        &default_ctx
    };
    do_call(ctx)
}

#[test]
fn test_job_from_str() {
    assert_eq!("build".parse(), Ok(Job::Build));
    assert_eq!("check".parse(), Ok(Job::Check));
    assert_eq!("fmt".parse(), Ok(Job::Fmt));
    assert_eq!("release".parse(), Ok(Job::Release));
    assert_eq!("run".parse(), Ok(Job::Run));
    assert_eq!("test".parse(), Ok(Job::Test));
    assert!("unknown".parse::<Job>().is_err());
}
