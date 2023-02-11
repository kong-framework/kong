use crate::validate::{Validate, ValidationError};
use serde::Deserialize;

/// Data used as input to create a new account.
#[derive(Deserialize)]
pub struct AccountCreationInput {
    /// Account's username
    pub username: String,
    /// Account email address
    pub email: Option<String>,
    /// Account master key
    pub password: String,
}

impl AccountCreationInput {
    pub fn is_valid(&self) -> Result<(), ValidationError> {
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

/// Input provided by the user, to log into their account.
#[derive(Deserialize)]
pub struct AccountLoginInput {
    /// Account's username
    pub username: String,
    /// Account master key
    pub password: String,
}
