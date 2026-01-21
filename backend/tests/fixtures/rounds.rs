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

// pub fn new_round_two(tid: Uuid, round_name: &str) -> NewRound {
//     NewRound {
//         tid: tid,
//         name: round_name.to_string(),
//         building: "Bldng 2".to_string(),
//         comments: "I thought I recognized this place.".to_string()
//     }
// }

// pub fn new_round_three(tid: Uuid, round_name: &str) -> NewRound {
//     NewRound {
//         tid: tid,
//         name: round_name.to_string(),
//         building: "Building H".to_string(),
//         comments: "How'd we get here?".to_string()
//     }
// }

pub fn get_round_payload(did: Uuid) -> NewRound {
    new_round_one(did)
}

// fn create_and_insert_round(conn: &mut PgConnection, new_round: NewRound) -> Round {
//     diesel::insert_into(rounds::table)
//         .values(new_round)
//         .returning(Round::as_returning())
//         .get_result::<Round>(conn)
//         .expect("Failed to create round")
// }

// pub fn seed_round(conn: &mut PgConnection, tid: Uuid) -> Round {
//     let new_round = new_round_one(tid, "Test Round 3276");
//     create_and_insert_round(conn, new_round)
// }

// pub fn seed_rounds(conn: &mut PgConnection, tid: Uuid) -> Vec<Round> {
//     seed_rounds_with_names(
//         conn, 
//         tid, 
//         "Test Round 3276", 
//         "Test Round 9078", 
//         "Test Round 4611")
// }

// pub fn seed_rounds_with_names(
//     conn: &mut PgConnection, 
//     tid: Uuid, 
//     round_1_name: &str,
//     round_2_name: &str,
//     round_3_name: &str,
// ) -> Vec<Round> {
//     let new_round_1 = new_round_one(tid, round_1_name);
//     let new_round_2 = new_round_two(tid, round_2_name);
//     let new_round_3 = new_round_three(tid, round_3_name);

//     vec![
//         create_and_insert_round(conn, new_round_1),
//         create_and_insert_round(conn, new_round_2),
//         create_and_insert_round(conn, new_round_3),
//     ]
// }
