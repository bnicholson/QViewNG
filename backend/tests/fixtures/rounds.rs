use backend::models::round::{Round,NewRound};
use chrono::{TimeZone, Utc};
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

pub fn seed_rounds(conn: &mut PgConnection, did: Uuid) -> Vec<Round> {
    seed_rounds_with_names(
        conn, 
        did, 
        "Test Round 6276", 
        "Test Round 1078", 
        "Test Round 7611")
}

pub fn seed_rounds_with_names(
    conn: &mut PgConnection, 
    did: Uuid, 
    round_1_name: &str,
    round_2_name: &str,
    round_3_name: &str,
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
