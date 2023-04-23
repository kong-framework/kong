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

mod kontrol;
mod kroute;
pub mod prelude;
use kollection::{Kollection, KollectionInput};
use konfig::{
    defaults::{ACCONTS_DB, DBS_DIRECTORY, WORKING_DIRECTORY},
    Konfig,
};
use krypto::kpassport::Kpassport;

/// Kong object
pub struct Kong {
    /// Kong database
    pub database: Kollection,
    /// Kong configuration
    pub config: Konfig,
    /// Request authentication + authorization token
    pub kpassport: Option<Kpassport>,
    /// Validated user input
    pub input: Option<serde_json::Value>,
}

impl Kong {
    /// Create new kong instance
    pub fn new() -> Self {
        let config = Konfig::read().expect("Could not read configuration file.");
        create_dirs(&config);
        let mut kollection_input = KollectionInput { accounts: None };

        if config.accounts {
            let working_dir = if let Some(dir) = &config.working_directory {
                dir.clone()
            } else {
                WORKING_DIRECTORY.to_string()
            };
            let db = ACCONTS_DB;
            let path = format!("{working_dir}{db}");
            kollection_input.accounts = Some(path);
        }

        let database = Kollection::new(kollection_input);

        Kong {
            database,
            config,
            kpassport: None,
            input: None,
        }
    }
    /// Start up runtime
    pub fn start(&mut self) -> Result<(), kerror::KError> {
        self.database.connect()
    }
}

/// Create the directories that are required by kong, if they dont
/// already exit
fn create_dirs(config: &Konfig) {
    let working_dir = if let Some(working_dir) = &config.working_directory {
        working_dir.clone()
    } else {
        WORKING_DIRECTORY.to_string()
    };
    let dbs = DBS_DIRECTORY;
    let database_dir = format!("{working_dir}{dbs}");
    let database_dir = std::path::Path::new(&database_dir);
    let working_dir = std::path::Path::new(&working_dir);

    if !std::path::Path::exists(working_dir) {
        // create working directory
        std::fs::create_dir(working_dir).unwrap()
    }

    if !std::path::Path::exists(database_dir) {
        // create databases directory
        std::fs::create_dir(database_dir).unwrap()
    }
}
