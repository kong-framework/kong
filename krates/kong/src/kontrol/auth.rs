//! Authorization and Authentication API endpoint controller
use super::{KontrolError, KontrolHandle};
use crate::Kong;

use kdata::{accounts::Account, inputs::AccountAuthInput};
use rouille::{Request, Response};

/// Authorization and Authentication API endpoint controller
pub struct AuthKontroller;

impl KontrolHandle<AccountAuthInput> for AuthKontroller {
    fn get_input(request: &Request) -> AccountAuthInput {
        let input: AccountAuthInput = rouille::input::json_input(request).unwrap();
        input
    }

    /// Authenticate user
    fn handle(kong: &mut Kong<AccountAuthInput>, input: AccountAuthInput) -> Response {
        // Find user account in database
        let account = kong
            .database
            .private_get_account_by_username(&input.username);

        match account {
            Ok(account) => {
                if let Some(account) = account {
                    // Verify user password
                    match krypto::password::verify(&account.password, &input.password) {
                        Ok(password_verification) => {
                            if password_verification {
                                // Password correct, create cookie based sessions
                                AuthKontroller::cookie_auth(
                                    account,
                                    &kong.config.host,
                                    &kong.config.secret_key,
                                    &kong.config.auth_cookie_name,
                                )
                            } else {
                                // Wrong password provided
                                Response::json(&KontrolError {
                                    msg: "Invalid username or password".to_owned(),
                                })
                                .with_status_code(401)
                            }
                        }
                        Err(_) => Response::json(&KontrolError {
                            msg: "Could validate password".to_owned(),
                        })
                        .with_status_code(500),
                    }
                } else {
                    Response::json(&KontrolError {
                        msg: "Invalid username or password".to_owned(),
                    })
                    .with_status_code(401)
                }
            }
            Err(_) => Response::json(&KontrolError {
                msg: "Could not get account".to_owned(),
            })
            .with_status_code(404),
        }
    }
}

impl AuthKontroller {
    fn cookie_auth(account: Account, host: &str, signing_key: &str, cookie_name: &str) -> Response {
        // Create cookie
        let cookie = krypto::authentication::Auth::issue_kpassport_cookie(
            &account.username,
            host,
            signing_key,
            cookie_name,
        );

        match cookie {
            Ok(cookie) => {
                let mut response = Response::text("");
                response.headers.push(cookie);
                response.status_code = 200;
                response
            }
            Err(_) => Response::json(&KontrolError {
                msg: "Could not create cookie".to_owned(),
            })
            .with_status_code(500),
        }
    }
}
