pub mod commands;
pub mod config;
pub mod error;
pub mod inputs;
pub mod outputs;
pub mod progress;
pub mod template;
pub mod utils;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn test_error_display() {
        let io_err = error::Error::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        let io_err_str = format!("{}", io_err);
        assert!(io_err_str.contains("File system error"));
        assert!(io_err_str.contains("file not found"));

        let config_err = error::Error::Config {
            message: "invalid config".to_string(),
        };
        let config_err_str = format!("{}", config_err);
        assert!(config_err_str.contains("Configuration error"));
        assert!(config_err_str.contains("Error: invalid config"));
        assert!(config_err_str.contains("Suggestion:"));

        let input_err = error::Error::Input {
            message: "invalid input".to_string(),
        };
        let input_err_str = format!("{}", input_err);
        assert!(input_err_str.contains("Input error"));
        assert!(input_err_str.contains("Error: invalid input"));
        assert!(input_err_str.contains("Suggestion:"));
    }
}
