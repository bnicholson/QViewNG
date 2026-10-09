use backend::{database, models::{division::{Division, DivisionBuilder}, roundgroup::RoundGroupBuilder, round::{NewRound, Round, RoundBuilder}, tournament::{Tournament, TournamentBuilder}, tournament_admin::TournamentAdminBuilder, user::{User, UserBuilder}}};
use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

/// Rounds now belong to a division roundgroup. Create one roundgroup for `did` (attributing it to the
/// division's tournament owner) and return its id, so round fixtures still take a division id.
fn roundgroup_for(db: &mut database::Connection, did: Uuid) -> Uuid {
    let division = backend::models::division::read(db, did).unwrap();
    let tournament = backend::models::tournament::read(db, division.tid).unwrap();
    RoundGroupBuilder::new(did)
        .set_name("Test Session")
        .set_creator_userid(tournament.owner_id)
        .build_and_insert(db)
        .unwrap()
        .roundgroup_id
}

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

/// Returns `(tournament, division, round_1, round_2, owner, admin_user, unrelated_user)` for testing
/// round delete ABAC: owner and admin should be allowed, unrelated user should not.
/// round_1 is used for fail cases and the owner success case; round_2 is used for the admin success case.
pub fn arrange_round_delete_works_integration_test(
    db: &mut database::Connection,
) -> (Tournament, Division, Round, Round, User, User, User) {
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
    let roundgroup_id = roundgroup_for(db, division.did);

    let round_1 = RoundBuilder::new_default(roundgroup_id)
        .set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2060, 1, 1, 0, 0, 0).unwrap())
        .build_and_insert(db)
        .unwrap();

    let round_2 = RoundBuilder::new_default(roundgroup_id)
        .set_name("2")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2061, 1, 1, 0, 0, 0).unwrap())
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

    (tournament, division, round_1, round_2, owner, admin_user, unrelated_user)
}

/// Returns `(tournament, division, round, owner, admin_user, unrelated_user)` for testing
/// round update ABAC: owner and admin should be allowed, unrelated user should not.
pub fn arrange_round_update_works_integration_test(
    db: &mut database::Connection,
) -> (Tournament, Division, Round, User, User, User) {
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
    let roundgroup_id = roundgroup_for(db, division.did);

    let round = RoundBuilder::new_default(roundgroup_id)
        .set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap())
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

    (tournament, division, round, owner, admin_user, unrelated_user)
}

/// Builds a round create payload for the given division roundgroup (rounds now belong to a roundgroup).
pub fn get_round_payload(roundgroup_id: Uuid) -> NewRound {
    RoundBuilder::new_default(roundgroup_id)
        .set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap())
        .build()
        .unwrap()
}

pub fn seed_round(db: &mut database::Connection, did: Uuid) -> Round {
    let roundgroup_id = roundgroup_for(db, did);
    RoundBuilder::new_default(roundgroup_id)
        .set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap())
        .build_and_insert(db)
        .unwrap()
}

pub fn seed_rounds(
    db: &mut database::Connection,
    did: Uuid
) -> Vec<Round> {
    let roundgroup_id = roundgroup_for(db, did);
    vec![
        RoundBuilder::new_default(roundgroup_id)
            .set_name("1")
            .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap())
            .build_and_insert(db)
            .unwrap(),
        RoundBuilder::new_default(roundgroup_id)
            .set_name("2")
            .set_scheduled_start_time(Utc.with_ymd_and_hms(2045, 5, 23, 00, 00, 0).unwrap())
            .build_and_insert(db)
            .unwrap(),
        RoundBuilder::new_default(roundgroup_id)
            .set_name("3")
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
    let roundgroup_id = roundgroup_for(db, did);
    vec![
        RoundBuilder::new_default(roundgroup_id)
            .set_name("1")
            .set_scheduled_start_time(start_time_1)
            .build_and_insert(db)
            .unwrap(),
        RoundBuilder::new_default(roundgroup_id)
            .set_name("2")
            .set_scheduled_start_time(start_time_2)
            .build_and_insert(db)
            .unwrap(),
        RoundBuilder::new_default(roundgroup_id)
            .set_name("3")
            .set_scheduled_start_time(start_time_3)
            .build_and_insert(db)
            .unwrap()
    ]
}
