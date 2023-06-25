use std::fmt::Write;

use actix_identity::Identity;
use actix_session::Session;
use actix_web::{http::StatusCode, web, HttpMessage, HttpRequest, Responder, Result};
use openidconnect::core::{
    CoreClient, CoreGenderClaim, CoreIdTokenClaims, CoreIdTokenVerifier, CoreJsonWebKeyType,
    CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm, CoreTokenType,
};
use openidconnect::{
    EmptyAdditionalClaims, EmptyExtraTokenFields, IdTokenFields, Nonce, StandardTokenResponse,
};
use sqlx;

use crate::errors::{ApiError, STANDARD_INTERNAL_SERVER_ERROR};
use crate::models::User;
use crate::settings::SupportedOauthProviders;

pub async fn process_open_id_info(
    oauth_type: &SupportedOauthProviders,
    token_response: StandardTokenResponse<
        IdTokenFields<
            EmptyAdditionalClaims,
            EmptyExtraTokenFields,
            CoreGenderClaim,
            CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm,
            CoreJsonWebKeyType,
        >,
        CoreTokenType,
    >,
    request: HttpRequest,
    session: Session,
    client: &CoreClient,
    pg_pool: web::Data<sqlx::PgPool>,
    nonce: &Nonce,
    auth_success_redirect_url: String,
    mut new_user_redirect_url: String,
    failure_redirect_url: String,
) -> Result<impl Responder, ApiError> {
    let id_token_verifier: CoreIdTokenVerifier = client.id_token_verifier();
    let id_token_clims: &CoreIdTokenClaims = token_response
        .extra_fields()
        .id_token()
        .ok_or(ApiError::Redirect(
            failure_redirect_url.clone(),
            String::from("Authorization server did not return id token"),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?
        .claims(&id_token_verifier, nonce)
        .map_err(|_| {
            ApiError::Redirect(
                failure_redirect_url.clone(),
                String::from("Authentication error"),
                StatusCode::UNAUTHORIZED,
            )
        })?;

    let google_user_id = id_token_clims.subject();
    let user_result: Result<User, sqlx::Error>;
    match *oauth_type {
        SupportedOauthProviders::google => {
            user_result = User::get_user_by_google_id(&google_user_id, pg_pool).await;
        }
    };

    match user_result {
        Ok(user) => {
            match Identity::login(&request.extensions(), google_user_id.to_string()) {
                Ok(_) => {
                    // Add user role to a session
                    if let Err(err) = session.insert("role", &user.role) {
                        println!(
                            "Error while trying to insert user_role into a session: {}",
                            err
                        );
                    };
                    Ok(web::Redirect::to(auth_success_redirect_url))
                }
                Err(err) => {
                    println!("Error happened during identity login: {}", err);
                    Err(ApiError::Unauthorized(Some(
                        "Unable to login. Please try again.".to_string(),
                    )))
                }
            }
        }
        Err(err) => {
            match err {
                sqlx::Error::RowNotFound => {
                    let mut query_params = Vec::with_capacity(4);
                    // Check if openId info contains needed parameters
                    if let Some(first_name_loc) = id_token_clims.given_name() {
                        if let Some(first_name) = first_name_loc.get(None) {
                            let query = format!("firstName={}", first_name.as_str());
                            query_params.push(query);
                        };
                    };
                    if let Some(last_name_loc) = id_token_clims.family_name() {
                        if let Some(last_name) = last_name_loc.get(None) {
                            let query = format!("lastName={}", last_name.as_str());
                            query_params.push(query);
                        }
                    };
                    if let Some(email) = id_token_clims.email() {
                        let query = format!("email={}", email.as_str());
                        query_params.push(query);
                    };
                    let google_id_query = format!("googleId={}", google_user_id.as_str());
                    query_params.push(google_id_query);

                    if query_params.len() != 0 {
                        // Add ? to the url first
                        if let Err(error) = write!(new_user_redirect_url, "?") {
                            println!("Got an error while trying add ? to the new_user_redirect_url: {:?}", error);
                        } else {
                            // Add query parameters to the url
                            let joined_query_params = query_params.join("&");
                            if let Err(error) =
                                write!(new_user_redirect_url, "{}", joined_query_params)
                            {
                                println!("Got an error while trying add joined_query_params to the new_user_redirect_url: {:?}", error);
                            };
                        };
                    }

                    Ok(web::Redirect::to(new_user_redirect_url))
                }
                _ => Err(ApiError::Redirect(
                    failure_redirect_url,
                    STANDARD_INTERNAL_SERVER_ERROR.to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            }
        }
    }
}
