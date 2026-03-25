
pub mod tournament;

use uuid::Uuid;

#[derive(Debug)]
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
    fn can_edit(&self, resource: &Resource) -> bool;
    fn can_delete(&self, resource: &Resource) -> bool;
    fn can_view(&self, resource: &Resource) -> bool;
}

pub struct PolicyContext<R> {
    pub user: UserContext,
    pub resource: R
}