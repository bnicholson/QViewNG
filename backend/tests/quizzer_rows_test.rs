mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::team::QuizzerRow;
use backend::models::user::UserBuilder;
use backend::models::tournament::TournamentBuilder;
use backend::models::division::DivisionBuilder;
use backend::routes::configure_routes;
use backend::services::common::PagedResponse;
use crate::common::{TEST_DB_URL, clean_database};

/// Seeds one tournament → one division → three teams (with 0, 2, and 6 quizzers = 8 distinct
/// quizzers) and returns (tid, did).
fn seed(conn: &mut backend::database::Connection) -> (uuid::Uuid, uuid::Uuid) {
    let owner = UserBuilder::new_default("Quizzer Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(conn)
        .unwrap();
    let tournament = TournamentBuilder::new_default("Quizzer Rows Tour")
        .set_owner_id(owner.id)
        .build_and_insert(conn)
        .unwrap();
    let division = DivisionBuilder::new_default("Test Div", tournament.tid)
        .build_and_insert(conn)
        .unwrap();
    fixtures::teams::seed_teams(conn, division.did);
    (tournament.tid, division.did)
}

/// The 8 distinct quizzer first names, ordered the way the endpoint returns them (by name).
fn expected_sorted_fnames() -> Vec<&'static str> {
    vec!["David", "John", "Lucas", "Sam", "Taylor", "Tiffany", "Trishell", "Tyler"]
}

#[actix_web::test]
async fn tournament_quizzer_rows_returns_enriched_sorted_paginated_rows() {
    // Arrange
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (tid, _did) = seed(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes),
    )
    .await;

    // Act — request the full first page.
    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/quizzer-rows?page=0&page_size=100", tid))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert — total count and every enriched, sorted row.
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<QuizzerRow> = test::read_body_json(resp).await;
    assert_eq!(body.count, 8);
    assert_eq!(body.items.len(), 8);
    let fnames: Vec<&str> = body.items.iter().map(|r| r.fname.as_str()).collect();
    assert_eq!(fnames, expected_sorted_fnames());
    for row in &body.items {
        assert_eq!(row.divisions.len(), 1);
        assert_eq!(row.divisions[0].name, "Test Div");
        assert!(!row.teams.is_empty());
    }

    // Act — request the middle page (page_size 3, page index 1 → items 3..6).
    let req_page = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/quizzer-rows?page=1&page_size=3", tid))
        .to_request();
    let resp_page = test::call_service(&app, req_page).await;

    // Assert — count is still the full total, but only this page's rows are returned.
    assert_eq!(resp_page.status(), StatusCode::OK);
    let page_body: PagedResponse<QuizzerRow> = test::read_body_json(resp_page).await;
    assert_eq!(page_body.count, 8, "count reflects all matching rows, not just the page");
    let page_fnames: Vec<&str> = page_body.items.iter().map(|r| r.fname.as_str()).collect();
    assert_eq!(page_fnames, vec!["Sam", "Taylor", "Tiffany"]);
}

#[actix_web::test]
async fn division_quizzer_rows_returns_enriched_sorted_paginated_rows() {
    // Arrange
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (_tid, did) = seed(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes),
    )
    .await;

    // Act — full first page.
    let req = test::TestRequest::get()
        .uri(&format!("/api/divisions/{}/quizzer-rows?page=0&page_size=100", did))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<QuizzerRow> = test::read_body_json(resp).await;
    assert_eq!(body.count, 8);
    assert_eq!(body.items.len(), 8);
    let fnames: Vec<&str> = body.items.iter().map(|r| r.fname.as_str()).collect();
    assert_eq!(fnames, expected_sorted_fnames());

    // Act — last page (page_size 3, page index 2 → items 6..8).
    let req_page = test::TestRequest::get()
        .uri(&format!("/api/divisions/{}/quizzer-rows?page=2&page_size=3", did))
        .to_request();
    let resp_page = test::call_service(&app, req_page).await;

    // Assert
    assert_eq!(resp_page.status(), StatusCode::OK);
    let page_body: PagedResponse<QuizzerRow> = test::read_body_json(resp_page).await;
    assert_eq!(page_body.count, 8);
    let page_fnames: Vec<&str> = page_body.items.iter().map(|r| r.fname.as_str()).collect();
    assert_eq!(page_fnames, vec!["Trishell", "Tyler"]);
}
