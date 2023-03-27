//! # kroute
//! `kong` request routing

use crate::Kong;

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
}
