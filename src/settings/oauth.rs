use std::collections::HashMap;

use serde::Deserialize;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub enum SupportedOauthProviders {
    google,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub url: String,
    pub scopes: Vec<String>,
}

pub type OAuth = HashMap<SupportedOauthProviders, OAuthConfig>;
