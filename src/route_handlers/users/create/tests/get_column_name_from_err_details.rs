use actix_web::test;

use super::super::get_column_name_from_err_details;
use crate::errors::ApiError;

#[test]
async fn returns_correct_api_err() {
    let test_details = "Key";
    let result_err = get_column_name_from_err_details(test_details)
        .expect_err("Did not get Err result when provided details with no column name");

    assert_eq!(
        result_err,
        ApiError::BadRequest("User with provided configuration already exists".to_string()),
        "Did not get correct error when provided details with no column name"
    );
}

#[test]
async fn returns_correct_column_name() {
    let test_column = "test_column_name";
    let test_details = format!(r"Key ({})=", test_column);

    let result = get_column_name_from_err_details(test_details.as_str())
        .expect("Did not get Ok retun wnen provided details with valid column name");

    assert_eq!(
        result,
        test_column.to_string(),
        "Did not get correct column name when provided details with valid column name"
    );
}
