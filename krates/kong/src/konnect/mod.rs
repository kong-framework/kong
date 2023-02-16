use crate::Kong;
use rouille::{Request, Response};
use serde::Serialize;

pub mod accounts;

/// Trait for an HTTP endpoint /  request handler
pub trait Konnect {
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

    /// Handle request from a HTTP POST method
    fn post(kong: &mut Kong, req: &Request) -> Response;
}

pub struct KustomRoute<'a> {
    pub address: &'a str,
    pub handler: fn(kong: &mut Kong, req: &Request) -> Response,
}

#[derive(Serialize)]
pub struct KonnectError {
    msg: String,
}
