//! 🌀 `kong` request router

use crate::{error_response::ErrorResponse, konfig::Konfig, Kong, Kontrol};

use crate::log::Log;
use crate::{read_kpassport::get_kpassport, KError};
use core::fmt;
use route_recognizer::Router;
use std::str::FromStr;
use std::sync::Mutex;

/// Kontoller Handle
type KontrollerHandle = Box<dyn Kontrol + std::marker::Sync + std::marker::Send + 'static>;

/// 🌀 `kong` request routing
pub fn kroute(kontrollers: Vec<KontrollerHandle>) -> rouille::Response {
    let port = Konfig::read_port();
    let hostname = Konfig::read_hostname();
    let address = format!("localhost:{port}");
    let kong: Kong = Default::default();
    let kong: Mutex<Kong> = Mutex::new(kong);
    let mut router = Router::new();

    // prepare kontrollers for routing
    for kontroller in kontrollers {
        let kontroller_id = format!("{}{}", kontroller.method(), kontroller.address());
        router.add(&kontroller_id, kontroller);
    }

    Log::log(&format!("{hostname} node started @ {address}")).expect("Error while logging");

    rouille::start_server(address, move |request| {
        let mut kong = kong.lock().unwrap();

        // Handle static files
        if let Some(path) = &kong.config.static_files_path {
            let response = rouille::match_assets(request, &path);
            if response.is_success() {
                log_request(request, response.status_code);
                return response;
            }
        }

        let response = filter(request, &router, &mut kong);
        log_request(request, response.status_code);
        response
    })
}

// filter route
fn filter(
    request: &rouille::Request,
    router: &Router<KontrollerHandle>,
    kong: &mut Kong,
) -> rouille::Response {
    // check request url
    let kontroller_id = format!("{}{}", request.method(), request.url());
    let recognized_route = router.recognize(&kontroller_id);

    match recognized_route {
        Ok(route) => {
            // get url parameters
            kong.url_parameters = Some(route.params().clone());

            // get a valid kpassport token
            if let Ok(kpassport) = get_kpassport(kong, request) {
                kong.kpassport = Some(kpassport);
            } else {
                kong.kpassport = None
            };

            // Get input
            let input_json_str = route.handler().get_input(request);

            // validate input_json_str
            if let Ok(input) = route.handler().validate(input_json_str) {
                kong.input = input;

                // kontrol
                route.handler().kontrol(kong)
            } else {
                ErrorResponse::bad_request()
            }
        }
        Err(_) => ErrorResponse::not_found(),
    }
}

/// Log request
fn log_request(request: &rouille::Request, status_code: u16) {
    let log = format!("{} {} = {}", request.method(), request.url(), status_code);
    Log::log(&log).expect("Error while logging");
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
impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Head => write!(f, "HEAD"),
            Self::Delete => write!(f, "DELETE"),
            Self::Options => write!(f, "OPTIONS"),
        }
    }
}
