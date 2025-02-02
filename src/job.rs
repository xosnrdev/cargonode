use std::collections::HashSet;

use anyhow::{bail, Context};
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
        if s.eq_ignore_ascii_case("build") {
            Ok(Job::Build)
        } else if s.eq_ignore_ascii_case("check") {
            Ok(Job::Check)
        } else if s.eq_ignore_ascii_case("fmt") {
            Ok(Job::Fmt)
        } else if s.eq_ignore_ascii_case("release") {
            Ok(Job::Release)
        } else if s.eq_ignore_ascii_case("run") {
            Ok(Job::Run)
        } else if s.eq_ignore_ascii_case("test") {
            Ok(Job::Test)
        } else {
            Err(serde::de::Error::custom(format!("Unknown job: {}", s)))
        }
    }
}

impl Job {
    pub fn call(&self, config: &Config) -> Result<(), CliError> {
        let mut visited = HashSet::new();
        self.call_with_visited(&mut visited, config)?;
        Ok(())
    }

    fn call_with_visited(&self, visited: &mut HashSet<Job>, config: &Config) -> AppResult<()> {
        if !visited.insert(*self) {
            bail!("Cyclic dependency detected: {:?}", visited);
        }
        let ctx = config
            .cargonode
            .get(self)
            .with_context(|| format!("Missing configuration for job: {:?}", self))?;
        for step in &ctx.steps {
            step.call_with_visited(visited, config)
                .with_context(|| format!("Failed in step {:?} for job {:?}", step, self))?;
        }
        do_call(ctx).with_context(|| format!("Failed to execute job: {:?}", self))?;
        visited.remove(self);
        Ok(())
    }
}
