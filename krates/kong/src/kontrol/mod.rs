//! kontrol
//!
//! `kong` API request controllers

use crate::Kong;
use kdata::inputs::UserInput;
use rouille::{Request, Response};
use serde::Serialize;
pub mod accounts;
pub mod auth;
use kerror::KError;
use route_recognizer::{Params, Router};

/// Trait for an HTTP endpoint /  request handler
pub trait Kontrol {
    /// Check request HTTP methods and handle accordingly
    fn handle_request(kong: &mut Kong, req: &Request) -> Response {
        match req.method() {
            "POST" => Self::post(kong, req),
            _ => Response::html(
                "404 error. Try <a href=\"/README.md\"`>README.md</a> or \
                        <a href=\"/src/lib.rs\">src/lib.rs</a> for example.",
            )
            .with_status_code(404),
        }
    }

    /// Validate input from user
    fn validate_user_input(input: impl UserInput) -> bool;

    /// Handle request from a HTTP POST method
    fn post(kong: &mut Kong, req: &Request) -> Response;

    /// url parameters extractor
    fn url_params(
        router: &Router<for<'a> fn(&mut Kong, &'a Request) -> Response>,
        url: &str,
    ) -> Result<Params, KError> {
        let router = router.clone();
        let m = router.recognize(url);

        match m {
            Ok(mtch) => Ok(mtch.params().clone()),
            Err(_) => Err(KError::UrlParsing),
        }
    }
}

/// Request Kontroller
pub struct Kontroller<'a> {
    /// API request address
    pub address: &'a str,
    /// API request handler
    pub handle: fn(kong: &mut Kong, req: &Request) -> Response,
}

/// API request handling error
#[derive(Serialize)]
pub struct KontrolError {
    msg: String,
}
