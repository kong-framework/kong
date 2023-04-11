//! kontrol
//!
//! `kong` API request controllers

use crate::Kong;
use rouille::{Request, Response};
use serde::Serialize;
pub mod accounts;
pub mod auth;
use kdata::inputs::UserInput;
use kerror::KError;
use route_recognizer::{Params, Router};
use std::str::FromStr;

/// Functionality for endpoint kontrollers
pub struct Kontrol<I: UserInput> {
    /// Read user input
    pub get_input: fn(request: &Request) -> I,
    /// validate input
    pub validate: fn(input: I) -> Result<I, ()>,
    /// Handle request
    pub kontrol: fn(kong: &mut Kong<I>, input: I) -> Response,
}

impl<I: UserInput> Copy for Kontrol<I> {}

impl<I: UserInput> Clone for Kontrol<I> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Request endpoint
pub struct Kontroller<'a, I: UserInput> {
    /// API request address
    pub address: &'a str,
    /// API request kontrol handler
    pub kontrol: Kontrol<I>,
    /// Supported HTTP method
    pub method: Method,
}

impl<'a, I: UserInput> Kontroller<'a, I> {
    /// url parameters extractor
    pub fn url_params(
        router: &Router<fn(&mut Kong<I>, &'a Request) -> Response>,
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

impl Copy for Method {}
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

/// Handlers
pub trait KontrolHandle<I: UserInput> {
    /// kontrol everything
    fn kontrol(kong: &mut Kong<I>, input: I) -> Response {
        let input = Self::validate(input);

        if let Ok(input) = input {
            Self::handle(kong, input)
        } else {
            panic!()
        }
    }

    /// Get input
    fn get_input(request: &Request) -> I;

    /// Validate user input
    fn validate(input: I) -> Result<I, ()> {
        if input.is_valid().is_ok() {
            Ok(input)
        } else {
            // TODO: proper error handling
            Err(())
        }
    }

    /// Handle request
    fn handle(kong: &mut Kong<I>, input: I) -> Response;
}

/// API request handling error
#[derive(Serialize)]
pub struct KontrolError {
    msg: String,
}

#[cfg(test)]
mod test {
    use kdata::inputs::AccountCreationInput;

    use crate::kontrol::accounts::CreateAccountKontroller;

    use super::*;

    #[test]
    fn usage() {
        let kontrol: Kontrol<AccountCreationInput> = Kontrol {
            get_input: CreateAccountKontroller::get_input,
            validate: CreateAccountKontroller::validate,
            kontrol: CreateAccountKontroller::kontrol,
        };

        let kontroller = Kontroller {
            address: "/",
            kontrol,
            method: Method::Post,
        };

        let kontroller1 = Kontroller {
            address: "/",
            kontrol,
            method: Method::Post,
        };

        let kontroller2 = Kontroller {
            address: "/",
            kontrol,
            method: Method::Put,
        };

        // let kong = Kong::new(vec![kontroller, kontroller1, kontroller2]);
    }
}
