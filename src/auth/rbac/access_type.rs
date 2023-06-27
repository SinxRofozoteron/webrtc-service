use sqlx;

#[derive(Debug, sqlx::Type, PartialEq)]
#[sqlx(type_name = "access_type")]
#[sqlx(rename_all = "snake_case")]
pub enum AccessType {
    NoAccess,
    RestrictedAccess,
    UnrestrictedAccess,
}

impl TryFrom<&str> for AccessType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "no_access" => Ok(AccessType::NoAccess),
            "unrestricted_access" => Ok(AccessType::UnrestrictedAccess),
            "restricted_access" => Ok(AccessType::RestrictedAccess),
            unknown => Err(format!(
                "Unable to convert {} &str into AccessType",
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
        let no_access_str = "no_access";
        let unrestricted_access_str = "unrestricted_access";
        let restricted_access_str = "restricted_access";

        let no_access_enum = AccessType::try_from(no_access_str).unwrap();
        let unrestricted_access_enum = AccessType::try_from(unrestricted_access_str).unwrap();
        let restricted_access_enum = AccessType::try_from(restricted_access_str).unwrap();

        assert_eq!(no_access_enum, AccessType::NoAccess);
        assert_eq!(unrestricted_access_enum, AccessType::UnrestrictedAccess);
        assert_eq!(restricted_access_enum, AccessType::RestrictedAccess);
    }

    #[test]
    fn try_from_returns_error_if_cannot_convert_type() {
        AccessType::try_from("unknown_access_type")
            .expect_err("AccessType::try_from did not return expected error.");
    }
}
