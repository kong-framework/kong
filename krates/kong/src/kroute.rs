//! # kroute
//! `kong` request routing

use kdata::{inputs::UserInput, validate};

use crate::{kontrol::Method, Kong};
use std::str::FromStr;

/// Kong request routing
pub struct Kroute;

impl Kroute {
    /// Kong request router
    pub fn kroute<I: UserInput>(
        kong: &mut Kong<I>,
        request: &rouille::Request,
    ) -> rouille::Response {
        // Handle static files
        if let Some(path) = &kong.config.static_files_path {
            let response = rouille::match_assets(request, &path);
            if response.is_success() {
                return response;
            }
        }

        let router = kong.router.clone();
        // check request url
        let recognized_route = router.recognize(&request.url());

        match recognized_route {
            Ok(mut route) => {
                // Check HTTP method
                if let Ok(request_method) = Method::from_str(&request.method()) {
                    let supported_method = &route.handler_mut().method;

                    // check if method is supported by handler
                    if supported_method == &request_method {
                        let kontrol = (route.handler_mut()).kontrol.kontrol;

                        // kontrol request
                        kontrol(kong, request)
                    } else {
                        rouille::Response::html("404 error").with_status_code(404)
                    }
                } else {
                    rouille::Response::html("Invalid HTTP Method").with_status_code(400)
                }
            }
            Err(_) => rouille::Response::html("404 error").with_status_code(404),
        }
    }
}
