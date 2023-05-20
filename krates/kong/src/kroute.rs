//! 🌀 `kong` request router

use crate::{error_response::ErrorResponse, konfig::Konfig, Kong, Kontrol};

use crate::log::Log;
use crate::KError;
use krypto::{error::KryptoError, kpassport::Kpassport};
use route_recognizer::Router;
use std::str::FromStr;
use std::sync::Mutex;

/// 🌀 `kong` request routing
pub fn kroute(
    kontrollers: Vec<Box<dyn Kontrol + std::marker::Sync + std::marker::Send + 'static>>,
) -> rouille::Response {
    let port = Konfig::read_port();
    let address = format!("localhost:{}", port);
    let kong: Kong = Default::default();
    let kong: Mutex<Kong> = Mutex::new(kong);
    let mut router = Router::new();

    // prepare kontrollers
    for kontrol in kontrollers {
        router.add(&kontrol.address(), kontrol);
    }

    Log::log(&format!("kong node started @ {address}")).expect("Error while logging");

    rouille::start_server(address, move |request| {
        let mut kong = kong.lock().unwrap();
        let request_log = format!("{} {}", request.method(), request.url());

        Log::log(&request_log).expect("Error while logging");

        // Handle static files
        if let Some(path) = &kong.config.static_files_path {
            let response = rouille::match_assets(request, &path);
            if response.is_success() {
                return response;
            }
        }

        // check request url
        let recognized_route = router.recognize(&request.url());

        match recognized_route {
            Ok(route) => {
                if let Ok(kpassport) = get_valid_auth_token(&kong, request) {
                    kong.kpassport = Some(kpassport);
                } else {
                    kong.kpassport = None
                };

                let expected_method = route.handler().method();
                let input_json_str = route.handler().get_input(request);

                // validate input_json_str
                if let Ok(input) = route.handler().validate(input_json_str) {
                    kong.input = input;
                    let response = route.handler().kontrol(&kong);

                    // check if HTTP method is supported
                    if is_method_supported(request, &expected_method) {
                        response
                    } else {
                        ErrorResponse::not_allowed()
                    }
                } else {
                    ErrorResponse::bad_request()
                }
            }
            Err(_) => ErrorResponse::not_found(),
        }
    })
}

/// Check HTTP method
fn is_method_supported(request: &rouille::Request, expected_method: &Method) -> bool {
    if let Ok(request_method) = Method::from_str(request.method()) {
        let supported_method = expected_method;
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
fn get_valid_auth_token(kong: &Kong, request: &rouille::Request) -> Result<Kpassport, KryptoError> {
    let cookie_signing_key = &kong.config.secret_key;
    let auth_cookie_name = &kong.config.auth_cookie_name;

    if let Ok(kpassport) = get_cookie_token(auth_cookie_name, request) {
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
        // Cookie not found
        Err(KryptoError::InvalidKpassport)
    }
}

#[derive(Clone, PartialEq)]
/// 🚥 HTTP methods
pub enum Method {
    /// HTTP GET method
    Get,
    /// HTTP POST method
    Post,
    /// HTTP PUT method
    Put,
    /// HTTP HEAD method
    Head,
    /// HTTP DELETE method
    Delete,
    /// HTTP OPTIONS method
    Options,
}

impl Copy for Method {}
impl FromStr for Method {
    type Err = KError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "HEAD" => Ok(Method::Head),
            "DELETE" => Ok(Method::Delete),
            "OPTIONS" => Ok(Method::Options),
            _ => Err(KError::InvalidHttpMethod),
        }
    }
}
