use std::path::{Path, PathBuf};

use crate::{
    cmd::{do_call, validate_executable, CommandContext},
    error::CliError,
    job::ParseError,
};

#[derive(Debug, clap::ValueEnum, Clone, PartialEq)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl AsRef<str> for PackageManager {
    fn as_ref(&self) -> &str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Bun => "bun",
        }
    }
}

impl TryFrom<&Path> for PackageManager {
    type Error = ParseError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if path.join("package-lock.json").exists() {
            Ok(PackageManager::Npm)
        } else if path.join("yarn.lock").exists() {
            Ok(PackageManager::Yarn)
        } else if path.join("pnpm-lock.yaml").exists() {
            Ok(PackageManager::Pnpm)
        } else if path.join("bun.lock").exists() {
            Ok(PackageManager::Bun)
        } else {
            Err(ParseError(format!(
                "Unsupported package manager: {}",
                path.display()
            )))
        }
    }
}

impl PackageManager {
    fn with_context(&self, ctx: &mut CommandContext, executable_path: PathBuf) {
        ctx.executable = executable_path;
        ctx.subcommand = "install".to_string();
    }
}

pub fn call_with_pm(pm: PackageManager, dir_name: PathBuf) -> Result<(), CliError> {
    let mut ctx = CommandContext {
        working_dir: dir_name,
        ..Default::default()
    };
    let executable_path = validate_executable(pm.as_ref())?;
    pm.with_context(&mut ctx, executable_path);
    do_call(&ctx)
}
