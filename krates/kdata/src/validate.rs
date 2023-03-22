//! # validate
//!
//! User input data validation
//!
//! ## References
//! - <https://beesbuzz.biz/code/439-Falsehoods-programmers-believe-about-email>

pub struct Validate;

impl Validate {
    /// Validate username (using Twitter style usernames)
    pub fn username(username: &str) -> bool {
        // Username cannot be empty
        if username.is_empty() {
            return false;
        }

        // Username length cannot be greater than 15 characters
        if username.len() > 15 {
            return false;
        }

        let chars: Vec<char> = username.chars().collect();
        let mut underscore_count = 0;

        for i in 0..chars.len() {
            // Username cannot start with a underscore (_)
            if i == 0 && chars[i] == '_' {
                return false;
            }

            // Username can only contain letters, numbers, and one underscore
            if chars[i] != '_' {
                if !chars[i].is_ascii_alphanumeric() {
                    return false;
                }
            } else {
                // Count underscores because username can have only one (_) underscore
                underscore_count += 1;
            }
        }

        // Username can have only one (_) underscore
        if underscore_count > 1 {
            return false;
        }
        true
    }

    /// TODO: better email validation
    pub fn email(email: &str) -> bool {
        // Email address cannot be empty
        if email.is_empty() {
            return false;
        }

        // Email address should contain @ and . symbols
        if !email.contains('@') || !email.contains('.') {
            return false;
        }
        true
    }

    /// TODO: better password validation
    pub fn password(password: &str) -> bool {
        // Password should be at least 10 characters long
        if password.len() < 10 {
            return false;
        }
        true
    }
}

use std::fmt;

#[derive(Debug)]
pub enum ValidationError {
    Generic,
    Username,
    Email,
    Password,
}

impl std::error::Error for ValidationError {}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::Generic => write!(f, "General input error"),
            ValidationError::Username => write!(f, "Invalid username"),
            ValidationError::Email => write!(f, "Invalid email"),
            ValidationError::Password => write!(f, "Invalid password"),
        }
    }
}

/// Information about the running node
#[cfg(feature = "node_information")]
#[derive(Deserialize, Serialize)]
pub struct NodeInfo {
    /// Version of kong that is running on the node
    pub version: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn username_validation() {
        let valid_username = "firephoenix";
        let valid_username1 = "natty_dread";
        let valid_username2 = "ironlion_ironii";
        let invalid_username = "";
        let invalid_username1 = "natty_dread_iron";
        let invalid_username2 = "_natty_dread_iron";
        let invalid_username3 = "nattydre@d";
        let invalid_username4 = "nattydread$";
        let invalid_username5 = "firephoenixfirephoenixfirephoenixfirephoenixfirephoenix";

        assert!(Validate::username(valid_username));
        assert!(Validate::username(valid_username1));
        assert!(Validate::username(valid_username2));

        // username cannot be empty
        assert!(!Validate::username(invalid_username));
        // username cannot contain more than 1 underscore
        assert!(!Validate::username(invalid_username1));
        // username cannot start with an underscore
        assert!(!Validate::username(invalid_username2));
        // username cannot contain special symbols (only the _ is allowed)
        assert!(!Validate::username(invalid_username3));
        assert!(!Validate::username(invalid_username4));
        // username cannot be more than 15 characters long
        assert!(!Validate::username(invalid_username5));
    }

    #[test]
    fn email_validation() {
        let valid_email = "example@example.com";
        let valid_email1 = "test@gmail.com";
        let invalid_email = "kdkdf dfkfd@uurt";
        let invalid_email2 = "lolz.lolz";
        let invalid_email3 = "lolz@lolz";

        assert!(Validate::email(valid_email));
        assert!(Validate::email(valid_email1));

        assert!(!Validate::email(invalid_email));
        assert!(!Validate::email(invalid_email2));
        assert!(!Validate::email(invalid_email3));
    }

    #[test]
    fn password_validation() {
        let valid_password = "passwordShoul be at least 10 chars";
        let invalid_password = "lolz";

        assert!(Validate::password(valid_password));
        // Password should be at least 10 characters long
        assert!(!Validate::password(invalid_password));
    }
}
