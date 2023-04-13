//! # kroute
//! `kong` request routing

use crate::RouterObject;
use crate::{kontrol::Method, Kong};
use kdata::resource::{self, ResourceError};
use kdata::{
    accounts::Account,
    inputs::{AccountAuthInput, UserInput},
    resource::{GenericResource, Resource},
};
use krypto::{error::KryptoError, kpassport::Kpassport};
use serde::Serialize;
use std::str::FromStr;
/// Kong request routing
pub struct Kroute;

impl Kroute {
    /// Kong request router
    pub fn kroute<I: UserInput, R: Resource + serde::Serialize>(
        kong: &mut Kong<I, R>,
        request: &rouille::Request,
    ) -> rouille::Response {
        // Handle static files
        if let Some(path) = &kong.config.static_files_path {
            let response = rouille::match_assets(request, &path);
            if response.is_success() {
                return response;
            }
        }

        let router = kong.router.clone();
        // check request url
        let recognized_route = router.recognize(&request.url());

        match recognized_route {
            Ok(mut route) => {
                // check if HTTP method is supported
                if Kroute::is_method_supported(request, &mut route) {
                    let kontrol = (route.handler_mut()).kontrol.kontrol;
                    Kroute::respond(kong, request, &mut route, kontrol)
                } else {
                    ErrorResponse::not_allowed()
                }
            }
            Err(_) => ErrorResponse::not_found(),
        }
    }

    /// finally respond to the server after all pre-processing has been
    /// done on the request
    fn respond<I: UserInput, R: Resource + serde::Serialize>(
        kong: &mut Kong<I, R>,
        request: &rouille::Request,
        route: &mut route_recognizer::Match<&RouterObject<I, R>>,
        kontrol: fn(
            &mut Kong<I, R>,
            Option<I>,
            kpassport: Option<Kpassport>,
        ) -> Result<R, ResourceError>,
    ) -> rouille::Response {
        let kpassport = if let Ok(kpassport) = Kroute::get_valid_auth_token(kong, request) {
            Some(kpassport)
        } else {
            None
        };

        // Get user input
        if let Some(get_input) = (route.handler_mut()).kontrol.get_input {
            let input = get_input(request);
            // handle request with user input
            let resource = kontrol(kong, input, kpassport);
            match resource {
                Ok(resource) => rouille::Response::json(&resource).with_status_code(201),
                Err(err) => ErrorResponse::map_resource_error(err),
            }
        } else {
            // handle request with no user input
            let resource = kontrol(kong, None, kpassport);
            match resource {
                Ok(resource) => rouille::Response::json(&resource).with_status_code(201),
                Err(err) => ErrorResponse::map_resource_error(err),
            }
        }
    }
    /// Check HTTP method
    fn is_method_supported<I: UserInput, R: Resource + serde::Serialize>(
        request: &rouille::Request,
        route: &mut route_recognizer::Match<&RouterObject<I, R>>,
    ) -> bool {
        if let Ok(request_method) = Method::from_str(&request.method()) {
            let supported_method = &route.handler_mut().method;
            // check if method is supported by handler
            if supported_method == &request_method {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// check if client auth token is valid
    fn get_valid_auth_token<I: UserInput, R: Resource + serde::Serialize>(
        kong: &Kong<I, R>,
        request: &rouille::Request,
    ) -> Result<Kpassport, KryptoError> {
        let cookie_signing_key = &kong.config.secret_key;
        let auth_cookie_name = &kong.config.auth_cookie_name;

        if let Ok(kpassport) = Kroute::get_cookie_token(auth_cookie_name, request) {
            // validate kpassport
            if kpassport.validate(cookie_signing_key).is_ok() {
                Ok(kpassport)
            } else {
                Err(KryptoError::InvalidKpassport)
            }
        } else {
            Err(KryptoError::InvalidKpassport)
        }
    }

    /// check if request is authorized based on the authorization cookie
    fn get_cookie_token(
        auth_cookie_name: &str,
        request: &rouille::Request,
    ) -> Result<krypto::kpassport::Kpassport, krypto::error::KryptoError> {
        if let Some((_, cookie_value)) =
            rouille::input::cookies(request).find(|&(n, _)| n == auth_cookie_name)
        {
            let auth = krypto::authentication::AuthHeaders {
                cookie: Some(cookie_value),
                bearer_token: None,
            };
            // TODO: don't use unwrap
            let auth = krypto::authentication::Auth::detect_auth_type(auth).unwrap();

            match auth {
                krypto::authentication::Auth::Cookie(kpassport_str) => {
                    krypto::kpassport::Kpassport::from_str(&kpassport_str)
                }
                // TODO: implement
                _ => unimplemented!(),
            }
        } else {
            // TODO: implement
            unimplemented!()
        }
    }
}

/// Issue kpassport in other words login
struct IssueKpassport;

impl IssueKpassport {
    fn get_input(request: &rouille::Request) -> AccountAuthInput {
        let input: AccountAuthInput = rouille::input::json_input(request).unwrap();
        input
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
                let mut response = rouille::Response::text("");
                response.headers.push(cookie);
                response.status_code = 200;
                response
            }
            Err(_) => ErrorResponse::internal(),
        }
    }

    /// Authenticate user
    fn handle(
        kong: &mut Kong<AccountAuthInput, GenericResource>,
        input: Option<AccountAuthInput>,
        _kpassport: Option<Kpassport>,
    ) -> rouille::Response {
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
}

/// API request handling error
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_message: String,
}

impl ErrorResponse {
    pub fn bad_request() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Bad request".to_string(),
        })
        .with_status_code(400)
    }
    pub fn unauthorized() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Unauthorized".to_string(),
        })
        .with_status_code(401)
    }
    pub fn forbidden() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Forbidden".to_string(),
        })
        .with_status_code(402)
    }
    pub fn not_found() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Could not find resource".to_string(),
        })
        .with_status_code(404)
    }
    pub fn not_allowed() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Method Not Allowed".to_string(),
        })
        .with_status_code(405)
    }
    pub fn conflict() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Conflict".to_string(),
        })
        .with_status_code(405)
    }
    pub fn pre_condition() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Pre-Condition failed".to_string(),
        })
        .with_status_code(412)
    }
    pub fn internal() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Internal Server Error".to_string(),
        })
        .with_status_code(500)
    }

    pub fn map_resource_error(err: ResourceError) -> rouille::Response {
        match err {
            ResourceError::BadRequest => ErrorResponse::bad_request(),
            ResourceError::Unauthorized => ErrorResponse::unauthorized(),
            ResourceError::NotFound => ErrorResponse::not_found(),
            ResourceError::Forbidden => ErrorResponse::forbidden(),
            ResourceError::Conflict => ErrorResponse::conflict(),
            ResourceError::PreConditionFailed => ErrorResponse::pre_condition(),
            ResourceError::Internal => ErrorResponse::internal(),
        }
    }
}
