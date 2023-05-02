//! kontrol
//!
//! `kong` API request controllers

pub mod error_response;
pub mod issue_kpassport;

use crate::Kong;
use kdata::resource::ResourceError;
use kerror::KError;
use rouille::{Request, Response};
use route_recognizer::{Params, Router};
use std::str::FromStr;

/// API Enpoint kontrollers
pub trait Kontrol {
    /// Endpoint address
    fn address(&self) -> String;
    /// Enpoint method
    fn method(&self) -> Method;

    /// Get user input
    fn get_input(&self, request: &Request) -> Option<serde_json::Value> {
        None
    }
    /// Validate user input
    fn validate(&self, input: Option<serde_json::Value>) -> Result<Option<serde_json::Value>, ()> {
        Ok(input)
    }

    /// Handle endpoint (business logic)
    fn kontrol(&self, kong: &Kong) -> Result<serde_json::Value, ResourceError>;

    /// url parameters extractor
    fn url_params(
        &self,
        router: &Router<fn(&Kong, &Request) -> Response>,
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
