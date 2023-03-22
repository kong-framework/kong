#![doc(html_favicon_url = "https://kwatafana.org/logo.jpeg")]
#![doc(html_logo_url = "https://kwatafana.org/logo.jpeg")]
#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

use kerror::KError;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

mod sql {
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

/// Data type to represent a physical property such as a house.
#[derive(Serialize, Deserialize, Clone)]
pub struct Property {
    /// Name of the property
    pub name: String,
    /// Price of the property
    pub price: Option<f64>,
    /// Number of bedrooms
    pub bedrooms: u16,
    /// Number of bathrooms
    pub bathrooms: u16,
    /// Area size
    pub sqft: f64,
    /// Address of property
    pub address: String,
    /// ID of Agent representing the property
    pub agentid: Option<i64>,
    /// Description of the property
    pub description: String,
    /// Number of online views the property got
    pub online_views: u64,
    /// Number of physical irl views the property got
    pub physical_views: u16,
    /// Number of likes for the property
    pub likes: u64,
    /// Number of bookmarks for the property
    pub bookmarks: u64,
    /// Contains a JSON string with paths to the actual images
    pub photos: String,
    /// Date when the property was added
    pub added: String,
}

/// Input provided by the user, to create a new property.
#[derive(Serialize, Deserialize, Clone)]
pub struct PropertyInput {
    /// Name of the property
    pub name: String,
    /// Price of the property
    pub price: Option<f64>,
    /// Number of bedrooms
    pub bedrooms: u16,
    /// Number of bathrooms
    pub bathrooms: u16,
    /// Area size
    pub sqft: f64,
    /// Address of property
    pub address: String,
    /// ID of Agent representing the property
    pub agentid: Option<i64>,
    /// Description of the property
    pub description: String,
}

struct PropertiesDB {
    /// Database file path
    path: String,
    /// An SQLite connection handle
    conn: Option<Connection>,
}

impl PropertiesDB {
    pub fn new(path: &str) -> Self {
        PropertiesDB {
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

                tx.execute(sql::CREATE_PROPERTIES_TABLE, ())
                    .map_err(|_| KError::DbTableCreation)?;

                tx.commit().map_err(|_| KError::DbTableCreation)?;

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
    use super::*;
    const TEST_DB_PATH: &str = "test-data/EUM6O_TEST_DATABASE.sqlite";

    #[test]
    fn test_store_get_property() {
        let mut db = PropertiesDB::new(TEST_DB_PATH);

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
