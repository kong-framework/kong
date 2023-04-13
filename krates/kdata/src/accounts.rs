use crate::{inputs::AccountCreationInput, resource::Resource};
use chrono::prelude::*;
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
    /// Short bio of Account
    pub description: Option<String>,
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
    /// Date account last logged in
    pub last_login: Option<DateTime<Utc>>,
}

impl Resource for Account {
    fn is_authorized(&self, kpassport: krypto::kpassport::Kpassport) -> bool {
        if kpassport.content.username == self.username {
            true
        } else {
            false
        }
    }
}

impl From<AccountCreationInput> for Account {
    fn from(input: AccountCreationInput) -> Account {
        let password = krypto::password::hash(&input.password).unwrap();

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
            email: input.email,
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

impl Resource for PublicAccount {
    fn is_authorized(&self, _kpassport: krypto::kpassport::Kpassport) -> bool {
        // everyone can access a public account
        true
    }
}

impl From<Account> for PublicAccount {
    fn from(account: Account) -> Self {
        PublicAccount {
            username: account.username,
        }
    }
}
