use crate::auth::policies::{Policy, PolicyContext};
use crate::models::tournament::Tournament;

/// Carries the data needed to evaluate team-level ABAC policies.
/// `user_is_tournament_admin` should be pre-loaded from the `tournaments_admins` table.
/// `user_is_team_coach` should be set when the requesting user is the coach of the specific team.
pub struct TeamPolicyResource {
    pub tournament: Tournament,
    pub user_is_tournament_admin: bool,
    pub user_is_team_coach: bool,
}

impl Policy<TeamPolicyResource> for PolicyContext<TeamPolicyResource> {
    fn can_create(&self, resource: &TeamPolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
    }
    fn can_update(&self, resource: &TeamPolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
            || resource.user_is_team_coach
    }
    fn can_delete(&self, resource: &TeamPolicyResource) -> bool {
        self.user_ctx.user_id == resource.tournament.owner_id
            || resource.user_is_tournament_admin
    }
}
