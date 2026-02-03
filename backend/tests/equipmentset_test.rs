
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::database::Database;
use backend::models::equipmentset::EquipmentSet;
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

    let payload = fixtures::equipmentsets::arrange_create_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/equipmentsets")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<EquipmentSet> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let equipmentset = body.data.unwrap();
    assert_eq!(equipmentset.is_active, payload.is_active);
    assert_eq!(equipmentset.is_default.unwrap(), payload.is_default.unwrap());
    assert_eq!(equipmentset.name, payload.name);
    assert_eq!(equipmentset.description.unwrap(), payload.description.unwrap());
}

// #[actix_web::test]
// async fn get_all_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     fixtures::equipmentsets::arrange_get_all_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/equipmentsets?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     // Act:
    
//     let resp = test::call_service(&app, req).await;
    
//     // Assert:
    
//     assert_eq!(resp.status(), StatusCode::OK);

//     let body: Vec<EquipmentSet> = test::read_body_json(resp).await;

//     let len = 2;

//     assert_eq!(body.len(), len);

//     let mut equipmentset_1_interest_idx = 10;
//     let mut equipmentset_2_interest_idx = 10;
//     for idx in 0..len {
//         if body[idx].name == "Test EquipmentSet 1" {
//             equipmentset_1_interest_idx = idx;
//             continue;
//         }
//         if body[idx].name == "Test EquipmentSet 2" {
//             equipmentset_2_interest_idx = idx;
//             continue;
//         }
//     }
//     assert_ne!(equipmentset_1_interest_idx, 10);
//     assert_ne!(equipmentset_2_interest_idx, 10);
// }

// #[actix_web::test]
// async fn get_by_id_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let equipmentset = 
//         fixtures::equipmentsets::arrange_get_equipmentset_by_id_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     let uri = format!("/api/equipmentsets/{}", &equipmentset.equipmentsetid);
//     println!("EquipmentSets Get by ID URI: {}", &uri);
//     let req = test::TestRequest::get()
//         .uri(uri.as_str())
//         .to_request();

//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:
    
//     let equipmentset: EquipmentSet = test::read_body_json(resp).await;
//     assert_eq!(equipmentset.name.as_str(), "Test EquipmentSet 2");
//     assert_eq!(equipmentset.description.unwrap().as_str(), "This is EquipmentSet 2's description.");
// }

// #[actix_web::test]
// async fn update_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let equipmentset = 
//         fixtures::equipmentsets::arrange_update_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     let new_name = "My NEW name".to_string();
//     let new_description = "NEW description".to_string();

//     let put_payload = json!({
//         "name": &new_name,
//         "description": &new_description,
//     });
    
//     let put_uri = format!("/api/equipmentsets/{}", equipmentset.equipmentsetid);
//     let put_req = test::TestRequest::put()
//         .uri(&put_uri)
//         .set_json(&put_payload)
//         .to_request();

//     // Act:
    
//     let put_resp = test::call_service(&app, put_req).await;

//     // Assert:
    
//     assert_eq!(put_resp.status(), StatusCode::OK);

//     let put_resp_body: EntityResponse<EquipmentSet> = test::read_body_json(put_resp).await;
//     assert_eq!(put_resp_body.code, 200);
//     assert_eq!(put_resp_body.message, "");

//     let new_equipmentset = put_resp_body.data.unwrap();
//     assert_eq!(new_equipmentset.equipmentsetid, equipmentset.equipmentsetid);
//     assert_eq!(new_equipmentset.name.as_str(), new_name);
//     assert_eq!(new_equipmentset.description.as_ref().unwrap().as_str(), new_description);
//     assert_ne!(new_equipmentset.created_at, new_equipmentset.updated_at);
// }

// #[actix_web::test]
// async fn delete_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let equipmentset = fixtures::equipmentsets::arrange_delete_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let delete_uri = format!("/api/equipmentsets/{}", equipmentset.equipmentsetid);
//     let delete_req = test::TestRequest::delete()
//         .uri(&delete_uri)
//         .to_request();

//     // Act:
    
//     let delete_resp = test::call_service(&app, delete_req).await;

//     // Assert:
    
//     assert_eq!(delete_resp.status(), StatusCode::OK);

//     let delete_resp_body_bytes: Bytes = test::read_body(delete_resp).await;
//     let delete_resp_body_string = String::from_utf8(delete_resp_body_bytes.to_vec()).unwrap();
//     assert_eq!(&delete_resp_body_string, "");


//     let get_by_id_uri = format!("/api/equipmentsets/{}", equipmentset.equipmentsetid);
//     let get_by_id_req = test::TestRequest::get()
//         .uri(&get_by_id_uri)
//         .to_request();

//     let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

//     assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
// }
