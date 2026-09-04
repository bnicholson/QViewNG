mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::team::TeamRow;
use backend::models::user::UserBuilder;
use backend::models::tournament::TournamentBuilder;
use backend::models::division::DivisionBuilder;
use backend::routes::configure_routes;
use backend::services::common::PagedResponse;
use crate::common::{TEST_DB_URL, clean_database};

/// Seeds one tournament → one division → three teams and returns (tid, did).
/// `fixtures::teams::seed_teams` creates: "Team 1" (coach Tiffany), "Come Get Some" (coach
/// Seth), and "Luke Found a Frog" (coach Kimberly).
fn seed(conn: &mut backend::database::Connection) -> (uuid::Uuid, uuid::Uuid) {
    let owner = UserBuilder::new_default("Team Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(conn)
        .unwrap();
    let tournament = TournamentBuilder::new_default("Team Rows Tour")
        .set_owner_id(owner.id)
        .build_and_insert(conn)
        .unwrap();
    let division = DivisionBuilder::new_default("Test Div", tournament.tid)
        .build_and_insert(conn)
        .unwrap();
    fixtures::teams::seed_teams(conn, division.did);
    (tournament.tid, division.did)
}

/// The three team names, ordered the way the endpoint returns them (by name ascending).
fn expected_sorted_team_names() -> Vec<&'static str> {
    vec!["Come Get Some", "Luke Found a Frog", "Team 1"]
}

/// Asserts the full-page response: total count 3, all rows enriched and sorted.
fn assert_full_page(body: &PagedResponse<TeamRow>) {
    assert_eq!(body.count, 3);
    assert_eq!(body.items.len(), 3);
    let names: Vec<&str> = body.items.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, expected_sorted_team_names());

    for row in &body.items {
        // Division and coach names are embedded — no separate lookups needed by the client.
        assert_eq!(row.division_name, "Test Div");
        assert!(row.coach_name.ends_with("Maurice Den"), "coach_name = {}", row.coach_name);
    }
    let cgs = body.items.iter().find(|r| r.name == "Come Get Some").expect("team present");
    assert_eq!(cgs.coach_name, "Seth Maurice Den");
    let luke = body.items.iter().find(|r| r.name == "Luke Found a Frog").expect("team present");
    assert_eq!(luke.coach_name, "Kimberly Maurice Den");
}

#[actix_web::test]
async fn tournament_team_rows_returns_enriched_sorted_paginated_rows() {
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

    // Act — full first page.
    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/team-rows?page=0&page_size=100", tid))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<TeamRow> = test::read_body_json(resp).await;
    assert_full_page(&body);

    // Act — second page (page_size 2, page index 1 → the third team only).
    let req_page = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/team-rows?page=1&page_size=2", tid))
        .to_request();
    let resp_page = test::call_service(&app, req_page).await;

    // Assert — count is still the total, page returns just the remaining team.
    assert_eq!(resp_page.status(), StatusCode::OK);
    let page_body: PagedResponse<TeamRow> = test::read_body_json(resp_page).await;
    assert_eq!(page_body.count, 3);
    let page_names: Vec<&str> = page_body.items.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(page_names, vec!["Team 1"]);
}

#[actix_web::test]
async fn division_team_rows_returns_enriched_sorted_paginated_rows() {
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
        .uri(&format!("/api/divisions/{}/team-rows?page=0&page_size=100", did))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<TeamRow> = test::read_body_json(resp).await;
    assert_full_page(&body);

    // Act — first page (page_size 2, page index 0 → the first two teams).
    let req_page = test::TestRequest::get()
        .uri(&format!("/api/divisions/{}/team-rows?page=0&page_size=2", did))
        .to_request();
    let resp_page = test::call_service(&app, req_page).await;

    // Assert
    assert_eq!(resp_page.status(), StatusCode::OK);
    let page_body: PagedResponse<TeamRow> = test::read_body_json(resp_page).await;
    assert_eq!(page_body.count, 3);
    let page_names: Vec<&str> = page_body.items.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(page_names, vec!["Come Get Some", "Luke Found a Frog"]);
}
