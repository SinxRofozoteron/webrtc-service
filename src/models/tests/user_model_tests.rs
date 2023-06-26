use actix_web::test;
use serde_test::{assert_de_tokens, assert_ser_tokens, Configure, Token};
use sqlx::types::{chrono, Uuid};

use super::super::User;
use crate::auth::rbac::UserRole;

#[test]
async fn serializes_correctly() {
    let expected_uuid_str = "b296d83b-85f8-442c-90c4-d31fec8f851b";
    let expected_username = "UserName100";
    let expected_first_name = "Alex";
    let expected_last_name = "Burakou";
    let expected_email = "testemail@gmail.com";
    let expected_created_at = "2023-01-29T19:53:24.906Z";

    let test_user = User {
        id: Uuid::parse_str(expected_uuid_str).unwrap(),
        google_id: Some("test_google_id".to_string()),
        username: expected_username.to_string(),
        first_name: expected_first_name.to_string(),
        last_name: expected_last_name.to_string(),
        email: expected_email.to_string(),
        created_at: chrono::DateTime::parse_from_str(
            "2023-01-29 12:53:24.906 -0700",
            "%Y-%m-%d %H:%M:%S%.3f %z",
        )
        .unwrap()
        .with_timezone(&chrono::Utc),
        role: UserRole::User,
    };

    let expected_result = &[
        Token::Struct {
            name: "User",
            len: 7,
        },
        Token::Str("id"),
        Token::Str(expected_uuid_str),
        Token::Str("username"),
        Token::Str(expected_username),
        Token::Str("first_name"),
        Token::Str(expected_first_name),
        Token::Str("last_name"),
        Token::Str(expected_last_name),
        Token::Str("email"),
        Token::Str(expected_email),
        Token::Str("created_at"),
        Token::Str(expected_created_at),
        Token::Str("role"),
        Token::UnitVariant {
            name: "UserRole",
            variant: "User",
        },
        Token::StructEnd,
    ];

    assert_ser_tokens(&test_user.readable(), expected_result);
}

#[test]
async fn deserializes_correctly() {
    let expected_google_id = "test_google_id";
    let expected_username = "UserName100";
    let expected_first_name = "Alex";
    let expected_last_name = "Burakou";
    let expected_email = "testemail@gmail.com";

    let test_str = format!(
        r#"
    {{
        "google_id": "{}",
        "username": "{}",
        "first_name": "{}",
        "last_name": "{}",
        "email": "{}"
    }}
    "#,
        expected_google_id,
        expected_username,
        expected_first_name,
        expected_last_name,
        expected_email
    );

    let test_user: User =
        serde_json::from_str(&test_str).expect("Error during desereialization of test user");

    let expected_result = &[
        Token::Struct {
            name: "User",
            len: 5,
        },
        Token::Str("google_id"),
        Token::Some,
        Token::String(expected_google_id),
        Token::Str("username"),
        Token::String(expected_username),
        Token::Str("first_name"),
        Token::String(expected_first_name),
        Token::Str("last_name"),
        Token::String(expected_last_name),
        Token::Str("email"),
        Token::String(expected_email),
        Token::StructEnd,
    ];

    assert_de_tokens(&test_user, expected_result);
}
