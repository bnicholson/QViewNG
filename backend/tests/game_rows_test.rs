mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::game::{GameRow, PersonGameRow};
use backend::models::user::User;
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

#[actix_web::test]
async fn team_game_rows_returns_only_that_teams_games() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let s = fixtures::games::seed_person_and_team_games(&mut conn);

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes),
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/teams/{}/game-rows?page=0&page_size=100", s.team_1_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<GameRow> = test::read_body_json(resp).await;
    assert_eq!(body.count, 1);
    assert!(body.items.iter().any(|r| r.gid == s.game_1.gid), "the team's game should be present");
    assert!(!body.items.iter().any(|r| r.gid == s.game_2.gid), "another team's game should be absent");
    assert_enriched(&body.items);
}

#[actix_web::test]
async fn person_game_rows_returns_games_for_every_role() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let s = fixtures::games::seed_person_and_team_games(&mut conn);

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes),
    ).await;

    // Each of these people is tied to game_1 through a different role — all should surface it, tagged
    // with the right role (and, for coach/quizzer, the team they're on).
    for (role, uid, expected_role, expects_team) in [
        ("coach", s.coach_id, "Coach", true),
        ("quizzer", s.quizzer_id, "Quizzer", true),
        ("quizmaster", s.quizmaster_id, "Quizmaster", false),
        ("content judge", s.contentjudge_id, "Content Judge", false),
    ] {
        let req = test::TestRequest::get()
            .uri(&format!("/api/tournaments/{}/persons/{}/game-rows?page=0&page_size=100", s.tid, uid))
            .to_request();
        let body: PagedResponse<PersonGameRow> = test::read_body_json(test::call_service(&app, req).await).await;
        assert_eq!(body.count, 1, "{role} should be in exactly one game");
        let rows: Vec<GameRow> = body.items.iter().map(|p| p.row.clone()).collect();
        assert!(rows.iter().any(|r| r.gid == s.game_1.gid), "{role} should see game_1");
        assert!(!rows.iter().any(|r| r.gid == s.game_2.gid), "{role} should not see game_2");
        assert_enriched(&rows);
        let pr = &body.items[0];
        assert_eq!(pr.person_role, expected_role, "{role} should be tagged {expected_role}");
        if expects_team {
            assert_eq!(pr.person_role_team_id, Some(s.team_1_id), "{role}'s role team should be team_1");
            assert!(pr.person_role_team_name.is_some(), "{role}'s role team name should be present");
        } else {
            assert!(pr.person_role_team_id.is_none(), "{role} has no role team");
        }
    }

    // Control: a coach only on a game_2 team sees game_2, never game_1.
    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/persons/{}/game-rows?page=0&page_size=100", s.tid, s.other_coach_id))
        .to_request();
    let body: PagedResponse<PersonGameRow> = test::read_body_json(test::call_service(&app, req).await).await;
    assert!(body.items.iter().any(|r| r.row.gid == s.game_2.gid), "other coach should see game_2");
    assert!(!body.items.iter().any(|r| r.row.gid == s.game_1.gid), "other coach should not see game_1");
}

#[actix_web::test]
async fn tournament_persons_lists_people_of_every_role() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let s = fixtures::games::seed_person_and_team_games(&mut conn);

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes),
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/persons", s.tid))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<User> = test::read_body_json(resp).await;
    let ids: Vec<uuid::Uuid> = body.items.iter().map(|u| u.id).collect();
    for (role, uid) in [
        ("coach", s.coach_id),
        ("quizzer", s.quizzer_id),
        ("quizmaster", s.quizmaster_id),
        ("content judge", s.contentjudge_id),
    ] {
        assert!(ids.contains(&uid), "{role} should be listed as a person of the tournament");
    }
    assert_eq!(body.count as usize, body.items.len());
}

#[actix_web::test]
async fn tournament_persons_can_be_filtered_by_role() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let s = fixtures::games::seed_person_and_team_games(&mut conn);

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes),
    ).await;

    // For each role, that role's person is listed and people of the other roles are not.
    for (role, included, excluded) in [
        ("quizzer", s.quizzer_id, [s.coach_id, s.quizmaster_id, s.contentjudge_id]),
        ("coach", s.coach_id, [s.quizzer_id, s.quizmaster_id, s.contentjudge_id]),
        ("quizmaster", s.quizmaster_id, [s.quizzer_id, s.coach_id, s.contentjudge_id]),
        ("content_judge", s.contentjudge_id, [s.quizzer_id, s.coach_id, s.quizmaster_id]),
    ] {
        let req = test::TestRequest::get()
            .uri(&format!("/api/tournaments/{}/persons?role={}", s.tid, role))
            .to_request();
        let body: PagedResponse<User> = test::read_body_json(test::call_service(&app, req).await).await;
        let ids: Vec<uuid::Uuid> = body.items.iter().map(|u| u.id).collect();
        assert!(ids.contains(&included), "role={role} should include its own person");
        for other in excluded {
            assert!(!ids.contains(&other), "role={role} should exclude other-role people");
        }
    }
}
