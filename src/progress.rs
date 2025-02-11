use std::{
    env,
    io::{self, Write},
    time::Instant,
};

/// Represents the state of progress tracking
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProgressState {
    current_step: usize,
    total_steps: usize,
    has_vcs: bool,
    last_update: Option<Instant>,
}

/// Represents different types of progress messages
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageType {
    Progress,
    Success,
    Warning,
    Error,
    Note,
}

/// Represents a styled message with its type and content
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StyledMessage {
    message_type: MessageType,
    content: String,
}

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

pub fn new_progress(total_steps: usize, has_vcs: bool) -> ProgressState {
    ProgressState {
        current_step: 0,
        total_steps: total_steps - if !has_vcs { 1 } else { 0 },
        has_vcs,
        last_update: None,
    }
}

pub fn create_message(message_type: MessageType, content: &str) -> StyledMessage {
    StyledMessage {
        message_type,
        content: content.to_string(),
    }
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

pub fn format_progress_message(state: &ProgressState, message: &str) -> Option<String> {
    if !should_show_message(state, message) {
        return None;
    }

    let next_step = state.current_step + 1;
    let status = if next_step == state.total_steps {
        style_text("Finished", Color::Green, false)
    } else {
        style_text("Progress", Color::Blue, false)
    };

    Some(format!(
        "    {} [{}/{}] {}",
        status, next_step, state.total_steps, message
    ))
}

fn should_show_message(state: &ProgressState, message: &str) -> bool {
    state.has_vcs || !(message.contains("git") || message.contains("repository"))
}

fn should_use_colors() -> bool {
    // We'll use a simple heuristic: check if TERM environment variable is set
    env::var("TERM").is_ok()
}

pub fn update_progress(state: ProgressState, message: &str) -> (ProgressState, Option<String>) {
    let formatted_message = format_progress_message(&state, message);
    let next_state = ProgressState {
        current_step: state.current_step + 1,
        last_update: Some(Instant::now()),
        ..state
    };
    (next_state, formatted_message)
}

pub fn write_progress(message: &str) -> io::Result<()> {
    print!("{}\r", message);
    io::stdout().flush()
}

pub fn write_message(message: &str) -> io::Result<()> {
    println!("{}", message);
    io::stdout().flush()
}

pub fn format_error(message: &str, details: &str) -> String {
    let error_prefix = style_text("error", Color::Red, true);
    format!("{}: {}\n\nCause: {}", error_prefix, message, details)
}

pub fn format_warning(message: &str) -> String {
    let warning_prefix = style_text("warning", Color::Yellow, false);
    format!("{}: {}", warning_prefix, message)
}

pub fn format_note(message: &str) -> String {
    let note_prefix = style_text("note", Color::Blue, false);
    format!("{}: {}", note_prefix, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_progress() {
        let state = new_progress(5, true);
        assert_eq!(state.current_step, 0);
        assert_eq!(state.total_steps, 5);
        assert_eq!(state.has_vcs, true);
    }

    #[test]
    fn test_should_show_message() {
        let state = new_progress(5, false);
        assert!(!should_show_message(&state, "Initializing git repository"));
        assert!(should_show_message(&state, "Creating package.json"));
    }

    #[test]
    fn test_update_progress() {
        let state = new_progress(5, true);
        let (new_state, message) = update_progress(state, "Test message");
        assert_eq!(new_state.current_step, 1);
        assert!(message.is_some());
        assert!(message.unwrap().contains("Test message"));
    }

    #[test]
    fn test_format_error() {
        let error = format_error("Test error", "Test details");
        assert!(error.contains("Test error"));
        assert!(error.contains("Test details"));
    }

    #[test]
    fn test_create_message() {
        let message = create_message(MessageType::Warning, "Test warning");
        assert_eq!(message.message_type, MessageType::Warning);
        assert_eq!(message.content, "Test warning");
    }
}
