//! 🏴 `kong` error response

use serde::Serialize;

/// 🏴 API error response
#[derive(Serialize)]
pub struct ErrorResponse {
    /// Error message
    pub error_message: String,
}

impl ErrorResponse {
    /// HTTP Bad request (400)
    pub fn bad_request() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Bad request".to_string(),
        })
        .with_status_code(400)
    }
    /// HTTP unauthorized request (401)
    pub fn unauthorized() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Unauthorized".to_string(),
        })
        .with_status_code(401)
    }
    /// HTTP forbidden request (402)
    pub fn forbidden() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Forbidden".to_string(),
        })
        .with_status_code(402)
    }
    /// HTTP not foud resource (404)
    pub fn not_found() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Could not find resource".to_string(),
        })
        .with_status_code(404)
    }

    /// HTTP method not allowed resource (405)
    pub fn not_allowed() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Method Not Allowed".to_string(),
        })
        .with_status_code(405)
    }
    /// HTTP request conflict (409)
    pub fn conflict() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Conflict".to_string(),
        })
        .with_status_code(409)
    }
    /// HTTP precondition failed (412)
    pub fn pre_condition() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Pre-Condition failed".to_string(),
        })
        .with_status_code(412)
    }
    /// HTTP internal server error (500)
    pub fn internal() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Internal Server Error".to_string(),
        })
        .with_status_code(500)
    }
}
