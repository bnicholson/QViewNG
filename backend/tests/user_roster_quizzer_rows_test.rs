mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::user::UserBuilder;
use backend::models::roster::{UserRosterQuizzerRow, RosterBuilder};
use backend::models::roster_quizzer::RosterQuizzerBuilder;
use backend::routes::configure_routes;
use backend::services::common::PagedResponse;
use crate::common::{TEST_DB_URL, clean_database};

/// Seeds a coach with two rosters and three quizzers; one quizzer is on both rosters, so the
/// aggregate must de-duplicate. Returns the coach id.
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

    // Three distinct quizzers (Bob only once), ordered by (lname, fname) — all share lname "Den".
    assert_eq!(body.count, 3);
    let fnames: Vec<&str> = body.items.iter().map(|q| q.fname.as_str()).collect();
    assert_eq!(fnames, vec!["Anna", "Bob", "Cara"]);

    // Pagination: first page of size 2.
    let req2 = test::TestRequest::get()
        .uri(&format!("/api/users/{}/roster-quizzer-rows?page=0&page_size=2", coach_id))
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    let body2: PagedResponse<UserRosterQuizzerRow> = test::read_body_json(resp2).await;
    assert_eq!(body2.count, 3);
    let fnames2: Vec<&str> = body2.items.iter().map(|q| q.fname.as_str()).collect();
    assert_eq!(fnames2, vec!["Anna", "Bob"]);
}
