use backend::{database, models::tournament::{NewTournament, TournamentBuilder}};
use uuid::Uuid;


pub fn arrange_create_works_integration_test() -> (NewTournament, NewTournament) {
    let placeholder_owner_id = Uuid::nil();
    (
        TournamentBuilder::new_default("Tour 4")
            .set_owner_id(placeholder_owner_id)
            .build()
            .unwrap(),
        TournamentBuilder::new_default("Tour 5")
            .set_owner_id(placeholder_owner_id)
            .build()
            .unwrap()
    )
}
