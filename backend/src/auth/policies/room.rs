use crate::auth::policies::{Policy, PolicyContext};
use crate::models::tournament::Tournament;

/// Carries the data needed to evaluate room-level ABAC policies.
/// `user_is_tournament_admin` should be pre-loaded from the `tournaments_admins` table.
pub struct RoomPolicyResource {
    pub tournament: Tournament,
    pub user_is_tournament_admin: bool,
}

impl Policy<RoomPolicyResource> for PolicyContext<RoomPolicyResource> {
    fn can_create(&self, resource: &RoomPolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
    }
    fn can_update(&self, resource: &RoomPolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
    }
    fn can_delete(&self, resource: &RoomPolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
    }
}
