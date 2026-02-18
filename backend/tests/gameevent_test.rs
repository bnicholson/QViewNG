
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::{self, apicalllog::ApiCalllog}};
use backend::models::gameevent::GameEvent;
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let payload = fixtures::gameevents::arrange_create_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = "/api/gameevents";
    let req = test::TestRequest::post()
        .uri(&uri)
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<GameEvent> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let gameevent = body.data.unwrap();
    assert_eq!(gameevent.name, payload.name);
    assert_eq!(gameevent.event, payload.event);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

// #[actix_web::test]
// async fn get_all_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let (gameevent_1, gameevent_2) = fixtures::gameevents::arrange_get_all_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/gameevents?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     // Act:
    
//     let resp = test::call_service(&app, req).await;
    
//     // Assert:
    
//     assert_eq!(resp.status(), StatusCode::OK);

//     let body: Vec<GameEvent> = test::read_body_json(resp).await;

//     let len = 2;

//     assert_eq!(body.len(), len);

//     let mut gameevent_1_interest_idx = 10;
//     let mut gameevent_2_interest_idx = 10;
//     for idx in 0..len {
//         if body[idx].brand == gameevent_1.brand {
//             gameevent_1_interest_idx = idx;
//             continue;
//         }
//         if body[idx].brand == gameevent_2.brand {
//             gameevent_2_interest_idx = idx;
//             continue;
//         }
//     }
//     assert_ne!(gameevent_1_interest_idx, 10);
//     assert_ne!(gameevent_2_interest_idx, 10);

//     // Check that ApiCalllog is recording API calls for this endpoint:
//     let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
//     assert!(apicalllog_get_result.is_ok());
//     let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
//     assert_eq!(apicalllog_records.iter().count(), 1);
//     assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
//     assert_eq!(apicalllog_records.first().unwrap().uri, uri);
// }

// #[actix_web::test]
// async fn get_by_id_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let gameevent = 
//         fixtures::gameevents::arrange_get_gameevent_by_id_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     let uri = format!("/api/gameevents/{}", &gameevent.gameeventid);
//     println!("GameEvents Get by ID URI: {}", &uri);
//     let req = test::TestRequest::get()
//         .uri(uri.as_str())
//         .to_request();

//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:
    
//     let resp_gameevent: GameEvent = test::read_body_json(resp).await;
//     assert_eq!(resp_gameevent.brand, gameevent.brand);
//     assert_eq!(resp_gameevent.operating_system, gameevent.operating_system);

//     // Check that ApiCalllog is recording API calls for this endpoint:
//     let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
//     assert!(apicalllog_get_result.is_ok());
//     let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
//     assert_eq!(apicalllog_records.iter().count(), 1);
//     assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
//     assert_eq!(apicalllog_records.first().unwrap().uri, uri);
// }
