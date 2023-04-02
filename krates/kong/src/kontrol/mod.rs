//! kontrol
//!
//! `kong` API request controllers

use crate::Kong;
use rouille::{Request, Response};
use serde::Serialize;
//pub mod accounts;
//pub mod auth;
use kerror::KError;
use route_recognizer::{Params, Router};
use std::str::FromStr;

/// Request endpoint
pub struct Kontroller<'a> {
    /// API request address
    pub address: &'a str,
    /// API request handler
    pub kontrol: fn(kong: &mut Kong, req: &Request) -> Response,
    /// Supported HTTP methods
    pub methods: Vec<Method>,
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

/// HTTP methods
#[derive(Clone, PartialEq)]
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

impl FromStr for Method {
    type Err = KError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "HEAD" => Ok(Method::Head),
            "Delete" => Ok(Method::Delete),
            "Options" => Ok(Method::Options),
            _ => Err(KError::InvalidHttpMethod),
        }
    }
}

/// API request handling error
#[derive(Serialize)]
pub struct KontrolError {
    msg: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn usage() {
        fn kontrol(_kong: &mut Kong, _request: &Request) -> Response {
            Response::text("Hello World")
        }

        let kontroller = Kontroller {
            address: "/",
            kontrol,
            methods: vec![Method::Get, Method::Post],
        };

        let kontroller1 = Kontroller {
            address: "/",
            kontrol,
            methods: vec![Method::Post],
        };

        let kontroller2 = Kontroller {
            address: "/",
            kontrol,
            methods: vec![Method::Put],
        };

        let kong = Kong::new(vec![kontroller, kontroller1, kontroller2]);
    }
}
