
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::database::Database;
use backend::models::room::Room;
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

    let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn);

    let payload = fixtures::rooms::get_room_payload(parent_tournament.tid);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/rooms")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Room> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let room = body.data.unwrap();
    assert_eq!(room.tid, parent_tournament.tid);
    assert_eq!(room.name.as_str(), "Test Room 2217");
    assert_eq!(room.building.as_str(), "Building 451");
    assert_eq!(room.comments.as_str(), "None at this time.");
}

// #[actix_web::test]
// async fn get_all_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn);

//     fixtures::rooms::seed_rooms(&mut conn, parent_tournament.tid);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/rooms?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:

//     let body: Vec<Room> = test::read_body_json(resp).await;

//     assert_eq!(body.len(), 3);

//     let mut div_or_interest_idx = 10;
//     for idx in 0..3 {
//         if body[idx].dname == "Test Div 9078" {
//             div_or_interest_idx = idx;
//             break;
//         }
//     }

//     let div_of_interest = &body[div_or_interest_idx];
//     assert_eq!(div_of_interest.tid, parent_tournament.tid);
//     assert_ne!(div_of_interest.did.to_string().as_str(),"");  // "ne" in "assert_ne!" means Not Equal
//     assert_eq!(div_of_interest.breadcrumb,"/test/post/for/room/2");
//     assert!(!div_of_interest.is_public);
//     assert_eq!(div_of_interest.shortinfo, "Novice");
// }


// #[actix_web::test]
// async fn get_by_id_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn);

//     let rooms: Vec<Room> = fixtures::rooms::seed_rooms(&mut conn, parent_tournament.tid);
//     let room_of_interest_idx = 0;

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     let uri = format!("/api/rooms/{}", &rooms[room_of_interest_idx].did);
//     println!("Rooms Get by ID URI: {}", &uri);
//     let req = test::TestRequest::get()
//         .uri(uri.as_str())
//         .to_request();

//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:
    
//     let room: Room = test::read_body_json(resp).await;
//     assert_eq!(room.dname, rooms[room_of_interest_idx].dname);
//     assert_eq!(room.shortinfo, rooms[room_of_interest_idx].shortinfo);
//     assert_eq!(room.breadcrumb, rooms[room_of_interest_idx].breadcrumb);
// }

// #[actix_web::test]
// async fn update_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn);

//     let room: Room = fixtures::rooms::seed_room(&mut conn, parent_tournament.tid);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     let new_dname = "Test Div NEW".to_string();
//     let new_breadcrumb = "/latest/breadcrumb".to_string();
//     let new_is_public = true;

//     let put_payload = json!({
//         "dname": &new_dname,
//         "breadcrumb": new_breadcrumb,
//         "is_public": &new_is_public
//     });
    
//     let put_uri = format!("/api/rooms/{}", room.did);
//     let put_req = test::TestRequest::put()
//         .uri(&put_uri)
//         .set_json(&put_payload)
//         .to_request();

//     // Act:
    
//     let put_resp = test::call_service(&app, put_req).await;

//     // Assert:
    
//     assert_eq!(put_resp.status(), StatusCode::OK);

//     let put_resp_body: EntityResponse<Room> = test::read_body_json(put_resp).await;
//     assert_eq!(put_resp_body.code, 200);
//     assert_eq!(put_resp_body.message, "");

//     let new_room = put_resp_body.data.unwrap();
//     assert_eq!(new_room.tid, parent_tournament.tid);
//     assert_eq!(new_room.did, room.did);
//     assert_eq!(new_room.dname.as_str(), new_dname);
//     assert_eq!(new_room.breadcrumb.as_str(), new_breadcrumb);
//     assert_eq!(new_room.is_public, new_is_public);
// }

// #[actix_web::test]
// async fn delete_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn);

//     let room: Room = fixtures::rooms::seed_room(&mut conn, parent_tournament.tid);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let delete_uri = format!("/api/rooms/{}", room.did);
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


//     let get_by_id_uri = format!("/api/rooms/{}", room.did);
//     let get_by_id_req = test::TestRequest::get()
//         .uri(&get_by_id_uri)
//         .to_request();

//     let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

//     assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
// }
