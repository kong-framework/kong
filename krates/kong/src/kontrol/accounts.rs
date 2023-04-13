//! Accounts API endpoint controller

use super::KontrolHandle;
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
    fn get_input(request: &Request) -> AccountCreationInput {
        // TODO: don't use unwrap
        let input: AccountCreationInput = rouille::input::json_input(request).unwrap();
        input
    }

    /// Create a new user
    fn handle(
        kong: &mut Kong<AccountCreationInput, PublicAccount>,
        input: Option<AccountCreationInput>,
        kpassport: Option<Kpassport>,
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
}
