use backend::{database, models::user::{NewUser, User, UserBuilder}};
use diesel::prelude::*;

pub fn get_user_payload(unhashed_pwd: &str) -> NewUser {
    UserBuilder::new_default("Test User 3276")
        .set_hash_password(unhashed_pwd)
        .build()
        .unwrap()
}

pub fn create_and_insert_user(db: &mut database::Connection, fname: &str, pwd: &str) -> User {
    UserBuilder::new_default(fname)
        .set_hash_password(pwd)
        .build_and_insert(db)
        .unwrap()
}

pub fn seed_user(db: &mut database::Connection) -> User {
    UserBuilder::new_default("Test User 3276")
        .set_hash_password("phunkeypazwurd")
        .build_and_insert(db)
        .unwrap()
}

pub fn seed_users(db: &mut database::Connection) -> Vec<User> {
    seed_users_with_fnames(
        db, 
        "Test User 3276", 
        "Test User 9078", 
        "Test User 4611")
}

pub fn seed_users_for_get_all_admins_of_tour(db: &mut database::Connection) -> Vec<User> {
    seed_users_with_fnames_for_get_all_admins_of_tour(
        db, 
        "Test User 3", 
        "Test User 9")
}

pub fn seed_users_with_fnames(
    db: &mut database::Connection, 
    user_1_name: &str,
    user_2_name: &str,
    user_3_name: &str,
) -> Vec<User> {
    vec![
        UserBuilder::new_default(user_1_name)
            .set_hash_password("Some pwd&7")
            .build_and_insert(db)
            .unwrap(),
        UserBuilder::new_default(user_2_name)
            .set_email("edbashful@fakeemail.com")
            .set_hash_password("Grace_abundantly90")
            .set_activated(true)
            .set_mname("Eugene")
            .set_lname("Davidson")
            .set_username("edbashful")
            .build_and_insert(db)
            .unwrap(),
        UserBuilder::new_default(user_3_name)
            .set_email("chbringit@fakeemail.com")
            .set_hash_password("Manypwdsfailthetest")
            .set_activated(true)
            .set_mname("Clarence")
            .set_lname("Kennedy")
            .set_username("ckbringit")
            .build_and_insert(db)
            .unwrap()
    ]
}

pub fn seed_users_with_fnames_for_get_all_admins_of_tour(
    db: &mut database::Connection, 
    user_1_name: &str,
    user_2_name: &str,
) -> Vec<User> {
    vec![
        UserBuilder::new_default(user_1_name)
            .set_hash_password("Some pwd&7")
            .build_and_insert(db)
            .unwrap(),
        UserBuilder::new_default(user_2_name)
            .set_email("edbashful@fakeemail.com")
            .set_hash_password("Grace_abundantly90")
            .set_activated(true)
            .set_mname("Eugene")
            .set_lname("Davidson")
            .set_username("edbashful")
            .build_and_insert(db)
            .unwrap()
    ]
}
