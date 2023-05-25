//! 🎮 Kong request endpoint kontroller

use crate::{KError, Kong, Method};
use rouille::{Request, Response};
use route_recognizer::{Params, Router};

/// 🎮 API Enpoint kontrollers
pub trait Kontrol {
    /// Endpoint address
    fn address(&self) -> String;
    /// Enpoint method
    fn method(&self) -> Method;

    /// Get user input
    fn get_input(&self, _request: &Request) -> Option<serde_json::Value> {
        None
    }
    /// Validate user input
    fn validate(&self, input: Option<serde_json::Value>) -> Result<Option<serde_json::Value>, ()> {
        Ok(input)
    }

    /// Handle endpoint (business logic)
    fn kontrol(&self, kong: &Kong) -> Response;

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
