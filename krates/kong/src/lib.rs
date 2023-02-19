pub use kdata;
use kdata::accounts::Account;
pub use kerror::KError;
pub use kollection::Kollection;
pub use konfig::Konfig;
use kontrol::{Kontrol, Kontroller};
use std::collections::HashMap;

pub mod kontrol;

pub struct Kong<'a> {
    pub database: Kollection,
    pub config: Konfig,
    pub admin_sessions: HashMap<String, Account>,
    pub kontrollers: Vec<Kontroller<'a>>,
}

impl<'a> Kong<'a> {
    pub fn new(kontrollers: Vec<Kontroller<'a>>) -> Self {
        let config = Konfig::read().unwrap();
        let database = Kollection::new(&config.admin_accounts_database);
        let admin_sessions = HashMap::new();

        Kong {
            database,
            config,
            admin_sessions,
            kontrollers,
        }
    }
    /// Start up runtime
    pub fn start(&mut self) {
        self.database.connect();
    }
}

pub fn kroute(kong: &mut Kong, request: &rouille::Request) -> rouille::Response {
    // Route built in APIs
    match request.url().as_str() {
        kontrol::accounts::ADDRESS => {
            return kontrol::accounts::Accounts::handle_request(kong, request);
        }
        _ => (),
    }

    // Route user provided APIs
    for route in &kong.kontrollers {
        if route.address == request.url().as_str() {
            return (route.handle)(kong, request);
        }
    }

    if let Some(path) = &kong.config.static_files_path {
        let response = rouille::match_assets(request, &path);
        if response.is_success() {
            return response;
        }
    }
    rouille::Response::html("404 error").with_status_code(404)
}
