//! Management of cryptographic keys

use chrono::prelude::*;

/// The context of a key derivation
pub(crate) struct Context<'a> {
    /// Server or application that derives and use the key
    pub(crate) host: &'a str,
    /// The time when the key is derived
    pub(crate) timestamp: DateTime<Utc>,
}

impl<'a> Context<'a> {
    /// Convert context to string
    pub(crate) fn to_string(&self) -> String {
        format!("{} {} kpassport-token", self.host, self.timestamp)
    }
}

/// Derive a 32 byte key from specified context and
/// key material
pub(crate) fn derive_key(context: Context<'_>, key_material: &[u8]) -> [u8; 32] {
    blake3::derive_key(&context.to_string(), key_material)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let key_material = "my super secure secret key".as_bytes();
        let context = Context {
            host: "My Application",
            timestamp: Utc::now(),
        };

        let derived_key = derive_key(context, key_material);

        let key_material2 = "secret".as_bytes();
        let context2 = Context {
            host: "My Application",
            timestamp: Utc::now(),
        };

        let derived_key2 = derive_key(context2, key_material2);

        assert_eq!(derived_key.len(), 32);
        assert_eq!(derived_key2.len(), 32);
        assert_ne!(derived_key, derived_key2);
    }
}
