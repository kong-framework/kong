use kdata::{
    accounts::{Account, PublicAccount},
    property::Property,
};
use kerror::KError;

use rusqlite::{params, Connection};

pub mod sql {
    pub const CREATE_ACCOUNTS_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS accounts (
        id INTEGER PRIMARY KEY,                      -- The Identifier of the account, the Rust Type is `i64`
        username TEXT UNIQUE NOT NULL,               -- The username of the account
        password TEXT NOT NULL,                      -- The user's login password
        created TEXT DEFAULT(date('now')) NOT NULL,  -- The date when the account was created, the Rust Type is `chrono::DateTime`
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
        password
       )
      VALUES (?1, ?2, ?3)";

    pub const CREATE_PROPERTIES_TABLE: &str = "
      CREATE TABLE IF NOT EXISTS properties (
        id INTEGER PRIMARY KEY,                       -- The Identifier of the property, the Rust Type is `i64`
        name TEXT NOT NULL,                           -- Name of the property
        price FLOAT,                                  -- Price of the property
        bedrooms INTEGER DEFAULT(0) NOT NULL,         -- Number of bedrooms
        bathrooms INTEGER DEFAULT(0) NOT NULL,        -- Number of bathrooms
        sqft FLOAT,                                   -- Square foot area size of the property
        address TEXT,                                 -- Address of the property
        agentid INTEGER,                              -- The id of the agent in charge of the property
        description TEXT,                             -- A description of the property
        online_views INTEGER DEFAULT(0) NOT NULL,     -- Number of online views the property got
        physical_views INTEGER DEFAULT(0) NOT NULL,   -- Number of physical irl views the property got
        likes INTEGER DEFAULT(0) NOT NULL,            -- Number of times the property has been liked
        bookmarks INTEGER DEFAULT(0) NOT NULL,        -- Number of times the property has been bookmarked
        photos TEXT,                                  -- Contains a JSON string with paths to the actual images
        added TEXT DEFAULT(date('now')) NOT NULL)     -- The date when the property was added, the Rust Type is `chrono::DateTime`";

    /// Add a property
    pub const ADD_PROPERTY: &str = "
      INSERT INTO properties (name,
        price,
        bedrooms,
        bathrooms,
        sqft,
        address,
        agentid,
        description,
        online_views,
        physical_views,
        likes,
        bookmarks,
        photos,
        added
       )
      VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)";

    /// Get property by id
    pub const GET_PROPERTY_BY_ID: &str = "SELECT * FROM properties WHERE id = ?;";
}

/// database controller
pub struct Kollection {
    /// Database file path
    path: String,
    /// An SQLite connection handle
    conn: Option<Connection>,
}

impl Kollection {
    pub fn new(path: &str) -> Self {
        Kollection {
            path: path.to_string(),
            conn: None,
        }
    }

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

                tx.execute(sql::CREATE_PROPERTIES_TABLE, ())
                    .map_err(|_| KError::DbTableCreation)?;

                tx.commit().map_err(|_| KError::DbTableCreation)?;

                Ok(())
            }
            None => Err(KError::DbConnection),
        }
    }

    fn get_account_by_username(&self, username: &str) -> Result<Option<PublicAccount>, KError> {
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

    pub fn get_account_by_email(&self, email: &str) -> Result<Option<PublicAccount>, KError> {
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

    pub fn get_account_by_email_private(&self, email: &str) -> Result<Option<Account>, KError> {
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

    pub fn create_account_account(&self, account: &Account) -> Result<(), KError> {
        match &self.conn {
            Some(conn) => {
                conn.execute(
                    sql::CREATE_ACCOUNT,
                    params![&account.username, &account.email, account.password],
                )
                .map_err(|_| KError::DbField)?;
                Ok(())
            }
            None => Err(KError::DbConnection),
        }
    }

    pub fn add_property(&self, property: &Property) -> Result<(), KError> {
        match &self.conn {
            Some(conn) => {
                conn.execute(
                    sql::ADD_PROPERTY,
                    params![
                        &property.name,
                        &property.price,
                        &property.bedrooms,
                        &property.bathrooms,
                        &property.sqft,
                        &property.address,
                        &property.agentid,
                        &property.description,
                        &property.online_views,
                        &property.physical_views,
                        &property.likes,
                        &property.bookmarks,
                        &property.photos,
                        &property.added
                    ],
                )
                .map_err(|_| KError::DbField)?;
                Ok(())
            }
            None => Err(KError::DbConnection),
        }
    }

    pub fn get_property_by_id(&self, id: i64) -> Result<Option<Property>, KError> {
        match &self.conn {
            Some(conn) => {
                let mut stmt = conn
                    .prepare(sql::GET_PROPERTY_BY_ID)
                    .map_err(|_| KError::DbSQL)?;
                let mut properties: Vec<Property> = vec![];

                let property_iter = stmt
                    .query_map(params![id], |s| {
                        Ok(Property {
                            name: s.get(1).map_err(|_| KError::DbField).unwrap(),
                            price: s.get(2).map_err(|_| KError::DbField).unwrap(),
                            bedrooms: s.get(3).map_err(|_| KError::DbField).unwrap(),
                            bathrooms: s.get(4).map_err(|_| KError::DbField).unwrap(),
                            sqft: s.get(5).map_err(|_| KError::DbField).unwrap(),
                            address: s.get(6).map_err(|_| KError::DbField).unwrap(),
                            agentid: s.get(7).map_err(|_| KError::DbField).unwrap(),
                            description: s.get(8).map_err(|_| KError::DbField).unwrap(),
                            online_views: s.get(9).map_err(|_| KError::DbField).unwrap(),
                            physical_views: s.get(10).map_err(|_| KError::DbField).unwrap(),
                            likes: s.get(11).map_err(|_| KError::DbField).unwrap(),
                            bookmarks: s.get(12).map_err(|_| KError::DbField).unwrap(),
                            photos: s.get(13).map_err(|_| KError::DbField).unwrap(),
                            added: s.get(14).map_err(|_| KError::DbField).unwrap(),
                        })
                    })
                    .unwrap();

                for c in property_iter {
                    properties.push(c.unwrap());
                }

                if properties.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(properties[0].clone()))
                }
            }
            None => Err(KError::DbConnection),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};

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
        db.create_account_account(&account);

        let public_account = db.get_account_by_email("admin@example.com").unwrap();
        let public_account1 = db.get_account_by_username("testuszee").unwrap();

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
    fn test_store_get_property() {
        let mut db = Kollection::new(TEST_DB_PATH);

        let property = Property {
            name: "Luxury Hill".to_string(),
            price: None,
            bedrooms: 1,
            bathrooms: 1,
            sqft: 334.44,
            address: "Windhoek".to_string(),
            agentid: None,
            description: "Cool place".to_string(),
            online_views: 0,
            physical_views: 0,
            likes: 0,
            bookmarks: 0,
            photos: "".to_string(),
            added: "".to_string(),
        };

        db.connect().unwrap();
        db.add_property(&property).unwrap();

        let pr = db.get_property_by_id(1).unwrap();

        if let Some(p) = pr {
            assert_eq!(&p.name, &property.name);
        } else {
            panic!("Could not get property from database");
        }
    }

    fn remove_test_db() {
        let test_db_path = std::path::Path::new(TEST_DB_PATH);
        if std::path::Path::exists(test_db_path) {
            std::fs::remove_file(test_db_path).unwrap();
        }
    }
}
