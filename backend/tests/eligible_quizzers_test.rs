
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::database::Database;
use backend::database::seed_data::system_default_data::insert_system_default_data;
use backend::models::division::DivisionBuilder;
use backend::models::role::{self, AppRole};
use backend::models::roster::RosterBuilder;
use backend::models::roster_quizzer::RosterQuizzerBuilder;
use backend::models::team::TeamBuilder;
use backend::models::tournament::TournamentBuilder;
use backend::models::user::{User, UserBuilder};
use backend::models::users_roles::UsersRolesBuilder;
use backend::routes::configure_routes;
use crate::common::{TEST_DB_URL, clean_database};

/// A tournament manager's eligible-quizzer list = their "My Quizzers" (roster quizzers) plus every
/// participant of the tournaments they manage — and nothing from tournaments they don't manage.
#[actix_web::test]
async fn eligible_quizzers_for_manager_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    insert_system_default_data(&mut conn); // seeds the default roles (incl. tournament_manager)

    // The caller: a tournament manager.
    let manager = UserBuilder::new_default("Manager").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let tm_role = role::read_by_name(&mut conn, AppRole::TournamentManager.as_str()).unwrap();
    UsersRolesBuilder::new(manager.id).assign(tm_role.id).build_and_insert(&mut conn).unwrap();

    // (1) "My Quizzers": a roster the manager coaches, holding quizzer Q1.
    let q1 = UserBuilder::new_default("Quinn").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let roster = RosterBuilder::new_default("My Roster", manager.id).build_and_insert(&mut conn).unwrap();
    RosterQuizzerBuilder::new(q1.id, roster.rosterid).build_and_insert(&mut conn).unwrap();

    // (2) A tournament the manager owns, with a team (coach C, quizzer Q2).
    let coach = UserBuilder::new_default("Casey").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let q2 = UserBuilder::new_default("Quincy").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let tournament = TournamentBuilder::new_default("Managed Tour").set_owner_id(manager.id).build_and_insert(&mut conn).unwrap();
    let division = DivisionBuilder::new_default("Div A", tournament.tid).build_and_insert(&mut conn).unwrap();
    TeamBuilder::new_default(division.did)
        .set_name("Team A").set_coachid(coach.id).set_quizzer_one_id(q2.id)
        .build_and_insert(&mut conn).unwrap();

    // A DIFFERENT tournament the manager does NOT manage, with quizzer Q3 — must be excluded.
    let other_owner = UserBuilder::new_default("Owner2").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let q3 = UserBuilder::new_default("Quest").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let other_tour = TournamentBuilder::new_default("Other Tour").set_owner_id(other_owner.id).build_and_insert(&mut conn).unwrap();
    let other_div = DivisionBuilder::new_default("Div B", other_tour.tid).build_and_insert(&mut conn).unwrap();
    TeamBuilder::new_default(other_div.did)
        .set_name("Team B").set_coachid(other_owner.id).set_quizzer_one_id(q3.id)
        .build_and_insert(&mut conn).unwrap();

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes)
    ).await;

    // Act:

    let uri = format!("/api/users/{}/eligible-quizzers", manager.id);
    let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<User> = test::read_body_json(resp).await;
    let ids: Vec<uuid::Uuid> = body.iter().map(|u| u.id).collect();

    assert!(ids.contains(&q1.id), "My-Quizzers quizzer Q1 should be eligible");
    assert!(ids.contains(&q2.id), "managed tournament's quizzer Q2 should be eligible");
    assert!(ids.contains(&coach.id), "managed tournament's coach should be eligible");
    assert!(ids.contains(&manager.id), "managed tournament's owner should be eligible");
    assert!(!ids.contains(&q3.id), "quizzer of an unmanaged tournament must be excluded");
    assert!(!ids.contains(&other_owner.id), "owner of an unmanaged tournament must be excluded");

    // Ordered alphabetically by first name.
    let fnames: Vec<&str> = body.iter().map(|u| u.fname.as_str()).collect();
    let mut sorted = fnames.clone();
    sorted.sort();
    assert_eq!(fnames, sorted, "results should be alphabetical by first name");
}

/// A plain member (no manager/super-user role) gets only their "My Quizzers" — no tournament
/// participants leak in.
#[actix_web::test]
async fn eligible_quizzers_for_member_is_my_quizzers_only() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    insert_system_default_data(&mut conn);

    let coach = UserBuilder::new_default("Coach").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    // Coach's roster with quizzer Q1.
    let q1 = UserBuilder::new_default("Quinn").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let roster = RosterBuilder::new_default("My Roster", coach.id).build_and_insert(&mut conn).unwrap();
    RosterQuizzerBuilder::new(q1.id, roster.rosterid).build_and_insert(&mut conn).unwrap();

    // A tournament the coach owns but, lacking the manager role, its participants should NOT appear.
    let q2 = UserBuilder::new_default("Quincy").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let tournament = TournamentBuilder::new_default("Some Tour").set_owner_id(coach.id).build_and_insert(&mut conn).unwrap();
    let division = DivisionBuilder::new_default("Div A", tournament.tid).build_and_insert(&mut conn).unwrap();
    TeamBuilder::new_default(division.did)
        .set_name("Team A").set_coachid(coach.id).set_quizzer_one_id(q2.id)
        .build_and_insert(&mut conn).unwrap();

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes)
    ).await;

    // Act:

    let uri = format!("/api/users/{}/eligible-quizzers", coach.id);
    let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<User> = test::read_body_json(resp).await;
    let ids: Vec<uuid::Uuid> = body.iter().map(|u| u.id).collect();
    assert!(ids.contains(&q1.id), "roster quizzer should be eligible");
    assert!(!ids.contains(&q2.id), "without the manager role, tournament quizzers must not appear");
}
