use std::path::{Path, PathBuf};

use crate::{
    cmd::{do_call, validate_executable, CommandContext},
    error::CliError,
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
    type Error = anyhow::Error;

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
            Err(anyhow::format_err!(
                "Unsupported package manager: {}",
                path.display()
            ))
        }
    }
}

impl PackageManager {
    pub fn call(&self, dir_name: PathBuf) -> Result<(), CliError> {
        let ctx = CommandContext {
            executable: validate_executable(self.as_ref())?,
            subcommand: "install".to_string(),
            working_dir: dir_name,
            ..Default::default()
        };
        do_call(&ctx, &[])
    }
}
