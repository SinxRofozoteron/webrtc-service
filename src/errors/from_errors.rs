use actix_web::http::header::ToStrError;
use sqlx;
use validator;

use super::{ApiError, STANDARD_INTERNAL_SERVER_ERROR};

impl From<validator::ValidationErrors> for ApiError {
    fn from(error: validator::ValidationErrors) -> Self {
        ApiError::BadRequest(error.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(_: serde_json::Error) -> Self {
        ApiError::InternalServerError(STANDARD_INTERNAL_SERVER_ERROR.to_string())
    }
}

impl From<ToStrError> for ApiError {
    fn from(_: ToStrError) -> Self {
        ApiError::InternalServerError(STANDARD_INTERNAL_SERVER_ERROR.to_string())
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(_: sqlx::Error) -> Self {
        ApiError::InternalServerError(STANDARD_INTERNAL_SERVER_ERROR.to_string())
    }
}
