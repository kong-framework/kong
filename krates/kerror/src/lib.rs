#![doc(html_favicon_url = "https://kwatafana.org/logo.jpeg")]
#![doc(html_logo_url = "https://kwatafana.org/logo.jpeg")]
#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

use std::fmt;

#[derive(Debug)]
pub enum KError {
    /// Database connection error
    DbConnection,
    /// Database table creation error
    DbTableCreation,
    /// Database transaction error
    DbTransaction,
    /// Database sql statement error
    DbSQL,
    /// Database field refferencing error
    DbField,
    /// Password hashing error
    PasswordHashing,
    /// Password hash verification
    PasswordVerifyHash,
    /// Configuration error
    Config,
    /// URL parsing error
    UrlParsing,
    /// Invalid HTTP Method
    InvalidHttpMethod,
}

impl std::error::Error for KError {}

impl fmt::Display for KError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbConnection => write!(f, "Database connection error"),
            Self::DbTableCreation => write!(
                f,
                "Ann error occured while trying to create a database table"
            ),
            Self::DbTransaction => write!(f, "Database transaction error"),
            Self::DbSQL => write!(f, "Something went wrong while processing the SQL statement"),
            Self::DbField => write!(f, "Could not refference the database table field"),
            Self::PasswordHashing => write!(f, "Could not hash password"),
            Self::PasswordVerifyHash => write!(f, "Could not verify password hash"),
            Self::Config => write!(f, "Could not read config file"),
            Self::UrlParsing => write!(f, "Could not parse error"),
            Self::InvalidHttpMethod => write!(f, "Invalid HTTP method"),
        }
    }
}
