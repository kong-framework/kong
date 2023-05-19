use std::fmt;

#[derive(Debug)]
/// Kong errors
pub enum KError {
    /// Configuration error
    Config,
    /// URL parsing error
    UrlParsing,
    /// Invalid HTTP Method
    InvalidHttpMethod,
    /// Log file error
    LogFile,
}

impl std::error::Error for KError {}

impl fmt::Display for KError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config => write!(f, "Could not read config file"),
            Self::UrlParsing => write!(f, "Could not parse error"),
            Self::InvalidHttpMethod => write!(f, "Invalid HTTP method"),
            Self::LogFile => write!(f, "Log file error"),
        }
    }
}
