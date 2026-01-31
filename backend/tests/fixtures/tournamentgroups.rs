use backend::{database, models::{tournament::{Tournament, TournamentBuilder}, tournamentgroup::{NewTournamentGroup, TournamentGroup, TournamentGroupBuilder}, tournamentgroup_tournament::{TournamentGroupTournament, TournamentGroupTournamentBuilder}}};

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
    TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is Tour 1's payload.".to_string()))
        .build_and_insert(db)
        .unwrap();
    TournamentGroupBuilder::new_default("Test TourGroup 2")
        .set_description(Some("This is Tour 2's payload.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> TournamentGroup {
    TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is TourGroup 1 testing.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> TournamentGroup {
    TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is TourGroup 1 testing.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_get_all_tournaments_of_tournamentgroup_works_integration_test(db: &mut database::Connection) -> (TournamentGroup, Tournament, Tournament) {
    let tg_1 = TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is TourGroup 1 testing.".to_string()))
        .build_and_insert(db)
        .unwrap();

    let tour_1 = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();
    let tour_2 = TournamentBuilder::new_default("Tour 2")
        .build_and_insert(db)
        .unwrap();
    let tour_3 = TournamentBuilder::new_default("Tour 3")
        .build_and_insert(db)
        .unwrap();
    let tour_4 = TournamentBuilder::new_default("Tour 4")
        .build_and_insert(db)
        .unwrap();

    let tg_1_bridge_tour_1 = TournamentGroupTournamentBuilder::new_default(tg_1.tgid, tour_1.tid)
        .build_and_insert(db)
        .unwrap();
    let tg_1_bridge_tour_2 = TournamentGroupTournamentBuilder::new_default(tg_1.tgid, tour_2.tid)
        .build_and_insert(db)
        .unwrap();

    (tg_1, tour_1, tour_2)
}