use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Serialize, Debug, Deserialize, PartialEq, sqlx::Type, Clone)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    SuperAdmin,
    Admin,
    User,
}

impl UserRole {
    pub async fn get_role_id(
        db_pool: web::Data<sqlx::Pool<sqlx::Postgres>>,
        role: &UserRole,
    ) -> sqlx::Result<i32> {
        let (role_id,) = sqlx::query_as::<_, (i32,)>("SELECT id FROM roles WHERE role = $1")
            .bind(role)
            .fetch_one(db_pool.as_ref())
            .await?;

        Ok(role_id)
    }
}

impl TryFrom<&str> for UserRole {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SuperAdmin" | "super_admin" => Ok(UserRole::SuperAdmin),
            "Admin" | "admin" => Ok(UserRole::Admin),
            "User" | "user" => Ok(UserRole::User),
            unknown => Err(format!("Unable to convert {} &str into UserRole", unknown)),
        }
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};
    use test_case::test_case;

    use super::*;

    #[test]
    fn serializes_deserializes_correctly() {
        let super_admin = "SuperAdmin";
        let admin = "Admin";
        let user = "User";

        let expected_result_sa = &[
            Token::Enum { name: "UserRole" },
            Token::Str(super_admin),
            Token::Unit,
        ];
        let expected_result_a = &[
            Token::Enum { name: "UserRole" },
            Token::Str(admin),
            Token::Unit,
        ];
        let expected_result_u = &[
            Token::Enum { name: "UserRole" },
            Token::Str(user),
            Token::Unit,
        ];

        assert_tokens(&UserRole::SuperAdmin, expected_result_sa);
        assert_tokens(&UserRole::Admin, expected_result_a);
        assert_tokens(&UserRole::User, expected_result_u);
    }

    #[test_case("SuperAdmin", UserRole::SuperAdmin; "from SuperAdmin")]
    #[test_case("super_admin", UserRole::SuperAdmin; "from super_admin")]
    #[test_case("Admin", UserRole::Admin; "from Admin uppercase")]
    #[test_case("admin", UserRole::Admin; "from admin lowercase")]
    #[test_case("User", UserRole::User; "from User uppercase")]
    #[test_case("user", UserRole::User; "from User lowercase")]
    fn try_from_str_converts_correctly(str: &str, expected_result: UserRole) {
        let converted_enum = UserRole::try_from(str).unwrap();
        assert_eq!(
            converted_enum, expected_result,
            "Incorrectly converted &str into UserRole. Expected: {:?}, actual: {:?}",
            expected_result, converted_enum
        );
    }

    #[test]
    fn try_from_str_returns_error_for_unrecognized_str() {
        let bogus_str = "UnrecognizedRole";

        let conversion_result = UserRole::try_from(bogus_str);
        assert!(
            conversion_result.is_err(),
            "Did not get error while trying to convert unrecognized &str into UserRole."
        )
    }
}
