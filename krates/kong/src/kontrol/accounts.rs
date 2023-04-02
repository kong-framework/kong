//! Accounts API endpoint controller

use super::KontrolError;
use crate::Kong;
use kdata::{
    accounts::{Account, PublicAccount},
    inputs::AccountCreationInput,
};
use rouille::{try_or_400, Request, Response};

/// Accounts API endpoint controller
pub struct AccountsKontroller;

impl AccountsKontroller {
    /// Kontrol request by matching the methods to their handlers
    pub fn kontrol(kong: &mut Kong, req: &Request) -> Response {
        match req.method() {
            "POST" => Self::post(kong, req),
            _ => Response::html("404 error").with_status_code(404),
        }
    }

    /// POST request handler
    fn post(kong: &mut Kong, request: &Request) -> Response {
        let input: AccountCreationInput = try_or_400!(rouille::input::json_input(request));

        if Self::validate_user_input(input.clone()) {
            AccountsKontroller::create(input, kong)
        } else {
            Response::json(&KontrolError {
                msg: "Invalid Input".to_owned(),
            })
            .with_status_code(400)
        }
    }

    /// Validate user input
    fn validate_user_input(input: impl kdata::inputs::UserInput) -> bool {
        if input.is_valid().is_ok() {
            true
        } else {
            false
        }
    }

    fn create(input: AccountCreationInput, kong: &Kong) -> Response {
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
