
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::{self, apicalllog::ApiCalllog}};
use backend::models::equipmentregistration::EquipmentRegistration;
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

    let new_equipment_registration = fixtures::equipmentregistrations::arrange_create_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/equipmentregistrations")
        .set_json(&new_equipment_registration)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<EquipmentRegistration> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let equipmentregistration: EquipmentRegistration = body.data.unwrap();
    assert_eq!(equipmentregistration.status, new_equipment_registration.status);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri.as_str(), "/api/equipmentregistrations");
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (equipmentregistration_1, equipmentregistration_2) = 
        fixtures::equipmentregistrations::arrange_get_all_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/equipmentregistrations?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<EquipmentRegistration> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut equipmentregistration_1_interest_idx = 10;
    let mut equipmentregistration_2_interest_idx = 10;
    for idx in 0..len {
        if body[idx].id == equipmentregistration_1.id {
            equipmentregistration_1_interest_idx = idx;
            continue;
        }
        if body[idx].id == equipmentregistration_2.id {
            equipmentregistration_2_interest_idx = idx;
            continue;
        }
    }
    assert_ne!(equipmentregistration_1_interest_idx, 10);
    assert_ne!(equipmentregistration_2_interest_idx, 10);

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
    
    let equipmentregistration: EquipmentRegistration = 
        fixtures::equipmentregistrations::arrange_get_by_id_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/equipmentregistrations/{}", &equipmentregistration.id);
    println!("EquipmentRegistrations Get by ID URI: {}", &uri);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let body_equipmentregistration: EquipmentRegistration = test::read_body_json(resp).await;
    assert_eq!(body_equipmentregistration.id, equipmentregistration.id);

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
    
    let (equipmentregistration, room) = 
        fixtures::equipmentregistrations::arrange_update_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let new_status = "Prepared for Assignment".to_string();
    let new_room_id = room.roomid;

    let put_payload = json!({
        "status": &new_status,
        "roomid": &new_room_id
    });
    
    let put_uri = format!("/api/equipmentregistrations/{}", equipmentregistration.id);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<EquipmentRegistration> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let new_equipmentregistration = put_resp_body.data.unwrap();
    assert_eq!(new_equipmentregistration.id, equipmentregistration.id);
    assert_eq!(new_equipmentregistration.status, new_status);
    assert_eq!(new_equipmentregistration.roomid.unwrap(), new_room_id);
    assert_ne!(new_equipmentregistration.created_at, new_equipmentregistration.updated_at);
    
    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "PUT");
    assert_eq!(apicalllog_records.first().unwrap().uri, put_uri);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let equipmentregistration = 
        fixtures::equipmentregistrations::arrange_delete_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let delete_uri = format!("/api/equipmentregistrations/{}", equipmentregistration.id);
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


    let get_by_id_uri = format!("/api/equipmentregistrations/{}", equipmentregistration.id);
    let get_by_id_req = test::TestRequest::get()
        .uri(&get_by_id_uri)
        .to_request();

    let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

    assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
    
    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 2);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "DELETE");
    assert_eq!(apicalllog_records.first().unwrap().uri, delete_uri);
}
