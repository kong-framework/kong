use std::fmt;

#[derive(Debug)]
pub enum KryptoError {
    /// Invalid kpassport size
    KpassportSize,
    /// Invalid kpassport signature
    InvalidKpassportSignature,
    /// kpassport has not yet been signed
    KpassportNotSigned,
    /// Authentication credentials missing
    MissingAuthenticationCredentials,
    /// Kpassport username and host seperator not found
    MissingUsernameHostSeperator,
    /// Invalid kpassport
    InvalidKpassport,
    /// Invalid kpassport username
    InvalidKpassportUsername,
    /// Invalid kpassport host
    InvalidKpassportHost,
    /// Invalid kpassport timestamp
    InvalidKpassportTimestamp,
}

impl std::error::Error for KryptoError {}

impl fmt::Display for KryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KpassportSize => write!(f, "Invalid kpassport size"),
            Self::InvalidKpassportSignature => write!(f, "Invalid kpassport signature"),
            Self::KpassportNotSigned => write!(f, "kpassport has not yet been signed"),
            Self::MissingAuthenticationCredentials => {
                write!(f, "Authentication credentials missing")
            }
            Self::MissingUsernameHostSeperator => {
                write!(f, "Kpassport username and host seperator not found")
            }
            Self::InvalidKpassportUsername => {
                write!(f, "Invalid Kpassport username")
            }
            Self::InvalidKpassportHost => {
                write!(f, "Invalid Kpassport host")
            }
            Self::InvalidKpassportTimestamp => {
                write!(f, "Invalid Kpassport timestamp")
            }
            Self::InvalidKpassport => {
                write!(f, "Invalid Kpassport")
            }
        }
    }
}
