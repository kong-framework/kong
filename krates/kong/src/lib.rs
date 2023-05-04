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
use konfig::Konfig;
use krypto::kpassport::Kpassport;

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
    /// Create new kong instance
    pub fn new() -> Self {
        let config = Konfig::read().expect("Could not read configuration file.");

        Kong {
            config,
            kpassport: None,
            input: None,
        }
    }
}
