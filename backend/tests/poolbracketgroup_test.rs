
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::database::Database;
use backend::models::poolbracketgroup::{PoolBracketGroup, PoolBracketGroupBuilder};
use backend::models::pool_bracket::{PoolBracket, PoolBracketBuilder};
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use serde_json::json;
use crate::common::{TEST_DB_URL, clean_database, make_token};

#[actix_web::test]
async fn create_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, admin_user, unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes)
    ).await;

    // ── Success: tournament owner with division:create ───────────────────────
    let owner_token = make_token(owner.id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);
    let owner_req = test::TestRequest::post()
        .uri("/api/poolbracketgroups")
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(json!({ "divisionid": division.did, "name": "Group 1" }))
        .to_request();
    let owner_resp = test::call_service(&app, owner_req).await;
    assert_eq!(owner_resp.status(), StatusCode::CREATED);
    let body: EntityResponse<PoolBracketGroup> = test::read_body_json(owner_resp).await;
    assert_eq!(body.code, 201);
    let group = body.data.unwrap();
    assert_eq!(group.divisionid, division.did);
    assert_eq!(group.name.as_str(), "Group 1");
    assert_eq!(group.creator_userid, owner.id);

    // ── Success: tournament admin with division:create ───────────────────────
    let admin_token = make_token(admin_user.id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);
    let admin_req = test::TestRequest::post()
        .uri("/api/poolbracketgroups")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(json!({ "divisionid": division.did, "name": "Group 2" }))
        .to_request();
    assert_eq!(test::call_service(&app, admin_req).await.status(), StatusCode::CREATED);

    // ── Fail: name must be unique within the division ────────────────────────
    let dup_req = test::TestRequest::post()
        .uri("/api/poolbracketgroups")
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(json!({ "divisionid": division.did, "name": "Group 1" }))
        .to_request();
    assert_ne!(test::call_service(&app, dup_req).await.status(), StatusCode::CREATED);

    // ── Fail: has division:create but is neither owner nor tournament admin ──
    let unrelated_token = make_token(unrelated_user.id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);
    let unrelated_req = test::TestRequest::post()
        .uri("/api/poolbracketgroups")
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(json!({ "divisionid": division.did, "name": "Group 3" }))
        .to_request();
    assert_eq!(test::call_service(&app, unrelated_req).await.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn read_index_and_list_by_division_work() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, _admin_user, _unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);

    let group = PoolBracketGroupBuilder::new(division.did)
        .set_name("Group 1").set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes)
    ).await;

    // read by id
    let read_resp = test::call_service(&app, test::TestRequest::get()
        .uri(&format!("/api/poolbracketgroups/{}", group.poolbracketgroupid)).to_request()).await;
    assert_eq!(read_resp.status(), StatusCode::OK);
    let read_body: PoolBracketGroup = test::read_body_json(read_resp).await;
    assert_eq!(read_body.poolbracketgroupid, group.poolbracketgroupid);
    assert_eq!(read_body.name.as_str(), "Group 1");

    // read missing → 404
    let missing_resp = test::call_service(&app, test::TestRequest::get()
        .uri(&format!("/api/poolbracketgroups/{}", uuid::Uuid::new_v4())).to_request()).await;
    assert_eq!(missing_resp.status(), StatusCode::NOT_FOUND);

    // list by division
    let list_resp = test::call_service(&app, test::TestRequest::get()
        .uri(&format!("/api/divisions/{}/poolbracketgroups", division.did)).to_request()).await;
    assert_eq!(list_resp.status(), StatusCode::OK);
    let list: Vec<PoolBracketGroup> = test::read_body_json(list_resp).await;
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].poolbracketgroupid, group.poolbracketgroupid);
}

#[actix_web::test]
async fn list_poolbrackets_of_group_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, _admin_user, _unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);

    let group = PoolBracketGroupBuilder::new(division.did)
        .set_name("Group 1").set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();
    let pool = PoolBracketBuilder::new(division.did)
        .set_name("Pool A").set_poolbracketgroupid(group.poolbracketgroupid)
        .set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes)
    ).await;

    let resp = test::call_service(&app, test::TestRequest::get()
        .uri(&format!("/api/poolbracketgroups/{}/poolbrackets", group.poolbracketgroupid)).to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let pools: Vec<PoolBracket> = test::read_body_json(resp).await;
    assert_eq!(pools.len(), 1);
    assert_eq!(pools[0].pool_bracket_id, pool.pool_bracket_id);
    assert_eq!(pools[0].poolbracketgroupid, Some(group.poolbracketgroupid));
}

#[actix_web::test]
async fn update_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, _admin_user, unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);
    let group = PoolBracketGroupBuilder::new(division.did)
        .set_name("Group 1").set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes)
    ).await;
    let put_uri = format!("/api/poolbracketgroups/{}", group.poolbracketgroupid);

    // owner with division:update
    let owner_token = make_token(owner.id, vec!["tournament_manager".to_string()], vec!["division:update".to_string()]);
    let owner_resp = test::call_service(&app, test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(json!({ "name": "Round 2 Groups" })).to_request()).await;
    assert_eq!(owner_resp.status(), StatusCode::OK);
    let body: EntityResponse<PoolBracketGroup> = test::read_body_json(owner_resp).await;
    assert_eq!(body.data.unwrap().name.as_str(), "Round 2 Groups");

    // unrelated user rejected
    let unrelated_token = make_token(unrelated_user.id, vec!["tournament_manager".to_string()], vec!["division:update".to_string()]);
    let unrelated_resp = test::call_service(&app, test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(json!({ "name": "Nope" })).to_request()).await;
    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn delete_works_and_is_blocked_with_pools() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner, _admin_user, _unrelated_user) =
        fixtures::divisions::arrange_division_update_works_integration_test(&mut conn);
    let group = PoolBracketGroupBuilder::new(division.did)
        .set_name("Group 1").set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();
    let pool = PoolBracketBuilder::new(division.did)
        .set_name("Pool A").set_poolbracketgroupid(group.poolbracketgroupid)
        .set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes)
    ).await;
    let del_uri = format!("/api/poolbracketgroups/{}", group.poolbracketgroupid);
    let owner_token = make_token(owner.id, vec!["tournament_manager".to_string()], vec!["division:delete".to_string()]);

    // ── Blocked (409) while the group still has a pool. ──
    let blocked = test::call_service(&app, test::TestRequest::delete()
        .uri(&del_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .to_request()).await;
    assert_eq!(blocked.status(), StatusCode::CONFLICT);
    assert!(backend::models::poolbracketgroup::read(&mut conn, group.poolbracketgroupid).is_ok());

    // Soft-delete the pool, then the group can be deleted.
    backend::models::pool_bracket::delete(&mut conn, pool.pool_bracket_id).unwrap();
    let ok = test::call_service(&app, test::TestRequest::delete()
        .uri(&del_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .to_request()).await;
    assert_eq!(ok.status(), StatusCode::OK);
    assert!(backend::models::poolbracketgroup::read(&mut conn, group.poolbracketgroupid).is_err());
}
