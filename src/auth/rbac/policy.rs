use std::collections::HashMap;

use super::{access_type::AccessType, protected_route::ProtectedRoute};

pub type AccessPolicy = HashMap<ProtectedRoute, AccessType>;
