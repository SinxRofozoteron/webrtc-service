use actix_web::{
    cookie::{time, Cookie},
    get,
    http::header,
    web, HttpRequest, HttpResponse, Result,
};
use openidconnect::core::CoreResponseType;
use openidconnect::{AuthenticationFlow, CsrfToken, Nonce, PkceCodeChallenge, Scope};
use serde_json::to_string;

use super::shared::OAuthCookie;
use crate::constants::OAUTH_SECURITY_COOKIE_NAME;
use crate::errors::ApiError;
use crate::settings::{Settings, SupportedOauthProviders};
use crate::state::AuthClients;

mod models;

#[get("/{oauth_type}")]
async fn oauth_start(
    path: web::Path<(SupportedOauthProviders,)>,
    query: web::Query<models::StartOAuthQuery>,
    config: web::Data<Settings>,
    auth_clients: web::Data<AuthClients>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let oauth_type = path.into_inner().0;
    let scopes = &config.oauth[&oauth_type].scopes;
    let mut success_redirect_url = query.success_redirect_url.clone();
    if success_redirect_url == None {
        let origin = req
            .headers()
            .get(header::REFERER)
            .ok_or_else(|| {
                ApiError::BadRequest(
                    "Please provide success_redirect_url query parameter".to_string(),
                )
            })?
            .to_str()?;
        success_redirect_url = Some(origin.to_string());
    }
    let failure_redirect_url = query.failure_redirect_url.clone();
    let new_user_redirect_url = query.new_user_redirect_url.clone();

    let client = auth_clients
        .get(&oauth_type)
        .ok_or(ApiError::InternalServerError(format!(
            "Unexpected error. Unable to perform authentication with {:?} service at this moment",
            oauth_type
        )))?;

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Construct Authorization Code request url
    let mut auth_request = client.authorize_url(
        AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
        CsrfToken::new_random,
        Nonce::new_random,
    );

    for scope in scopes {
        auth_request = auth_request.add_scope(Scope::new(scope.to_string()));
    }

    let (auth_url, _, nonce) = auth_request.set_pkce_challenge(pkce_challenge).url();

    // Create cookie value string with info needed to complete AAuth process
    let oauth_cookie_value = to_string(&OAuthCookie::new(
        pkce_verifier,
        // Can safely unwrap because if it was None we would return with ApiError a long time ago
        success_redirect_url.unwrap(),
        new_user_redirect_url,
        failure_redirect_url,
        nonce,
    ))?;

    // Send a response with a redirect to an authorization server
    Ok(HttpResponse::SeeOther()
        .insert_header((&header::LOCATION, auth_url.to_string()))
        .cookie(
            Cookie::build(OAUTH_SECURITY_COOKIE_NAME, oauth_cookie_value)
                .secure(true)
                .http_only(true)
                .max_age(time::Duration::MINUTE)
                .path("/v1/auth")
                .finish(),
        )
        .finish())
}
