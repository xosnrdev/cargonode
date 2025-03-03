pub mod cache;
pub mod commands;
pub mod config;
pub mod error;
pub mod inputs;
pub mod journal;
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
        assert!(format!("{}", io_err).contains("file not found"));

        let config_err = error::Error::Config {
            message: "invalid config".to_string(),
        };
        assert!(format!("{}", config_err).contains("Configuration error: invalid config"));

        let input_err = error::Error::Input {
            message: "invalid input".to_string(),
        };
        assert!(format!("{}", input_err).contains("Input error: invalid input"));

        let cache_err = error::Error::Cache {
            message: "cache error".to_string(),
        };
        assert!(format!("{}", cache_err).contains("Cache error: cache error"));
    }
}
