// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "access_type"))]
    pub struct AccessType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;
    use super::sql_types::AccessType;

    roles (id) {
        id -> Int4,
        role -> UserRole,
        delete_user -> Nullable<AccessType>,
        get_user -> Nullable<AccessType>,
        update_user -> Nullable<AccessType>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        google_id -> Nullable<Varchar>,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        created_at -> Timestamptz,
        role_id -> Int4,
    }
}

diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    users,
);
