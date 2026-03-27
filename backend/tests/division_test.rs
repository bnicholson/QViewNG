
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::{self, apicalllog::ApiCalllog, division::DivisionBuilder, game::Game}, services::common::PagedResponse};
use backend::models::{division::Division,round::Round,team::Team};
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use chrono::{TimeZone, Utc};
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database, make_token};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (tournament, owner, admin_user, unrelated_user) =
        fixtures::divisions::arrange_division_create_works_integration_test(&mut conn);

    let mut payload = fixtures::divisions::get_division_payload(tournament.tid);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    // ── Success: tournament owner with division:create ───────────────────────

    let owner_token = common::make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["division:create".to_string()],
    );

    let owner_req = test::TestRequest::post()
        .uri("/api/divisions")
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&payload)
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;

    assert_eq!(owner_resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Division> = test::read_body_json(owner_resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let division = body.data.unwrap();
    assert_ne!(division.did.to_string().as_str(), "");
    assert_eq!(division.tid, tournament.tid);
    assert_eq!(division.dname.as_str(), "Test Div 3276");
    assert_eq!(division.breadcrumb.as_str(), "/test/post/for/division/1");
    assert_eq!(division.is_public, false);
    assert_eq!(division.shortinfo.as_str(), "Experienced (but still young).");

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri.as_str(), "/api/divisions");

    // ── Success: tournament admin with division:create ───────────────────────

    let admin_token = common::make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:create".to_string()],
    );

    payload = DivisionBuilder::new_default("Test Div 4000", tournament.tid).build().unwrap();
    let admin_req = test::TestRequest::post()
        .uri("/api/divisions")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(&payload)
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;

    assert_eq!(admin_resp.status(), StatusCode::CREATED);

    // ── Fail: has division:create but is neither owner nor tournament admin ──

    let unrelated_token = common::make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:create".to_string()],
    );

    payload = DivisionBuilder::new_default("Test Div 4001", tournament.tid).build().unwrap();
    let unrelated_req = test::TestRequest::post()
        .uri("/api/divisions")
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(&payload)
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;

    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: no division:create permission at all ───────────────────────────

    let no_perm_token = common::make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["division:read".to_string()],
    );

    payload = DivisionBuilder::new_default("Test Div 4002", tournament.tid).build().unwrap();
    let no_perm_req = test::TestRequest::post()
        .uri("/api/divisions")
        .insert_header(("Authorization", format!("Bearer {}", no_perm_token)))
        .set_json(&payload)
        .to_request();

    let no_perm_resp = test::call_service(&app, no_perm_req).await;

    assert_eq!(no_perm_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");

    fixtures::divisions::seed_divisions(&mut conn, parent_tournament.tid);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/divisions?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: PagedResponse<Division> = test::read_body_json(resp).await;

    assert_eq!(body.items.len(), 3);
    assert_eq!(body.count, 3);

    let mut div_1_idx = 10;
    let mut div_2_idx = 10;
    let mut div_3_idx = 10;
    for idx in 0..3 {
        if body.items[idx].dname == "Test Div 3276" {
            div_1_idx = idx;
            continue;
        }
        if body.items[idx].dname == "Test Div 9078" {
            div_2_idx = idx;
            continue;
        }
        if body.items[idx].dname == "Test Div 4611" {
            div_3_idx = idx;
            continue;
        }
    }
    assert_ne!(div_1_idx, 10);
    assert_ne!(div_2_idx, 10);
    assert_ne!(div_3_idx, 10);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}


#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn,"Test Tour");

    let divisions: Vec<Division> = fixtures::divisions::seed_divisions(&mut conn, parent_tournament.tid);
    let division_of_interest_idx = 0;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/divisions/{}", &divisions[division_of_interest_idx].did);
    println!("Divisions Get by ID URI: {}", &uri);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let division: Division = test::read_body_json(resp).await;
    assert_eq!(division.dname, divisions[division_of_interest_idx].dname);
    assert_eq!(division.shortinfo, divisions[division_of_interest_idx].shortinfo);
    assert_eq!(division.breadcrumb, divisions[division_of_interest_idx].breadcrumb);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

#[actix_web::test]
async fn update_works() {

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

    let put_uri = format!("/api/divisions/{}", division.did);

    // ── Success: tournament owner with division:update ────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["division:update".to_string()],
    );

    let owner_payload = json!({
        "dname": "Test Div Owner Update",
        "breadcrumb": "/owner/update",
        "is_public": true
    });
    let owner_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&owner_payload)
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;

    assert_eq!(owner_resp.status(), StatusCode::OK);

    let owner_resp_body: EntityResponse<Division> = test::read_body_json(owner_resp).await;
    assert_eq!(owner_resp_body.code, 200);
    assert_eq!(owner_resp_body.message, "");

    let updated_division = owner_resp_body.data.unwrap();
    assert_eq!(updated_division.tid, tournament.tid);
    assert_eq!(updated_division.did, division.did);
    assert_eq!(updated_division.dname.as_str(), "Test Div Owner Update");
    assert_eq!(updated_division.breadcrumb.as_str(), "/owner/update");
    assert_eq!(updated_division.is_public, true);
    assert_ne!(updated_division.created_at, updated_division.updated_at);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "PUT");
    assert_eq!(apicalllog_records.first().unwrap().uri, put_uri);

    // ── Success: tournament admin with division:update ────────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:update".to_string()],
    );

    let admin_payload = json!({
        "dname": "Test Div Admin Update",
        "breadcrumb": "/admin/update",
        "is_public": false
    });
    let admin_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(&admin_payload)
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;

    assert_eq!(admin_resp.status(), StatusCode::OK);

    // ── Fail: has division:update but is neither owner nor tournament admin ───

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:update".to_string()],
    );

    let unrelated_payload = json!({
        "dname": "Test Div Unrelated Update",
        "breadcrumb": "/unrelated/update",
        "is_public": true
    });
    let unrelated_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(&unrelated_payload)
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;

    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: no division:update permission at all ────────────────────────────

    let no_perm_token = make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["division:read".to_string()],
    );

    let no_perm_payload = json!({
        "dname": "Test Div No Perm Update",
        "breadcrumb": "/noperm/update",
        "is_public": false
    });
    let no_perm_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", no_perm_token)))
        .set_json(&no_perm_payload)
        .to_request();

    let no_perm_resp = test::call_service(&app, no_perm_req).await;

    assert_eq!(no_perm_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division_1, division_2, owner, admin_user, unrelated_user) =
        fixtures::divisions::arrange_division_delete_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let delete_uri_1 = format!("/api/divisions/{}", division_1.did);
    let delete_uri_2 = format!("/api/divisions/{}", division_2.did);

    // ── Fail: has division:delete but is neither owner nor tournament admin ───

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:delete".to_string()],
    );

    let unrelated_req = test::TestRequest::delete()
        .uri(&delete_uri_1)
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;

    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "DELETE");
    assert_eq!(apicalllog_records.first().unwrap().uri, delete_uri_1);

    // ── Fail: no division:delete permission at all ────────────────────────────

    let no_perm_token = make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["division:read".to_string()],
    );

    let no_perm_req = test::TestRequest::delete()
        .uri(&delete_uri_1)
        .insert_header(("Authorization", format!("Bearer {}", no_perm_token)))
        .to_request();

    let no_perm_resp = test::call_service(&app, no_perm_req).await;

    assert_eq!(no_perm_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Success: tournament owner with division:delete ────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["division:delete".to_string()],
    );

    let owner_req = test::TestRequest::delete()
        .uri(&delete_uri_1)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;

    assert_eq!(owner_resp.status(), StatusCode::OK);

    let owner_resp_body_bytes: Bytes = test::read_body(owner_resp).await;
    let owner_resp_body_string = String::from_utf8(owner_resp_body_bytes.to_vec()).unwrap();
    assert_eq!(&owner_resp_body_string, "");

    let get_by_id_req_1 = test::TestRequest::get()
        .uri(&delete_uri_1)
        .to_request();
    let get_by_id_resp_1 = test::call_service(&app, get_by_id_req_1).await;
    assert_eq!(get_by_id_resp_1.status(), StatusCode::NOT_FOUND);

    // ── Success: tournament admin with division:delete ────────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["division:delete".to_string()],
    );

    let admin_req = test::TestRequest::delete()
        .uri(&delete_uri_2)
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;

    assert_eq!(admin_resp.status(), StatusCode::OK);

    let get_by_id_req_2 = test::TestRequest::get()
        .uri(&delete_uri_2)
        .to_request();
    let get_by_id_resp_2 = test::call_service(&app, get_by_id_req_2).await;
    assert_eq!(get_by_id_resp_2.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn get_all_rounds_of_division_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let division = fixtures::divisions::seed_get_rounds_by_division(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/divisions/{}/rounds?page={}&page_size={}", division.did, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Round> = test::read_body_json(resp).await;

    let len = 3;

    assert_eq!(body.len(), len);

    let mut round_1_idx = 10;
    let mut round_2_idx = 10;
    let mut round_3_idx = 10;
    for idx in 0..len {
        if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2061, 5, 23, 00, 00, 0).unwrap() {
            round_1_idx = idx;
        }
        if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2062, 5, 23, 00, 00, 0).unwrap() {
            round_2_idx = idx;
        }
        if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2063, 5, 23, 00, 00, 0).unwrap() {
            round_3_idx = idx;
        }
    }
    assert_ne!(round_1_idx, 10);
    assert_ne!(round_2_idx, 10);
    assert_ne!(round_3_idx, 10);
}

#[actix_web::test]
async fn get_all_teams_of_division_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let team: Team = fixtures::divisions::seed_get_teams_by_division(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/divisions/{}/teams?page={}&page_size={}", team.did, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Team> = test::read_body_json(resp).await;

    let len = 3;

    assert_eq!(body.len(), len);

    let mut round_1_idx = 10;
    let mut round_2_idx = 10;
    let mut round_3_idx = 10;
    for idx in 0..len {
        if body[idx].name == "Jefferons Team".to_string() {
            round_1_idx = idx;
        }
        if body[idx].name == "Andersons Team".to_string() {
            round_2_idx = idx;
        }
        if body[idx].name == "Smiths Team".to_string() {
            round_3_idx = idx;
        }
    }
    assert_ne!(round_1_idx, 10);
    assert_ne!(round_2_idx, 10);
    assert_ne!(round_3_idx, 10);
}

#[actix_web::test]
async fn get_all_games_of_division_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (division_id, game_1_of_div_2, game_2_of_div_2 ) = fixtures::games::seed_get_games_of_division(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/divisions/{}/games?page={}&page_size={}", division_id, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Game> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut game_1_idx = 10;
    let mut game_2_idx = 10;
    for idx in 0..len {
        if body[idx].gid == game_1_of_div_2.gid {
            game_1_idx = idx;
        }
        if body[idx].gid == game_2_of_div_2.gid {
            game_2_idx = idx;
        }
    }
    assert_ne!(game_1_idx, 10);
    assert_ne!(game_2_idx, 10);
}
