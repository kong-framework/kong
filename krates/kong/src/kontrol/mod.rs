//! kontrol
//!
//! `kong` API request controllers

use crate::Kong;
use rouille::{Request, Response};
use serde::Serialize;
pub mod accounts;
pub mod auth;
use kerror::KError;
use route_recognizer::{Params, Router};


/// Request Kontroller
pub struct Kontroller<'a> {
    /// API request address
    pub address: &'a str,
    /// API request handler
    pub handle: fn(kong: &mut Kong, req: &Request) -> Response,
}

impl<'a> Kontroller<'a> {
    /// url parameters extractor
    pub fn url_params(
        router: &Router<fn(&mut Kong, &'a Request) -> Response>,
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

/// API request handling error
#[derive(Serialize)]
pub struct KontrolError {
    msg: String,
}
