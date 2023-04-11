//! Accounts API endpoint controller

use super::{Kontrol, KontrolError, KontrolHandle};
use crate::Kong;
use kdata::{
    accounts::{Account, PublicAccount},
    inputs::{AccountCreationInput, UserInput},
};
use rouille::{try_or_400, Request, Response};

/// Accounts API endpoint controller
pub struct CreateAccountKontroller;

impl KontrolHandle<AccountCreationInput> for CreateAccountKontroller {
    fn get_input(request: &Request) -> AccountCreationInput {
        // TODO: don't use unwrap
        let input: AccountCreationInput = rouille::input::json_input(request).unwrap();
        input
    }

    /// Create a new user
    fn handle(kong: &mut Kong<AccountCreationInput>, input: AccountCreationInput) -> Response {
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
    }
}
