
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::database::Database;
use backend::models::division_session::DivisionSessionBuilder;
use backend::models::pool_bracket::{PoolBracket, PoolBracketBuilder};
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use serde_json::json;
use crate::common::{TEST_DB_URL, clean_database, make_token};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, admin_user, unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);

    let session = DivisionSessionBuilder::new(division.did)
        .set_name("Pool Play")
        .set_creator_userid(owner.id)
        .build_and_insert(&mut conn)
        .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    // ── Success: owner with division:create ──────────────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["division:create".to_string()],
    );

    let owner_req = test::TestRequest::post()
        .uri("/api/poolbrackets")
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(json!({ "division_session_id": session.division_session_id, "name": "Pool A", "type": "pool" }))
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;
    assert_eq!(owner_resp.status(), StatusCode::CREATED);

    let body: EntityResponse<PoolBracket> = test::read_body_json(owner_resp).await;
    assert_eq!(body.code, 201);
    let bracket = body.data.unwrap();
    assert_eq!(bracket.division_session_id, session.division_session_id);
    assert_eq!(bracket.name.as_str(), "Pool A");
    assert_eq!(bracket.type_.as_str(), "pool");
    assert_eq!(bracket.creator_userid, owner.id);

    // ── Success: admin with division:create, "bracket" type ──────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:create".to_string()],
    );

    let admin_req = test::TestRequest::post()
        .uri("/api/poolbrackets")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(json!({ "division_session_id": session.division_session_id, "name": "Bracket A", "type": "bracket" }))
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;
    assert_eq!(admin_resp.status(), StatusCode::CREATED);

    // ── Fail: name must be unique within the session ─────────────────────────

    let dup_req = test::TestRequest::post()
        .uri("/api/poolbrackets")
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(json!({ "division_session_id": session.division_session_id, "name": "Pool A", "type": "pool" }))
        .to_request();

    let dup_resp = test::call_service(&app, dup_req).await;
    assert_ne!(dup_resp.status(), StatusCode::CREATED);

    // ── Fail: unrelated user with division:create ────────────────────────────

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:create".to_string()],
    );

    let unrelated_req = test::TestRequest::post()
        .uri("/api/poolbrackets")
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(json!({ "division_session_id": session.division_session_id, "name": "Pool C", "type": "pool" }))
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;
    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, _admin_user, unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);

    let session = DivisionSessionBuilder::new(division.did)
        .set_name("Pool Play")
        .set_creator_userid(owner.id)
        .build_and_insert(&mut conn)
        .unwrap();
    let bracket = PoolBracketBuilder::new(session.division_session_id)
        .set_name("Pool A")
        .set_type("pool")
        .set_creator_userid(owner.id)
        .build_and_insert(&mut conn)
        .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let put_uri = format!("/api/poolbrackets/{}", bracket.pool_bracket_id);

    // ── Success: owner with division:update ──────────────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["division:update".to_string()],
    );

    let owner_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(json!({ "name": "Pool B" }))
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;
    assert_eq!(owner_resp.status(), StatusCode::OK);

    let body: EntityResponse<PoolBracket> = test::read_body_json(owner_resp).await;
    assert_eq!(body.code, 200);
    assert_eq!(body.data.unwrap().name.as_str(), "Pool B");

    // ── Fail: unrelated user with division:update ────────────────────────────

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:update".to_string()],
    );

    let unrelated_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(json!({ "name": "Nope" }))
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;
    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, _admin_user, unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);

    let session = DivisionSessionBuilder::new(division.did)
        .set_name("Pool Play")
        .set_creator_userid(owner.id)
        .build_and_insert(&mut conn)
        .unwrap();
    let bracket = PoolBracketBuilder::new(session.division_session_id)
        .set_name("Pool A")
        .set_type("pool")
        .set_creator_userid(owner.id)
        .build_and_insert(&mut conn)
        .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let delete_uri = format!("/api/poolbrackets/{}", bracket.pool_bracket_id);

    // ── Fail: unrelated user with division:delete ────────────────────────────

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:delete".to_string()],
    );

    let unrelated_req = test::TestRequest::delete()
        .uri(&delete_uri)
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;
    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Success: owner with division:delete ──────────────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["division:delete".to_string()],
    );

    let owner_req = test::TestRequest::delete()
        .uri(&delete_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;
    assert_eq!(owner_resp.status(), StatusCode::OK);

    // The bracket is gone.
    assert!(backend::models::pool_bracket::read(&mut conn, bracket.pool_bracket_id).is_err());
}
