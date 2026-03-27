use backend::{database, models::{division::{Division, DivisionBuilder}, game::Game, round::{NewRound, Round, RoundBuilder}, tournament::{Tournament, TournamentBuilder}, tournament_admin::TournamentAdminBuilder, user::{User, UserBuilder}}};
use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use uuid::Uuid;
use crate::fixtures;

/// Returns `(tournament, division, owner, admin_user, unrelated_user)` for testing
/// round create ABAC: owner and admin should be allowed, unrelated user should not.
pub fn arrange_round_create_works_integration_test(
    db: &mut database::Connection,
) -> (Tournament, Division, User, User, User) {
    let owner = UserBuilder::new_default("Tour Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap();

    let tournament = TournamentBuilder::new_default("Test Tour")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap();

    let division = DivisionBuilder::new_default("Test Div", tournament.tid)
        .build_and_insert(db)
        .unwrap();

    let admin_user = UserBuilder::new_default("Tour Admin")
        .set_hash_password("AdminPwd123!")
        .build_and_insert(db)
        .unwrap();
    TournamentAdminBuilder::new_default(tournament.tid, admin_user.id)
        .build_and_insert(db)
        .unwrap();

    let unrelated_user = UserBuilder::new_default("Unrelated User")
        .set_hash_password("UnrelPwd123!")
        .build_and_insert(db)
        .unwrap();

    (tournament, division, owner, admin_user, unrelated_user)
}

pub fn new_round(did: Uuid, sched_start_time: DateTime<Utc>) -> NewRound {
    NewRound {
        did: did,
        scheduled_start_time: sched_start_time
    }
}

pub fn get_round_payload(did: Uuid) -> NewRound {
    RoundBuilder::new_default(did)
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap())
        .build()
        .unwrap()
}

pub fn seed_round(db: &mut database::Connection, did: Uuid) -> Round {
    RoundBuilder::new_default(did)
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap())
        .build_and_insert(db)
        .unwrap()
}

pub fn seed_rounds(
    db: &mut database::Connection, 
    did: Uuid
) -> Vec<Round> {
    vec![
        RoundBuilder::new_default(did)
            .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap())
            .build_and_insert(db)
            .unwrap(),
        RoundBuilder::new_default(did)
            .set_scheduled_start_time(Utc.with_ymd_and_hms(2045, 5, 23, 00, 00, 0).unwrap())
            .build_and_insert(db)
            .unwrap(),
        RoundBuilder::new_default(did)
            .set_scheduled_start_time(Utc.with_ymd_and_hms(2065, 5, 23, 00, 00, 0).unwrap())
            .build_and_insert(db)
            .unwrap()
    ]
}

pub fn seed_rounds_with_sched_start_times(
    db: &mut database::Connection, 
    did: Uuid, 
    start_time_1: DateTime<Utc>,
    start_time_2: DateTime<Utc>,
    start_time_3: DateTime<Utc>,
) -> Vec<Round> {
    vec![
        RoundBuilder::new_default(did)
            .set_scheduled_start_time(start_time_1)
            .build_and_insert(db)
            .unwrap(),
        RoundBuilder::new_default(did)
            .set_scheduled_start_time(start_time_2)
            .build_and_insert(db)
            .unwrap(),
        RoundBuilder::new_default(did)
            .set_scheduled_start_time(start_time_3)
            .build_and_insert(db)
            .unwrap()
    ]
}

pub fn seed_get_games_by_round(db: &mut database::Connection) -> Vec<Game> {
    let (
        tid, 
        did_1, 
        room_id, 
        round_id, 
        team_1_id, 
        team_2_id, 
        team_3_id, 
        qm_id) = fixtures::games::seed_game_payload_dependencies(db, "Tour 1");
    
    let payload_1 = fixtures::games::get_game_payload(tid,did_1,room_id,round_id,team_1_id,Some(team_2_id),team_3_id,qm_id);
    let game_1 = fixtures::games::create_and_insert_game(db, payload_1);

    let payload_2 = fixtures::games::get_game_payload(tid,did_1,room_id,round_id,team_3_id,None,team_1_id,qm_id);
    let game_2 = fixtures::games::create_and_insert_game(db, payload_2);

    let div_2 = fixtures::divisions::seed_division(db, tid);
    let payload_3 = fixtures::games::get_game_payload(tid,div_2.did,room_id,round_id,team_1_id,None,team_2_id,qm_id);
    let payload_4 = fixtures::games::get_game_payload(tid,div_2.did,room_id,round_id,team_3_id,None,team_2_id,qm_id);

    vec![game_1, game_2]
}
