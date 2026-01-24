use backend::models::user::{User,NewUser};
use diesel::prelude::*;
use backend::schema::users;

pub fn new_user_one(fname: &str, unhashed_pwd: &str) -> NewUser {
    NewUser {
        email: "obviously@fakeemail.com".to_string(),
        hash_password: unhashed_pwd.to_string(),
        activated: true,
        fname: fname.to_string(),
        mname: "Maurice".to_string(),
        lname: "Den".to_string(),
        username: "1denmanforthejob1".to_string()
    }
}

pub fn new_user_two(fname: &str, unhashed_pwd: &str) -> NewUser {
    NewUser {
        email: "edbashful@fakeemail.com".to_string(),
        hash_password: unhashed_pwd.to_string(),
        activated: true,
        fname: fname.to_string(),
        mname: "Eugene".to_string(),
        lname: "Davidson".to_string(),
        username: "edbashful".to_string()
    }
}

pub fn new_user_three(fname: &str, unhashed_pwd: &str) -> NewUser {
    NewUser {
        email: "chbringit@fakeemail.com".to_string(),
        hash_password: unhashed_pwd.to_string(),
        activated: true,
        fname: fname.to_string(),
        mname: "Clarence".to_string(),
        lname: "Kennedy".to_string(),
        username: "ckbringit".to_string()
    }
}

pub fn get_user_payload(unhashed_pwd: &str) -> NewUser {
    new_user_one("Test User 3276", &unhashed_pwd)
}

pub fn create_and_insert_user(conn: &mut PgConnection, new_user: NewUser) -> User {
    diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result::<User>(conn)
        .expect("Failed to create user")
}

pub fn seed_user(conn: &mut PgConnection) -> User {
    let new_user = new_user_one("Test User 3276", "phunkeypazwurd");
    create_and_insert_user(conn, new_user)
}

pub fn seed_users(conn: &mut PgConnection) -> Vec<User> {
    seed_users_with_fnames(
        conn, 
        "Test User 3276", 
        "Test User 9078", 
        "Test User 4611")
}

pub fn seed_users_for_get_all_admins_of_tour(conn: &mut PgConnection) -> Vec<User> {
    seed_users_with_fnames_for_get_all_admins_of_tour(
        conn, 
        "Test User 3", 
        "Test User 9")
}

pub fn seed_users_with_fnames(
    conn: &mut PgConnection, 
    user_1_name: &str,
    user_2_name: &str,
    user_3_name: &str,
) -> Vec<User> {
    let new_user_1 = new_user_one(user_1_name, "Some pwd&7");
    let new_user_2 = new_user_two(user_2_name, "Grace_abundantly90");
    let new_user_3 = new_user_three(user_3_name, "Manypwdsfailthetest");

    vec![
        create_and_insert_user(conn, new_user_1),
        create_and_insert_user(conn, new_user_2),
        create_and_insert_user(conn, new_user_3),
    ]
}

pub fn seed_users_with_fnames_for_get_all_admins_of_tour(
    conn: &mut PgConnection, 
    user_1_name: &str,
    user_2_name: &str,
) -> Vec<User> {
    let new_user_1 = new_user_one(user_1_name, "Some pwd&7");
    let new_user_2 = new_user_two(user_2_name, "Grace_abundantly90");

    vec![
        create_and_insert_user(conn, new_user_1),
        create_and_insert_user(conn, new_user_2),
    ]
}
