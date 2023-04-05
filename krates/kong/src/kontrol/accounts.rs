//! Accounts API endpoint controller

use super::{Kontrol, KontrolError};
use crate::Kong;
use kdata::{
    accounts::{Account, PublicAccount},
    inputs::AccountCreationInput,
};
use rouille::{try_or_400, Request, Response};

/// Accounts API endpoint controller
pub struct AccountsKontroller;

impl Kontrol for AccountsKontroller {
    /// Validate user input
    fn validate_user_input(input: impl kdata::inputs::UserInput) -> bool {
        if input.is_valid().is_ok() {
            true
        } else {
            false
        }
    }
}

impl AccountsKontroller {
    /// Create a new user
    pub fn create(kong: &mut Kong, request: &Request) -> Response {
        let input: AccountCreationInput = try_or_400!(rouille::input::json_input(request));

        if Self::validate_user_input(input.clone()) {
            let account: Account = input.into();

            match kong.database.create_account(&account) {
                Ok(_) => {
                    let public_account: PublicAccount = account.into();
                    Response::json(&public_account).with_status_code(201)
                }

                Err(err) => match err {
                    kerror::KError::DbField => Response::json(&KontrolError {
                        msg: "Invalid input".to_owned(),
                    })
                    .with_status_code(401),
                    _ => Response::json(&KontrolError {
                        msg: "Could not create account".to_owned(),
                    })
                    .with_status_code(500),
                },
            }
        } else {
            Response::json(&KontrolError {
                msg: "Invalid Input".to_owned(),
            })
            .with_status_code(400)
        }
    }
}
