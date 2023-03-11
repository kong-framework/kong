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
//!
//! 2.  Authorization header:
//!
//! A `kpassport` can be transported using HTTP headers, the
//! kpassport is sent in the Authorization header:
//!
//! ```text
//! Authorization: Bearer <the kpassport kpassport>
//! ```
//!
//! #### Expiration
//! A `kpassport` is timestamped at the time it is issued

use crate::{
    error::KryptoError,
    kpassport::{self, Kpassport},
};
use std::borrow::Cow;

#[derive(Clone)]
/// Kong authentication and authorization management
pub enum Auth {
    BearerToken(String),
    Cookie(String),
}

impl Auth {
    /// Detect if the type of authentication used (cookie based or bearer token based)
    /// from the requests headers
    pub fn detect_auth_type(auth_headers: AuthHeaders) -> Result<Auth, KryptoError> {
        // detect cookie based auth
        if let Some(kpass) = auth_headers.cookie {
            return Ok(Auth::Cookie(kpass.to_string()));
        }

        // detect bearer token based auth
        if let Some(kpass) = auth_headers.bearer_token {
            return Ok(Auth::BearerToken(kpass.to_string()));
        }

        Err(KryptoError::MissingAuthenticationCredentials)
    }

    /// Issue a kpassport using HTTP cookies
    pub fn issue_kpassport_cookie(self) -> (Cow<'static, str>, Cow<'static, str>) {
        let key = Cow::from("Set-Cookie");

        match self {
            Auth::BearerToken(kpassport) => {
                let value = Auth::cookie_value_string(&kpassport);
                (key, Cow::from(value))
            }
            Auth::Cookie(kpassport) => {
                let value = Auth::cookie_value_string(&kpassport);
                (key, Cow::from(value))
            }
        }
    }

    /// Issue a kpassport using a Bearer token
    pub fn issue_kpassport_bearer_token(self) -> (Cow<'static, str>, Cow<'static, str>) {
        let key = Cow::from("Authorization");

        match self {
            Auth::BearerToken(kpassport) => {
                let value = Auth::bearer_token_value_string(&kpassport);
                (key, Cow::from(value))
            }
            Auth::Cookie(kpassport) => {
                let value = Auth::bearer_token_value_string(&kpassport);
                (key, Cow::from(value))
            }
        }
    }

    /// Authenticate
    pub fn authenticate(self, key: &str) -> Result<(), KryptoError> {
        match self {
            Auth::BearerToken(kpassport_str) => {
                let kpassport = Kpassport::from_str(&kpassport_str)?;
                kpassport.validate(key)
            }
            Auth::Cookie(kpassport_str) => {
                let kpassport = Kpassport::from_str(&kpassport_str)?;
                kpassport.validate(key)
            }
        }
    }

    /// Generate a cookie value for storing a `kpassport`
    fn cookie_value_string(kpassport: &str) -> String {
        let cookie_name = "kpassport";

        // This ensures that the cookie is only sent over an HTTPS
        // connection and not HTTP.
        let cookie_transport = "Secure";

        // This ensures that the cookie is inaccessible to
        // the JavaScript Document.cookie API
        let cookie_accessibility = "HttpOnly";

        // TODO: calculate cookies expiration date from a kpassport

        format!("{cookie_name}={kpassport}; {cookie_transport}; {cookie_accessibility}")
    }

    /// Generate a bearer token value for storing a `kpassport`
    fn bearer_token_value_string(kpassport: &str) -> String {
        format!("Bearer {kpassport}")
    }
}

/// HTTP authentication methods
pub struct AuthHeaders<'a> {
    /// Cookie based authentication
    cookie: Option<&'a str>,
    /// Bearer token based authentication
    bearer_token: Option<&'a str>,
}

#[cfg(test)]
mod test {
    use crate::kpassport::Kpassport;

    use super::*;

    #[test]
    fn detect_auth_type() {
        let auth_header = AuthHeaders {
            cookie: Some("Set-Cookie: kpassport=<kpassport>; Secure; HttpOnly"),
            bearer_token: None,
        };

        let auth = Auth::detect_auth_type(auth_header).unwrap();

        match auth {
            Auth::BearerToken(_) => {
                panic!("Cookies auth header used and to Athorization bearer token")
            }
            Auth::Cookie(_) => (),
        }

        let auth_header = AuthHeaders {
            cookie: None,
            bearer_token: Some("Authorization: Bearer <the kpassport kpassport>"),
        };

        let auth = Auth::detect_auth_type(auth_header).unwrap();

        match auth {
            Auth::BearerToken(_) => (),
            Auth::Cookie(_) => panic!("Athorization bearer token header used and not cookies"),
        }

        let auth_header = AuthHeaders {
            cookie: None,
            bearer_token: None,
        };

        let auth = Auth::detect_auth_type(auth_header);

        if auth.is_ok() {
            panic!("Should error because no header provided");
        }
    }

    #[test]
    fn issue_cookie_kpassport() {
        let mut kpassport = Kpassport::new_unsigned("My App", "a username").unwrap();
        kpassport.sign("my secret signing key").unwrap();
        let auth = Auth::Cookie(kpassport.clone().export().unwrap());
        let issued_cookie = auth.issue_kpassport_cookie();
        let expected_cookie_value = format!(
            "kpassport={}; Secure; HttpOnly",
            kpassport.export().unwrap()
        );
        assert_eq!(
            issued_cookie,
            (Cow::from("Set-Cookie"), Cow::from(expected_cookie_value))
        )
    }

    #[test]
    fn issue_bearer_token_kpassport() {
        let mut kpassport = Kpassport::new_unsigned("My App", "a username").unwrap();
        kpassport.sign("my secret signing key").unwrap();
        let auth = Auth::BearerToken(kpassport.clone().export().unwrap());
        let issued_bearer_token = auth.issue_kpassport_bearer_token();
        let expected_cookie_value = format!("Bearer {}", kpassport.export().unwrap());
        assert_eq!(
            issued_bearer_token,
            (Cow::from("Authorization"), Cow::from(expected_cookie_value))
        )
    }

    #[test]
    fn authenticate() {
        let mut kpassport = Kpassport::new_unsigned("My App", "a username").unwrap();
        let key = "my secret signing key";
        kpassport.sign(key).unwrap();
        let kpassport_str = kpassport.export().unwrap();
        let cookie_auth = Auth::Cookie(kpassport_str.clone());
        let bearer_auth = Auth::BearerToken(kpassport_str);

        match cookie_auth.clone().authenticate(key) {
            Ok(_) => (),
            Err(_) => panic!("Should not error"),
        }

        match bearer_auth.clone().authenticate(key) {
            Ok(_) => (),
            Err(_) => panic!("Should not error"),
        }

        match cookie_auth.authenticate("wrong key") {
            Ok(_) => panic!("should error"),
            Err(_) => (),
        }

        match bearer_auth.authenticate("wrong key") {
            Ok(_) => panic!("should error"),
            Err(_) => (),
        }
    }

    #[test]
    fn cookie_value_string() {
        let mut kpassport = Kpassport::new_unsigned("My App", "a username").unwrap();
        kpassport.sign("my secret signing key").unwrap();
        let kpassport_str = kpassport.export().unwrap();
        let cookie_value = Auth::cookie_value_string(&kpassport_str);
        let expected = format!("kpassport={kpassport_str}; Secure; HttpOnly");

        assert_eq!(cookie_value, expected);
    }

    #[test]
    fn bearer_token_value_string() {
        let mut kpassport = Kpassport::new_unsigned("My App", "a username").unwrap();
        kpassport.sign("my secret signing key").unwrap();
        let kpassport_str = kpassport.export().unwrap();
        let bearer_token_value = Auth::bearer_token_value_string(&kpassport_str);
        let expected = format!("Bearer {kpassport_str}");

        assert_eq!(bearer_token_value, expected);
    }
}
