use backend::models::user::{User,NewUser};
use diesel::prelude::*;
use uuid::Uuid;
use backend::schema::users;

pub fn new_user_one(fname: &str) -> NewUser {
    NewUser {
        email: "obviously@fakeemail.com".to_string(),
        hash_password: "FamouslySecure!23".to_string(),
        activated: true,
        fname: fname.to_string(),
        mname: "Maurice".to_string(),
        lname: "Den".to_string(),
        username: "Experienced (but still young).".to_string()
    }
}

pub fn get_user_payload() -> NewUser {
    new_user_one("Test User 3276")
}

// fn create_and_insert_division(conn: &mut PgConnection, new_division: NewUser) -> User {
//     diesel::insert_into(divisions::table)
//         .values(new_division)
//         .returning(Division::as_returning())
//         .get_result::<Division>(conn)
//         .expect("Failed to create division")
// }

// pub fn seed_division(conn: &mut PgConnection, tid: Uuid) -> User {
//     let new_division = new_division_one(tid, "Test Div 3276");
//     create_and_insert_division(conn, new_division)
// }

// pub fn seed_divisions(conn: &mut PgConnection, tid: Uuid) -> Vec<Division> {
//     seed_divisions_with_names(
//         conn, 
//         tid, 
//         "Test Div 3276", 
//         "Test Div 9078", 
//         "Test Div 4611")
// }

// pub fn seed_divisions_with_names(
//     conn: &mut PgConnection, 
//     tid: Uuid, 
//     div_1_name: &str,
//     div_2_name: &str,
//     div_3_name: &str,
// ) -> Vec<Division> {
//     let new_division_1 = new_division_one(tid, div_1_name);
//     let new_division_2 = new_division_two(tid, div_2_name);
//     let new_division_3 = new_division_three(tid, div_3_name);

//     vec![
//         create_and_insert_division(conn, new_division_1),
//         create_and_insert_division(conn, new_division_2),
//         create_and_insert_division(conn, new_division_3),
//     ]
// }
