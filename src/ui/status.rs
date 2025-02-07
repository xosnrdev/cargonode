use std::path::Path;

use console::{style, Emoji};

static CREATING: Emoji<'_, '_> = Emoji("🔨", "Creating");
static INITIALIZING: Emoji<'_, '_> = Emoji("🚀", "Initializing");
static MANIFEST: Emoji<'_, '_> = Emoji("📦", "package.json");
static WORKSPACE: Emoji<'_, '_> = Emoji("🏗️ ", "workspace");
static GIT: Emoji<'_, '_> = Emoji("📚", "git");
static SUCCESS: Emoji<'_, '_> = Emoji("✨", "*");
static WARNING: Emoji<'_, '_> = Emoji("⚠️ ", "!");

pub struct Status {
    package_type: &'static str,
    is_new: bool,
}

impl Status {
    #[must_use]
    pub const fn new(is_bin: bool, is_lib: bool, is_new: bool) -> Self {
        let package_type = match (is_bin, is_lib) {
            (_, true) => "library", // Library takes precedence
            _ => "binary",          // Binary is default
        };
        Self {
            package_type,
            is_new,
        }
    }

    pub fn start(&self, path: &Path) {
        let action = if self.is_new { CREATING } else { INITIALIZING };
        let display_path = if path == Path::new(".") {
            "current directory".to_string()
        } else {
            path.file_name().map_or_else(
                || path.display().to_string(),
                |n| n.to_string_lossy().to_string(),
            )
        };
        println!(
            "{} {} package `{}` {}",
            style(format!("   {action} ")).green(),
            style(self.package_type).cyan(),
            style(&display_path).yellow(),
            style(format!("in {}", path.display())).dim()
        );
    }

    pub fn created_manifest(&self) {
        println!(
            "{}",
            style(format!("      {MANIFEST} Created manifest file")).dim()
        );
    }

    pub fn created_source_files(&self) {
        println!("{}", style("      📄 Created source files").dim());
    }

    pub fn created_workspace(&self) {
        println!(
            "{}",
            style(format!("      {WORKSPACE} Created workspace configuration")).dim()
        );
    }

    pub fn initialized_git(&self) {
        println!(
            "{}",
            style(format!("      {GIT} Initialized git repository")).dim()
        );
    }

    pub fn created_package(&self) {
        println!(
            "{}",
            style(format!(
                "      {SUCCESS} Created {} package",
                self.package_type
            ))
            .dim()
        );
    }

    pub fn finish(&self, name: &str) {
        let action = if self.is_new {
            "created"
        } else {
            "initialized"
        };
        println!(
            "\n{} {} {} package `{}`",
            style("Successfully").green(),
            action,
            self.package_type,
            style(name).yellow()
        );
    }

    pub fn warning(&self, message: &str) {
        println!(
            "{} {}",
            style(format!("{WARNING} Warning:")).yellow(),
            message
        );
    }
}
