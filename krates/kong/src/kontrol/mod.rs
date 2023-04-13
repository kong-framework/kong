//! kontrol
//!
//! `kong` API request controllers

use crate::Kong;
use krypto::kpassport::Kpassport;
use rouille::{Request, Response};
pub mod accounts;
use kdata::{
    inputs::UserInput,
    resource::{Resource, ResourceError},
};
use kerror::KError;
use route_recognizer::{Params, Router};
use std::str::FromStr;

/// Functionality for endpoint kontrollers
pub struct Kontrol<I: UserInput, R: Resource + serde::Serialize> {
    /// Read user input
    pub get_input: Option<fn(request: &Request) -> I>,
    /// validate input, this can ofcourse be part of `Kontrol.get_input`
    /// but this way can ensure that validation is not hidden or forgotten
    pub validate: Option<fn(input: I) -> Result<I, ()>>,
    /// Handle request
    pub kontrol: fn(
        kong: &mut Kong<I, R>,
        input: Option<I>,
        kpassport: Option<Kpassport>,
    ) -> Result<R, ResourceError>,
}

impl<I: UserInput, R: Resource + serde::Serialize> Copy for Kontrol<I, R> {}

impl<I: UserInput, R: Resource + serde::Serialize> Clone for Kontrol<I, R> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Request endpoint
pub struct Kontroller<'a, I: UserInput, R: Resource + serde::Serialize> {
    /// API request address
    pub address: &'a str,
    /// API request kontrol handler
    pub kontrol: Kontrol<I, R>,
    /// Supported HTTP method
    pub method: Method,
}

impl<'a, I: UserInput, R: Resource + serde::Serialize> Kontroller<'a, I, R> {
    /// url parameters extractor
    pub fn url_params(
        router: &Router<fn(&mut Kong<I, R>, &'a Request) -> Response>,
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
pub trait KontrolHandle<I: UserInput, R: Resource + serde::Serialize> {
    /// kontrol everything
    fn kontrol(
        kong: &mut Kong<I, R>,
        input: Option<I>,
        kpassport: Option<Kpassport>,
    ) -> Result<R, ResourceError> {
        if let Some(input) = input {
            let input = Self::validate(input);

            if let Ok(input) = input {
                Self::handle(kong, Some(input), kpassport)
            } else {
                // TODO: do not panic
                panic!()
            }
        } else {
            Self::handle(kong, None, kpassport)
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
    fn handle(
        kong: &mut Kong<I, R>,
        input: Option<I>,
        kpassport: Option<Kpassport>,
    ) -> Result<R, ResourceError>;
}

#[cfg(test)]
mod test {
    use kdata::{accounts::PublicAccount, inputs::AccountCreationInput};

    use crate::kontrol::accounts::CreateAccountKontroller;

    use super::*;

    #[test]
    fn usage() {
        let kontrol: Kontrol<AccountCreationInput, PublicAccount> = Kontrol {
            get_input: Some(CreateAccountKontroller::get_input),
            validate: Some(CreateAccountKontroller::validate),
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
