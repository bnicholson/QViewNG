mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::user::UserBuilder;
use backend::models::roster::{UserRosterQuizzerRow, RosterBuilder};
use backend::models::roster_coach::RosterCoachBuilder;
use backend::models::roster_quizzer::RosterQuizzerBuilder;
use backend::routes::configure_routes;
use backend::services::common::PagedResponse;
use crate::common::{TEST_DB_URL, clean_database};

/// Seeds a coach with two rosters they created and three quizzers; one quizzer is on both rosters,
/// so the aggregate must de-duplicate. Additionally, a second coach owns a third roster (with a
/// fourth quizzer, Dana) that is shared with our coach — so the aggregate must include quizzers
/// from rosters shared with the coach, not only ones they created. Returns the coach id.
fn seed(conn: &mut backend::database::Connection) -> uuid::Uuid {
    let coach = UserBuilder::new_default("Cody").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    let anna = UserBuilder::new_default("Anna").set_email("anna@fakeemail.com").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    let bob = UserBuilder::new_default("Bob").set_email("bob@fakeemail.com").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    let cara = UserBuilder::new_default("Cara").set_email("cara@fakeemail.com").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();

    let roster_1 = RosterBuilder::new_default("Roster 1", coach.id).build_and_insert(conn).unwrap();
    let roster_2 = RosterBuilder::new_default("Roster 2", coach.id).build_and_insert(conn).unwrap();

    RosterQuizzerBuilder::new_default(anna.id, roster_1.rosterid).build_and_insert(conn).unwrap();
    RosterQuizzerBuilder::new_default(bob.id, roster_1.rosterid).build_and_insert(conn).unwrap();
    // Bob again on roster 2 (should be de-duplicated), plus Cara.
    RosterQuizzerBuilder::new_default(bob.id, roster_2.rosterid).build_and_insert(conn).unwrap();
    RosterQuizzerBuilder::new_default(cara.id, roster_2.rosterid).build_and_insert(conn).unwrap();

    // A roster owned by another coach, shared with our coach, whose quizzer (Dana) must still
    // appear in our coach's aggregate.
    let other_coach = UserBuilder::new_default("Owen").set_email("owen@fakeemail.com").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    let dana = UserBuilder::new_default("Dana").set_email("dana@fakeemail.com").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    let shared_roster = RosterBuilder::new_default("Shared Roster", other_coach.id).build_and_insert(conn).unwrap();
    RosterQuizzerBuilder::new_default(dana.id, shared_roster.rosterid).build_and_insert(conn).unwrap();
    RosterCoachBuilder::new_default(coach.id, shared_roster.rosterid).build_and_insert(conn).unwrap();

    // A quizzer our coach created but who is on NO roster — must still appear so the coach can get
    // them back (e.g. re-add them to a roster) after removing them from every roster.
    UserBuilder::new_default("Evan").set_email("evan@fakeemail.com").set_hash_password("Pwd123!")
        .set_created_by_userid(coach.id)
        .build_and_insert(conn).unwrap();

    coach.id
}

#[actix_web::test]
async fn user_roster_quizzer_rows_returns_distinct_paginated_quizzers() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let coach_id = seed(&mut conn);

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/users/{}/roster-quizzer-rows?page=0&page_size=100", coach_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<UserRosterQuizzerRow> = test::read_body_json(resp).await;

    // Five distinct quizzers (Bob only once; Dana from the shared roster; Evan created by the coach
    // but on no roster), ordered by (lname, fname) — all share lname "Den".
    assert_eq!(body.count, 5);
    let fnames: Vec<&str> = body.items.iter().map(|q| q.fname.as_str()).collect();
    assert_eq!(fnames, vec!["Anna", "Bob", "Cara", "Dana", "Evan"]);

    // Pagination: first page of size 2.
    let req2 = test::TestRequest::get()
        .uri(&format!("/api/users/{}/roster-quizzer-rows?page=0&page_size=2", coach_id))
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    let body2: PagedResponse<UserRosterQuizzerRow> = test::read_body_json(resp2).await;
    assert_eq!(body2.count, 5);
    let fnames2: Vec<&str> = body2.items.iter().map(|q| q.fname.as_str()).collect();
    assert_eq!(fnames2, vec!["Anna", "Bob"]);
}
