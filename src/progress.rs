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
    Gray,
    Reset,
}

impl Color {
    const fn as_ansi_code(self) -> &'static str {
        match self {
            Color::Green => "\x1b[32m",
            Color::Blue => "\x1b[34m",
            Color::Yellow => "\x1b[33m",
            Color::Red => "\x1b[31m",
            Color::Gray => "\x1b[90m",
            Color::Reset => "\x1b[0m",
        }
    }
}

fn should_use_colors() -> bool {
    if cfg!(test) {
        return false;
    }
    env::var("NO_COLOR").is_err() && env::var("TERM").is_ok()
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

/// Format an error message with consistent styling
pub fn format_error(message: &str) -> String {
    let parts: Vec<&str> = message.split("\n\n").collect();
    let main_message = parts[0];

    let mut formatted = vec![format!(
        "{}: {}",
        style_text("error", Color::Red, true),
        main_message
    )];

    for part in parts.iter().skip(1) {
        let styled = if part.starts_with("Error:") {
            style_text(part, Color::Red, false)
        } else if part.starts_with("Details:") {
            style_text(part, Color::Gray, false)
        } else if part.starts_with("Suggestion:") {
            style_text(part, Color::Blue, false)
        } else {
            part.to_string()
        };
        formatted.push(styled);
    }

    formatted.join("\n\n")
}

/// Format a warning message with consistent styling
pub fn format_warning(message: &str) -> String {
    format!(
        "{}: {}",
        style_text("warning", Color::Yellow, true),
        message
    )
}

/// Format an informational note with consistent styling
pub fn format_note(message: &str) -> String {
    format!("{}: {}", style_text("note", Color::Blue, true), message)
}

/// Format a status message with consistent styling
pub fn format_status(status: &str, message: &str) -> String {
    format!("{}: {}", style_text(status, Color::Green, true), message)
}

/// Write a message to stdout with proper formatting
pub fn write_message(message: &str) -> io::Result<()> {
    println!("{}", message);
    io::stdout().flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_error() {
        let message =
            "Failed to run command\n\nError: Exit code 1\n\nSuggestion: Check the command exists";
        let formatted = format_error(message);
        assert!(formatted.contains("error: Failed to run command"));
        assert!(formatted.contains("Error: Exit code 1"));
        assert!(formatted.contains("Suggestion: Check the command exists"));
    }

    #[test]
    fn test_format_warning() {
        let message = "File not found";
        let formatted = format_warning(message);
        assert!(formatted.contains("warning: File not found"));
    }

    #[test]
    fn test_format_note() {
        let message = "Using cached version";
        let formatted = format_note(message);
        assert!(formatted.contains("note: Using cached version"));
    }

    #[test]
    fn test_format_status() {
        let status = "Running";
        let message = "build command";
        let formatted = format_status(status, message);
        assert!(formatted.contains("Running: build command"));
    }

    #[test]
    fn test_format_error_sections() {
        let message = "Command failed\n\nError: Exit code 1\n\nDetails: Process terminated\n\nSuggestion: Check permissions";
        let formatted = format_error(message);
        assert!(formatted.contains("error: Command failed"));
        assert!(formatted.contains("Error: Exit code 1"));
        assert!(formatted.contains("Details: Process terminated"));
        assert!(formatted.contains("Suggestion: Check permissions"));
    }
}
