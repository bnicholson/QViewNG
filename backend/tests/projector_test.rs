
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models};
use backend::models::projector::Projector;
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

    let payload = fixtures::projectors::arrange_create_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/equipment/projectors")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Projector> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let projector = body.data.unwrap();
    assert_eq!(projector.brand, payload.brand);  // from ProjectorDbo ("projectors" table)
    assert_eq!(projector.equipmentsetid, payload.equipmentsetid);  // from EquipmentDbo ("equipment" table)
    assert_eq!(projector.misc_note, payload.misc_note);  // from EquipmentDbo ("equipment" table)
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (projector_1, projector_2) = fixtures::projectors::arrange_get_all_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/equipment/projectors?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<Projector> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut projector_1_interest_idx = 10;
    let mut projector_2_interest_idx = 10;
    for idx in 0..len {
        if body[idx].id == projector_1.id {
            projector_1_interest_idx = idx;
            continue;
        }
        if body[idx].id == projector_2.id {
            projector_2_interest_idx = idx;
            continue;
        }
    }
    assert_ne!(projector_1_interest_idx, 10);
    assert_ne!(projector_2_interest_idx, 10);
}

// #[actix_web::test]
// async fn get_by_id_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let projector = 
//         fixtures::projectors::arrange_get_projector_by_id_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     let uri = format!("/api/equipment/projectors/{}", &projector.equipmentid);
//     println!("Projectors Get by ID URI: {}", &uri);
//     let req = test::TestRequest::get()
//         .uri(uri.as_str())
//         .to_request();

//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:
    
//     let resp_projector: Projector = test::read_body_json(resp).await;
//     assert_eq!(resp_projector.id, projector.id);
//     assert_eq!(resp_projector.type_, projector.type_);
// }

// #[actix_web::test]
// async fn update_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let original_projector = 
//         fixtures::projectors::arrange_update_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     let new_type_ = "Built-in".to_string();
//     let new_misc_note = "Ah, something new!".to_string();

//     let put_payload = json!({
//         "type_": &new_type_,
//         "misc_note": &new_misc_note
//     });
    
//     let put_uri = format!("/api/equipment/projectors/{}", original_projector.equipmentid);
//     let put_req = test::TestRequest::put()
//         .uri(&put_uri)
//         .set_json(&put_payload)
//         .to_request();

//     // Act:
    
//     let put_resp = test::call_service(&app, put_req).await;

//     // Assert:
    
//     assert_eq!(put_resp.status(), StatusCode::OK);

//     let put_resp_body: EntityResponse<Projector> = test::read_body_json(put_resp).await;
//     assert_eq!(put_resp_body.code, 200);
//     assert_eq!(put_resp_body.message, "");

//     let new_projector = put_resp_body.data.unwrap();
//     assert_eq!(new_projector.type_, new_type_);
//     assert_eq!(new_projector.misc_note.unwrap(), new_misc_note);
//     assert_ne!(new_projector.created_at, new_projector.updated_at);

//     let new_equipment_dbo = models::equipment_dbo::read(&mut conn, new_projector.equipmentid).unwrap();
//     assert_ne!(new_equipment_dbo.created_at, new_equipment_dbo.updated_at);
// }

// #[actix_web::test]
// async fn delete_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let projector = fixtures::projectors::arrange_delete_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let delete_uri = format!("/api/equipment/projectors/{}", projector.equipmentid);
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


//     let get_by_id_uri = format!("/api/equipment/projectors/{}", projector.equipmentid);
//     let get_by_id_req = test::TestRequest::get()
//         .uri(&get_by_id_uri)
//         .to_request();

//     let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

//     assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
// }
