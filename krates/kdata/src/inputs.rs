//! # 🕹️ inputs
//!
//! Data that is received as input from users, usually other data
//! types are created from this input.

use crate::validate::{Validate, ValidationError};
use serde::Deserialize;
use serde_json::json;

/// User input
pub trait UserInput {
    /// Validate user input
    fn is_valid(&self) -> Result<(), ValidationError>;
}

/// Used when no user input is expected
pub struct NoInput;

impl UserInput for NoInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

/// Data used as input to create a new account.
#[derive(Deserialize, Clone)]
pub struct AccountCreationInput {
    /// Account's username
    pub username: String,
    /// Account email address
    pub email: Option<String>,
    /// Account master key
    pub password: String,
}

impl AccountCreationInput {
    /// new generic resource
    pub fn as_json(&self) -> serde_json::Value {
        json!({
            "username": self.username,
            "email": self.email,
            "password": self.password
        })
    }

    /// from json
    pub fn from_json_str(
        json_str: String,
    ) -> Result<AccountCreationInput, serde_json::error::Error> {
        let a: AccountCreationInput = serde_json::from_str(&json_str)?;
        Ok(a)
    }
}
impl UserInput for AccountCreationInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        if !Validate::username(&self.username) {
            return Err(ValidationError::Username);
        }

        if !Validate::password(&self.password) {
            return Err(ValidationError::Password);
        }

        if let Some(email) = &self.email {
            if !Validate::email(email) {
                return Err(ValidationError::Email);
            }
        }

        Ok(())
    }
}

/// Account authentication input
#[derive(Deserialize, Clone)]
pub struct AccountAuthInput {
    /// Account's username
    pub username: String,
    /// Account master key
    pub password: String,
}

impl UserInput for AccountAuthInput {
    fn is_valid(&self) -> Result<(), ValidationError> {
        if !Validate::username(&self.username) {
            return Err(ValidationError::Username);
        }

        if !Validate::password(&self.password) {
            return Err(ValidationError::Password);
        }

        Ok(())
    }
}
