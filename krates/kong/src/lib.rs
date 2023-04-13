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

use kdata::{inputs::UserInput, resource::Resource};
use kollection::Kollection;
use konfig::Konfig;
use kontrol::{Kontrol, Kontroller, Method};
use route_recognizer::Router;

/// Kong object
pub struct Kong<I: UserInput, R: Resource + serde::Serialize> {
    /// Kong database
    pub database: Kollection,
    /// Kong configuration
    pub config: Konfig,
    /// Kong router
    pub router: Router<RouterObject<I, R>>,
}

impl<I: UserInput, R: Resource + serde::Serialize> Kong<I, R> {
    /// Create new kong instance
    pub fn new<'a>(kontrollers: Vec<Kontroller<'a, I, R>>) -> Self {
        let config = Konfig::read().expect("Could not read configuration file.");
        let admin_db_path = if let Some(path) = &config.admin_accounts_database {
            path.clone()
        } else {
            "databases/ADMINS.sqlite".to_string()
        };

        let database = Kollection::new(&admin_db_path);
        let mut router = Router::new();

        for kontroller in &kontrollers {
            let router_object = RouterObject {
                kontrol: kontroller.kontrol.clone(),
                method: kontroller.method,
            };

            router.add(kontroller.address, router_object);
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

pub struct RouterObject<I: UserInput, R: Resource + serde::Serialize> {
    kontrol: Kontrol<I, R>,
    method: Method,
}

impl<I: UserInput, R: Resource + serde::Serialize> Copy for RouterObject<I, R> {}

impl<I: UserInput, R: Resource + serde::Serialize> Clone for RouterObject<I, R> {
    fn clone(&self) -> Self {
        *self
    }
}
