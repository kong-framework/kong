//! # 🕹️ inputs
//!
//! Data that is received as input from users, usually other data
//! types are created from this input.

use crate::validate::{Validate, ValidationError};
use serde::Deserialize;

/// User input
pub trait UserInput {
    fn is_valid(&self) -> Result<(), ValidationError>;
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
