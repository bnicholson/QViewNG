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

pub fn arrange_get_all_teams_where_user_is_quizzer_works_integration_test(
    db: &mut database::Connection
) -> (User, Team, Team, Team, Team, Team, Team) {

    // 1 quizzer on different 6 teams, each team with that quizzer in a different spot on the team (seats 1 through 6)
    // each team will require a different division because a quizzer can only compete in one division at a time
    // all 6 divisions can be in the same tournament for testing purposes, but let's do 3 different tournaments

    let quizzer_of_interest = UserBuilder::new_default("Quizzer of Interest")
        .set_hash_password("QuizzerPwd123!")
        .build_and_insert(db)
        .unwrap();
    let quizzer_2 = UserBuilder::new_default("Quizzer 2")
        .set_hash_password("QuizzerPwd123!")
        .build_and_insert(db)
        .unwrap();
    let quizzer_3 = UserBuilder::new_default("Quizzer 3")
        .set_hash_password("QuizzerPwd123!")
        .build_and_insert(db)
        .unwrap();
    let quizzer_4 = UserBuilder::new_default("Quizzer 4")
        .set_hash_password("QuizzerPwd123!")
        .build_and_insert(db)
        .unwrap();
    let quizzer_5 = UserBuilder::new_default("Quizzer 5")
        .set_hash_password("QuizzerPwd123!")
        .build_and_insert(db)
        .unwrap();
    let quizzer_6 = UserBuilder::new_default("Quizzer 6")
        .set_hash_password("QuizzerPwd123!")
        .build_and_insert(db)
        .unwrap();
    let coach = UserBuilder::new_default("Coach")
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
        .set_quizzer_one_id(quizzer_of_interest.id)  // <- on seat 1
        .set_quizzer_two_id(quizzer_2.id)
        .build_and_insert(db)
        .unwrap();

    let division_2 = DivisionBuilder::new_default("Div 2", tour_1.tid)
        .build_and_insert(db)
        .unwrap();
    let team_2 = TeamBuilder::new_default(division_2.did)
        .set_name("Team 2")
        .set_coachid(coach.id)
        .set_quizzer_one_id(quizzer_2.id)
        .set_quizzer_two_id(quizzer_of_interest.id)  // <- on seat 2
        .build_and_insert(db)
        .unwrap();

    let tour_2 = TournamentBuilder::new_default("Tour 2")
        .build_and_insert(db)
        .unwrap();

    let division_3 = DivisionBuilder::new_default("Div 3", tour_2.tid)
        .build_and_insert(db)
        .unwrap();
    let team_3 = TeamBuilder::new_default(division_3.did)
        .set_name("Team 3")
        .set_coachid(coach.id)
        .set_quizzer_one_id(quizzer_2.id)
        .set_quizzer_two_id(quizzer_3.id)
        .set_quizzer_three_id(quizzer_of_interest.id)  // <- on seat 3
        .build_and_insert(db)
        .unwrap();

    let division_4 = DivisionBuilder::new_default("Div 4", tour_2.tid)
        .build_and_insert(db)
        .unwrap();
    let team_4 = TeamBuilder::new_default(division_4.did)
        .set_name("Team 4")
        .set_coachid(coach.id)
        .set_quizzer_one_id(quizzer_2.id)
        .set_quizzer_two_id(quizzer_3.id)
        .set_quizzer_three_id(quizzer_4.id)  // <- on seat 3
        .set_quizzer_four_id(quizzer_of_interest.id)  // <- on seat 4
        .build_and_insert(db)
        .unwrap();

    let tour_3 = TournamentBuilder::new_default("Tour 3")
        .build_and_insert(db)
        .unwrap();

    let division_5 = DivisionBuilder::new_default("Div 5", tour_3.tid)
        .build_and_insert(db)
        .unwrap();
    let team_5 = TeamBuilder::new_default(division_5.did)
        .set_name("Team 5")
        .set_coachid(coach.id)
        .set_quizzer_one_id(quizzer_2.id)
        .set_quizzer_two_id(quizzer_3.id)
        .set_quizzer_three_id(quizzer_4.id) 
        .set_quizzer_four_id(quizzer_5.id)
        .set_quizzer_five_id(quizzer_of_interest.id)  // <- on seat 5
        .build_and_insert(db)
        .unwrap();

    let division_6 = DivisionBuilder::new_default("Div 6", tour_2.tid)
        .build_and_insert(db)
        .unwrap();
    let team_6 = TeamBuilder::new_default(division_6.did)
        .set_name("Team 6")
        .set_coachid(coach.id)
        .set_quizzer_one_id(quizzer_2.id)
        .set_quizzer_two_id(quizzer_3.id)
        .set_quizzer_three_id(quizzer_4.id) 
        .set_quizzer_four_id(quizzer_5.id)
        .set_quizzer_five_id(quizzer_6.id)
        .set_quizzer_six_id(quizzer_of_interest.id)  // <- on seat 6 (alternate)
        .build_and_insert(db)
        .unwrap();

    (quizzer_of_interest, team_1, team_2, team_3, team_4, team_5, team_6)
}
