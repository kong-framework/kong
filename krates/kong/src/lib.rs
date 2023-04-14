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
use kollection::{Kollection, KollectionInput};
use konfig::{defaults::WORKING_DIRECTORY, Konfig};
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
        let mut kollection_input = KollectionInput { accounts: None };

        if config.accounts {
            let working_dir = WORKING_DIRECTORY;
            let path = format!("{working_dir}db");
            kollection_input.accounts = Some(path);
        }

        let database = Kollection::new(kollection_input);
        let mut router = Router::new();

        for kontroller in &kontrollers {
            let router_object = RouterObject {
                kontrol: kontroller.kontrol,
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

/// Requests object to processed by Kroute
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
