use kerror::KError;
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use std::borrow::Cow;

pub struct Krypto;

impl Krypto {
    pub fn hash_password(cleartext_password: &str) -> Result<String, KError> {
        let cleartext_password_bytes = cleartext_password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt
            .hash_password(cleartext_password_bytes, &salt)
            .map_err(|_| KError::PasswordHashing)?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password(password_hash: &str, password_cleartext: &str) -> Result<bool, KError> {
        let parsed_hash =
            PasswordHash::new(&password_hash).map_err(|_| KError::PasswordVerifyHash)?;
        if Scrypt
            .verify_password(password_cleartext.as_bytes(), &parsed_hash)
            .is_ok()
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Clone)]
pub struct SessionCookie {
    pub id: String,
    pub name: String,
}

impl SessionCookie {
    // pub fn new(name: &str) -> Self {
    //     Self {
    //         id: rouille::session::generate_session_id().to_string(),
    //         name: name.to_string(),
    //     }
    // }

    pub fn to_header(&self) -> (Cow<'static, str>, Cow<'static, str>) {
        let key = Cow::from("Set-Cookie");
        let value = Cow::from(self.cookie_value_string());
        (key, value)
    }

    fn cookie_value_string(&self) -> String {
        let name = &self.name;
        let session_id = &self.id;
        format!("{name}={session_id};Secure;HttpOnly")
    }
}

// /// Check if a user is logged in as the admin.
// /// returns `Some(AdminAccount)` if user is logged
// /// as addmin and returns None if user is not logged in
// /// as admin
// pub fn is_logged_in_as_admin(
//     cookies: rouille::input::CookiesIter,
//     runtime: &Runtime,
// ) -> Option<AdminAccount> {
//     let cookie = cookies
//         .filter(|c| c.0 == runtime.config.auth_cookie_name)
//         .next();

//     if let Some(cookie) = cookie {
//         if let Some(admin) = runtime.admin_sessions.get(cookie.1) {
//             return Some(admin.clone());
//         } else {
//             return None;
//         }
//     }

//     None
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "manchester";
        let hash = Krypto::hash_password(password).unwrap();
        assert!(Krypto::verify_password(&hash, password).unwrap());
        //let password1 = "chelsea";
        //let password2 = "cheesecheescheese";
        //let hash1 = Krypto::hash_password(password1).unwrap();
        //let hash2 = Krypto::hash_password(password2).unwrap();
        //assert!(Krypto::verify_password(&hash1, password1).unwrap());
        //assert!(Krypto::verify_password(&hash2, password2).unwrap());
        //assert!(!Krypto::verify_password(&hash, password1).unwrap());
        //assert!(!Krypto::verify_password(&hash2, password).unwrap());
    }
}
