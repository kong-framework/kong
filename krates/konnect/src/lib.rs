use kore::Kore;
use rouille::{Request, Response};
use serde::Serialize;

pub mod accounts;

/// Trait for an HTTP endpoint /  request handler
pub trait Konnect {
    /// Check request HTTP methods and handle accordingly
    fn handle_request(runtime: &mut Kore, req: &Request) -> Response {
        match req.method() {
            "POST" => Self::post(runtime, req),
            _ => Response::html(
                "404 error. Try <a href=\"/README.md\"`>README.md</a> or \
                        <a href=\"/src/lib.rs\">src/lib.rs</a> for example.",
            )
            .with_status_code(404),
        }
    }

    /// Handle request from a HTTP POST method
    fn post(runtime: &mut Kore, req: &Request) -> Response;
}

pub struct KustomRoute<'a> {
    pub address: &'a str,
    pub handler: fn(runtime: &mut Kore, req: &Request) -> Response,
}

#[derive(Serialize)]
pub struct KonnectError {
    msg: String,
}
