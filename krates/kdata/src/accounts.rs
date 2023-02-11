use crate::inputs::AccountCreationInput;
use chrono::prelude::*;
use krypto::Krypto;
use serde::{Deserialize, Serialize};

/// A generic account
#[derive(Deserialize, Serialize)]
pub struct Account {
    //--- Required Data ---//
    /// Account's unique username
    pub username: String,
    /// Account's master key
    pub password: String,
    /// Date when account was created
    pub created: DateTime<Utc>,
    //--- Optional Personal Data ---//
    /// The fullname of the account holder.
    pub fullname: Option<String>,
    /// The date when the account holder was born.
    pub date_of_birth: Option<DateTime<Utc>>,
    /// ID number of the account owner
    pub id_number: Option<String>,
    /// The gender of the account holder
    pub gender: Option<String>,
    //--- Optional Education Data ---//
    /// User's current school name
    pub current_school_name: Option<String>,
    /// User's student number
    pub student_number: Option<String>,
    //--- Optional Bussiness Data ---//
    /// Name of the account's bussiness
    pub bussiness_name: Option<String>,
    //--- Optional Contact Data ---//
    /// User's email address
    pub email: Option<String>,
    /// Account owner's mobile number
    pub mobile_number: Option<String>,
    /// Account owner's web-address
    pub website: Option<String>,
    //--- Optional Meta Data ---//
    /// Short bio of Account
    pub description: Option<String>,
    /// Date account last logged in
    pub last_login: Option<DateTime<Utc>>,
}

impl From<AccountCreationInput> for Account {
    fn from(input: AccountCreationInput) -> Self {
        let password = Krypto::hash_password(&input.password).unwrap();

        Account {
            username: input.username,
            password,
            created: Utc::now(),
            fullname: None,
            date_of_birth: None,
            id_number: None,
            gender: None,
            current_school_name: None,
            student_number: None,
            bussiness_name: None,
            email: None,
            mobile_number: None,
            website: None,
            description: None,
            last_login: None,
        }
    }
}

/// Account Public Data
#[derive(Deserialize, Serialize)]
pub struct PublicAccount {
    /// The username of the user, also used as an unique identifier
    pub username: String,
}

impl From<Account> for PublicAccount {
    fn from(account: Account) -> Self {
        PublicAccount {
            username: account.username,
        }
    }
}
