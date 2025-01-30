use std::collections::HashSet;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    cmd::do_call,
    config::Config,
    error::{AppResult, CliError},
};

#[derive(Debug, Serialize, PartialEq, Eq, Hash, clap::ValueEnum, Clone, Copy)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub enum Job {
    Build,
    Check,
    Fmt,
    Release,
    Run,
    Test,
}

impl<'de> Deserialize<'de> for Job {
    fn deserialize<D>(deserializer: D) -> Result<Job, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "build" => Ok(Job::Build),
            "check" => Ok(Job::Check),
            "fmt" => Ok(Job::Fmt),
            "release" => Ok(Job::Release),
            "run" => Ok(Job::Run),
            "test" => Ok(Job::Test),
            _ => Err(serde::de::Error::custom(format!("Unknown job: {}", s))),
        }
    }
}

impl Job {
    pub fn call(&self, args: Vec<String>) -> Result<(), CliError> {
        let mut visited = HashSet::new();
        self.call_with_visited(args, &mut visited)?;
        Ok(())
    }

    fn call_with_visited(&self, args: Vec<String>, visited: &mut HashSet<Job>) -> AppResult<()> {
        if !visited.insert(*self) {
            anyhow::bail!("Circular dependency detected involving job: {:?}", self);
        }
        let config = Config::from_default();
        let ctx = config
            .cargonode
            .get(self)
            .with_context(|| format!("Missing configuration for job: {:?}", self))?;
        for step in &ctx.steps {
            step.call_with_visited(Vec::new(), visited)
                .with_context(|| format!("Failed in step {:?} for job {:?}", step, self))?;
        }
        do_call(ctx, &args).with_context(|| format!("Failed to execute job: {:?}", self))?;
        visited.remove(self);
        Ok(())
    }
}
