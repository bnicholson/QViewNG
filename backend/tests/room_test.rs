
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

    let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");

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

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");

    fixtures::rooms::seed_rooms(&mut conn, parent_tournament.tid);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/rooms?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<Room> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 3);

    let mut room_or_interest_idx = 10;
    for idx in 0..3 {
        if body[idx].name == "Test Room 9078" {
            room_or_interest_idx = idx;
            break;
        }
    }

    let room_of_interest = &body[room_or_interest_idx];
    assert_eq!(room_of_interest.tid, parent_tournament.tid);
    assert_eq!(room_of_interest.building.as_str(), "Bldng 2");
    assert_eq!(room_of_interest.comments.as_str(), "I thought I recognized this place.");
}

#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");

    let rooms: Vec<Room> = fixtures::rooms::seed_rooms(&mut conn, parent_tournament.tid);
    let room_of_interest_idx = 0;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/rooms/{}", &rooms[room_of_interest_idx].roomid);
    println!("Rooms Get by ID URI: {}", &uri);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let room: Room = test::read_body_json(resp).await;
    assert_eq!(room.name, rooms[room_of_interest_idx].name);
    assert_eq!(room.building, rooms[room_of_interest_idx].building);
    assert_eq!(room.comments, rooms[room_of_interest_idx].comments);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");

    let room: Room = fixtures::rooms::seed_room(&mut conn, parent_tournament.tid);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let new_name = "Test Room NEW".to_string();
    let new_building = "Johnson NEW".to_string();
    let new_comments = "I can't tell who this building was named after, it's such a common last name.".to_string();

    let put_payload = json!({
        "name": &new_name,
        "building": new_building,
        "comments": &new_comments
    });
    
    let put_uri = format!("/api/rooms/{}", room.roomid);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<Room> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let new_room = put_resp_body.data.unwrap();
    assert_eq!(new_room.tid, parent_tournament.tid);
    assert_eq!(new_room.roomid, room.roomid);
    assert_eq!(new_room.name.as_str(), new_name);
    assert_eq!(new_room.building.as_str(), new_building);
    assert_eq!(new_room.comments.as_str(), new_comments);
    assert_ne!(new_room.created_at, new_room.updated_at);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");

    let room: Room = fixtures::rooms::seed_room(&mut conn, parent_tournament.tid);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let delete_uri = format!("/api/rooms/{}", room.roomid);
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


    let get_by_id_uri = format!("/api/rooms/{}", room.roomid);
    let get_by_id_req = test::TestRequest::get()
        .uri(&get_by_id_uri)
        .to_request();

    let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

    assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
}
