use openidconnect::{Nonce, PkceCodeVerifier};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OAuthCookie {
    pub pkce_verifier: PkceCodeVerifier,
    pub success_redirect_url: String,
    pub new_user_redirect_url: String,
    pub failure_redirect_url: String,
    pub nonce: Nonce,
}

impl OAuthCookie {
    pub fn new(
        pkce_verifier: PkceCodeVerifier,
        success_redirect_url: String,
        new_user_redirect_url: String,
        failure_redirect_url: String,
        nonce: Nonce,
    ) -> Self {
        OAuthCookie {
            pkce_verifier,
            success_redirect_url,
            new_user_redirect_url,
            failure_redirect_url,
            nonce,
        }
    }
}
