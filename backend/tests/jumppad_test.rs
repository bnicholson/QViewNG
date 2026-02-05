
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models};
use backend::models::jumppad::JumpPad;
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

    let payload = fixtures::jumppads::arrange_create_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/equipment/jumppads")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<JumpPad> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let jumppad = body.data.unwrap();
    assert_eq!(jumppad.color, payload.color);  // from JumpPadDbo ("jumppads" table)
    assert_eq!(jumppad.equipmentsetid, payload.equipmentsetid);  // from EquipmentDbo ("equipment" table)
    assert_eq!(jumppad.misc_note, payload.misc_note);  // from EquipmentDbo ("equipment" table)
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (jumppad_1, jumppad_2) = fixtures::jumppads::arrange_get_all_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/equipment/jumppads?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<JumpPad> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut jumppad_1_interest_idx = 10;
    let mut jumppad_2_interest_idx = 10;
    for idx in 0..len {
        if body[idx].color == jumppad_1.color {
            jumppad_1_interest_idx = idx;
            continue;
        }
        if body[idx].color == jumppad_2.color {
            jumppad_2_interest_idx = idx;
            continue;
        }
    }
    assert_ne!(jumppad_1_interest_idx, 10);
    assert_ne!(jumppad_2_interest_idx, 10);
}

#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let jumppad = 
        fixtures::jumppads::arrange_get_jumppad_by_id_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/equipment/jumppads/{}", &jumppad.equipmentid);
    println!("JumpPads Get by ID URI: {}", &uri);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let resp_jumppad: JumpPad = test::read_body_json(resp).await;
    assert_eq!(resp_jumppad.color, jumppad.color);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let original_jumppad = 
        fixtures::jumppads::arrange_update_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let new_color = "turquoise".to_string();
    let new_misc_note = "Ah, yes".to_string();

    let put_payload = json!({
        "color": &new_color,
        "misc_note": &new_misc_note
    });
    
    let put_uri = format!("/api/equipment/jumppads/{}", original_jumppad.equipmentid);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<JumpPad> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let new_jumppad = put_resp_body.data.unwrap();
    assert_eq!(new_jumppad.jumppadid, original_jumppad.jumppadid);
    assert_eq!(new_jumppad.color, new_color);
    assert_eq!(new_jumppad.misc_note.unwrap(), new_misc_note);
    assert_ne!(new_jumppad.created_at, new_jumppad.updated_at);

    let new_equipment_dbo = models::equipment_dbo::read(&mut conn, new_jumppad.equipmentid).unwrap();
    assert_ne!(new_equipment_dbo.created_at, new_equipment_dbo.updated_at);
}

// #[actix_web::test]
// async fn delete_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let jumppad = fixtures::jumppads::arrange_delete_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let delete_uri = format!("/api/equipment/jumppads/{}", jumppad.equipmentid);
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


//     let get_by_id_uri = format!("/api/equipment/jumppads/{}", jumppad.equipmentid);
//     let get_by_id_req = test::TestRequest::get()
//         .uri(&get_by_id_uri)
//         .to_request();

//     let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

//     assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
// }
