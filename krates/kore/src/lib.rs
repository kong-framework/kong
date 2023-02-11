use kdata::accounts::Account;
use kollection::Kollection;
use konfig::Konfig;
use std::collections::HashMap;

pub struct Kore {
    pub database: Kollection,
    pub config: Konfig,
    pub admin_sessions: HashMap<String, Account>,
}

impl Kore {
    pub fn new() -> Self {
        let config = Konfig::read().unwrap();
        let database = Kollection::new(&config.admin_accounts_database);
        let admin_sessions = HashMap::new();

        Kore {
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
