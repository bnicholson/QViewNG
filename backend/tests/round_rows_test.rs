mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use chrono::{TimeZone, Utc};
use backend::database::Database;
use backend::models::round::{RoundRow, RoundBuilder};
use backend::models::user::UserBuilder;
use backend::models::tournament::TournamentBuilder;
use backend::models::division::DivisionBuilder;
use backend::routes::configure_routes;
use backend::services::common::PagedResponse;
use crate::common::{TEST_DB_URL, clean_database};

/// Seeds one tournament → one division → three rounds with distinct scheduled start times, so
/// the endpoint's ordering (by scheduled_start_time, not name) is observable. Returns (tid, did).
fn seed(conn: &mut backend::database::Connection) -> (uuid::Uuid, uuid::Uuid) {
    let owner = UserBuilder::new_default("Round Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(conn)
        .unwrap();
    let tournament = TournamentBuilder::new_default("Round Rows Tour")
        .set_owner_id(owner.id)
        .build_and_insert(conn)
        .unwrap();
    let division = DivisionBuilder::new_default("Test Div", tournament.tid)
        .build_and_insert(conn)
        .unwrap();

    // Insert out of chronological order to prove the endpoint sorts by start time.
    RoundBuilder::new_default(division.did)
        .set_name("Round A")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 1, 1, 0, 0, 0).unwrap())
        .build_and_insert(conn)
        .unwrap();
    RoundBuilder::new_default(division.did)
        .set_name("Round B")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2060, 1, 1, 0, 0, 0).unwrap())
        .build_and_insert(conn)
        .unwrap();
    RoundBuilder::new_default(division.did)
        .set_name("Round C")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap())
        .build_and_insert(conn)
        .unwrap();

    (tournament.tid, division.did)
}

/// Round names ordered by scheduled_start_time ascending: C (2050), A (2055), B (2060).
fn expected_by_start_time() -> Vec<&'static str> {
    vec!["Round C", "Round A", "Round B"]
}

fn assert_full_page(body: &PagedResponse<RoundRow>) {
    assert_eq!(body.count, 3);
    assert_eq!(body.items.len(), 3);
    let names: Vec<&str> = body.items.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, expected_by_start_time());
    for row in &body.items {
        assert_eq!(row.division_name, "Test Div");
        assert!(row.scheduled_start_time.is_some());
    }
}

#[actix_web::test]
async fn tournament_round_rows_returns_enriched_sorted_paginated_rows() {
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
        .uri(&format!("/api/tournaments/{}/round-rows?page=0&page_size=100", tid))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<RoundRow> = test::read_body_json(resp).await;
    assert_full_page(&body);

    // Act — second page (page_size 2, page index 1 → the last round only).
    let req_page = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/round-rows?page=1&page_size=2", tid))
        .to_request();
    let resp_page = test::call_service(&app, req_page).await;

    // Assert — count is still the total, page returns just the remaining round.
    assert_eq!(resp_page.status(), StatusCode::OK);
    let page_body: PagedResponse<RoundRow> = test::read_body_json(resp_page).await;
    assert_eq!(page_body.count, 3);
    let page_names: Vec<&str> = page_body.items.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(page_names, vec!["Round B"]);
}

#[actix_web::test]
async fn division_round_rows_returns_enriched_sorted_paginated_rows() {
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
        .uri(&format!("/api/divisions/{}/round-rows?page=0&page_size=100", did))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<RoundRow> = test::read_body_json(resp).await;
    assert_full_page(&body);

    // Act — first page (page_size 2, page index 0 → the first two rounds by start time).
    let req_page = test::TestRequest::get()
        .uri(&format!("/api/divisions/{}/round-rows?page=0&page_size=2", did))
        .to_request();
    let resp_page = test::call_service(&app, req_page).await;

    // Assert
    assert_eq!(resp_page.status(), StatusCode::OK);
    let page_body: PagedResponse<RoundRow> = test::read_body_json(resp_page).await;
    assert_eq!(page_body.count, 3);
    let page_names: Vec<&str> = page_body.items.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(page_names, vec!["Round C", "Round A"]);
}
