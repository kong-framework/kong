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

#![doc(html_favicon_url = "https://kwatafana.org/logo.jpeg")]
#![doc(html_logo_url = "https://kwatafana.org/logo.jpeg")]
#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

pub mod accounts;
pub mod inputs;
pub mod resource;
pub mod validate;
