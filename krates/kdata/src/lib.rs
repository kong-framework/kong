//! # 📄 kdata
//!
//! The kore data structures used in `kong`.
//!
//! ## `username` format
//! - cannot start with a underscore (_)
//! - can only contain letters, numbers, and one underscore
//! - can have only one (_) underscore
//!
//! ## `password` format
//! - should be at least 10 characters long

pub mod accounts;
pub mod inputs;
pub mod validate;
