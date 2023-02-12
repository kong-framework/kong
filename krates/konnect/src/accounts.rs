use super::{Konnect, KonnectError};
use kdata::{
    accounts::{Account, PublicAccount},
    inputs::AccountCreationInput,
};
use kore::Kore;
use rouille::{try_or_400, Request, Response};

pub const ADDRESS: &str = "/accounts";

pub struct Accounts;

impl Konnect for Accounts {
    fn post(kore: &mut Kore, req: &Request) -> Response {
        let input: AccountCreationInput = try_or_400!(rouille::input::json_input(&req));

        // check if input email matches admins email
        if let Some(email) = &input.email {
            if email == &kore.config.admin_email {
                Accounts::create_admin(input, kore)
            } else {
                // TODO: create non-admin accounts
                unimplemented!()
            }
        } else {
            Response::json(&KonnectError {
                msg: "Email not provided".to_owned(),
            })
            .with_status_code(400)
        }
    }
}

impl Accounts {
    fn create_admin(input: AccountCreationInput, kore: &Kore) -> Response {
        if let Some(email) = &input.email {
            // Check if admin is already created
            let account = kore.database.get_account_by_email(email);

            match account {
                Ok(acc) => {
                    if let Some(acc) = acc {
                        // email already in use
                        Response::json(&KonnectError {
                            msg: "Email already in use.".to_owned(),
                        })
                        .with_status_code(401)
                    } else {
                        // admin email not used, create admin account
                        let admin_account: Account = input.into();

                        let r = kore.database.create_account(&admin_account);

                        match r {
                            Ok(_) => {
                                let public_admin_account: PublicAccount = admin_account.into();
                                Response::json(&public_admin_account).with_status_code(201)
                            }
                            Err(error) => Response::json(&KonnectError {
                                msg: "Could not create account".to_owned(),
                            })
                            .with_status_code(500),
                        }
                    }
                }
                Err(error) => Response::json(&KonnectError {
                    msg: "Could not create account".to_owned(),
                })
                .with_status_code(500),
            }
        } else {
            Response::json(&KonnectError {
                msg: "Email not provided".to_owned(),
            })
            .with_status_code(400)
        }
    }
}
