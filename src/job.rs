use serde::{Deserialize, Serialize};

use crate::{
    cmd::{do_call, CommandContext},
    config::Config,
    error::CliError,
};

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

impl Job {
    pub fn call(&self, args: Vec<String>) -> Result<(), CliError> {
        let mut config = Config::from_default();
        let mut default_ctx = CommandContext::default();
        let ctx = if let Some(ctx) = config.cargonode.get_mut(self) {
            ctx.args.extend(args);
            ctx
        } else {
            default_ctx.args.extend(args);
            &default_ctx
        };
        do_call(ctx)
    }
}
