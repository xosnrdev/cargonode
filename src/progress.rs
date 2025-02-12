use std::{
    env,
    io::{self, Write},
};

/// Terminal colors as ANSI escape codes
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Color {
    Green,
    Blue,
    Yellow,
    Red,
    Reset,
}

impl Color {
    const fn as_ansi_code(self) -> &'static str {
        match self {
            Color::Green => "\x1b[32m",
            Color::Blue => "\x1b[34m",
            Color::Yellow => "\x1b[33m",
            Color::Red => "\x1b[31m",
            Color::Reset => "\x1b[0m",
        }
    }
}

fn should_use_colors() -> bool {
    env::var("TERM").is_ok()
}

pub fn style_text(text: &str, color: Color, is_bold: bool) -> String {
    if !should_use_colors() {
        return text.to_string();
    }

    let bold_prefix = if is_bold { "\x1b[1m" } else { "" };
    format!(
        "{}{}{}{}",
        bold_prefix,
        color.as_ansi_code(),
        text,
        Color::Reset.as_ansi_code()
    )
}

pub fn write_message(message: &str) -> io::Result<()> {
    println!("{}", message);
    io::stdout().flush()
}

pub fn format_status(status: &str, message: &str) -> String {
    format!("{:>12} {}", style_text(status, Color::Green, true), message)
}

pub fn format_error(message: &str) -> String {
    let error_prefix = style_text("error:", Color::Red, true);
    format!("{} {}", error_prefix, message)
}

pub fn format_error_with_details(message: &str, details: &str) -> String {
    let error_prefix = style_text("error:", Color::Red, true);
    format!("{} {}\n\n{}", error_prefix, message, details)
}

pub fn format_warning(message: &str) -> String {
    let warning_prefix = style_text("warning:", Color::Yellow, true);
    format!("{} {}", warning_prefix, message)
}

pub fn format_note(message: &str) -> String {
    let note_prefix = style_text("note:", Color::Blue, true);
    format!("{} {}", note_prefix, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_status() {
        let message = format_status("Creating", "binary package");
        assert!(message.contains("Creating"));
        assert!(message.contains("binary package"));
    }

    #[test]
    fn test_format_error() {
        let error = format_error("failed to create package");
        assert!(error.contains("error:"));
        assert!(error.contains("failed to create package"));
    }

    #[test]
    fn test_format_warning() {
        let warning = format_warning("package name contains uppercase letters");
        assert!(warning.contains("warning:"));
        assert!(warning.contains("package name contains uppercase letters"));
    }

    #[test]
    fn test_format_note() {
        let note = format_note("see cargo.toml for package configuration");
        assert!(note.contains("note:"));
        assert!(note.contains("see cargo.toml for package configuration"));
    }
}
