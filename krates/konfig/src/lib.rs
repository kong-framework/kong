//! server configuration
use kerror::KError;
use serde::Deserialize;
use std::{env, fs};

/// Server configuration
#[derive(Deserialize)]
pub struct Konfig {
    /// Port to access the server
    pub port: u16,
    /// Admin email address
    pub admin_email: String,
    /// Admin Accounts database path
    pub admin_accounts_database: String,
    /// Name of the authorization session cookie id
    pub auth_cookie_name: String,
    /// Path to static files
    pub static_files_path: Option<String>,
}

impl Konfig {
    /// Read server config file from path provided as an argument when
    /// the program was started.
    pub fn read() -> Result<Konfig, KError> {
        let arg = env::args().nth(1);
        match arg {
            Some(a) => {
                let toml_str = fs::read_to_string(a).map_err(|_| KError::ConfigError)?;
                let config: Konfig = toml::from_str(&toml_str).map_err(|_| KError::ConfigError)?;
                Ok(config)
            }
            None => panic!("Path to config file is not provided!"),
        }
    }

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
}
