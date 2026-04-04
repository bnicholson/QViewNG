use backend::{database, models::{tournament::{Tournament, TournamentBuilder}, tournamentgroup::{NewTournamentGroup, TournamentGroup, TournamentGroupBuilder}, tournamentgroup_tournament::{NewTournamentGroupTournament, TournamentGroupTournament, TournamentGroupTournamentBuilder}, user::UserBuilder}};

fn seed_group_user(db: &mut database::Connection) -> uuid::Uuid {
    UserBuilder::new_default("Group Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap()
        .id
}

pub fn get_tournamentgroup_payload(creator_id: uuid::Uuid) -> NewTournamentGroup {
    TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is Tour 1's payload.".to_string()))
        .set_creator_id(creator_id)
        .set_owner_id(creator_id)
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_intergration_test(db: &mut database::Connection) -> (TournamentGroup, TournamentGroup) {
    let uid = seed_group_user(db);
    (
        TournamentGroupBuilder::new_default("Test TourGroup 1")
            .set_description(Some("This is Tour 1's payload.".to_string()))
            .set_creator_id(uid)
            .set_owner_id(uid)
            .build_and_insert(db)
            .unwrap(),
        TournamentGroupBuilder::new_default("Test TourGroup 2")
            .set_description(Some("This is Tour 2's payload.".to_string()))
            .set_creator_id(uid)
            .set_owner_id(uid)
            .build_and_insert(db)
            .unwrap()
    )
}

pub fn arrange_get_by_id_works_integration_test(db: &mut database::Connection) -> TournamentGroup {
    let uid = seed_group_user(db);
    TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is Tour 1's payload.".to_string()))
        .set_creator_id(uid)
        .set_owner_id(uid)
        .build_and_insert(db)
        .unwrap();
    TournamentGroupBuilder::new_default("Test TourGroup 2")
        .set_description(Some("This is Tour 2's payload.".to_string()))
        .set_creator_id(uid)
        .set_owner_id(uid)
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> TournamentGroup {
    let uid = seed_group_user(db);
    TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is TourGroup 1 testing.".to_string()))
        .set_creator_id(uid)
        .set_owner_id(uid)
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> TournamentGroup {
    let uid = seed_group_user(db);
    TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is TourGroup 1 testing.".to_string()))
        .set_creator_id(uid)
        .set_owner_id(uid)
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_get_all_tournaments_of_tournamentgroup_works_integration_test(db: &mut database::Connection) -> (TournamentGroup, Tournament, Tournament) {
    let uid = seed_group_user(db);
    let tg_1 = TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is TourGroup 1 testing.".to_string()))
        .set_creator_id(uid)
        .set_owner_id(uid)
        .build_and_insert(db)
        .unwrap();

    let owner = UserBuilder::new_default("Tour Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap();

    let tour_1 = TournamentBuilder::new_default("Tour 1")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap();
    let tour_2 = TournamentBuilder::new_default("Tour 2")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap();
    let _tour_3 = TournamentBuilder::new_default("Tour 3")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap();
    let _tour_4 = TournamentBuilder::new_default("Tour 4")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap();

    let _tg_1_bridge_tour_1 = TournamentGroupTournamentBuilder::new_default(tg_1.tgid, tour_1.tid)
        .build_and_insert(db)
        .unwrap();
    let _tg_1_bridge_tour_2 = TournamentGroupTournamentBuilder::new_default(tg_1.tgid, tour_2.tid)
        .build_and_insert(db)
        .unwrap();

    (tg_1, tour_1, tour_2)
}

pub fn arrange_add_tournament_to_tournamentgroup_works_integration_test(db: &mut database::Connection) -> NewTournamentGroupTournament {
    let uid = seed_group_user(db);
    let tournamentgroup_id = TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is TourGroup 1 testing.".to_string()))
        .set_creator_id(uid)
        .set_owner_id(uid)
        .build_and_insert(db)
        .unwrap()
        .tgid;

    let owner = UserBuilder::new_default("Tour Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap();

    let tournament_id = TournamentBuilder::new_default("Test Tournament 1")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap()
        .tid;

    TournamentGroupTournamentBuilder::new_default(tournamentgroup_id, tournament_id)
        .build()
        .unwrap()
}

pub fn arrange_remove_tournament_from_tournamentgroup_works_integration_test(db: &mut database::Connection) -> (TournamentGroup, Tournament) {
    let uid = seed_group_user(db);
    let tournamentgroup = TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is TourGroup 1 testing.".to_string()))
        .set_creator_id(uid)
        .set_owner_id(uid)
        .build_and_insert(db)
        .unwrap();

    let owner = UserBuilder::new_default("Tour Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap();

    let tournament = TournamentBuilder::new_default("Test Tournament 1")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap();

    let _bridge = TournamentGroupTournamentBuilder::new_default(tournamentgroup.tgid, tournament.tid)
        .build_and_insert(db)
        .unwrap();

    (tournamentgroup, tournament)
}
