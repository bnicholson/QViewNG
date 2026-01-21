use backend::models::round::{Round,NewRound};
use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use uuid::Uuid;
use backend::schema::rounds;

pub fn new_round_one(did: Uuid) -> NewRound {
    NewRound {
        did: did,
        scheduled_start_time: Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap()
    }
}

pub fn new_round_two(did: Uuid) -> NewRound {
    NewRound {
        did: did,
        scheduled_start_time: Utc.with_ymd_and_hms(2045, 5, 23, 00, 00, 0).unwrap()
    }
}

pub fn new_round_three(did: Uuid) -> NewRound {
    NewRound {
        did: did,
        scheduled_start_time: Utc.with_ymd_and_hms(2065, 5, 23, 00, 00, 0).unwrap()
    }
}

pub fn new_round(did: Uuid, sched_start_time: DateTime<Utc>) -> NewRound {
    NewRound {
        did: did,
        scheduled_start_time: sched_start_time
    }
}

pub fn get_round_payload(did: Uuid) -> NewRound {
    new_round_one(did)
}

fn create_and_insert_round(conn: &mut PgConnection, new_round: NewRound) -> Round {
    diesel::insert_into(rounds::table)
        .values(new_round)
        .returning(Round::as_returning())
        .get_result::<Round>(conn)
        .expect("Failed to create round")
}

pub fn seed_round(conn: &mut PgConnection, did: Uuid) -> Round {
    let new_round = new_round_one(did);
    create_and_insert_round(conn, new_round)
}

pub fn seed_rounds(
    conn: &mut PgConnection, 
    did: Uuid
) -> Vec<Round> {
    let new_round_1 = new_round_one(did);
    let new_round_2 = new_round_two(did);
    let new_round_3 = new_round_three(did);

    vec![
        create_and_insert_round(conn, new_round_1),
        create_and_insert_round(conn, new_round_2),
        create_and_insert_round(conn, new_round_3),
    ]
}

pub fn seed_rounds_with_sched_start_times(
    conn: &mut PgConnection, 
    did: Uuid, 
    start_time_1: DateTime<Utc>,
    start_time_2: DateTime<Utc>,
    start_time_3: DateTime<Utc>,
) -> Vec<Round> {
    let new_round_1 = new_round(did, start_time_1);
    let new_round_2 = new_round(did, start_time_2);
    let new_round_3 = new_round(did, start_time_3);

    vec![
        create_and_insert_round(conn, new_round_1),
        create_and_insert_round(conn, new_round_2),
        create_and_insert_round(conn, new_round_3),
    ]
}
