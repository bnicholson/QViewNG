use backend::models::division::{Division,NewDivision};
use diesel::prelude::*;
use uuid::Uuid;
use backend::schema::divisions;

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
