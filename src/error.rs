#[derive(Debug)]
pub struct CliError {
    error: Option<anyhow::Error>,
    code: i32,
}

impl CliError {
    pub fn silent_with_code(code: i32) -> Self {
        Self { error: None, code }
    }

    pub fn silent() -> Self {
        Self::silent_with_code(0)
    }

    pub fn message_with_code(e: impl Into<anyhow::Error>, code: i32) -> Self {
        Self {
            error: Some(e.into()),
            code,
        }
    }

    pub fn message(e: impl Into<anyhow::Error>) -> Self {
        Self::message_with_code(e, 101)
    }
}

macro_rules! process_error_from {
    ($from:ty) => {
        impl From<$from> for CliError {
            fn from(error: $from) -> Self {
                Self::message(error)
            }
        }
    };
}

process_error_from!(anyhow::Error);
process_error_from!(std::io::Error);
process_error_from!(std::string::FromUtf8Error);

impl From<i32> for CliError {
    fn from(code: i32) -> Self {
        Self::silent_with_code(code)
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(error) = self.error.as_ref() {
            error.fmt(f)
        } else {
            write!(f, "Exit code: {}", self.code)
        }
    }
}

/// Report, delegating exiting to the caller.
pub fn report(result: Result<(), CliError>) -> i32 {
    match result {
        Ok(()) => 0,
        Err(err) => {
            if let Some(error) = err.error {
                // At this point, we might be exiting due to a broken pipe, just do our best and
                // move on.
                let _ = crate::ops::shell::error(error);
            }
            err.code
        }
    }
}

impl std::error::Error for CliError {}

pub type AppResult<T> = anyhow::Result<T>;
