use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

use crate::common::Repository;

pub type AssignmentScope = fn(&Session, &Permission) -> bool;

#[derive(PartialEq, Eq)]
pub struct Role {
    name: String,
    intrinsic: bool,
    permissions: Option<HashMap<Permission, AssignmentScope>>
}

impl Role {

    pub fn new(name: &str, intrinsic: bool) -> Self {
        Role {
            name: name.to_owned(),
            intrinsic,
            permissions: Option::None
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_intrinsic(&self) -> bool {
        self.intrinsic
    }

    pub fn permission_applies(&self, permission: &Permission, to: &Session) -> bool {
        if let Some(perm_map) = &self.permissions {
            if let Some(scope) = &perm_map.get(permission) {
                return scope(to, permission);
            }
        }
        false
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Permission {
    
}

pub struct Session {
    roles: Vec<Role>,
    expires: u128
}

impl Session {

    pub fn new(expiry: u128) -> Self {
        Session {
            roles: vec![],
            expires: expiry
        }
    }

    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(role)
    }

    pub fn is_valid(&self) -> bool {
        SystemTime::now().duration_since(UNIX_EPOCH).is_ok_and(|t| self.expires > t.as_millis())
    }

    pub fn check_permission(&self, permission: &Permission) -> bool {
        for role in &self.roles {
            if role.permission_applies(permission, self) {
                return true;
            }
        }
        false
    }

}

pub trait RoleRepository: Repository<Role, String> {
    
}