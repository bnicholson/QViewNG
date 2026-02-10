use backend::{database, models::tournament::{NewTournament, TournamentBuilder}};


pub fn arrange_create_works_integration_test() -> (NewTournament, NewTournament) {
    (
        TournamentBuilder::new_default("Tour 4")
            .build()
            .unwrap(),
        TournamentBuilder::new_default("Tour 5")
            .build()
            .unwrap()
    )
}
