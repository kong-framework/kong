//! # 🕹️ inputs
//!
//! Data that is received as input from users, usually other data
//! types are created from this input.

use crate::validate::ValidationError;

/// User input
pub trait UserInput {
    /// Validate user input
    fn is_valid(&self) -> Result<(), ValidationError>;
}
