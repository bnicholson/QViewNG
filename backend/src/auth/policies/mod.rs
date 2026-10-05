
pub mod tournament;
pub mod division;
pub mod round;
pub mod room;
pub mod game;
pub mod team;

use uuid::Uuid;

use crate::models::permission::AppSpecialPermission;

#[derive(Debug,Clone)]
pub struct UserContext {
    pub user_id: Uuid,
    pub roles: Vec<String>,
    pub permissions: Vec<String>
}
impl UserContext {
    pub fn new(user_id: Uuid, roles: Vec<String>, permissions: Vec<String>) -> Self {
        Self {
            user_id,
            roles,
            permissions
        }
    }

    /// True when the user was granted `name`. Authorization reads permissions only — `roles` is
    /// carried for display and is never consulted here.
    pub fn has_permission(&self, name: &str) -> bool {
        self.permissions.iter().any(|p| p == name)
    }

    pub fn has(&self, permission: AppSpecialPermission) -> bool {
        self.has_permission(permission.as_str())
    }

    /// Unrestricted access: the permission that replaced the former `super_user` role bypass.
    pub fn is_system_administrator(&self) -> bool {
        self.has(AppSpecialPermission::SystemAdminister)
    }

    /// The check to reach for in a handler: `name`, or the unrestricted-access bypass. Prefer this
    /// over inspecting `permissions` directly so every endpoint honours the bypass the same way
    /// `is_rbac_and_abac_authorized` does.
    pub fn can(&self, name: &str) -> bool {
        self.has_permission(name) || self.is_system_administrator()
    }
}

pub trait Policy<Resource> {
    fn can_create(&self, _resource: &Resource) -> bool { true }
    fn can_update(&self, resource: &Resource) -> bool;
    fn can_delete(&self, resource: &Resource) -> bool;
}

pub struct PolicyContext<R> {
    pub user_ctx: UserContext,
    pub resource: R
}