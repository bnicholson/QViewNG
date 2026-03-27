use backend::database;
use backend::models::division::{Division, DivisionBuilder, NewDivision};
use backend::models::team::{Team};
use backend::models::tournament::{Tournament, TournamentBuilder};
use backend::models::tournament_admin::TournamentAdminBuilder;
use backend::models::user::{User, UserBuilder};
use chrono::{TimeZone, Utc};
use uuid::Uuid;

use crate::fixtures::{rounds::seed_rounds_with_sched_start_times, teams::seed_teams_with_names, tournaments::seed_tournament};

/// Returns `(tournament, division_1, division_2, owner, admin_user, unrelated_user)` for testing
/// division delete ABAC: owner and admin should be allowed, unrelated user should not.
/// division_1 is used for fail cases and the owner success case; division_2 is used for the admin success case.
pub fn arrange_division_delete_works_integration_test(
    db: &mut database::Connection,
) -> (Tournament, Division, Division, User, User, User) {
    let owner = UserBuilder::new_default("Tour Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap();

    let tournament = TournamentBuilder::new_default("Test Tour")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap();

    let division_1 = DivisionBuilder::new_default("Test Div Delete 1", tournament.tid)
        .build_and_insert(db)
        .unwrap();

    let division_2 = DivisionBuilder::new_default("Test Div Delete 2", tournament.tid)
        .set_breadcrumb("/test/delete/division/2")
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

    (tournament, division_1, division_2, owner, admin_user, unrelated_user)
}

/// Returns `(tournament, division, owner, admin_user, unrelated_user)` for testing
/// division update ABAC: owner and admin should be allowed, unrelated user should not.
pub fn arrange_division_update_works_integration_test(
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

    let division = DivisionBuilder::new_default("Test Div 3276", tournament.tid)
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

/// Returns `(tournament, owner, admin_user, unrelated_user)` for testing
/// division create ABAC: owner and admin should be allowed, unrelated user should not.
pub fn arrange_division_create_works_integration_test(
    db: &mut database::Connection,
) -> (Tournament, User, User, User) {
    let owner = UserBuilder::new_default("Tour Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap();

    let tournament = TournamentBuilder::new_default("Test Tour")
        .set_owner_id(owner.id)
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

    (tournament, owner, admin_user, unrelated_user)
}

pub fn get_division_payload(tid: Uuid) -> NewDivision {
    DivisionBuilder::new_default("Test Div 3276", tid).build().unwrap()
}

pub fn seed_division(db: &mut database::Connection, tid: Uuid) -> Division {
    DivisionBuilder::new_default("Test Div 3276", tid)
        .build_and_insert(db)
        .unwrap()
}

pub fn seed_division_with_name(db: &mut database::Connection, tid: Uuid, div_name: &str) -> Division {
    DivisionBuilder::new_default(div_name, tid)
        .build_and_insert(db)
        .unwrap()
}

pub fn seed_divisions(db: &mut database::Connection, tid: Uuid) -> Vec<Division> {
    vec![
        DivisionBuilder::new_default("Test Div 3276", tid)
            .build_and_insert(db)
            .unwrap(),
        DivisionBuilder::new_default("Test Div 9078", tid)
            .set_breadcrumb("/test/post/for/division/2")
            .build_and_insert(db)
            .unwrap(),
        DivisionBuilder::new_default("Test Div 4611", tid)
            .set_breadcrumb("/test/post/for/division/3")
            .build_and_insert(db)
            .unwrap()
    ]
}

pub fn seed_divisions_with_names(
    db: &mut database::Connection, 
    tid: Uuid, 
    div_1_name: &str,
    div_2_name: &str,
    div_3_name: &str,
) -> Vec<Division> {
    vec![
        DivisionBuilder::new_default(div_1_name, tid)
            .build_and_insert(db)
            .unwrap(),
        DivisionBuilder::new_default(div_2_name, tid)
            .build_and_insert(db)
            .unwrap(),
        DivisionBuilder::new_default(div_3_name, tid)
            .build_and_insert(db)
            .unwrap()
    ]
}

pub fn seed_get_rounds_by_division(db: &mut database::Connection) -> Division {
    let tournament = seed_tournament(db, "Test Post");

    let divisions = vec![
        DivisionBuilder::new_default("D1", tournament.tid)
            .build_and_insert(db)
            .unwrap(),
        DivisionBuilder::new_default("D2", tournament.tid)
            .build_and_insert(db)
            .unwrap(),
        DivisionBuilder::new_default("D42", tournament.tid)
            .build_and_insert(db)
            .unwrap()
    ];

    let div_1 = &divisions[0];
    let start_time_1 = Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap();
    let start_time_2 = Utc.with_ymd_and_hms(2056, 5, 23, 00, 00, 0).unwrap();
    let start_time_3 = Utc.with_ymd_and_hms(2057, 5, 23, 00, 00, 0).unwrap();
    seed_rounds_with_sched_start_times(db, div_1.did, start_time_1, start_time_2, start_time_3);

    let div_2 = &divisions[1];
    let start_time_4 = Utc.with_ymd_and_hms(2058, 5, 23, 00, 00, 0).unwrap();
    let start_time_5 = Utc.with_ymd_and_hms(2059, 5, 23, 00, 00, 0).unwrap();
    let start_time_6 = Utc.with_ymd_and_hms(2060, 5, 23, 00, 00, 0).unwrap();
    seed_rounds_with_sched_start_times(db, div_2.did, start_time_4, start_time_5, start_time_6);

    let div_3 = &divisions[2];
    let start_time_7 = Utc.with_ymd_and_hms(2061, 5, 23, 00, 00, 0).unwrap();
    let start_time_8 = Utc.with_ymd_and_hms(2062, 5, 23, 00, 00, 0).unwrap();
    let start_time_9 = Utc.with_ymd_and_hms(2063, 5, 23, 00, 00, 0).unwrap();
    seed_rounds_with_sched_start_times(db, div_3.did, start_time_7, start_time_8, start_time_9);

    div_3.clone()
}

pub fn seed_get_teams_by_division(db: &mut database::Connection) -> Team {
    let owner = UserBuilder::new_default("Tour Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap();
    let tournament = TournamentBuilder::new_default("Test Post")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap();

    let divisions = vec![
        DivisionBuilder::new_default("D1", tournament.tid)
            .build_and_insert(db)
            .unwrap(),
        DivisionBuilder::new_default("D2", tournament.tid)
            .build_and_insert(db)
            .unwrap(),
        DivisionBuilder::new_default("D42", tournament.tid)
            .build_and_insert(db)
            .unwrap()
    ];

    let div_1 = &divisions[0];
    let div_2 = &divisions[1];

    seed_teams_with_names(db, div_2.did, "Keiths Team", "Jans Team", "Tobys Team");
    seed_teams_with_names(db, div_1.did, "Jefferons Team", "Andersons Team", "Smiths Team").0
}

pub fn seed_rounds_in_division(db: &mut database::Connection, tid: Uuid) -> Division {

    let divisions = vec![
        DivisionBuilder::new_default("D1", tid)
            .build_and_insert(db)
            .unwrap(),
        DivisionBuilder::new_default("D2", tid)
            .build_and_insert(db)
            .unwrap(),
        DivisionBuilder::new_default("D42", tid)
            .build_and_insert(db)
            .unwrap()
    ];

    let div_1 = &divisions[0];
    let start_time_1 = Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap();
    let start_time_2 = Utc.with_ymd_and_hms(2056, 5, 23, 00, 00, 0).unwrap();
    let start_time_3 = Utc.with_ymd_and_hms(2057, 5, 23, 00, 00, 0).unwrap();
    seed_rounds_with_sched_start_times(db, div_1.did, start_time_1, start_time_2, start_time_3);

    div_1.clone()
}
