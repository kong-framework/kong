//! # 🪪 Kong Passport
//!
//! An authorization token that uses the `keyed_hash()` function from
//! __Blake3__ instead of using __HMAC__. It provides authentication
//! using a secret key. We call such a token a `kpassport` (short for kong passport).
//!
//! ### Format
//! ```text
//! Base64([HOST][USERNAME][TIMESTAMP][SIGNATURE])
//!          45B    15B       30B        32B
//! ```
//! - __HOST__: The issuer of the `kpassport` can be a, the maximum length
//! 45bytes because that is the maximum IPv6 string length.  But any
//! string identifier can be used not just IP addresses as long as it
//! fits into 45bytes
//! - __USERNAME__: The username of the entity the `kpassport` issued to.
//! The maximum length is 15bytes because `kong` account username have a
//! maximum length of 15 characters.
//! - __TIMESTAMP__: The time the `kpassport` was issued, it is 30bytes long
//! - __SIGNATURE__: `blake3::keyed_hash()` of the `host`, `username` and `timestamp`,
//! it is 32bytes long.
//!
//! #### Why use blake3
//!
//! - Fast
//! - Pure __Rust__ implementation written by the creators of blake3
//! (`kong` is also written in Rust).
//!
//! #### HTTPS
//!
//! The HTTP protocol transfers data in cleartext, and a
//! "man-in-the-middle" can see all the data (HTTP requests and responses)
//! being transferred between client and server. This is why an HTTPS
//! connection should be used, it creates a secure channel between client
//! and server that is not vulnerable to "MITM" attacks.
//!
//! #### Size
//!
//! Web browsers limit the amount of storage size a domain can use for
//! cookies. All the cookies under one domain cannot exceed 4KB (4093
//! bytes per domain). This means you can have 1 cookie of 4093 bytes,
//! or 2 cookies of 2045 bytes, etc.
//!
//! __The maximum size of a `kpassport` is  122bytes__
//!
//! #### Security
//! - [ ] A `kpassport` is unique
//! - [ ] A `kpassport` is not guessable (they are randomly generated).
//!
//!
//! ## 🚪 Authorization
//!
//! The main idea is to store the user’s info in the `kpassport`.  And
//! to secure it, have the user's info be signed using a secret that’s
//! only known to the server.
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
//! ```
//! TODO: how to calculate a cookies expiration date from a kpassport
//! ```
//!
//! ```
//! Set-Cookie: session=<kpassport kpassport>; Expires=Thu, 21 Oct 2021 07:28:00 GMT; Secure; HttpOnly
//! ```
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

use crate::error::KryptoError;
use base64::{engine::general_purpose, Engine as _};
use chrono::prelude::*;

/// The length of a username
/// A username cannot be longer than 15 characters.
/// A username can only contain alphanumeric characters (letters A-Z,
/// numbers 0-9) with the exception of underscores, as noted above.
pub const USERNAME_LENGTH_LIMIT: usize = 15;

/// The issuer of the `kpassport` can be a, the maximum length
/// 45bytes because that is the maximum IPv6 string length.  But any
/// string identifier can be used not just IP addresses as long as it
/// fits into 45bytes
pub const HOSTNAME_LENGTH_LIMIT: usize = 45;

/// The length of a timestamp created by chrono::Utc::now();
pub const TIMESTAMP_LENGTH: usize = 30;

/// The content of a `kpassport`
pub struct Content<'a> {
    /// Server or application that generated the kpassport
    pub host: &'a str,
    /// The user that is being that is handed the kpassport
    pub username: &'a str,
    /// The time when the kpassport was generated
    pub timestamp: DateTime<Utc>,
}

impl<'a> Content<'a> {
    /// Convert content to a string of bytes that can be signed
    /// the maximum length of the content is 90 bytes (45bytes for
    /// the host, 15bytes for the username and 30bytes for the signature)
    pub fn as_bytes(&self) -> Result<Vec<u8>, KryptoError> {
        let host_bytes: Vec<u8> = self.host.as_bytes().into();
        let username_bytes: Vec<u8> = self.username.as_bytes().to_vec();
        let timestamp_bytes: Vec<u8> = self.timestamp.to_string().as_bytes().to_vec();

        let bytes: Vec<u8> = vec![host_bytes, username_bytes, timestamp_bytes]
            .into_iter()
            .flatten()
            .collect();

        let length_limit = HOSTNAME_LENGTH_LIMIT + USERNAME_LENGTH_LIMIT + TIMESTAMP_LENGTH;

        if bytes.len() > length_limit {
            Err(KryptoError::KpassportSize)
        } else {
            Ok(bytes)
        }
    }
}

/// The `kpassport` authorization token
pub struct Kpassport<'a> {
    /// Kpassport content
    pub content: Content<'a>,
    /// The signature of the kpassport
    pub signature: Option<blake3::Hash>,
}

impl<'a> Kpassport<'a> {
    /// Generates a new __unsigned__ `kpassport`
    pub fn new_unsigned(host: &'a str, username: &'a str) -> Result<Kpassport<'a>, KryptoError> {
        if username.len() > USERNAME_LENGTH_LIMIT {
            return Err(KryptoError::KpassportSize);
        }

        if host.len() > HOSTNAME_LENGTH_LIMIT {
            return Err(KryptoError::KpassportSize);
        }

        let content = Content {
            host,
            username,
            timestamp: Utc::now(),
        };

        Ok(Kpassport {
            content,
            signature: None,
        })
    }

    /// Sign the __kpassport__
    pub fn sign(&mut self, key: &str) -> Result<(), KryptoError> {
        let key_derivation_context = crate::key_derivation::Context {
            host: self.content.host,
            timestamp: self.content.timestamp,
        };

        let derived_key = crate::key_derivation::derive_key(key_derivation_context, key.as_bytes());
        self.signature = Some(blake3::keyed_hash(&derived_key, &self.content.as_bytes()?));

        Ok(())
    }

    /// Validate the __kpassport__ (see if it was signed with the key)
    pub fn validate(&self, key: &str) -> Result<(), KryptoError> {
        if let Some(signature) = self.signature {
            let key_derivation_context = crate::key_derivation::Context {
                host: self.content.host,
                timestamp: self.content.timestamp,
            };

            let derived_key =
                crate::key_derivation::derive_key(key_derivation_context, key.as_bytes());
            let s = blake3::keyed_hash(&derived_key, &self.content.as_bytes()?);

            if s != signature {
                return Err(KryptoError::InvalidKpassportSignature);
            }

            Ok(())
        } else {
            Err(KryptoError::KpassportNotSigned)
        }
    }

    /// Export kpassport as url safe base64 String
    pub fn export(self) -> Result<String, KryptoError> {
        if let Some(signature) = self.signature {
            let content_bytes = self.content.as_bytes()?;
            let signature_bytes: Vec<u8> = signature.as_bytes().to_vec();
            let kpassport_bytes: Vec<u8> = vec![content_bytes, signature_bytes]
                .into_iter()
                .flatten()
                .collect();

            let encoded_kpassport: String = general_purpose::URL_SAFE.encode(kpassport_bytes);
            Ok(encoded_kpassport)
        } else {
            Err(KryptoError::KpassportNotSigned)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kpassport_content_as_bytes() {
        let content = Content {
            host: "fdjdkfdjk",
            username: "kjkdffdjdfjf",
            timestamp: chrono::Utc::now(),
        };
        let content2 = Content {
            host: "fdjdkfdjk",
            username: "kjkdffdjdfjfkdfjdfddkffdjdfjdflkdjdflkdflddflkdfjldfjdljdkjdfkldfldfjlkdjdkljdfkjdfkldjdkjdfkdjfkdfjdkfjdfkdf",
            timestamp: chrono::Utc::now(),
        };

        let content_bytes = content.as_bytes();
        let content2_bytes = content2.as_bytes();

        if content_bytes.is_err() {
            panic!("Should not error");
        }

        if content2_bytes.is_ok() {
            panic!("Should error because the username is too large");
        }
    }

    #[test]
    fn new_unsigned_kpassport() {
        let kpassport = Kpassport::new_unsigned("My App", "my_username");
        let kpassport2 = Kpassport::new_unsigned(
            "My fdkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkddffffffffakjdfApp",
            "my_username",
        );
        let kpassport3 = Kpassport::new_unsigned(
            "My App",
            "my_usernamedsfhjdffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        );

        match kpassport {
            Ok(kp) => {
                if let Some(sig) = kp.signature {
                    panic!("kpassport should not be signed");
                }
            }
            Err(_) => panic!("Should not error"),
        }

        if kpassport2.is_ok() {
            panic!("Should error because the hostname is too large")
        }

        if kpassport3.is_ok() {
            panic!("Should error because the username is too large")
        }
    }

    #[test]
    fn kpassport_sign() {
        let mut kpassport = Kpassport::new_unsigned("My App", "my_username").unwrap();
        let key = "My super secret signing key";
        kpassport.sign(key).unwrap();

        match kpassport.signature {
            Some(sig) => {
                assert_eq!(sig.as_bytes().len(), 32)
            }
            _ => panic!("Kpassport signature not found"),
        }
    }

    #[test]
    fn kpassport_validate() {
        let mut kpassport = Kpassport::new_unsigned("My App", "my_username").unwrap();
        let key = "My super secret signing key";
        kpassport.sign(key).unwrap();

        let validation = kpassport.validate(key);
        let validation2 = kpassport.validate("wrong key");

        if validation.is_err() {
            panic!("Should not error");
        }

        if validation2.is_ok() {
            panic!("Should error because wrong key was provided");
        }

        let mut kpassport = Kpassport::new_unsigned("My App", "my_username").unwrap();
        let validation = kpassport.validate(key);

        if validation.is_ok() {
            panic!("Should error, because kpassport is not signed");
        }
    }

    #[test]
    fn kpassport_export() {
        let mut kpassport = Kpassport::new_unsigned("My App", "my_username").unwrap();
        let key = "My super secret signing key";
        kpassport.sign(key).unwrap();
        let export = kpassport.export();

        match export {
            Ok(kpassport_string) => {
                assert!(kpassport_string.len() < 122);
            }
            Err(_) => panic!("Could not export kpassport"),
        }
    }
}
