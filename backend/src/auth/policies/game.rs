use crate::auth::policies::{Policy, PolicyContext};
use crate::models::tournament::Tournament;

/// Carries the data needed to evaluate game-level ABAC policies.
/// `user_is_tournament_admin` should be pre-loaded from the `tournaments_admins` table.
pub struct GamePolicyResource {
    pub tournament: Tournament,
    pub user_is_tournament_admin: bool,
}

impl Policy<GamePolicyResource> for PolicyContext<GamePolicyResource> {
    fn can_create(&self, resource: &GamePolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
    }
    fn can_update(&self, resource: &GamePolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
    }
    fn can_delete(&self, resource: &GamePolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
    }
}
