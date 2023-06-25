use actix_session::Session;
use actix_web::{get, http::StatusCode, web, HttpRequest, Responder, Result};
use openidconnect::reqwest::async_http_client;
use openidconnect::AuthorizationCode;
use sqlx::PgPool;

use super::shared::OAuthCookie;
use crate::constants::OAUTH_SECURITY_COOKIE_NAME;
use crate::errors::ApiError;
use crate::settings::SupportedOauthProviders;
use crate::state::AuthClients;
use models::OAuthCallbackQuery;
use process_id_token::process_open_id_info;

mod models;
mod process_id_token;

#[get("/{oauth_type}/callback")]
pub async fn oauth_get_access_token(
    path: web::Path<(SupportedOauthProviders,)>,
    query: web::Query<OAuthCallbackQuery>,
    req: HttpRequest,
    session: Session,
    auth_clients: web::Data<AuthClients>,
    pg_pool: web::Data<PgPool>,
) -> Result<impl Responder, ApiError> {
    let oauth_type = path.into_inner().0;

    let oauth_cookie_raw = req
        .cookie(OAUTH_SECURITY_COOKIE_NAME)
        .ok_or(ApiError::BadRequest(
            "Unable to find oauth cookie".to_string(),
        ))?;
    let oauth_cookie: OAuthCookie = serde_json::from_str(oauth_cookie_raw.value())?;

    let client = auth_clients.get(&oauth_type).ok_or(ApiError::Redirect(
        oauth_cookie.success_redirect_url.clone(),
        format!(
            "Unexpected error. Unable to perform authentication with {:?} service at this moment",
            oauth_type
        ),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))?;

    let token_response = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .set_pkce_verifier(oauth_cookie.pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            ApiError::Redirect(
                oauth_cookie.success_redirect_url.clone(),
                String::from("Error happened during authentication process"),
                StatusCode::BAD_REQUEST,
            )
        })?;

    process_open_id_info(
        &oauth_type,
        token_response,
        req,
        session,
        client,
        pg_pool,
        &oauth_cookie.nonce,
        oauth_cookie.success_redirect_url,
        oauth_cookie.new_user_redirect_url,
        oauth_cookie.failure_redirect_url,
    )
    .await
}
