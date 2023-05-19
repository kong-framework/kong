//!```text
//!                              )                 
//!                           ( /(          (  (   
//!                            )\())(   (    )\))(  
//!                           ((_)\ )\  )\ )((_))\  
//!                           | |(_|(_)_(_/( (()(_)
//!                           | / / _ \ ' \)) _` |  
//!                           |_\_\___/_||_|\__, |  
//!                         secure web node |___/ v0.1.0
//! ```
#![doc(html_favicon_url = "https://kwatafana.org/logo.jpeg")]
#![doc(html_logo_url = "https://kwatafana.org/logo.jpeg")]
#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

pub mod defaults;
mod error;
mod error_response;
pub mod inputs;
mod konfig;
mod kroute;
pub mod log;
pub mod validate;

pub use error::KError;
pub use error_response::ErrorResponse;
pub use konfig::Konfig;
pub use kroute::kroute;
pub use krypto;
pub use rouille as server;
pub use serde_json::error::Error as JsonError;
pub use serde_json::from_str as json_from_str;
pub use serde_json::json;
pub use serde_json::value::Value as JsonValue;

use krypto::kpassport::Kpassport;
use rouille::{Request, Response};
use route_recognizer::{Params, Router};
use std::{fs::File, str::FromStr};

/// 🔥 Kong object
pub struct Kong {
    /// Kong configuration
    pub config: Konfig,
    /// Request authentication + authorization token
    pub kpassport: Option<Kpassport>,
    /// Validated user input
    pub input: Option<serde_json::Value>,
}

impl Kong {
    /// Initialize kong, by creating the working directory if it does
    /// not exist and it content if it does not exist (for example the
    /// LOG file)
    fn init(config: &Konfig) {
        Kong::create_working_directory(config);
        Kong::create_log_file(config);
    }

    /// Create working dirctory if it does not already exist
    fn create_working_directory(config: &Konfig) {
        // Get path to working directory
        let working_dir = Konfig::read_working_dir();
        let working_dir = std::path::Path::new(&working_dir);

        if !std::path::Path::exists(working_dir) {
            // create working directory
            // XXX: Note that using unwrap() here is safe, because
            // this function is a called a start up during the
            // initialization phase of kong.
            std::fs::create_dir(working_dir).unwrap()
        }
    }

    /// Create log file if file logging is enabled and the LOG
    /// file does not yet exist.
    fn create_log_file(config: &Konfig) {
        //let logging = config.log_file
        if let Some(file_logging) = config.log_file {
            if file_logging {
                let working_dir = Konfig::read_working_dir();
                let working_dir_path = std::path::Path::new(&working_dir);
                let log_file = std::path::Path::new(defaults::LOG_FILE);
                let log_file_path = working_dir_path.join(log_file);

                if !std::path::Path::exists(&log_file_path) {
                    // create log file in the working directory
                    // XXX: Note that using unwrap() here is safe, because
                    // this function is a called a start up during the
                    // initialization phase of kong.
                    File::create(log_file_path).unwrap();
                }
            }
        }
    }
}
impl Default for Kong {
    /// Create new kong instance
    fn default() -> Self {
        let config = Konfig::read().expect("Could not read configuration file.");

        Kong::init(&config);

        Kong {
            config,
            kpassport: None,
            input: None,
        }
    }
}

/// API Enpoint kontrollers
pub trait Kontrol {
    /// Endpoint address
    fn address(&self) -> String;
    /// Enpoint method
    fn method(&self) -> Method;

    /// Get user input
    fn get_input(&self, _request: &Request) -> Option<serde_json::Value> {
        None
    }
    /// Validate user input
    fn validate(&self, input: Option<serde_json::Value>) -> Result<Option<serde_json::Value>, ()> {
        Ok(input)
    }

    /// Handle endpoint (business logic)
    fn kontrol(&self, kong: &Kong) -> Response;

    /// url parameters extractor
    fn url_params(
        &self,
        router: &Router<fn(&Kong, &Request) -> Response>,
        url: &str,
    ) -> Result<Params, KError> {
        let router = router.clone();
        let m = router.recognize(url);

        match m {
            Ok(mtch) => Ok(mtch.params().clone()),
            Err(_) => Err(KError::UrlParsing),
        }
    }
}

#[derive(Clone, PartialEq)]
/// HTTP methods
pub enum Method {
    /// HTTP GET method
    Get,
    /// HTTP POST method
    Post,
    /// HTTP PUT method
    Put,
    /// HTTP HEAD method
    Head,
    /// HTTP DELETE method
    Delete,
    /// HTTP OPTIONS method
    Options,
}

impl Copy for Method {}
impl FromStr for Method {
    type Err = KError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "HEAD" => Ok(Method::Head),
            "DELETE" => Ok(Method::Delete),
            "OPTIONS" => Ok(Method::Options),
            _ => Err(KError::InvalidHttpMethod),
        }
    }
}
