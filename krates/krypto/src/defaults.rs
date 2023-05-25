//! Default values

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
