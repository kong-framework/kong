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

pub use kdata;
pub use kerror::KError;
pub use kollection::Kollection;
pub use konfig::Konfig;
use kontrol::{Kontrol, Kontroller};
pub mod kontrol;
pub mod kroute;
pub mod outsource;
use rouille::{Request, Response};
use route_recognizer::Router;

/// Kong object
pub struct Kong {
    /// Kong database
    pub database: Kollection,
    /// Kong configuration
    pub config: Konfig,
    /// Kong router
    pub router: Router<for<'a, 'b> fn(&mut Kong, &'b Request) -> Response>,
}

impl Kong {
    /// Create new kong instance
    pub fn new<'a>(kontrollers: Vec<Kontroller<'a>>) -> Self {
        let config = Konfig::read().expect("Could not read configuration file.");
        let admin_db_path = if let Some(path) = &config.admin_accounts_database {
            path.clone()
        } else {
            "databases/ADMINS.sqlite".to_string()
        };

        let database = Kollection::new(&admin_db_path);
        let mut router = Router::new();

        for kontroller in &kontrollers {
            router.add(kontroller.address, kontroller.handle);
        }

        Kong {
            database,
            config,
            router,
        }
    }
    /// Start up runtime
    pub fn start(&mut self) -> Result<(), kerror::KError> {
        self.database.connect()
    }
}
