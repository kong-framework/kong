use kdata::resource::ResourceError;
use serde::Serialize;

/// API request handling error
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_message: String,
}

impl ErrorResponse {
    pub fn bad_request() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Bad request".to_string(),
        })
        .with_status_code(400)
    }
    pub fn unauthorized() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Unauthorized".to_string(),
        })
        .with_status_code(401)
    }
    pub fn forbidden() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Forbidden".to_string(),
        })
        .with_status_code(402)
    }
    pub fn not_found() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Could not find resource".to_string(),
        })
        .with_status_code(404)
    }
    pub fn not_allowed() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Method Not Allowed".to_string(),
        })
        .with_status_code(405)
    }
    pub fn conflict() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Conflict".to_string(),
        })
        .with_status_code(405)
    }
    pub fn pre_condition() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Pre-Condition failed".to_string(),
        })
        .with_status_code(412)
    }
    pub fn internal() -> rouille::Response {
        rouille::Response::json(&ErrorResponse {
            error_message: "Internal Server Error".to_string(),
        })
        .with_status_code(500)
    }

    pub fn map_resource_error(err: ResourceError) -> rouille::Response {
        match err {
            ResourceError::BadRequest => ErrorResponse::bad_request(),
            ResourceError::Unauthorized => ErrorResponse::unauthorized(),
            ResourceError::NotFound => ErrorResponse::not_found(),
            ResourceError::Forbidden => ErrorResponse::forbidden(),
            ResourceError::Conflict => ErrorResponse::conflict(),
            ResourceError::PreConditionFailed => ErrorResponse::pre_condition(),
            ResourceError::Internal => ErrorResponse::internal(),
        }
    }
}
