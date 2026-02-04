
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::{self, common::PaginationParams, equipment_dbo::EquipmentDbo}};
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

    let payload = fixtures::equipment_dbos::arrange_create_works_integration_test(&mut conn);

    // Act:

    let equipment_dbo_result = models::equipment_dbo::create(&mut conn, &payload);    
    
    // Assert:

    // if equipment_dbo_result.is_err() {
    //     println!("Error creating EquipmentDbo: {:?}", equipment_dbo_result.as_ref().unwrap_err());
    //     assert!(false);
    // }
    
    assert!(equipment_dbo_result.is_ok());

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();
    assert_eq!(equipment_dbo.computerid, payload.computerid);
    assert_eq!(equipment_dbo.jumppadid, payload.jumppadid);
    assert_eq!(equipment_dbo.interfaceboxid, payload.interfaceboxid);
    assert_eq!(equipment_dbo.monitorid, payload.monitorid);
    assert_eq!(equipment_dbo.microphonerecorderid, payload.microphonerecorderid);
    assert_eq!(equipment_dbo.projectorid, payload.projectorid);
    assert_eq!(equipment_dbo.powerstripid, payload.powerstripid);
    assert_eq!(equipment_dbo.extensioncordid, payload.extensioncordid);
    assert_eq!(equipment_dbo.misc_note, payload.misc_note);
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (equipment_computer_1, equipment_computer_2) = 
        fixtures::equipment_dbos::arrange_get_all_works_integration_test(&mut conn);

    let pagination = PaginationParams {
        page: 0,
        page_size: PAGE_SIZE,
    };
    
    // Act:
    
    let equipment_dbo_result = models::equipment_dbo::read_all(&mut conn, &pagination);    
    
    // Assert:
        
    let equipment_dbo_vec = equipment_dbo_result.unwrap();

    let len = 2;

    let mut equipment_1_interest_idx = 10;
    let mut equipment_2_interest_idx = 10;
    for idx in 0..len {
        if equipment_dbo_vec[idx].id == equipment_computer_1.id {
            equipment_1_interest_idx = idx;
            continue;
        }
        if equipment_dbo_vec[idx].id == equipment_computer_2.id {
            equipment_2_interest_idx = idx;
            continue;
        }
    }
    assert_ne!(equipment_1_interest_idx, 10);
    assert_ne!(equipment_2_interest_idx, 10);
}

#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let equipment = 
        fixtures::equipment_dbos::arrange_get_equipment_by_id_works_integration_test(&mut conn);

    // Act:
    
    let equipment_dbo_result = models::equipment_dbo::read(&mut conn, equipment.id);    

    // Assert:

    assert!(equipment_dbo_result.is_ok());

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();
    assert_eq!(equipment.misc_note, equipment_dbo.misc_note);
}

// #[actix_web::test]
// async fn update_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let original_equipment = 
//         fixtures::equipment_dbos::arrange_update_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     let new_name = "NEW Eq Set".to_string();
//     let new_description = "NEW description".to_string();

//     let put_payload = json!({
//         "name": &new_name,
//         "description": &new_description,
//     });
    
//     let put_uri = format!("/api/equipmentdbos/{}", original_equipment.id);
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

//     let new_equipment = put_resp_body.data.unwrap();
//     assert_eq!(new_equipment.id, original_equipment.id);
//     assert_eq!(new_equipment.name, new_name);
//     assert_eq!(new_equipment.description.unwrap(), new_description);
//     assert_ne!(new_equipment.created_at, new_equipment.updated_at);
// }

// #[actix_web::test]
// async fn delete_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let equipment = fixtures::equipment_dbos::arrange_delete_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let delete_uri = format!("/api/equipmentdbos/{}", equipment.id);
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


//     let get_by_id_uri = format!("/api/equipmentdbos/{}", equipment.id);
//     let get_by_id_req = test::TestRequest::get()
//         .uri(&get_by_id_uri)
//         .to_request();

//     let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

//     assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
// }
