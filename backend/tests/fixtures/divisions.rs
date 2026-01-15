use backend::models::division::{Division,NewDivision};
use diesel::prelude::*;
use uuid::Uuid;
use backend::schema::divisions;

pub fn new_division_one(tid: Uuid) -> NewDivision {
    NewDivision {
        tid: tid,
        dname: "Test Div 3276".to_string(),
        breadcrumb: "/test/post/for/division".to_string(),
        is_public: false,
        shortinfo: "Experienced (but still young).".to_string()
    }
}

pub fn get_division_payload(tid: Uuid) -> NewDivision {
    new_division_one(tid)
}

fn create_and_insert_division(conn: &mut PgConnection, new_division: NewDivision) -> Division {
    diesel::insert_into(divisions::table)
        .values(new_division)
        .returning(Division::as_returning())
        .get_result::<Division>(conn)
        .expect("Failed to create division")
}

pub fn seed_division(conn: &mut PgConnection, tid: Uuid) -> Division {
    let new_division = new_division_one(tid);
    create_and_insert_division(conn, new_division)
}
