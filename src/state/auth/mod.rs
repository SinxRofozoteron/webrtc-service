use std::collections::HashMap;

use openidconnect::core::CoreClient;
use openidconnect::reqwest::http_client;
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl, RevocationUrl};

use crate::settings::{OAuthConfig, Settings, SupportedOauthProviders};
use metadata::CommonProviderMetadata;

mod metadata;

pub type AuthClients = HashMap<SupportedOauthProviders, CoreClient>;

pub fn setup_auth_clients(config: &Settings) -> AuthClients {
    let base_url = &config.base_url;

    let mut clients: AuthClients = HashMap::new();

    for (provider, oauth_config) in config.oauth.iter() {
        let OAuthConfig {
            client_id,
            client_secret,
            url,
            scopes: _,
        } = oauth_config;

        let issuer_url = match IssuerUrl::new(url.to_owned()) {
            Ok(url) => url,
            Err(_) => {
                println!("Error during issue_url creation ({})", &url);
                continue;
            }
        };
        let provider_metadata = match CommonProviderMetadata::discover(&issuer_url, http_client) {
            Ok(metadata) => metadata,
            Err(err) => {
                println!(
                    "Error during retrospection for {:?} OAuth porovider {:?}",
                    provider, err
                );
                continue;
            }
        };

        let revocation_endpoint = provider_metadata
            .additional_metadata()
            .revocation_endpoint
            .clone();
        let revocation_uri = match RevocationUrl::new(revocation_endpoint) {
            Ok(url) => url,
            Err(err) => {
                println!("{:?}", err);
                continue;
            }
        };

        let redirect_endpoint = format!("{}/v1/auth/{:?}/callback", base_url, provider);
        let redirect_url = match RedirectUrl::new(redirect_endpoint) {
            Ok(url) => url,
            Err(_) => {
                continue;
            }
        };

        let client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(client_id.to_owned()),
            Some(ClientSecret::new(client_secret.to_owned())),
        )
        .set_redirect_uri(redirect_url)
        .set_revocation_uri(revocation_uri);

        clients.insert(provider.to_owned(), client);
    }

    clients
}
