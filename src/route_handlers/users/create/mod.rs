use actix_identity::Identity;
use actix_session::Session;
use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Result};
use sqlx::{self, postgres::PgDatabaseError, PgPool};
use validator::Validate;

use crate::errors::ApiError;
use crate::models::User;
use helpers::*;
mod helpers;
#[cfg(test)]
mod tests;

#[post("")]
async fn create_user(
    create_user_info: User,
    pg_pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    create_user_info.validate()?;

    match create_user_info.insert_user(pg_pool).await {
        Ok(user) => match Identity::login(&req.extensions(), user.id.to_string()) {
            Ok(_) => {
                // Add user role to a session
                if let Err(err) = session.insert("role", &user.role) {
                    println!(
                        "Error while trying to insert user_role into a session: {}",
                        err
                    );
                };
                Ok(HttpResponse::Created().json(user))
            }
            Err(err) => {
                println!("Error happened during identity login: {}", err);
                Err(ApiError::Unauthorized(Some(
                    "Unable to login new user".to_string(),
                )))
            }
        },
        Err(sqlx_err) => {
            let generic_err = ApiError::InternalServerError("Unable to create user".to_string());

            match sqlx_err {
                sqlx::Error::Database(err) => {
                    let pg_err = err.downcast_ref::<PgDatabaseError>();
                    match pg_err.code() {
                        // Unique constraint violation
                        "23505" => {
                            let generic_unique_constraint_err = ApiError::BadRequest(
                                "User with provided configuration already exists".to_string(),
                            );

                            let details = pg_err
                                .detail()
                                .ok_or_else(|| generic_unique_constraint_err.clone())?;

                            let col_name = get_column_name_from_err_details(details)?;

                            Err(ApiError::BadRequest(format!(
                                "User with provided {} already exists",
                                &col_name
                            )))
                        }
                        _ => Err(generic_err),
                    }
                }
                err => {
                    println!("{:?}", err);
                    Err(generic_err)
                }
            }
        }
    }
}
