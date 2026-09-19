
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::database::Database;
use backend::models::roundgroup::{RoundGroup, RoundGroupBuilder};
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
        .uri("/api/roundgroups")
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(json!({ "did": division.did, "name": "Pool Play" }))
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;
    assert_eq!(owner_resp.status(), StatusCode::CREATED);

    let body: EntityResponse<RoundGroup> = test::read_body_json(owner_resp).await;
    assert_eq!(body.code, 201);
    let roundgroup = body.data.unwrap();
    assert_eq!(roundgroup.did, division.did);
    assert_eq!(roundgroup.name.as_str(), "Pool Play");
    assert_eq!(roundgroup.creator_userid, owner.id);
    assert_eq!(roundgroup.last_modified_userid, owner.id);

    // ── Success: tournament admin with division:create ───────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:create".to_string()],
    );

    let admin_req = test::TestRequest::post()
        .uri("/api/roundgroups")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(json!({ "did": division.did, "name": "Bracket Play" }))
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;
    assert_eq!(admin_resp.status(), StatusCode::CREATED);

    // ── Fail: name must be unique within the division ────────────────────────

    let dup_req = test::TestRequest::post()
        .uri("/api/roundgroups")
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
        .uri("/api/roundgroups")
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
        .uri("/api/roundgroups")
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

    let roundgroup = RoundGroupBuilder::new(division.did)
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
        .uri(&format!("/api/roundgroups/{}", roundgroup.roundgroup_id))
        .to_request();
    let read_resp = test::call_service(&app, read_req).await;
    assert_eq!(read_resp.status(), StatusCode::OK);
    let read_body: RoundGroup = test::read_body_json(read_resp).await;
    assert_eq!(read_body.roundgroup_id, roundgroup.roundgroup_id);
    assert_eq!(read_body.name.as_str(), "Pool Play");

    // read missing → 404
    let missing_req = test::TestRequest::get()
        .uri(&format!("/api/roundgroups/{}", uuid::Uuid::new_v4()))
        .to_request();
    let missing_resp = test::call_service(&app, missing_req).await;
    assert_eq!(missing_resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn get_game_rows_of_roundgroup_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // Seeds a division with one roundgroup, one pool bracket, and two games in a round of that roundgroup.
    let bracket_id = fixtures::pool_brackets::seed_pool_bracket_profile(&mut conn);
    let did = backend::models::pool_bracket::read(&mut conn, bracket_id).unwrap().divisionid;
    // Games belong to a roundgroup through their round, so query the division's (sole) roundgroup.
    let roundgroup_id = backend::models::roundgroup::read_all_of_division(&mut conn, did).unwrap()[0].roundgroup_id;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/roundgroups/{}/game-rows?page={}&page_size={}", roundgroup_id, PAGE_NUM, PAGE_SIZE);
    let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // The two games belonging to the roundgroup's pool bracket.
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

    let roundgroup = RoundGroupBuilder::new(division.did)
        .set_name("Pool Play")
        .set_creator_userid(owner.id)
        .build_and_insert(&mut conn)
        .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let put_uri = format!("/api/roundgroups/{}", roundgroup.roundgroup_id);

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

    let body: EntityResponse<RoundGroup> = test::read_body_json(owner_resp).await;
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

    let roundgroup = RoundGroupBuilder::new(division.did)
        .set_name("Pool Play")
        .set_creator_userid(owner.id)
        .build_and_insert(&mut conn)
        .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let delete_uri = format!("/api/roundgroups/{}", roundgroup.roundgroup_id);

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

    // The roundgroup is gone.
    assert!(backend::models::roundgroup::read(&mut conn, roundgroup.roundgroup_id).is_err());
}

#[actix_web::test]
async fn delete_soft_deletes_and_purge_removes() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, _admin_user, unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);

    let roundgroup = RoundGroupBuilder::new(division.did)
        .set_name("Pool Play")
        .set_creator_userid(owner.id)
        .build_and_insert(&mut conn)
        .unwrap();

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes)
    ).await;

    let owner_token = make_token(owner.id, vec!["tournament_manager".to_string()], vec!["division:delete".to_string()]);
    let unrelated_token = make_token(unrelated_user.id, vec!["tournament_manager".to_string()], vec!["division:delete".to_string()]);

    // ── DELETE is a soft delete: the row is hidden from reads but still present. ──
    let del_resp = test::call_service(&app, test::TestRequest::delete()
        .uri(&format!("/api/roundgroups/{}", roundgroup.roundgroup_id))
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .to_request()).await;
    assert_eq!(del_resp.status(), StatusCode::OK);

    assert!(backend::models::roundgroup::read(&mut conn, roundgroup.roundgroup_id).is_err(),
        "soft-deleted roundgroup should be hidden from read");
    let raw = backend::models::roundgroup::read_including_deleted(&mut conn, roundgroup.roundgroup_id).unwrap();
    assert!(raw.del_fl, "row should remain with del_fl = true");

    // It no longer appears in the division's roundgroups list.
    let list_resp = test::call_service(&app, test::TestRequest::get()
        .uri(&format!("/api/divisions/{}/roundgroups", division.did))
        .to_request()).await;
    let list: Vec<RoundGroup> = test::read_body_json(list_resp).await;
    assert!(list.is_empty(), "soft-deleted roundgroup should be excluded from the list");

    // ── Purge: unrelated user is rejected. ──
    let unrelated_purge = test::call_service(&app, test::TestRequest::delete()
        .uri(&format!("/api/roundgroups/{}/purge", roundgroup.roundgroup_id))
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .to_request()).await;
    assert_eq!(unrelated_purge.status(), StatusCode::UNAUTHORIZED);

    // ── Purge permanently removes the (already soft-deleted) row. ──
    let purge_resp = test::call_service(&app, test::TestRequest::delete()
        .uri(&format!("/api/roundgroups/{}/purge", roundgroup.roundgroup_id))
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .to_request()).await;
    assert_eq!(purge_resp.status(), StatusCode::OK);
    assert!(backend::models::roundgroup::read_including_deleted(&mut conn, roundgroup.roundgroup_id).is_err(),
        "purged roundgroup row should be gone");
}

/// DELETE /api/roundgroups/{id} is blocked (409) when the session still has rounds.
#[actix_web::test]
async fn delete_roundgroup_with_rounds_is_blocked() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, _admin_user, _unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);
    let roundgroup = RoundGroupBuilder::new(division.did)
        .set_name("Session 1").set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();
    backend::models::round::RoundBuilder::new_default(roundgroup.roundgroup_id)
        .set_name("Round 1").set_last_modified_user(owner.id).build_and_insert(&mut conn).unwrap();

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    let token = make_token(owner.id, vec!["tournament_manager".to_string()], vec!["division:delete".to_string()]);
    let req = test::TestRequest::delete()
        .uri(&format!("/api/roundgroups/{}", roundgroup.roundgroup_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    assert!(backend::models::roundgroup::read(&mut conn, roundgroup.roundgroup_id).is_ok(), "the session is not deleted");
}
