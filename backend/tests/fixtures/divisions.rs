use backend::models::division::{Division,NewDivision};
use chrono::{TimeZone, Utc};
use diesel::prelude::*;
use uuid::Uuid;
use backend::schema::divisions;

use crate::fixtures::{rounds::seed_rounds_with_sched_start_times, tournaments::seed_tournament};

pub fn new_division_one(tid: Uuid, dname: &str) -> NewDivision {
    NewDivision {
        tid: tid,
        dname: dname.to_string(),
        breadcrumb: "/test/post/for/division/1".to_string(),
        is_public: false,
        shortinfo: "Experienced (but still young).".to_string()
    }
}

pub fn new_division_two(tid: Uuid, dname: &str) -> NewDivision {
    NewDivision {
        tid: tid,
        dname: dname.to_string(),
        breadcrumb: "/test/post/for/division/2".to_string(),
        is_public: false,
        shortinfo: "Novice".to_string()
    }
}

pub fn new_division_three(tid: Uuid, dname: &str) -> NewDivision {
    NewDivision {
        tid: tid,
        dname: dname.to_string(),
        breadcrumb: "/test/post/for/division/3".to_string(),
        is_public: false,
        shortinfo: "Decades quizzing".to_string()
    }
}

pub fn get_division_payload(tid: Uuid) -> NewDivision {
    new_division_one(tid, "Test Div 3276")
}

fn create_and_insert_division(conn: &mut PgConnection, new_division: NewDivision) -> Division {
    diesel::insert_into(divisions::table)
        .values(new_division)
        .returning(Division::as_returning())
        .get_result::<Division>(conn)
        .expect("Failed to create division")
}

pub fn seed_division(conn: &mut PgConnection, tid: Uuid) -> Division {
    let new_division = new_division_one(tid, "Test Div 3276");
    create_and_insert_division(conn, new_division)
}

pub fn seed_division_with_name(conn: &mut PgConnection, tid: Uuid, dname: &str) -> Division {
    let new_division = new_division_one(tid, dname);
    create_and_insert_division(conn, new_division)
}

pub fn seed_divisions(conn: &mut PgConnection, tid: Uuid) -> Vec<Division> {
    seed_divisions_with_names(
        conn, 
        tid, 
        "Test Div 3276", 
        "Test Div 9078", 
        "Test Div 4611")
}

pub fn seed_divisions_with_names(
    conn: &mut PgConnection, 
    tid: Uuid, 
    div_1_name: &str,
    div_2_name: &str,
    div_3_name: &str,
) -> Vec<Division> {
    let new_division_1 = new_division_one(tid, div_1_name);
    let new_division_2 = new_division_two(tid, div_2_name);
    let new_division_3 = new_division_three(tid, div_3_name);

    vec![
        create_and_insert_division(conn, new_division_1),
        create_and_insert_division(conn, new_division_2),
        create_and_insert_division(conn, new_division_3),
    ]
}

pub fn seed_get_rounds_by_division(conn: &mut PgConnection) -> Division {
    let tournament = seed_tournament(conn);
    let divisions = seed_divisions_with_names(conn, tournament.tid, "D1", "D2", "D42");

    let div_1 = &divisions[0];
    let start_time_1 = Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap();
    let start_time_2 = Utc.with_ymd_and_hms(2056, 5, 23, 00, 00, 0).unwrap();
    let start_time_3 = Utc.with_ymd_and_hms(2057, 5, 23, 00, 00, 0).unwrap();
    seed_rounds_with_sched_start_times(conn, div_1.did, start_time_1, start_time_2, start_time_3);

    let div_2 = &divisions[1];
    let start_time_4 = Utc.with_ymd_and_hms(2058, 5, 23, 00, 00, 0).unwrap();
    let start_time_5 = Utc.with_ymd_and_hms(2059, 5, 23, 00, 00, 0).unwrap();
    let start_time_6 = Utc.with_ymd_and_hms(2060, 5, 23, 00, 00, 0).unwrap();
    seed_rounds_with_sched_start_times(conn, div_2.did, start_time_4, start_time_5, start_time_6);

    let div_3 = &divisions[2];
    let start_time_7 = Utc.with_ymd_and_hms(2061, 5, 23, 00, 00, 0).unwrap();
    let start_time_8 = Utc.with_ymd_and_hms(2062, 5, 23, 00, 00, 0).unwrap();
    let start_time_9 = Utc.with_ymd_and_hms(2063, 5, 23, 00, 00, 0).unwrap();
    seed_rounds_with_sched_start_times(conn, div_3.did, start_time_7, start_time_8, start_time_9);

    div_3.clone()
}

pub fn seed_rounds_in_division(conn: &mut PgConnection, tid: Uuid) -> Division {
    let divisions = seed_divisions_with_names(conn, tid, "D1", "D2", "D42");

    let div_1 = &divisions[0];
    let start_time_1 = Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap();
    let start_time_2 = Utc.with_ymd_and_hms(2056, 5, 23, 00, 00, 0).unwrap();
    let start_time_3 = Utc.with_ymd_and_hms(2057, 5, 23, 00, 00, 0).unwrap();
    seed_rounds_with_sched_start_times(conn, div_1.did, start_time_1, start_time_2, start_time_3);

    div_1.clone()
}
