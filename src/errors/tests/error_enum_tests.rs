use actix_web::{http, test, ResponseError};
use test_case::test_case;

use super::super::ApiError;

#[test_case(ApiError::BadRequest("test_bad_request".to_string()), http::StatusCode::BAD_REQUEST; "bad request status")]
#[test_case(ApiError::InternalServerError("test_internal_server_error".to_string()), http::StatusCode::INTERNAL_SERVER_ERROR; "internal server error status")]
#[test_case(ApiError::Unauthorized(None), http::StatusCode::UNAUTHORIZED; "unauthorized status")]
fn returns_correct(api_error: ApiError, expected_status_code: http::StatusCode) {
    let response = api_error.error_response();

    assert_eq!(response.status(), expected_status_code);
}

#[test]
async fn returns_correct_redirect_response() {
    let test_redirect_error_url = "http://example.com";
    let test_redirect_error_msg = "test redirect error message";
    let test_error_code = http::StatusCode::BAD_GATEWAY;
    let expected_location_header = format!(
        "{}?error={}:{}",
        test_redirect_error_url, test_error_code, test_redirect_error_msg
    );

    let api_error = ApiError::Redirect(
        test_redirect_error_url.to_string(),
        test_redirect_error_msg.to_string(),
        test_error_code,
    );

    let error_response = api_error.error_response();
    let location_header = error_response
        .headers()
        .get(http::header::LOCATION)
        .expect("Location header is not set on ApiError::Redirect error response")
        .to_str()
        .expect("Unable to convert location header value to &str")
        .to_string();

    assert_eq!(error_response.status(), http::StatusCode::FOUND);
    assert_eq!(location_header, expected_location_header);
}
