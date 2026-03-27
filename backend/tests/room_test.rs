
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::{self, apicalllog::ApiCalllog, equipmentregistration::EquipmentRegistration, game::Game, room::RoomBuilder}, services::common::PagedResponse};
use backend::models::room::Room;
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database, make_token};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (tournament, owner, admin_user, unrelated_user) =
        fixtures::rooms::arrange_room_create_works_integration_test(&mut conn);

    let mut payload = fixtures::rooms::get_room_payload(tournament.tid);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = "/api/rooms";

    // ── Success: tournament owner with room:create ───────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["room:create".to_string()],
    );

    let owner_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&payload)
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;

    assert_eq!(owner_resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Room> = test::read_body_json(owner_resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let room = body.data.unwrap();
    assert_eq!(room.tid, tournament.tid);
    assert_eq!(room.name.as_str(), "Test Room 2217");
    assert_eq!(room.building.as_str(), "Building 451");
    assert_eq!(room.comments.as_str(), "None at this time.");

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);

    // ── Success: tournament admin with room:create ───────────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["room:create".to_string()],
    );

    payload = RoomBuilder::new_default("Test Room 4000", tournament.tid).build().unwrap();
    let admin_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(&payload)
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;

    assert_eq!(admin_resp.status(), StatusCode::CREATED);

    // ── Fail: has room:create but is neither owner nor tournament admin ───────

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["room:create".to_string()],
    );

    payload = RoomBuilder::new_default("Test Room 4001", tournament.tid).build().unwrap();
    let unrelated_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(&payload)
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;

    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: no room:create permission at all ───────────────────────────────

    let no_perm_token = make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["room:read".to_string()],
    );

    payload = RoomBuilder::new_default("Test Room 4002", tournament.tid).build().unwrap();
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

    let body: PagedResponse<Room> = test::read_body_json(resp).await;

    assert_eq!(body.items.len(), 3);
    assert_eq!(body.count, 3);

    let mut room_or_interest_idx = 10;
    for idx in 0..3 {
        if body.items[idx].name == "Test Room 9078" {
            room_or_interest_idx = idx;
            break;
        }
    }

    let room_of_interest = &body.items[room_or_interest_idx];
    assert_eq!(room_of_interest.tid, parent_tournament.tid);
    assert_eq!(room_of_interest.building.as_str(), "Bldng 2");
    assert_eq!(room_of_interest.comments.as_str(), "I thought I recognized this place.");
    
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

    let (tournament, room, owner, admin_user, unrelated_user) =
        fixtures::rooms::arrange_room_update_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let put_uri = format!("/api/rooms/{}", room.roomid);

    // ── Success: tournament owner with room:update ───────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["room:update".to_string()],
    );

    let owner_payload = json!({
        "name": "Test Room NEW",
        "building": "Johnson NEW",
        "comments": "I can't tell who this building was named after, it's such a common last name."
    });
    let owner_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&owner_payload)
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;

    assert_eq!(owner_resp.status(), StatusCode::OK);

    let owner_resp_body: EntityResponse<Room> = test::read_body_json(owner_resp).await;
    assert_eq!(owner_resp_body.code, 200);
    assert_eq!(owner_resp_body.message, "");

    let updated_room = owner_resp_body.data.unwrap();
    assert_eq!(updated_room.tid, tournament.tid);
    assert_eq!(updated_room.roomid, room.roomid);
    assert_eq!(updated_room.name.as_str(), "Test Room NEW");
    assert_eq!(updated_room.building.as_str(), "Johnson NEW");
    assert_ne!(updated_room.created_at, updated_room.updated_at);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "PUT");
    assert_eq!(apicalllog_records.first().unwrap().uri, put_uri);

    // ── Success: tournament admin with room:update ───────────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["room:update".to_string()],
    );

    let admin_payload = json!({
        "name": "Test Room ADMIN UPDATE",
        "building": "Admin Building",
        "comments": "Updated by admin."
    });
    let admin_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(&admin_payload)
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;

    assert_eq!(admin_resp.status(), StatusCode::OK);

    // ── Fail: has room:update but is neither owner nor tournament admin ───────

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["room:update".to_string()],
    );

    let unrelated_payload = json!({
        "name": "Unauthorized Update",
        "building": "No Entry",
        "comments": "Should not succeed."
    });
    let unrelated_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(&unrelated_payload)
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;

    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: no room:update permission at all ───────────────────────────────

    let no_perm_token = make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["room:read".to_string()],
    );

    let no_perm_payload = json!({
        "name": "No Permission Update",
        "building": "Blocked Building",
        "comments": "This should also not succeed."
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

    let (_tournament, room_1, room_2, owner, admin_user, unrelated_user) =
        fixtures::rooms::arrange_room_delete_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let delete_uri_1 = format!("/api/rooms/{}", room_1.roomid);
    let delete_uri_2 = format!("/api/rooms/{}", room_2.roomid);

    // ── Fail: has room:delete but is neither owner nor tournament admin ───────

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["room:delete".to_string()],
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

    // ── Fail: no room:delete permission at all ───────────────────────────────

    let no_perm_token = make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["room:read".to_string()],
    );

    let no_perm_req = test::TestRequest::delete()
        .uri(&delete_uri_1)
        .insert_header(("Authorization", format!("Bearer {}", no_perm_token)))
        .to_request();

    let no_perm_resp = test::call_service(&app, no_perm_req).await;

    assert_eq!(no_perm_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Success: tournament owner with room:delete ───────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["room:delete".to_string()],
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

    // ── Success: tournament admin with room:delete ───────────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["room:delete".to_string()],
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
async fn get_all_games_of_room_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (game_2, game_4 ) = fixtures::games::seed_get_games_of_room(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/rooms/{}/games?page={}&page_size={}", game_2.roomid, PAGE_NUM, PAGE_SIZE);
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
        if body[idx].gid == game_2.gid {
            game_1_idx = idx;
        }
        if body[idx].gid == game_4.gid {
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

#[actix_web::test]
async fn get_all_equipmentregistrations_of_room_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (room, er_1, er_2) = 
        fixtures::rooms::arrange_get_all_equipmentregistrations_of_room_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/rooms/{}/equipmentregistrations?page={}&page_size={}", room.roomid, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<EquipmentRegistration> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut equipmentregistration_1_idx = 10;
    let mut equipmentregistration_2_idx = 10;
    for idx in 0..len {
        if body[idx].id == er_1.id {
            equipmentregistration_1_idx = idx;
        }
        if body[idx].id == er_2.id {
            equipmentregistration_2_idx = idx;
        }
    }
    assert_ne!(equipmentregistration_1_idx, 10);
    assert_ne!(equipmentregistration_2_idx, 10);
    
    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}
