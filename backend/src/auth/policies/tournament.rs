use crate::{auth::policies::{Policy, PolicyContext}, models::{role::AppRole, tournament::Tournament}};


impl Policy<Tournament> for PolicyContext<Tournament> {
    fn can_edit(&self, resource: &Tournament) -> bool {
        self.user.user_id == resource.owner_id
        || self.user.roles.iter().any(|r| r == AppRole::SuperUser.as_str())
    }
    fn can_delete(&self, resource: &Tournament) -> bool {
        self.user.user_id == resource.owner_id
        || self.user.roles.iter().any(|r| r == AppRole::SuperUser.as_str())
    }
    fn can_view(&self, resource: &Tournament) -> bool {
        true
    }
}
