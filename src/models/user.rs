use actix_web::{web, FromRequest};
use futures::Future;
use serde::{Deserialize, Serialize};
use sqlx;
use std::pin::Pin;
use validator::Validate;

use crate::auth::rbac::UserRole;
use crate::errors::ApiError;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Serialize, Deserialize, Debug, Validate, sqlx::FromRow)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: sqlx::types::Uuid,
    #[serde(skip_serializing)]
    pub google_id: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub username: String,
    #[validate(length(min = 1, max = 748))]
    pub first_name: String,
    #[validate(length(min = 1, max = 748))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_deserializing)]
    pub role: UserRole,
}

impl User {
    pub async fn get_user_by_google_id(
        google_id: &str,
        db_pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
    ) -> sqlx::Result<Self> {
        sqlx::query_as::<_, User>(
             "SELECT users.id, google_id, username, first_name, last_name, email, created_at, role FROM users JOIN roles ON users.role_id = roles.id WHERE google_id = $1"
            ).bind(google_id).fetch_one(db_pool.get_ref()).await
    }

    pub async fn insert_user(
        &self,
        db_pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
    ) -> sqlx::Result<User> {
        let role_id = UserRole::get_role_id(db_pool.clone(), &self.role).await?;

        let user = sqlx::query_as::<_, User>(
            r#"
                INSERT INTO users (google_id, username, first_name, last_name, email, role_id) 
                VALUES ($1, $2, $3, $4, $5, $6) 
                RETURNING id, google_id, username, first_name, last_name, email, created_at, (SELECT role FROM roles WHERE id = $6) as role
                "#
        )
            .bind(&self.google_id)
            .bind(&self.username)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.email)
            .bind(role_id)
            .fetch_one(db_pool.as_ref()).await?;

        Ok(user)
    }
}

impl FromRequest for User {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let json_future = web::Json::<Self>::from_request(req, payload);

        let fut = async move {
            match json_future.await {
                Ok(json) => {
                    if json.google_id == None {
                        Err(ApiError::BadRequest(
                            "Please provide google_id property".to_string(),
                        ))
                    } else {
                        Ok(json.into_inner())
                    }
                }
                Err(err) => {
                    println!("Error during User struct deserialization: {:?}", err);
                    Err(ApiError::BadRequest(
                    "Please provide following properties: first_name, last_name, username, email"
                        .to_string(),
                ))
                }
            }
        };

        Box::pin(fut)
    }
}
