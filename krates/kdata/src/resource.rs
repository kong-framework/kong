//! A resource that can be accessed via http
use serde_json::json;
use std::fmt;

/// A generic resource
pub struct GenericResource {
    pub message: String,
}
impl GenericResource {
    /// new generic resource
    pub fn as_json(&self) -> serde_json::Value {
        json!({ "message": self.message })
    }
}

#[derive(Debug)]
pub enum ResourceError {
    /// Generic error (400)
    BadRequest,
    /// Unauthorized (401)
    Unauthorized,
    /// Resource could not be found (404)
    NotFound,
    /// Forbidden (402)
    Forbidden,
    /// Conflict (409)
    Conflict,
    /// Pre-condition failed (412)
    PreConditionFailed,
    /// Internal Error (500)
    Internal,
}

impl std::error::Error for ResourceError {}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest => write!(f, "Bad Request"),
            Self::Unauthorized => write!(f, "Unnauthorized"),
            Self::NotFound => write!(f, "Resource not found"),
            Self::Forbidden => write!(f, "Forbidden"),
            Self::Conflict => write!(f, "Conflict"),
            Self::PreConditionFailed => write!(f, "Pre-condition failed"),
            Self::Internal => write!(f, "Internal"),
        }
    }
}
