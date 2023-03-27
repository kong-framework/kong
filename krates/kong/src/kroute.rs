//! # kroute
//! `kong` request routing

use crate::Kong;
use kerror::KError;
use rouille::{Request, Response};
use route_recognizer::{Params, Router};

/// Kong request routing
pub struct Kroute;

impl Kroute {
    /// Kong request router
    pub fn kroute(kong: &mut Kong, request: &rouille::Request) -> rouille::Response {
        // Handle static files
        if let Some(path) = &kong.config.static_files_path {
            let response = rouille::match_assets(request, &path);
            if response.is_success() {
                return response;
            }
        }

        let router = kong.router.clone();
        let m = router.recognize(&request.url());

        match m {
            Ok(mut mtch) => (mtch.handler_mut())(kong, request),
            Err(_) => rouille::Response::html("404 error").with_status_code(404),
        }
    }

    /// Kong url parameters extractor
    pub fn get_url_params(
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
