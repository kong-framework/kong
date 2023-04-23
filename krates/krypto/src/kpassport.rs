//! # 🪪 Kong Passport
//!
//! An authorization token that uses the `keyed_hash()` function from
//! __Blake3__ instead of using __HMAC__. It provides authentication
//! using a secret key. We call such a token a `kpassport` (short for kong passport).
//!
//! ### Format
//! ```text
//! Base64([USERNAME]@[HOST][TIMESTAMP][SIGNATURE])
//!           15B      45B      33B        32B
//! ```
//! - __USERNAME__: The username of the entity the `kpassport` issued to.
//! The maximum length is 15bytes because `kong` account username have a
//! maximum length of 15 characters.
//! - __HOST__: The issuer of the `kpassport` can be a, the maximum length
//! 45bytes because that is the maximum IPv6 string length.  But any
//! string identifier can be used not just IP addresses as long as it
//! fits into 45bytes
//! - The __USERNAME__ and __HOST__ are seperated by the `@` characters (1byte)
//! - __TIMESTAMP__: The time the `kpassport` was issued, it is 3bytes long
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
//! __The maximum size of a `kpassport` is  125bytes__
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

use crate::error::KryptoError;
use base64::{engine::general_purpose, Engine as _};
use blake3::Hash;
use chrono::prelude::*;
use std::str;

/// The length of a username
/// A username cannot be longer than 15 characters.
/// A username can only contain alphanumeric characters (letters A-Z,
/// numbers 0-9) with the exception of underscores, as noted above.
const USERNAME_LENGTH_LIMIT: usize = 15;

/// The issuer of the `kpassport` can be a, the maximum length
/// 45bytes because that is the maximum IPv6 string length.  But any
/// string identifier can be used not just IP addresses as long as it
/// fits into 45bytes
const HOSTNAME_LENGTH_LIMIT: usize = 45;

/// The length of a timestamp created by chrono::Utc::now();
const TIMESTAMP_LENGTH: usize = 33;

/// The length of blake3::keyed_hash(), it is 32bytes long.
const SIGNATURE_LENGTH: usize = 32;

#[derive(Clone, PartialEq, Debug)]
/// The content of a `kpassport`
pub struct Content {
    /// The user that is being that is handed the kpassport
    pub username: String,
    /// Server or application that generated the kpassport
    pub host: String,
    /// The time when the kpassport was generated
    pub timestamp: DateTime<Utc>,
}

impl Content {
    /// Convert content to a string of bytes that can be signed
    /// the maximum length of the content is 90 bytes (45bytes for
    /// the host, 15bytes for the username and 33bytes for the timestamp)
    pub fn as_bytes(&self) -> Result<Vec<u8>, KryptoError> {
        let host_bytes: Vec<u8> = self.host.as_bytes().into();
        let username_bytes: Vec<u8> = self.username.as_bytes().to_vec();
        let timestamp_bytes: Vec<u8> = self.timestamp.to_string().as_bytes().to_vec();
        let seperator: Vec<u8> = "@".as_bytes().to_vec();

        let length_limit =
            HOSTNAME_LENGTH_LIMIT + USERNAME_LENGTH_LIMIT + TIMESTAMP_LENGTH + seperator.len();

        let bytes: Vec<u8> = vec![username_bytes, seperator, host_bytes, timestamp_bytes]
            .into_iter()
            .flatten()
            .collect();

        if bytes.len() > length_limit {
            Err(KryptoError::KpassportSize)
        } else {
            Ok(bytes)
        }
    }

    /// Derive a kpassport's content from bytes
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, KryptoError> {
        let username = Content::get_username(bytes.clone())?;
        let host = Content::get_host(bytes.clone())?;
        let timestamp = Content::get_timestamp(bytes)?;

        Ok(Content {
            host,
            username,
            timestamp,
        })
    }

    /// Get username from bytes
    fn get_username(bytes: Vec<u8>) -> Result<String, KryptoError> {
        let seperator_index = Content::get_seperator_index(bytes.clone())?;
        if let Some(username_bytes) = bytes.get(0..seperator_index) {
            let username = str::from_utf8(username_bytes);

            match username {
                Ok(username) => return Ok(String::from(username)),
                Err(_) => return Err(KryptoError::InvalidKpassportUsername),
            }
        }
        Err(KryptoError::InvalidKpassportUsername)
    }

    /// Get host from bytes
    fn get_host(bytes: Vec<u8>) -> Result<String, KryptoError> {
        let seperator_index = Content::get_seperator_index(bytes.clone())?;
        let timestamp_start = bytes.len() - TIMESTAMP_LENGTH;

        if let Some(host_bytes) = bytes.get(seperator_index + 1..timestamp_start) {
            let host = str::from_utf8(host_bytes);

            match host {
                Ok(host) => return Ok(String::from(host)),
                Err(_) => return Err(KryptoError::InvalidKpassportHost),
            }
        }
        Err(KryptoError::InvalidKpassportHost)
    }

    /// Get timestamp from bytes
    fn get_timestamp(bytes: Vec<u8>) -> Result<DateTime<Utc>, KryptoError> {
        let timestamp_start = bytes.len() - TIMESTAMP_LENGTH;

        if let Some(timestamp_bytes) = bytes.get(timestamp_start..) {
            let timestamp_str = str::from_utf8(timestamp_bytes);

            match timestamp_str {
                Ok(timestamp_str) => {
                    let timestamp = timestamp_str.parse::<DateTime<Utc>>();

                    match timestamp {
                        Ok(timestamp) => return Ok(timestamp),
                        Err(_) => return Err(KryptoError::InvalidKpassportTimestamp),
                    }
                }
                Err(_) => return Err(KryptoError::InvalidKpassportTimestamp),
            }
        }
        Err(KryptoError::InvalidKpassportTimestamp)
    }

    /// Get the index of a `kpassport` username and host seperator
    fn get_seperator_index(kpassport_bytes: Vec<u8>) -> Result<usize, KryptoError> {
        for i in 0..kpassport_bytes.len() {
            if kpassport_bytes[i] == b"@"[0] {
                return Ok(i);
            }
        }

        Err(KryptoError::MissingUsernameHostSeperator)
    }
}

#[derive(Clone, PartialEq, Debug)]
/// The `kpassport` authorization token
pub struct Kpassport {
    /// Kpassport content
    pub content: Content,
    /// The signature of the kpassport
    pub signature: Option<blake3::Hash>,
}

impl Kpassport {
    /// Generates a new __unsigned__ `kpassport`
    pub fn new_unsigned(username: &str, host: &str) -> Result<Kpassport, KryptoError> {
        if username.len() > USERNAME_LENGTH_LIMIT {
            return Err(KryptoError::KpassportSize);
        }

        if host.len() > HOSTNAME_LENGTH_LIMIT {
            return Err(KryptoError::KpassportSize);
        }

        let content = Content {
            username: username.to_string(),
            host: host.to_string(),
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
            host: &self.content.host,
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
                host: &self.content.host,
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

    /// Derive a `kpassport` from a base64 encoded string
    pub fn from_str(kpassport_str: &str) -> Result<Kpassport, KryptoError> {
        let kpassport_bytes = Kpassport::as_bytes(kpassport_str)?;
        let content_bytes = Kpassport::get_content_bytes(&kpassport_bytes)?;
        let signature_bytes = Kpassport::get_signature_bytes(&kpassport_bytes)?;

        Ok(Kpassport {
            content: Content::from_bytes(content_bytes.to_vec())?,
            signature: Some(Kpassport::signature_from_bytes(signature_bytes.to_vec())?),
        })
    }

    pub fn signature_from_bytes(bytes: Vec<u8>) -> Result<Hash, KryptoError> {
        let signature_hex = hex::encode(bytes);
        let signature = Hash::from_hex(signature_hex);

        match signature {
            Ok(signature) => Ok(signature),
            Err(_) => Err(KryptoError::InvalidKpassportSignature),
        }
    }

    /// Convert a `kpassport` base64 encoded string to bytes
    fn as_bytes(kpassport_str: &str) -> Result<Vec<u8>, KryptoError> {
        let bytes = general_purpose::URL_SAFE.decode(kpassport_str);

        match bytes {
            Ok(b) => Ok(b),
            Err(_) => Err(KryptoError::InvalidKpassport),
        }
    }

    /// Get Kpassport Content bytes from kpassport bytes
    fn get_content_bytes(kpassport_bytes: &[u8]) -> Result<&[u8], KryptoError> {
        let kpassport_content_length = kpassport_bytes.len() - SIGNATURE_LENGTH;

        if let Some(content_bytes) = kpassport_bytes.get(0..kpassport_content_length) {
            Ok(content_bytes)
        } else {
            Err(KryptoError::InvalidKpassport)
        }
    }

    /// Get Kpassport Signature bytes from kpassport bytes
    fn get_signature_bytes(kpassport_bytes: &[u8]) -> Result<&[u8], KryptoError> {
        let kpassport_content_length = kpassport_bytes.len() - SIGNATURE_LENGTH;

        if let Some(signature_bytes) = kpassport_bytes.get(kpassport_content_length..) {
            Ok(signature_bytes)
        } else {
            Err(KryptoError::InvalidKpassport)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kpassport_content_as_bytes() {
        let content = Content {
            host: "fdjdkfdjk".to_string(),
            username: "kjkdffdjdfjf".to_string(),
            timestamp: chrono::Utc::now(),
        };
        let content2 = Content {
            host: "fdjdkfdjk".to_string(),
            username: "kjkdffdjdfjfkdfjdfddkffdjdfjdflkdjdflkdflddflkdfjldfjdljdkjdfkldfldfjlkdjdkljdfkjdfkldjdkjdfkdjfkdfjdkfjdfkdf".to_string(),
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
    fn kpassport_content_from_bytes() {
        let content = Content {
            host: "fdjdkfdjk".to_string(),
            username: "kjkdffdjdfjf".to_string(),
            timestamp: chrono::Utc::now(),
        };
        let content_bytes = content.as_bytes().unwrap();
        let content2 = Content::from_bytes(content_bytes).unwrap();

        assert_eq!(content.username, content2.username);
        assert_eq!(content.host, content2.host);
        assert_eq!(content.timestamp, content2.timestamp);
    }
    #[test]
    fn get_username_from_kpassport_content_bytes() {
        let content = Content {
            host: "fdjdkfdjk".to_string(),
            username: "kjkdffdjdfjf".to_string(),
            timestamp: chrono::Utc::now(),
        };
        let content_bytes = content.as_bytes().unwrap();

        assert_eq!(
            content.username,
            Content::get_username(content_bytes.clone()).unwrap()
        );

        assert_ne!(
            "wrong username",
            Content::get_username(content_bytes).unwrap()
        );
    }

    #[test]
    fn get_host_from_kpassport_content_bytes() {
        let content = Content {
            host: "fdjdkfdjk".to_string(),
            username: "kjkdffdjdfjf".to_string(),
            timestamp: chrono::Utc::now(),
        };
        let content_bytes = content.as_bytes().unwrap();

        assert_eq!(
            content.host,
            Content::get_host(content_bytes.clone()).unwrap()
        );

        assert_ne!("wrong host", Content::get_host(content_bytes).unwrap());

        let content = Content {
            host: "ddlneuykjnnrsslin".to_string(),
            username: "difimnnn".to_string(),
            timestamp: chrono::Utc::now(),
        };
        let content_bytes = content.as_bytes().unwrap();

        assert_eq!(
            content.host,
            Content::get_host(content_bytes.clone()).unwrap()
        );

        assert_ne!(
            "wrong host very wrong",
            Content::get_host(content_bytes).unwrap()
        );
    }

    #[test]
    fn get_timestamp_from_kpassport_content_bytes() {
        let content = Content {
            host: "fdjdkfdjk".to_string(),
            username: "kjkdffdjdfjf".to_string(),
            timestamp: chrono::Utc::now(),
        };
        let content_bytes = content.as_bytes().unwrap();

        assert_eq!(
            content.timestamp,
            Content::get_timestamp(content_bytes.clone()).unwrap()
        );

        assert_ne!(
            "wrong username",
            Content::get_username(content_bytes).unwrap()
        );
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
                assert!(kpassport_string.len() < 125);
            }
            Err(_) => panic!("Could not export kpassport"),
        }
    }

    #[test]
    fn signature_from_bytes() {
        let mut kpassport = Kpassport::new_unsigned("My App", "my_username").unwrap();
        let key = "My super secret signing key";
        kpassport.sign(key).unwrap();

        match kpassport.signature {
            Some(signature) => {
                let signature_bytes = signature.as_bytes();
                let signature_hex = hex::encode(signature_bytes);
                let derived_signature = Hash::from_hex(signature_hex).unwrap();
                assert_eq!(signature, derived_signature);

                let key_derivation_context = crate::key_derivation::Context {
                    host: "www.example.com",
                    timestamp: Utc::now(),
                };
                let derived_key =
                    crate::key_derivation::derive_key(key_derivation_context, key.as_bytes());
                let wrong_hash = blake3::keyed_hash(&derived_key, b"wrong content");
                assert_ne!(wrong_hash, signature);
            }
            None => panic!("Kpassport not signed"),
        }
    }

    #[test]
    fn from_str() {
        let mut kpassport = Kpassport::new_unsigned("myusername", "myhostname").unwrap();
        kpassport.sign("secret key").unwrap();
        let kpassport_str = kpassport.clone().export().unwrap();
        let derived_kpassport = Kpassport::from_str(&kpassport_str).unwrap();

        assert_eq!(kpassport.content, derived_kpassport.content);
        assert_eq!(kpassport.signature, derived_kpassport.signature);
        assert_eq!(kpassport, derived_kpassport);
        assert_eq!(
            kpassport.content.username,
            derived_kpassport.content.username
        );
    }
}
