use super::error_response::ErrorResponse;
use crate::Kong;
use kdata::{accounts::Account, inputs::AccountAuthInput};
use kollection::Kollection;
use krypto::kpassport::Kpassport;
use serde::Serialize;

/// Issue kpassport in other words login
#[derive(Serialize)]
pub struct IssueKpassport {
    /// Message sent as JSON to user after successful login
    message: String,
    account_type: Option<String>,
}

impl IssueKpassport {
    /// Get auth input
    pub fn get_input(request: &rouille::Request) -> Option<AccountAuthInput> {
        let input: Result<AccountAuthInput, rouille::input::json::JsonError> =
            rouille::input::json_input(request);

        if let Ok(input) = input {
            Some(input)
        } else {
            None
        }
    }

    /// Authenticate user
    pub fn handle(kong: &mut Kong, input: Option<AccountAuthInput>) -> rouille::Response {
        if let Some(input) = input {
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
                                    IssueKpassport::cookie_auth(
                                        account,
                                        &kong.config.host,
                                        &kong.config.secret_key,
                                        &kong.config.auth_cookie_name,
                                    )
                                } else {
                                    // Wrong password provided
                                    ErrorResponse::bad_request()
                                }
                            }
                            Err(_) => ErrorResponse::internal(),
                        }
                    } else {
                        ErrorResponse::bad_request()
                    }
                }
                Err(_) => ErrorResponse::not_found(),
            }
        } else {
            ErrorResponse::bad_request()
        }
    }

    fn cookie_auth(
        account: Account,
        host: &str,
        signing_key: &str,
        cookie_name: &str,
    ) -> rouille::Response {
        // Create cookie
        let cookie = krypto::authentication::Auth::issue_kpassport_cookie(
            &account.username,
            host,
            signing_key,
            cookie_name,
        );

        match cookie {
            Ok(cookie) => {
                let mut response = rouille::Response::json(&IssueKpassport {
                    message: "Loggin successful".to_string(),
                    account_type: account.account_type,
                });
                response.headers.push(cookie);
                response.status_code = 200;
                response
            }
            Err(_) => ErrorResponse::internal(),
        }
    }

    // TODO: add error handling
    /// check if user is admin
    pub fn is_admin(
        admin_email: &str,
        kong_database: &mut Kollection,
        kpassport: Option<Kpassport>,
    ) -> bool {
        if let Some(kpassport) = kpassport {
            // get admin from database

            if let Ok(admin_account) = kong_database.private_get_account_by_email(admin_email) {
                if let Some(admin_account) = admin_account {
                    // check if admin account username matches the username is the kpassport
                    if admin_account.username == kpassport.content.username {
                        // user is admin
                        true
                    } else {
                        // user is not admin
                        false
                    }
                } else {
                    // Admin account not found, for some reason. Cannot check if user is admin
                    false
                }
            } else {
                // Could not get admin account from database
                false
            }
        } else {
            // No kpassport found (user not logged in), cannot check if user is admin
            false
        }
    }
}
