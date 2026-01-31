use backend::{database, models::tournamentgroup::{NewTournamentGroup, TournamentGroup, TournamentGroupBuilder}};
use uuid::Uuid;

pub fn get_tournamentgroup_payload() -> NewTournamentGroup {
    TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is Tour 1's payload.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_intergration_test(db: &mut database::Connection) -> (TournamentGroup, TournamentGroup) {
    (
        TournamentGroupBuilder::new_default("Test TourGroup 1")
            .set_description(Some("This is Tour 1's payload.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        TournamentGroupBuilder::new_default("Test TourGroup 2")
            .set_description(Some("This is Tour 2's payload.".to_string()))
            .build_and_insert(db)
            .unwrap()
    )
}

pub fn arrange_get_by_id_works_integration_test(db: &mut database::Connection) -> TournamentGroup {
    let tg_1 = TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is Tour 1's payload.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let tg_2 = TournamentGroupBuilder::new_default("Test TourGroup 2")
        .set_description(Some("This is Tour 2's payload.".to_string()))
        .build_and_insert(db)
        .unwrap();
    tg_2
}