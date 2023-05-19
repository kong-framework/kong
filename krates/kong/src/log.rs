//! Kong node logging

use crate::konfig::Konfig;
use crate::KError;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::prelude::*;

/// Kinds of logging
pub struct Log;

impl Log {
    /// Log data
    pub fn log(message: &str) -> Result<(), KError> {
        let logging = Konfig::read_logging();

        if let Some(console_logging) = logging.0 {
            if console_logging {
                // Log to console
                Log::log_to_console(message);
            }
        }

        if let Some(file_logging) = logging.1 {
            if file_logging {
                // Log to file
                Log::log_to_file(message)?;
            }
        }

        Ok(())
    }

    fn log_to_console(message: &str) {
        let now = Utc::now();
        eprintln!("---+ [{now}]: {message}");
    }

    fn log_to_file(message: &str) -> Result<(), KError> {
        let file = if let Some(dir) = Konfig::read_working_dir() {
            format!("{dir}LOG")
        } else {
            return Err(KError::LogFile);
        };

        let file = OpenOptions::new().write(true).append(true).open(file);

        if let Ok(mut file) = file {
            let now = Utc::now();
            if let Err(_e) = writeln!(file, "---+ [{now}]: {message}") {
                Err(KError::LogFile)
            } else {
                Ok(())
            }
        } else {
            Err(KError::LogFile)
        }
    }
}
