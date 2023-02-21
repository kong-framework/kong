use super::{Kontrol, KontrolError};
use crate::Kong;
use kdata::{
    accounts::{Account, PublicAccount},
    inputs::AccountCreationInput,
};
use rouille::{try_or_400, Request, Response};

pub const ADDRESS: &str = "/accounts";

pub struct AccountsKontroller;

impl Kontrol for AccountsKontroller {
    fn post(kong: &mut Kong, request: &Request) -> Response {
        let input: AccountCreationInput = try_or_400!(rouille::input::json_input(request));
        let validation_result = input.is_valid();

        match validation_result {
            Ok(_) => AccountsKontroller::create(input, kong),
            Err(_) => Response::json(&KontrolError {
                msg: "Invalid Input".to_owned(),
            })
            .with_status_code(400),
        }
    }
}

impl AccountsKontroller {
    fn create(input: AccountCreationInput, kong: &Kong) -> Response {
        let account: Account = input.into();

        match kong.database.create_account(&account) {
            Ok(_) => {
                let public_account: PublicAccount = account.into();
                Response::json(&public_account).with_status_code(201)
            }
            // TODO: Better error handling
            Err(_) => Response::json(&KontrolError {
                msg: "Could not create account".to_owned(),
            })
            .with_status_code(500),
        }
    }
}
