use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Hash, Debug, EnumIter)]
pub enum ProtectedRoute {
    DeleteUser,
    GetUser,
    UpdateUser,
}

impl TryFrom<&str> for ProtectedRoute {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "delete_user" => Ok(ProtectedRoute::DeleteUser),
            "get_user" => Ok(ProtectedRoute::GetUser),
            "update_user" => Ok(ProtectedRoute::UpdateUser),
            unknown => Err(format!(
                "Unable to convert {} &str into ProtectedRoute",
                unknown
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str_converts_type_correctly() {
        let delete_user_str = "delete_user";
        let get_user_str = "get_user";
        let update_user_str = "update_user";

        let delete_user_enum = ProtectedRoute::try_from(delete_user_str).unwrap();
        let get_user_enum = ProtectedRoute::try_from(get_user_str).unwrap();
        let update_user_enum = ProtectedRoute::try_from(update_user_str).unwrap();

        assert_eq!(delete_user_enum, ProtectedRoute::DeleteUser);
        assert_eq!(get_user_enum, ProtectedRoute::GetUser);
        assert_eq!(update_user_enum, ProtectedRoute::UpdateUser);
    }

    #[test]
    fn try_from_returns_error_if_cannot_convert_type() {
        ProtectedRoute::try_from("unknown_access_type")
            .expect_err("ProtectedRoute::try_from did not return expected error.");
    }
}
