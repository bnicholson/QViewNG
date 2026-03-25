use crate::{auth::policies::{Policy, PolicyContext}, constants::SUPER_USER, models::tournament::Tournament};


impl Policy<Tournament> for PolicyContext<Tournament> {
    fn can_create(&self, resource: &Tournament) -> bool {
        self.user.user_id == resource.owner_id
        || self.user.roles.iter().any(|r| r == SUPER_USER)
    }
    fn can_edit(&self, resource: &Tournament) -> bool {
        self.user.user_id == resource.owner_id
        || self.user.roles.iter().any(|r| r == SUPER_USER)
    }
    fn can_delete(&self, resource: &Tournament) -> bool {
        self.user.user_id == resource.owner_id
        || self.user.roles.iter().any(|r| r == SUPER_USER)
    }
    fn can_view(&self, resource: &Tournament) -> bool {
        true
    }
}
