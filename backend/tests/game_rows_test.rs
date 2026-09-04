mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::game::GameRow;
use backend::routes::configure_routes;
use backend::services::common::PagedResponse;
use crate::common::{TEST_DB_URL, clean_database};

/// Asserts a page of game rows is enriched: names and the room sequence number are present.
fn assert_enriched(rows: &[GameRow]) {
    for row in rows {
        assert!(!row.division_name.is_empty(), "division_name should be embedded");
        assert!(!row.room_name.is_empty(), "room_name should be embedded");
        assert!(!row.left_team_name.is_empty(), "left_team_name should be embedded");
        assert!(!row.right_team_name.is_empty(), "right_team_name should be embedded");
        assert!(row.round_number.is_some(), "each game has a room sequence number");
    }
}

fn contains_both(rows: &[GameRow], gid_1: uuid::Uuid, gid_2: uuid::Uuid) {
    assert!(rows.iter().any(|r| r.gid == gid_1), "expected game 1 in the rows");
    assert!(rows.iter().any(|r| r.gid == gid_2), "expected game 2 in the rows");
}

#[actix_web::test]
async fn tournament_game_rows_returns_enriched_paginated_rows() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (tid, g1, g2) = fixtures::games::seed_get_games_of_tournament(&mut conn);

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes),
    ).await;

    // Full page.
    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/game-rows?page=0&page_size=100", tid))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<GameRow> = test::read_body_json(resp).await;
    assert_eq!(body.count, 2);
    assert_eq!(body.items.len(), 2);
    contains_both(&body.items, g1.gid, g2.gid);
    assert_enriched(&body.items);

    // Paginated: one row per page, count still reflects the total.
    let req_page = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/game-rows?page=0&page_size=1", tid))
        .to_request();
    let page_body: PagedResponse<GameRow> = test::read_body_json(test::call_service(&app, req_page).await).await;
    assert_eq!(page_body.count, 2);
    assert_eq!(page_body.items.len(), 1);
}

#[actix_web::test]
async fn division_game_rows_returns_enriched_paginated_rows() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (division_id, g1, g2) = fixtures::games::seed_get_games_of_division(&mut conn);

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes),
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/divisions/{}/game-rows?page=0&page_size=100", division_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<GameRow> = test::read_body_json(resp).await;
    assert_eq!(body.count, 2);
    assert_eq!(body.items.len(), 2);
    contains_both(&body.items, g1.gid, g2.gid);
    assert_enriched(&body.items);

    let req_page = test::TestRequest::get()
        .uri(&format!("/api/divisions/{}/game-rows?page=0&page_size=1", division_id))
        .to_request();
    let page_body: PagedResponse<GameRow> = test::read_body_json(test::call_service(&app, req_page).await).await;
    assert_eq!(page_body.count, 2);
    assert_eq!(page_body.items.len(), 1);
}

#[actix_web::test]
async fn round_game_rows_returns_enriched_paginated_rows() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (g1, g2) = fixtures::games::seed_get_games_of_round(&mut conn);

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes),
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/rounds/{}/game-rows?page=0&page_size=100", g1.roundid))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<GameRow> = test::read_body_json(resp).await;
    assert_eq!(body.count, 2);
    assert_eq!(body.items.len(), 2);
    contains_both(&body.items, g1.gid, g2.gid);
    assert_enriched(&body.items);

    let req_page = test::TestRequest::get()
        .uri(&format!("/api/rounds/{}/game-rows?page=0&page_size=1", g1.roundid))
        .to_request();
    let page_body: PagedResponse<GameRow> = test::read_body_json(test::call_service(&app, req_page).await).await;
    assert_eq!(page_body.count, 2);
    assert_eq!(page_body.items.len(), 1);
}

#[actix_web::test]
async fn room_game_rows_returns_enriched_paginated_rows() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (g1, g2) = fixtures::games::seed_get_games_of_room(&mut conn);

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes),
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/rooms/{}/game-rows?page=0&page_size=100", g1.roomid))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<GameRow> = test::read_body_json(resp).await;
    assert_eq!(body.count, 2);
    assert_eq!(body.items.len(), 2);
    contains_both(&body.items, g1.gid, g2.gid);
    assert_enriched(&body.items);

    let req_page = test::TestRequest::get()
        .uri(&format!("/api/rooms/{}/game-rows?page=0&page_size=1", g1.roomid))
        .to_request();
    let page_body: PagedResponse<GameRow> = test::read_body_json(test::call_service(&app, req_page).await).await;
    assert_eq!(page_body.count, 2);
    assert_eq!(page_body.items.len(), 1);
}
