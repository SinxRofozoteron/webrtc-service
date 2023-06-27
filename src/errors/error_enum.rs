use actix_web::{error, http, HttpResponse};
use std::fmt::Display;

use super::constants::STANDARD_UNAUTHORIZED_ERROR;

type RedirectErrorMessage = String;
type RedirectErrorUrl = String;
type RedirectErrorCode = http::StatusCode;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub enum ApiError {
    BadRequest(String),
    InternalServerError(String),
    Redirect(RedirectErrorUrl, RedirectErrorMessage, RedirectErrorCode),
    Unauthorized(Option<String>),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest(message) => write!(f, "Bad Request: {}", &message),
            ApiError::InternalServerError(message) => {
                write!(f, "Internal Server Error: {}", &message)
            }
            ApiError::Redirect(url, message, code) => {
                write!(
                    f,
                    "Redirect Error: {{ redirect_url: {}, message: {}, code: {} }}",
                    url, message, code
                )
            }
            ApiError::Unauthorized(message_opt) => {
                write!(f, "Unauthorized: ")?;
                if let Some(message) = message_opt {
                    write!(f, "{}", message)?;
                } else {
                    write!(f, "{}", STANDARD_UNAUTHORIZED_ERROR)?;
                };
                Ok(())
            }
        }
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message: &str;
        let mut redirect_url: Option<String> = None;
        match self {
            ApiError::BadRequest(msg) => {
                message = msg;
            }
            ApiError::InternalServerError(msg) => {
                message = msg;
            }
            ApiError::Redirect(url, msg, code) => {
                message = msg;
                redirect_url = Some(format!("{}?error={}:{}", url, code, message));
            }
            ApiError::Unauthorized(msg_opt) => {
                if let Some(msg) = msg_opt {
                    message = msg;
                } else {
                    message = STANDARD_UNAUTHORIZED_ERROR;
                }
            }
        };

        if let Some(url) = redirect_url {
            return HttpResponse::Found()
                .insert_header((&http::header::LOCATION, url))
                .finish();
        }

        HttpResponse::build(self.status_code()).json(message)
    }

    fn status_code(&self) -> http::StatusCode {
        match *self {
            ApiError::BadRequest(_) => http::StatusCode::BAD_REQUEST,
            ApiError::InternalServerError(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Redirect(_, _, _) => http::StatusCode::FOUND,
            ApiError::Unauthorized(_) => http::StatusCode::UNAUTHORIZED,
        }
    }
}
