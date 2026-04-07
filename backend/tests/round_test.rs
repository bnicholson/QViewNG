
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::{self, apicalllog::ApiCalllog, game::Game, round::RoundBuilder}, services::common::PagedResponse};
use backend::models::round::Round;
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

    let (_, division, owner, admin_user, unrelated_user) =
        fixtures::rounds::arrange_round_create_works_integration_test(&mut conn);

    let mut payload = fixtures::rounds::get_round_payload(division.did);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    // ── Success: tournament owner with round:create ──────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["round:create".to_string()],
    );

    let uri = "/api/rounds";
    let owner_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&payload)
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;

    assert_eq!(owner_resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Round> = test::read_body_json(owner_resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let round = body.data.unwrap();
    assert_eq!(round.did, division.did);
    assert_eq!(round.scheduled_start_time.unwrap(), Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap());

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);

    // ── Success: tournament admin with round:create ──────────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["round:create".to_string()],
    );

    payload = RoundBuilder::new_default(division.did)
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2056, 5, 23, 00, 00, 0).unwrap())
        .build()
        .unwrap();
    let admin_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(&payload)
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;

    assert_eq!(admin_resp.status(), StatusCode::CREATED);

    // ── Fail: has round:create but is neither owner nor tournament admin ─────

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["round:create".to_string()],
    );

    payload = RoundBuilder::new_default(division.did)
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2057, 5, 23, 00, 00, 0).unwrap())
        .build()
        .unwrap();
    let unrelated_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(&payload)
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;

    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: no round:create permission at all ──────────────────────────────

    let no_perm_token = make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["round:read".to_string()],
    );

    payload = RoundBuilder::new_default(division.did)
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2058, 5, 23, 00, 00, 0).unwrap())
        .build()
        .unwrap();
    let no_perm_req = test::TestRequest::post()
        .uri(uri)
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
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn,"Test Tour");
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    fixtures::rounds::seed_rounds(&mut conn, division.did);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/rounds?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::OK);

    let body: PagedResponse<Round> = test::read_body_json(resp).await;

    assert_eq!(body.items.len(), 3);
    assert_eq!(body.count, 3);

    let mut round_or_interest_idx = 10;
    for idx in 0..3 {
        if body.items[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2045, 5, 23, 00, 00, 0).unwrap() {
            round_or_interest_idx = idx;
            break;
        }
    }
    assert_ne!(round_or_interest_idx, 10);
    
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
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn,"Test Tour");
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    let rounds: Vec<Round> = fixtures::rounds::seed_rounds(&mut conn, division.did);
    let round_of_interest_idx = 0;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/rounds/{}", &rounds[round_of_interest_idx].roundid);
    println!("Rounds Get by ID URI: {}", &uri);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let round: Round = test::read_body_json(resp).await;
    assert_eq!(round.did, division.did);
    assert_eq!(round.scheduled_start_time.unwrap(), Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap());
    
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

    let (_, division, round, owner, admin_user, unrelated_user) =
        fixtures::rounds::arrange_round_update_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let put_uri = format!("/api/rounds/{}", round.roundid);

    // ── Success: tournament owner with round:update ───────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["round:update".to_string()],
    );

    let owner_payload = json!({
        "scheduled_start_time": Utc.with_ymd_and_hms(2055, 5, 23, 0, 0, 0).unwrap()
    });
    let owner_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&owner_payload)
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;

    assert_eq!(owner_resp.status(), StatusCode::OK);

    let owner_resp_body: EntityResponse<Round> = test::read_body_json(owner_resp).await;
    assert_eq!(owner_resp_body.code, 200);
    assert_eq!(owner_resp_body.message, "");

    let updated_round = owner_resp_body.data.unwrap();
    assert_eq!(updated_round.did, division.did);
    assert_eq!(updated_round.roundid, round.roundid);
    assert_eq!(updated_round.scheduled_start_time.unwrap(), Utc.with_ymd_and_hms(2055, 5, 23, 0, 0, 0).unwrap());
    assert_ne!(updated_round.created_at, updated_round.updated_at);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "PUT");
    assert_eq!(apicalllog_records.first().unwrap().uri, put_uri);

    // ── Success: tournament admin with round:update ───────────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["round:update".to_string()],
    );

    let admin_payload = json!({
        "scheduled_start_time": Utc.with_ymd_and_hms(2056, 6, 15, 0, 0, 0).unwrap()
    });
    let admin_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(&admin_payload)
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;

    assert_eq!(admin_resp.status(), StatusCode::OK);

    // ── Fail: has round:update but is neither owner nor tournament admin ──────

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["round:update".to_string()],
    );

    let unrelated_payload = json!({
        "scheduled_start_time": Utc.with_ymd_and_hms(2057, 7, 20, 0, 0, 0).unwrap()
    });
    let unrelated_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(&unrelated_payload)
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;

    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: no round:update permission at all ───────────────────────────────

    let no_perm_token = make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["round:read".to_string()],
    );

    let no_perm_payload = json!({
        "scheduled_start_time": Utc.with_ymd_and_hms(2058, 8, 25, 0, 0, 0).unwrap()
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

    let (_tournament, _division, round_1, round_2, owner, admin_user, unrelated_user) =
        fixtures::rounds::arrange_round_delete_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let delete_uri_1 = format!("/api/rounds/{}", round_1.roundid);
    let delete_uri_2 = format!("/api/rounds/{}", round_2.roundid);

    // ── Fail: has round:delete but is neither owner nor tournament admin ──────

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["round:delete".to_string()],
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

    // ── Fail: no round:delete permission at all ───────────────────────────────

    let no_perm_token = make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["round:read".to_string()],
    );

    let no_perm_req = test::TestRequest::delete()
        .uri(&delete_uri_1)
        .insert_header(("Authorization", format!("Bearer {}", no_perm_token)))
        .to_request();

    let no_perm_resp = test::call_service(&app, no_perm_req).await;

    assert_eq!(no_perm_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Success: tournament owner with round:delete ───────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["round:delete".to_string()],
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

    // ── Success: tournament admin with round:delete ───────────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["round:delete".to_string()],
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
async fn get_all_games_of_round_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (game_1_of_round_2, game_2_of_round_2 ) = fixtures::games::seed_get_games_of_round(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/rounds/{}/games?page={}&page_size={}", game_1_of_round_2.roundid, PAGE_NUM, PAGE_SIZE);
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
        if body[idx].gid == game_1_of_round_2.gid {
            game_1_idx = idx;
        }
        if body[idx].gid == game_2_of_round_2.gid {
            game_2_idx = idx;
        }
    }
    assert_ne!(game_1_idx, 10);
    assert_ne!(game_2_idx, 10);
    
    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}
