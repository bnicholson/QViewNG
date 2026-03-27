use crate::{auth::policies::{Policy, PolicyContext}, models::{role::AppRole, tournament::Tournament}};


impl Policy<Tournament> for PolicyContext<Tournament> {
    fn can_update(&self, resource: &Tournament) -> bool {
        self.user_ctx.user_id == resource.owner_id
        || self.user_ctx.roles.iter().any(|r| r == AppRole::SuperUser.as_str())
    }
    fn can_delete(&self, resource: &Tournament) -> bool {
        self.user_ctx.user_id == resource.owner_id
        || self.user_ctx.roles.iter().any(|r| r == AppRole::SuperUser.as_str())
    }
}
