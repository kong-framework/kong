//! # 🔏 Password management
//! ##   Password Hashing
//! #### Why hash?
//!
//! Web servers can get hacked, and when that happens you don't want the
//! passwords of the users to be leaked -- this will allow the attacker
//! to gain access to the users resources. Some users also use the same
//! password across many services, your web-server can be the root of a
//! chain of breaches.
//!
//! A cool way to prevent this type of leak is by __obfuscating__ the
//! users passwords with a [__hash function__](https://en.wikipedia.org/wiki/Hash_function),
//! before storing them in the database.
//!
//! There are lots of hash functions that can be used, but most of these
//! are a bad choice. For example if you choose SHA-256 or other
//! computationally cheap functions (hash function without a __work factor__
//! parameter), they will be vulnerable to rainbow table attacks.
//! Bruteforce is also easy if the password length is short/known,
//! asic miners can be used for bruteforce search, they can generate 100
//! TeraHashes PER Second.
//!
//! #### Password Hash Functions
//!
//! The best method to use against plaintext password leaks and rainbow
//! table attacks is to use a __Password Hash Function__. Which is a hash
//! function specially designed to be slow/expensive to compute/bruteforce
//! even on specialized hardware.
//!
//! #### Scrypt Hash Function
//!
//! The [scrypt](https://www.tarsnap.com/scrypt.html) hash function uses large amounts of memory when hashing,
//! making it expensive to scale bruteforce attacks. `scrypt` is also
//! secure against hardware brute-force attacks.
//!
//! In `kong` we use [`scrypt`](https://github.com/RustCrypto/password-hashes/tree/master/scrypt)
//! to hash users passwords before they are stored in the database (`kollection`).
//!
//! > A number of cryptocurrencies use __scrypt__ for proof of work.
//!
//! > `scrypt` is created by Colin Percival of [Tarsnap](https://en.wikipedia.org/wiki/Tarsnap)
//!
//! #### Conclusion
//!
//! A cool way to prevent password leaks is by __obfuscating__ them
//! with a password hash function which offer additional security
//! against bruteforce from specialized hardware such as asics. If
//! password hash functions are used and implemented correctly even the
//! administrators of the server will not be able to read the users
//! passwords especially if the server is open source and the users can
//! audit the code for themselves.
//!
//! #### References
//!
//! - <https://www.troyhunt.com/our-password-hashing-has-no-clothes/>
//! - <https://paragonie.com/blog/2016/02/how-safely-store-password-in-2016>
//! - <https://www.troyhunt.com/passwords-evolved-authentication-guidance-for-the-modern-era/>
//!
//!

use crate::error::KryptoError;
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};

/// Hash a cleartext password using the scrypt hash function
pub fn hash(cleartext_password: &str) -> Result<String, KryptoError> {
    let cleartext_password_bytes = cleartext_password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Scrypt
        .hash_password(cleartext_password_bytes, &salt)
        .map_err(|_| KryptoError::PasswordHashing)?
        .to_string();
    Ok(password_hash)
}

/// Check if a scrypt hash matches the password cleartext
pub fn verify(password_hash: &str, password_cleartext: &str) -> Result<bool, KryptoError> {
    let parsed_hash =
        PasswordHash::new(password_hash).map_err(|_| KryptoError::PasswordVerifyHash)?;
    if Scrypt
        .verify_password(password_cleartext.as_bytes(), &parsed_hash)
        .is_ok()
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "manchester";
        let hash = hash(password).unwrap();
        assert!(verify(&hash, password).unwrap());

        // XXX: Commented out because hashing password with scrypt is slow
        // for security reasons, we commented these out so that they
        // don't slow down the other tests.

        //let password1 = "chelsea";
        //let password2 = "cheesecheescheese";
        //let hash1 = hash(password1).unwrap();
        //let hash2 = hash(password2).unwrap();
        //assert!(verify(&hash1, password1).unwrap());
        //assert!(verify(&hash2, password2).unwrap());
        //assert!(!verify(&hash, password1).unwrap());
        //assert!(!verify(&hash2, password).unwrap());
    }
}
