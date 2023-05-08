//! server configuration

use crate::error::KError;
use serde::Deserialize;
use std::{env, fs};

/// Server configuration
#[derive(Deserialize)]
pub struct Konfig {
    /// Port to access the server
    pub port: u16,
    /// Admin email address
    pub admin_email: Option<String>,
    /// Kong server working directory, path should end with `/`
    /// defaults to ~/kong/
    pub working_directory: Option<String>,
    /// Name of the authorization session cookie id
    pub auth_cookie_name: String,
    /// Path to static files, if not provided no static files will be served
    pub static_files_path: Option<String>,
    /// Node hostname
    pub host: String,
    /// Kong secret key
    pub secret_key: String,
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
    pub fn read_working_dir() -> Option<String> {
        let arg = env::args().nth(1);
        match arg {
            Some(a) => {
                let toml_str = fs::read_to_string(a).unwrap();
                let config: Konfig = toml::from_str(&toml_str).unwrap();
                config.working_directory
            }
            None => panic!("Path to config file is not provided!"),
        }
    }
}
