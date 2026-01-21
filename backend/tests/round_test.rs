
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::database::Database;
use backend::models::round::Round;
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use chrono::{NaiveDate, TimeZone, Utc};
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let tournament = fixtures::tournaments::seed_tournament(&mut conn);
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    let payload = fixtures::rounds::get_round_payload(division.did);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/rounds")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Round> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let round = body.data.unwrap();
    assert_eq!(round.did, division.did);
    assert_eq!(round.scheduled_start_time.unwrap(), Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap());
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn);
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

    let body: Vec<Round> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 3);

    let mut round_or_interest_idx = 10;
    for idx in 0..3 {
        if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2045, 5, 23, 00, 00, 0).unwrap() {
            round_or_interest_idx = idx;
            break;
        }
    }
    assert_ne!(round_or_interest_idx, 10);
}

#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn);
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
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn);
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    let round: Round = fixtures::rounds::seed_round(&mut conn, division.did);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let new_scheduled_start_time = Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap();

    let put_payload = json!({
        "scheduled_start_time": &new_scheduled_start_time
    });
    
    let put_uri = format!("/api/rounds/{}", round.roundid);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<Round> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let new_round = put_resp_body.data.unwrap();
    assert_eq!(new_round.did, division.did);
    assert_eq!(new_round.roundid, round.roundid);
    assert_eq!(new_round.scheduled_start_time.unwrap(), new_scheduled_start_time);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn);
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    let round: Round = fixtures::rounds::seed_round(&mut conn, division.did);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let delete_uri = format!("/api/rounds/{}", round.roundid);
    let delete_req = test::TestRequest::delete()
        .uri(&delete_uri)
        .to_request();

    // Act:
    
    let delete_resp = test::call_service(&app, delete_req).await;

    // Assert:
    
    assert_eq!(delete_resp.status(), StatusCode::OK);

    let delete_resp_body_bytes: Bytes = test::read_body(delete_resp).await;
    let delete_resp_body_string = String::from_utf8(delete_resp_body_bytes.to_vec()).unwrap();
    assert_eq!(&delete_resp_body_string, "");


    let get_by_id_uri = format!("/api/rounds/{}", round.roundid);
    let get_by_id_req = test::TestRequest::get()
        .uri(&get_by_id_uri)
        .to_request();

    let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

    assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
}
