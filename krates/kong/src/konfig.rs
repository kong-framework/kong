//! 🎛️ `kong` server configuration

use crate::defaults;
use crate::error::KError;
use serde::Deserialize;
use std::{env, fs};

/// 🎛️ Server configuration
#[derive(Deserialize)]
pub struct Konfig {
    /// Port to access the server
    pub port: u16,
    /// Admin email address
    pub admin_email: Option<String>,
    /// Kong server working directory, path should end with `/`
    /// __defaults to kong/__
    pub working_directory: Option<String>,
    /// Name of the authorization session cookie id
    pub auth_cookie_name: String,
    /// Path to static files, __if not provided no static files will be served__
    pub static_files_path: Option<String>,
    /// Node hostname
    pub hostname: String,
    /// Kong secret key
    pub secret_key: String,
    /// Weather the server should log information to console.
    /// Console logging is __enabled__ by default.
    pub console_log: Option<bool>,
    /// Weather the server should log information to file, the file is
    /// stored in the working directory as LOG.
    /// Logging to the LOG file is __disabled__ by default
    pub log_file: Option<bool>,
}

impl Konfig {
    /// Read server config file from path provided as an argument when
    /// the program was started.
    pub fn read() -> Result<Konfig, KError> {
        let arg = env::args().nth(1);
        match arg {
            Some(a) => {
                let toml_str = fs::read_to_string(a).map_err(|_| KError::Config)?;
                let config: Konfig = toml::from_str(&toml_str).map_err(|_| KError::Config)?;
                Ok(config)
            }
            None => panic!("Path to config file is not provided!"),
        }
    }

    /// read port from config file
    pub fn read_port() -> u16 {
        let arg = env::args().nth(1);
        match arg {
            Some(a) => {
                let toml_str = fs::read_to_string(a).unwrap();
                let config: Konfig = toml::from_str(&toml_str).unwrap();
                config.port
            }
            None => panic!("Path to config file is not provided!"),
        }
    }

    /// Read working directory from config file
    pub fn read_working_dir() -> String {
        let arg = env::args().nth(1);
        match arg {
            Some(a) => {
                let toml_str = fs::read_to_string(a).unwrap();
                let config: Konfig = toml::from_str(&toml_str).unwrap();

                if let Some(working_directory) = config.working_directory {
                    working_directory
                } else {
                    defaults::WORKING_DIRECTORY.to_string()
                }
            }
            None => defaults::WORKING_DIRECTORY.to_string(),
        }
    }

    /// read loggin
    pub fn read_logging() -> (Option<bool>, Option<bool>) {
        let arg = env::args().nth(1);
        match arg {
            Some(a) => {
                let toml_str = fs::read_to_string(a).unwrap();
                let config: Konfig = toml::from_str(&toml_str).unwrap();
                let console = config.console_log;
                let file = config.log_file;
                (console, file)
            }
            None => panic!("Path to config file is not provided!"),
        }
    }

    /// read hostname
    pub fn read_hostname() -> String {
        let arg = env::args().nth(1);
        match arg {
            Some(a) => {
                let toml_str = fs::read_to_string(a).unwrap();
                let config: Konfig = toml::from_str(&toml_str).unwrap();
                let hostname = config.hostname;
                hostname
            }
            None => panic!("Path to config file is not provided!"),
        }
    }
}
