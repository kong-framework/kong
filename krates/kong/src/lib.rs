pub use kdata;
use kdata::accounts::Account;
pub use kerror::KError;
pub use kollection::Kollection;
pub use konfig::Konfig;
use konnect::Konnect;
use std::collections::HashMap;

pub mod konnect;

pub struct Kong {
    pub database: Kollection,
    pub config: Konfig,
    pub admin_sessions: HashMap<String, Account>,
}

impl Kong {
    pub fn new() -> Self {
        let config = Konfig::read().unwrap();
        let database = Kollection::new(&config.admin_accounts_database);
        let admin_sessions = HashMap::new();

        Kong {
            database,
            config,
            admin_sessions,
        }
    }
    /// Start up runtime
    pub fn start(&mut self) {
        self.database.connect();
    }
}

pub fn kroute(
    kore: &mut Kong,
    request: &rouille::Request,
    kustom_routes: Vec<konnect::KustomRoute>,
    static_files_path: &str,
) -> rouille::Response {
    // Route built in APIs
    match request.url().as_str() {
        konnect::accounts::ADDRESS => {
            return konnect::accounts::Accounts::handle_request(kore, request);
        }
        _ => (),
    }

    // Route user provided APIs
    for route in kustom_routes {
        if route.address == request.url().as_str() {
            return (route.handler)(kore, request);
        }
    }

    let response = rouille::match_assets(request, static_files_path);

    if response.is_success() {
        return response;
    }

    rouille::Response::html("404 error").with_status_code(404)
}
