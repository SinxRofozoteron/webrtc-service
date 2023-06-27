use super::{AccessPolicy, UserRole};

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct RBACTable {
    pub SuperAdmin: Option<AccessPolicy>,
    pub Admin: Option<AccessPolicy>,
    pub User: Option<AccessPolicy>,
}

impl RBACTable {
    pub fn init() -> Self {
        RBACTable {
            SuperAdmin: None,
            Admin: None,
            User: None,
        }
    }

    pub fn get_role_policy(&self, role: &UserRole) -> Option<&AccessPolicy> {
        let policy_option = match role {
            UserRole::SuperAdmin => &self.SuperAdmin,
            UserRole::Admin => &self.Admin,
            UserRole::User => &self.User,
        };

        policy_option.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use test_case::test_case;

    use super::*;
    use crate::auth::rbac::{AccessPolicy, AccessType, ProtectedRoute, UserRole};

    fn create_test_table() -> RBACTable {
        let mut super_admin_policy: AccessPolicy = HashMap::new();
        super_admin_policy.insert(ProtectedRoute::GetUser, AccessType::RestrictedAccess);
        let mut admin_policy: AccessPolicy = HashMap::new();
        admin_policy.insert(ProtectedRoute::DeleteUser, AccessType::RestrictedAccess);
        let mut user_policy: AccessPolicy = HashMap::new();
        user_policy.insert(ProtectedRoute::UpdateUser, AccessType::RestrictedAccess);

        RBACTable {
            SuperAdmin: Some(super_admin_policy),
            Admin: Some(admin_policy),
            User: Some(user_policy),
        }
    }

    #[test]
    fn init_returns_none_for_policies() {
        let table = RBACTable::init();

        assert!(
            table.Admin.is_none(),
            "RBACTable::init() did not return None as Admin policy"
        );
        assert!(
            table.SuperAdmin.is_none(),
            "RBACTable::init() did not return None as SuperAdmin policy"
        );
        assert!(
            table.User.is_none(),
            "RBACTable::init() did not return None as User policy"
        );
    }

    #[test_case(UserRole::SuperAdmin, ProtectedRoute::GetUser; "for super admin")]
    #[test_case(UserRole::Admin, ProtectedRoute::DeleteUser; "for admin")]
    #[test_case(UserRole::User, ProtectedRoute::UpdateUser; "for user")]
    fn get_role_policy_returns_correct_policy(role: UserRole, route: ProtectedRoute) {
        let table = create_test_table();
        let policy_option = table.get_role_policy(&role);

        assert!(
            policy_option.is_some(),
            "Could not find policy for {:?}",
            &role
        );

        let policy = policy_option.unwrap();
        let access_type_option = policy.get(&route);

        assert!(
            access_type_option.is_some(),
            "Could not find expected policy for {:?} route",
            &route
        );

        let access_type = access_type_option.unwrap();

        assert_eq!(
            *access_type,
            AccessType::RestrictedAccess,
            "Unexpected AccessType returned for {:?} route. Expected: {:?}, actual: {:?}",
            route,
            AccessType::RestrictedAccess,
            access_type
        );
    }
}
