
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::database::Database;
use backend::models::division_session::{DivisionSession, DivisionSessionBuilder};
use backend::models::pool_bracket::{PoolBracketBuilder, PoolBracketRow};
use backend::models::game::GameRow;
use backend::routes::configure_routes;
use backend::services::common::{EntityResponse, PagedResponse};
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database, make_token};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (tournament, division, owner, admin_user, unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    // ── Success: tournament owner with division:create ───────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["division:create".to_string()],
    );

    let owner_req = test::TestRequest::post()
        .uri("/api/divisionsessions")
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(json!({ "did": division.did, "name": "Pool Play" }))
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;
    assert_eq!(owner_resp.status(), StatusCode::CREATED);

    let body: EntityResponse<DivisionSession> = test::read_body_json(owner_resp).await;
    assert_eq!(body.code, 201);
    let session = body.data.unwrap();
    assert_eq!(session.did, division.did);
    assert_eq!(session.name.as_str(), "Pool Play");
    assert_eq!(session.creator_userid, owner.id);
    assert_eq!(session.last_modified_userid, owner.id);

    // ── Success: tournament admin with division:create ───────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:create".to_string()],
    );

    let admin_req = test::TestRequest::post()
        .uri("/api/divisionsessions")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(json!({ "did": division.did, "name": "Bracket Play" }))
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;
    assert_eq!(admin_resp.status(), StatusCode::CREATED);

    // ── Fail: name must be unique within the division ────────────────────────

    let dup_req = test::TestRequest::post()
        .uri("/api/divisionsessions")
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(json!({ "did": division.did, "name": "Pool Play" }))
        .to_request();

    let dup_resp = test::call_service(&app, dup_req).await;
    assert_ne!(dup_resp.status(), StatusCode::CREATED);

    // ── Fail: has division:create but is neither owner nor tournament admin ──

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:create".to_string()],
    );

    let unrelated_req = test::TestRequest::post()
        .uri("/api/divisionsessions")
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(json!({ "did": division.did, "name": "Finals" }))
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;
    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: no division:create permission at all ───────────────────────────

    let no_perm_token = make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["division:read".to_string()],
    );

    let no_perm_req = test::TestRequest::post()
        .uri("/api/divisionsessions")
        .insert_header(("Authorization", format!("Bearer {}", no_perm_token)))
        .set_json(json!({ "did": division.did, "name": "Finals" }))
        .to_request();

    let no_perm_resp = test::call_service(&app, no_perm_req).await;
    assert_eq!(no_perm_resp.status(), StatusCode::UNAUTHORIZED);

    let _ = tournament;
}

#[actix_web::test]
async fn read_and_index_work() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, _admin_user, _unrelated_user) =
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

    // read by id
    let read_req = test::TestRequest::get()
        .uri(&format!("/api/divisionsessions/{}", session.division_session_id))
        .to_request();
    let read_resp = test::call_service(&app, read_req).await;
    assert_eq!(read_resp.status(), StatusCode::OK);
    let read_body: DivisionSession = test::read_body_json(read_resp).await;
    assert_eq!(read_body.division_session_id, session.division_session_id);
    assert_eq!(read_body.name.as_str(), "Pool Play");

    // read missing → 404
    let missing_req = test::TestRequest::get()
        .uri(&format!("/api/divisionsessions/{}", uuid::Uuid::new_v4()))
        .to_request();
    let missing_resp = test::call_service(&app, missing_req).await;
    assert_eq!(missing_resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn get_pool_bracket_rows_of_session_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, _admin_user, _unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);

    // Two sessions in the division; brackets are created in both, but the endpoint must only return
    // the queried session's brackets.
    let session = DivisionSessionBuilder::new(division.did)
        .set_name("Pool Play")
        .set_creator_userid(owner.id)
        .build_and_insert(&mut conn)
        .unwrap();
    let other_session = DivisionSessionBuilder::new(division.did)
        .set_name("Other Session")
        .set_creator_userid(owner.id)
        .build_and_insert(&mut conn)
        .unwrap();

    for name in ["Pool A", "Pool B"] {
        PoolBracketBuilder::new(session.division_session_id)
            .set_name(name).set_type("pool").set_creator_userid(owner.id)
            .build_and_insert(&mut conn).unwrap();
    }
    PoolBracketBuilder::new(session.division_session_id)
        .set_name("Bracket A").set_type("bracket").set_creator_userid(owner.id)
        .build_and_insert(&mut conn).unwrap();
    // A pool in the OTHER session that must be excluded from this session's results.
    PoolBracketBuilder::new(other_session.division_session_id)
        .set_name("Other Pool").set_type("pool").set_creator_userid(owner.id)
        .build_and_insert(&mut conn).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    // ── type=pool returns only this session's two pools ──────────────────────
    let pool_uri = format!("/api/divisionsessions/{}/pool-bracket-rows?type=pool&page={}&page_size={}", session.division_session_id, PAGE_NUM, PAGE_SIZE);
    let pool_resp = test::call_service(&app, test::TestRequest::get().uri(&pool_uri).to_request()).await;
    assert_eq!(pool_resp.status(), StatusCode::OK);
    let pool_body: PagedResponse<PoolBracketRow> = test::read_body_json(pool_resp).await;
    assert_eq!(pool_body.count, 2);
    let pool_names: Vec<&str> = pool_body.items.iter().map(|b| b.name.as_str()).collect();
    assert!(pool_names.contains(&"Pool A"));
    assert!(pool_names.contains(&"Pool B"));
    assert!(!pool_names.contains(&"Other Pool"));
    for row in &pool_body.items {
        assert_eq!(row.session_name, "Pool Play");
        assert_eq!(row.division_session_id, session.division_session_id);
    }

    // ── type=bracket returns only this session's one bracket ─────────────────
    let bracket_uri = format!("/api/divisionsessions/{}/pool-bracket-rows?type=bracket&page={}&page_size={}", session.division_session_id, PAGE_NUM, PAGE_SIZE);
    let bracket_resp = test::call_service(&app, test::TestRequest::get().uri(&bracket_uri).to_request()).await;
    assert_eq!(bracket_resp.status(), StatusCode::OK);
    let bracket_body: PagedResponse<PoolBracketRow> = test::read_body_json(bracket_resp).await;
    assert_eq!(bracket_body.count, 1);
    assert_eq!(bracket_body.items[0].name.as_str(), "Bracket A");
}

#[actix_web::test]
async fn get_game_rows_of_session_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // Seeds a session with one pool bracket holding two games (via poolbracket_id).
    let bracket_id = fixtures::pool_brackets::seed_pool_bracket_profile(&mut conn);
    let session_id = backend::models::pool_bracket::read(&mut conn, bracket_id).unwrap().division_session_id;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/divisionsessions/{}/game-rows?page={}&page_size={}", session_id, PAGE_NUM, PAGE_SIZE);
    let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // The two games belonging to the session's pool bracket.
    let body: PagedResponse<GameRow> = test::read_body_json(resp).await;
    assert_eq!(body.count, 2);
    assert_eq!(body.items.len(), 2);
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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let put_uri = format!("/api/divisionsessions/{}", session.division_session_id);

    // ── Success: owner with division:update ──────────────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["division:update".to_string()],
    );

    let owner_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(json!({ "name": "Bracket Play" }))
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;
    assert_eq!(owner_resp.status(), StatusCode::OK);

    let body: EntityResponse<DivisionSession> = test::read_body_json(owner_resp).await;
    assert_eq!(body.code, 200);
    assert_eq!(body.data.unwrap().name.as_str(), "Bracket Play");

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

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let delete_uri = format!("/api/divisionsessions/{}", session.division_session_id);

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

    // The session is gone.
    assert!(backend::models::division_session::read(&mut conn, session.division_session_id).is_err());
}
