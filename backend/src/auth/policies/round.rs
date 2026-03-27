use crate::auth::policies::{Policy, PolicyContext};
use crate::models::tournament::Tournament;

/// Carries the data needed to evaluate round-level ABAC policies.
/// `user_is_tournament_admin` should be pre-loaded from the `tournaments_admins` table.
pub struct RoundPolicyResource {
    pub tournament: Tournament,
    pub user_is_tournament_admin: bool,
}

impl Policy<RoundPolicyResource> for PolicyContext<RoundPolicyResource> {
    fn can_create(&self, resource: &RoundPolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
    }
    fn can_update(&self, resource: &RoundPolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
    }
    fn can_delete(&self, resource: &RoundPolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
    }
}
