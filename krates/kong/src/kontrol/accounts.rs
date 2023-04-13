//! Accounts API endpoint controller

use super::{Kontrol, KontrolHandle, Kontroller, Method};
use crate::Kong;
use kdata::{
    accounts::{Account, PublicAccount},
    inputs::AccountCreationInput,
    resource::ResourceError,
};
use krypto::kpassport::Kpassport;
use rouille::Request;

/// Accounts API endpoint controller
pub struct CreateAccountKontroller;

impl KontrolHandle<AccountCreationInput, PublicAccount> for CreateAccountKontroller {
    fn get_input(request: &Request) -> Option<AccountCreationInput> {
        // TODO: don't use unwrap
        let input: AccountCreationInput = rouille::input::json_input(request).unwrap();
        Some(input)
    }

    /// Create a new user
    fn handle(
        kong: &mut Kong<AccountCreationInput, PublicAccount>,
        input: Option<AccountCreationInput>,
        _kpassport: Option<Kpassport>,
    ) -> Result<PublicAccount, ResourceError> {
        if let Some(input) = input {
            let account: Account = input.into();

            match kong.database.create_account(&account) {
                Ok(_) => {
                    let public_account: PublicAccount = account.into();
                    Ok(public_account)
                }

                Err(err) => match err {
                    kerror::KError::DbField => Err(ResourceError::BadRequest),
                    _ => Err(ResourceError::Internal),
                },
            }
        } else {
            Err(ResourceError::BadRequest)
        }
    }

    /// Create an new account, can be used by users of kong to get a CreateAccountKontroller Kontroller
    fn kontroller<'a>() -> Kontroller<'a, AccountCreationInput, PublicAccount> {
        let create_account: Kontrol<AccountCreationInput, PublicAccount> = Kontrol {
            get_input: Some(CreateAccountKontroller::get_input),
            validate: Some(CreateAccountKontroller::validate),
            kontrol: CreateAccountKontroller::kontrol,
        };

        Kontroller {
            address: "/accounts",
            method: Method::Post,
            kontrol: create_account,
        }
    }
}
