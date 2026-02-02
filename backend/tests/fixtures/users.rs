use backend::{database, models::{division::DivisionBuilder, team::{Team, TeamBuilder}, tournament::TournamentBuilder, tournament_admin::{TournamentAdmin, TournamentAdminBuilder}, user::{NewUser, User, UserBuilder}}};
use uuid::Uuid;

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

pub fn seed_get_tournaments_where_user_is_admin(db: &mut database::Connection) -> (User, Uuid, Uuid) {

    // 4 tours where user is admin in 2 tours
    
    let admin = UserBuilder::new_default("Test User 3276")
        .set_hash_password("phunkeypazwurd")
        .build_and_insert(db)
        .unwrap();

    let tour_1 = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();

    let tour_2 = TournamentBuilder::new_default("Tour 2")
        .build_and_insert(db)
        .unwrap();

    TournamentAdminBuilder::new_default(tour_1.tid, admin.id)
        .build_and_insert(db)
        .unwrap();

    TournamentAdminBuilder::new_default(tour_2.tid, admin.id)
        .build_and_insert(db)
        .unwrap();

    TournamentBuilder::new_default("Tour 3")
        .build_and_insert(db)
        .unwrap();

    TournamentBuilder::new_default("Tour 4")
        .build_and_insert(db)
        .unwrap();

    (admin, tour_1.tid, tour_2.tid)
}

pub fn arrange_get_all_teams_where_user_is_coach_works_integration_test(
    db: &mut database::Connection
) -> (User, Team, Team) {

    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("CoachPwd123!")
        .build_and_insert(db)
        .unwrap();

    let tour_1 = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();
    let division_1 = DivisionBuilder::new_default("Div 1", tour_1.tid)
        .build_and_insert(db)
        .unwrap();
    let team_1 = TeamBuilder::new_default(division_1.did)
        .set_name("Team 1")
        .set_coachid(coach.id)
        .build_and_insert(db)
        .unwrap();

    let tour_2 = TournamentBuilder::new_default("Tour 2")
        .build_and_insert(db)
        .unwrap();
    let division_2 = DivisionBuilder::new_default("Div 2", tour_2.tid)
        .build_and_insert(db)
        .unwrap();
    let team_2 = TeamBuilder::new_default(division_2.did)
        .set_name("Team 2")
        .set_coachid(coach.id)
        .build_and_insert(db)
        .unwrap();

    (coach, team_1, team_2)
}
