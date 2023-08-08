use crate::Kong;

use krypto::{error::KryptoError, kpassport::Kpassport};

/// Get valid auth token
pub(crate) fn get_kpassport(
    kong: &Kong,
    request: &rouille::Request,
) -> Result<Kpassport, KryptoError> {
    let kpassport_signing_key = &kong.config.secret_key;
    let auth_cookie_name = &kong.config.auth_cookie_name;

    // Try to get kpassport from the HTTP cookie
    if let Some((_, cookie_value)) =
        rouille::input::cookies(request).find(|&(n, _)| n == auth_cookie_name)
    {
        if let Ok(kpassport) = krypto::kpassport::Kpassport::from_str(cookie_value) {
            // validate kpassport
            if kpassport.validate(kpassport_signing_key).is_ok() {
                Ok(kpassport)
            } else {
                // could not validate kpassport
                Err(KryptoError::InvalidKpassport)
            }
        } else {
            // could not read cookie token
            Err(KryptoError::InvalidKpassport)
        }
    } else {
        // Cookie not found
        Err(KryptoError::InvalidKpassport)
    }
}
