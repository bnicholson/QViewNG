
pub mod tournament;
pub mod division;

use uuid::Uuid;

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