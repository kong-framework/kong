//! # 🔑 Authentication
//!
//! - [ ] __Username__ and __password__ is used to authenticate users (humans).
//! - [x] Usernames are __alphanumeric__ (letters A-Z, numbers 0-9) with the exception of __underscores__.
//! - [x] Password should be at least 10 characters long
//! - [x] The user's password is __hashed__ with `scrypt` and the hash
//! is stored in the database.
//! - [ ] The username may be claimed by a suspended or deactivated
//! account. Suspended and deactivated usernames are not immediately
//! available for use.
//! - [ ] After the user has been authenticated, they are handed a
//! __passport__ that should send with requests to private resources.
//!- [ ] `kong` allows  a reserve list of usernames that
//!can never be used by end-users (e.g __admin__)
//!
//! #### Attaching to HTTP requests
//! Clients that request to access protected routes, need to provide a
//! valid `kpassport`, they do this by attaching a `kpassport` with every
//! request to a protected route. There are two ways to attach a
//! `kpassport` to a request:
//!
//! 1. HTTP Cookie
//!
//! The cookies __Secure__ attribute is set, this ensures that the
//! cookie is only sent over an HTTPS connection and not HTTP. This means
//! that the cookie (`kpassport`) cannot be accessed by "MITM" attackers.
//!
//! The cookies __HttpOnly__ attribute is also set, this ensures that
//! the cookie is inaccessible to the JavaScript `Document.cookie` API. So
//! the cookie cannot be read or modified by client side JavaScript.
//!
//! Cookie expiration date is also set. It is calculated from
//! the `kpassport`'s timestamp:
//!
//!
//! > TODO: how to calculate a cookies expiration date from a kpassport
//!
//! > Set-Cookie: session=<kpassport>; Expires=Thu, 21 Oct 2021 07:28:00 GMT; Secure; HttpOnly
//!
//! #### Expiration
//! A `kpassport` is timestamped at the time it is issued

use crate::{error::KryptoError, kpassport::Kpassport};
use std::borrow::Cow;

#[derive(Clone)]
/// Kong authentication and authorization management
pub struct Auth;

impl Auth {
    /// Issue a kpassport using HTTP cookies
    pub fn issue_kpassport_cookie(
        username: &str,
        host: &str,
        signing_key: &str,
        cookie_name: &str,
    ) -> Result<(Cow<'static, str>, Cow<'static, str>), KryptoError> {
        let mut kpassport = Kpassport::new_unsigned(username, host)?;
        kpassport.sign(signing_key)?;

        let header_key = Cow::from("Set-Cookie");

        match kpassport.export() {
            Ok(kpassport_str) => Ok((
                header_key,
                Cow::from(Auth::cookie_value_string(&kpassport_str, cookie_name)),
            )),
            Err(err) => Err(err),
        }
    }

    /// Generate a cookie value for storing a `kpassport`
    fn cookie_value_string(kpassport: &str, cookie_name: &str) -> String {
        // This ensures that the cookie is only sent over an HTTPS
        // connection and not HTTP.
        let cookie_transport = "Secure";

        // This ensures that the cookie is inaccessible to
        // the JavaScript Document.cookie API
        let cookie_accessibility = "HttpOnly";

        // TODO: calculate cookies expiration date from a kpassport

        format!("{cookie_name}={kpassport}; {cookie_transport}; {cookie_accessibility}")
    }
}

#[cfg(test)]
mod test {}
