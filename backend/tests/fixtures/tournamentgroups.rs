use backend::models::tournamentgroup::{NewTournamentGroup, TournamentGroupBuilder};

pub fn get_tournamentgroup_payload() -> NewTournamentGroup {
    TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is Tour 1's payload.".to_string()))
        .build()
        .unwrap()
}