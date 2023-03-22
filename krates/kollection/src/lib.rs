//! # 🗄️ kollection
//!
//! Kong stores structured data in a database. [SQLite](https://sqlite.org/)
//! is the database management system that is used, with help from the
//! [rusqlite](https://github.com/rusqlite/rusqlite/) crate.
//!
//! The following data is stored the database:
//! - `kdata::accounts::Account`

use kdata::accounts::{Account, PublicAccount};
use kerror::KError;

use rusqlite::{params, Connection};

/// SQL statements and queries
pub mod sql {
    pub const CREATE_ACCOUNTS_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS accounts (
        id INTEGER PRIMARY KEY,                      -- The Identifier of the account, the Rust Type is `i64`
        username TEXT UNIQUE NOT NULL,               -- The username of the account
        password TEXT NOT NULL,                      -- The user's login password
        created TEXT NOT NULL,                       -- The date when the account was created, the Rust Type is `chrono::DateTime`
        fullname TEXT,                               -- Fullname of the account
        date_of_birth TEXT,                          -- The date when the account holder was born
        id_number TEXT,                              -- ID number of the account owner
        gender TEXT,                                 -- The gender of the account holder
        current_school_name TEXT,                    -- User's current school name
        student_number TEXT,                         -- User's student number
        bussiness_name TEXT,                         -- Name of the account's bussiness        
        email TEXT UNIQUE,                           -- The email address of the account
        mobile_number TEXT,                          -- Account owner's mobile number
        website TEXT,                                -- Account owner's web-address
        description TEXT,                            -- Short bio of Account
        last_login TEXT)                             -- Date account last logged in";

    /// Get account by username
    pub const GET_ACCOUNT_BY_USERNAME: &str = "SELECT * FROM accounts WHERE username = :username;";

    /// Get account by email
    pub const GET_ACCOUNT_BY_EMAIL: &str = "SELECT * FROM accounts WHERE email = :email;";

    /// Insert a account in the accounts table
    pub const CREATE_ACCOUNT: &str = "
      INSERT INTO accounts (
        username,
        email,
        password,
        created
       )
      VALUES (?1, ?2, ?3, ?4)";
}

/// Database management system
pub struct Kollection {
    /// Database file path
    path: String,
    /// An SQLite connection handle
    conn: Option<Connection>,
}

impl Kollection {
    /// Create a new database controller
    pub fn new(path: &str) -> Self {
        Kollection {
            path: path.to_string(),
            conn: None,
        }
    }

    /// Open SQLite connection, create tables
    pub fn connect(&mut self) -> Result<(), KError> {
        // Open database connection
        let mut conn = Connection::open(self.path.clone()).map_err(|_| KError::DbConnection)?;
        self.conn = Some(conn);

        // Create database tables if they do not already exist
        match &mut self.conn {
            Some(conn) => {
                let tx = conn.transaction().map_err(|_| KError::DbTransaction)?;

                tx.execute(sql::CREATE_ACCOUNTS_TABLE, ())
                    .map_err(|_| KError::DbTableCreation)?;

                tx.commit().map_err(|_| KError::DbTableCreation)?;

                Ok(())
            }
            None => Err(KError::DbConnection),
        }
    }

    /// Create a new account
    pub fn create_account(&self, account: &Account) -> Result<(), KError> {
        match &self.conn {
            Some(conn) => {
                conn.execute(
                    sql::CREATE_ACCOUNT,
                    params![
                        &account.username,
                        &account.email,
                        account.password,
                        account.created
                    ],
                )
                .map_err(|_| KError::DbField)?;
                Ok(())
            }
            None => Err(KError::DbConnection),
        }
    }

    /// Get an account's public data by its username
    pub fn public_get_account_by_username(
        &self,
        username: &str,
    ) -> Result<Option<PublicAccount>, KError> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare(sql::GET_ACCOUNT_BY_USERNAME)
                    .map_err(|_| KError::DbSQL)?;
                let mut rows = stmt
                    .query(&[(":username", username)])
                    .map_err(|_| KError::DbSQL)?;
                match rows.next().map_err(|_| KError::DbSQL)? {
                    Some(s) => Ok(Some(PublicAccount {
                        username: s.get(2).map_err(|_| KError::DbField)?,
                    })),
                    None => Ok(None),
                }
            }
            None => Err(KError::DbConnection),
        }
    }

    /// Get an account's public data by its email
    pub fn public_get_account_by_email(
        &self,
        email: &str,
    ) -> Result<Option<PublicAccount>, KError> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare(sql::GET_ACCOUNT_BY_EMAIL)
                    .map_err(|_| KError::DbSQL)?;
                let mut rows = stmt
                    .query(&[(":email", email)])
                    .map_err(|_| KError::DbSQL)?;
                match rows.next().map_err(|_| KError::DbSQL)? {
                    Some(s) => Ok(Some(PublicAccount {
                        username: s.get(2).map_err(|_| KError::DbField)?,
                    })),
                    None => Ok(None),
                }
            }
            None => Err(KError::DbConnection),
        }
    }

    /// Get an account's private data by its email
    pub fn private_get_account_by_email(&self, email: &str) -> Result<Option<Account>, KError> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare(sql::GET_ACCOUNT_BY_EMAIL)
                    .map_err(|_| KError::DbSQL)?;
                let mut rows = stmt
                    .query(&[(":email", email)])
                    .map_err(|_| KError::DbSQL)?;
                match rows.next().map_err(|_| KError::DbSQL)? {
                    Some(s) => Ok(Some(Account {
                        username: s.get(1).map_err(|_| KError::DbField)?,
                        password: s.get(2).map_err(|_| KError::DbField)?,
                        created: s.get(3).unwrap(), //.map_err(|_| KError::DbField)?,
                        fullname: s.get(4).map_err(|_| KError::DbField)?,
                        date_of_birth: s.get(5).map_err(|_| KError::DbField)?,
                        id_number: s.get(6).map_err(|_| KError::DbField)?,
                        gender: s.get(7).map_err(|_| KError::DbField)?,
                        current_school_name: s.get(8).map_err(|_| KError::DbField)?,
                        student_number: s.get(9).map_err(|_| KError::DbField)?,
                        bussiness_name: s.get(10).map_err(|_| KError::DbField)?,
                        email: s.get(11).map_err(|_| KError::DbField)?,
                        mobile_number: s.get(12).map_err(|_| KError::DbField)?,
                        website: s.get(13).map_err(|_| KError::DbField)?,
                        description: s.get(14).map_err(|_| KError::DbField)?,
                        last_login: s.get(15).map_err(|_| KError::DbField)?,
                    })),
                    None => Ok(None),
                }
            }
            None => Err(KError::DbConnection),
        }
    }

    /// Get an account's private data by its username
    pub fn private_get_account_by_username(
        &self,
        username: &str,
    ) -> Result<Option<Account>, KError> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare(sql::GET_ACCOUNT_BY_USERNAME)
                    .map_err(|_| KError::DbSQL)?;
                let mut rows = stmt
                    .query(&[(":username", username)])
                    .map_err(|_| KError::DbSQL)?;
                match rows.next().map_err(|_| KError::DbSQL)? {
                    Some(s) => Ok(Some(Account {
                        username: s.get(1).map_err(|_| KError::DbField)?,
                        password: s.get(2).map_err(|_| KError::DbField)?,
                        created: s.get(3).map_err(|_| KError::DbField)?,
                        fullname: s.get(4).map_err(|_| KError::DbField)?,
                        date_of_birth: s.get(5).map_err(|_| KError::DbField)?,
                        id_number: s.get(6).map_err(|_| KError::DbField)?,
                        gender: s.get(7).map_err(|_| KError::DbField)?,
                        current_school_name: s.get(8).map_err(|_| KError::DbField)?,
                        student_number: s.get(9).map_err(|_| KError::DbField)?,
                        bussiness_name: s.get(10).map_err(|_| KError::DbField)?,
                        email: s.get(11).map_err(|_| KError::DbField)?,
                        mobile_number: s.get(12).map_err(|_| KError::DbField)?,
                        website: s.get(13).map_err(|_| KError::DbField)?,
                        description: s.get(14).map_err(|_| KError::DbField)?,
                        last_login: s.get(15).map_err(|_| KError::DbField)?,
                    })),
                    None => Ok(None),
                }
            }
            None => Err(KError::DbConnection),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;

    use super::*;

    const TEST_DB_PATH: &str = "test-data/EUM6O_TEST_DATABASE.sqlite";

    #[test]
    fn connect_db() {
        let mut db = Kollection::new(TEST_DB_PATH);

        // Connect to database
        db.connect().unwrap();

        match db.conn {
            Some(_conn) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_store_get_account_account() {
        remove_test_db();
        let mut db = Kollection::new(TEST_DB_PATH);
        let account = Account {
            username: String::from("testuszee"),
            password: String::from("12345678910"),
            created: Utc::now(),
            fullname: None,
            date_of_birth: None,
            id_number: None,
            gender: None,
            current_school_name: None,
            student_number: None,
            bussiness_name: None,
            email: Some("admin@example.com".to_string()),
            mobile_number: None,
            website: None,
            description: None,
            last_login: None,
        };

        db.connect().unwrap();
        db.create_account(&account).unwrap();

        let public_account = db.public_get_account_by_email("admin@example.com").unwrap();
        let public_account1 = db.public_get_account_by_username("testuszee").unwrap();

        if let Some(_) = public_account {
            assert!(true)
        } else {
            panic!("Account not found")
        }

        if let Some(_) = public_account1 {
            assert!(true)
        } else {
            panic!("Account not found")
        }
    }

    #[test]
    fn test_store_get_account_account_private() {
        //remove_test_db();
        let mut db = Kollection::new(TEST_DB_PATH);
        let account = Account {
            username: String::from("testus"),
            password: String::from("12345678910"),
            created: Utc::now(),
            fullname: None,
            date_of_birth: None,
            id_number: None,
            gender: None,
            current_school_name: None,
            student_number: None,
            bussiness_name: None,
            email: Some("admin@ple.com".to_string()),
            mobile_number: None,
            website: None,
            description: None,
            last_login: None,
        };

        db.connect().unwrap();
        db.create_account(&account).unwrap();

        let public_account = db.private_get_account_by_email("admin@ple.com").unwrap();
        let public_account1 = db.private_get_account_by_username("testus").unwrap();

        if let Some(_) = public_account {
            assert!(true)
        } else {
            panic!("Account not found")
        }

        if let Some(_) = public_account1 {
            assert!(true)
        } else {
            panic!("Account not found")
        }
    }

    fn remove_test_db() {
        let test_db_path = std::path::Path::new(TEST_DB_PATH);
        if std::path::Path::exists(test_db_path) {
            std::fs::remove_file(test_db_path).unwrap();
        }
    }
}
