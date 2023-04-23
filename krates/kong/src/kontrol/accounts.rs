//! Accounts API endpoint controller

use super::{Kontrol, Method};
use crate::Kong;
use kdata::{
    accounts::{Account, PublicAccount},
    inputs::{AccountCreationInput, UserInput},
    resource::ResourceError,
};
use rouille::Request;

/// Accounts API endpoint handler
pub struct CreateAccountKontroller;

impl Kontrol for CreateAccountKontroller {
    fn address(&self) -> String {
        "/accounts".to_string()
    }

    fn method(&self) -> Method {
        Method::Post
    }

    fn get_input(&self, request: &Request) -> Option<serde_json::Value> {
        // TODO: don't use unwrap
        let input: AccountCreationInput = rouille::input::json_input(request).unwrap();
        Some(input.as_json())
    }

    /// Validate user input
    fn validate(&self, input: Option<serde_json::Value>) -> Result<Option<serde_json::Value>, ()> {
        if let Some(input) = input {
            let input = AccountCreationInput::from_json_str(input.to_string());

            match input {
                Ok(input) => {
                    if input.is_valid().is_ok() {
                        Ok(Some(input.as_json()))
                    } else {
                        Err(())
                    }
                }
                Err(_) => Err(()),
            }
        } else {
            Err(())
        }
    }
    /// Create a new user
    fn kontrol(&self, kong: &Kong) -> Result<serde_json::Value, ResourceError> {
        if let Some(input) = &kong.input {
            let input = AccountCreationInput::from_json_str(input.to_string());

            match input {
                Ok(input) => {
                    let mut account: Account = input.clone().into();

                    // create admin account
                    if input.email == kong.config.admin_email {
                        account.account_type = Some("admin".to_string());
                        match kong.database.create_admin_account(&account) {
                            Ok(_) => {
                                let public_account: PublicAccount = account.into();
                                Ok(public_account.as_json())
                            }
                            Err(err) => match err {
                                kerror::KError::DbField => Err(ResourceError::BadRequest),
                                _ => Err(ResourceError::Internal),
                            },
                        }
                    } else {
                        // Create normal account
                        match kong.database.create_account(&account) {
                            Ok(_) => {
                                let public_account: PublicAccount = account.into();
                                Ok(public_account.as_json())
                            }

                            Err(err) => match err {
                                kerror::KError::DbField => Err(ResourceError::BadRequest),
                                _ => Err(ResourceError::Internal),
                            },
                        }
                    }
                }

                Err(_) => Err(ResourceError::BadRequest),
            }
        } else {
            Err(ResourceError::BadRequest)
        }
    }
}
