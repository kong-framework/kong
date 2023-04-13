//! # 🔐 krypto
//!
//! `kong` cryptography and security
//!
//! ## TODO
//!
//! - 🛡️ Physical Security
//!   - Machine Identity
//!   - Secure Boot Stack
//! - 🚀 Secure Service Deployment
//! - 🎫 Service Identity, Integrity, and Isolation
//! - 🎭 Inter-Service Access Management
//! - 🎭 🔐 Encryption of Inter-Service Communication
//! - 🚪 Access Management of End User Data
//! - 🚨  Intrusion Detection
//! - 🎡 Denial of Service (DoS) Protection
//! - 🔐📄 Secure Data Storage
//!   - Encryption at Rest
//!     - Deletion of data

#![doc(html_favicon_url = "https://kwatafana.org/logo.jpeg")]
#![doc(html_logo_url = "https://kwatafana.org/logo.jpeg")]
#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

pub mod authentication;
pub mod defaults;
pub mod error;
mod key_derivation;
pub mod kpassport;
pub mod password;
