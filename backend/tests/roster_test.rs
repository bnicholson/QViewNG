
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::game::Game};
use backend::models::roster::Roster;
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

    let payload = fixtures::rosters::arrange_create_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/rosters")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Roster> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let roster = body.data.unwrap();
    assert_ne!(roster.rosterid, uuid::Uuid::nil());
    assert_eq!(roster.name.as_str(), "Test Roster 2317");
    assert_eq!(roster.description.unwrap().as_str(), "Roster for integration test create.");
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (roster1, roster2) = fixtures::rosters::arrange_get_all_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/rosters?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<Roster> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut roster_1_interest_idx = 10;
    let mut roster_2_interest_idx = 10;
    for idx in 0..len {
        if body[idx].name == "Test Roster 1" {
            roster_1_interest_idx = idx;
            continue;
        }
        if body[idx].name == "Test Roster 2" {
            roster_2_interest_idx = idx;
            continue;
        }
    }
    assert_ne!(roster_1_interest_idx, 10);
    assert_ne!(roster_2_interest_idx, 10);
}

#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let roster = 
        fixtures::rosters::arrange_get_roster_by_id_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/rosters/{}", &roster.rosterid);
    println!("Rosters Get by ID URI: {}", &uri);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let roster: Roster = test::read_body_json(resp).await;
    assert_eq!(roster.name.as_str(), "Test Roster 2");
    assert_eq!(roster.description.unwrap().as_str(), "This is Roster 2's description.");
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let roster = 
        fixtures::rosters::arrange_update_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let new_name = "My NEW name".to_string();
    let new_description = "NEW description".to_string();

    let put_payload = json!({
        "name": &new_name,
        "description": &new_description,
    });
    
    let put_uri = format!("/api/rosters/{}", roster.rosterid);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<Roster> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let new_roster = put_resp_body.data.unwrap();
    assert_eq!(new_roster.rosterid, roster.rosterid);
    assert_eq!(new_roster.name.as_str(), new_name);
    assert_eq!(new_roster.description.as_ref().unwrap().as_str(), new_description);
    assert_ne!(new_roster.created_at, new_roster.updated_at);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let roster = fixtures::rosters::arrange_delete_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let delete_uri = format!("/api/rosters/{}", roster.rosterid);
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


    let get_by_id_uri = format!("/api/rosters/{}", roster.rosterid);
    let get_by_id_req = test::TestRequest::get()
        .uri(&get_by_id_uri)
        .to_request();

    let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

    assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
}

// #[actix_web::test]
// async fn add_game_to_roster_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");

//     let (roster, game, new_gsg) = fixtures::rosters::arrange_add_game_to_roster_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/rosters/{}/games", roster.rosterid);
//     let req = test::TestRequest::post()
//         .uri(&uri)
//         .set_json(&new_gsg)
//         .to_request();
    
//     // Act:

//     let resp = test::call_service(&app, req).await;
    
//     // Assert:
    
//     assert_eq!(resp.status(), StatusCode::CREATED);

//     let body: EntityResponse<GameRoster> = test::read_body_json(resp).await;
//     assert_eq!(body.code, 201);
//     assert_eq!(body.message, "");

//     let tournamentgroup_tournament = body.data.unwrap();
//     assert_eq!(tournamentgroup_tournament.rosterid, roster.rosterid);
//     assert_eq!(tournamentgroup_tournament.gameid, game.gid);
// }

// #[actix_web::test]
// async fn get_all_games_of_roster_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let (roster, game_1, game_2) = 
//         fixtures::rosters::arrange_get_all_games_of_roster_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/rosters/{}/games?page={}&page_size={}", roster.rosterid, PAGE_NUM, PAGE_SIZE);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:

//     let body: Vec<Game> = test::read_body_json(resp).await;

//     let len = 2;

//     assert_eq!(body.len(), len);

//     let mut game_1_idx = 10;
//     let mut game_2_idx = 10;
//     for idx in 0..len {
//         if body[idx].gid == game_1.gid {
//             game_1_idx = idx;
//         }
//         if body[idx].gid == game_2.gid {
//             game_2_idx = idx;
//         }
//     }
//     assert_ne!(game_1_idx, 10);
//     assert_ne!(game_2_idx, 10);
// }

// #[actix_web::test]
// async fn remove_game_from_roster_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");

//     let (roster, game, game_roster) = 
//         fixtures::rosters::arrange_remove_game_from_roster_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let delete_uri = format!("/api/rosters/{}/games/{}", roster.rosterid, game.gid);
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


//     let get_games_uri = format!("/api/rosters/{}/games?page={}&page_size={}", roster.rosterid, PAGE_NUM, PAGE_SIZE);
//     let get_games_req = test::TestRequest::get()
//         .uri(&get_games_uri)
//         .to_request();

//     let get_games_resp = test::call_service(&app, get_games_req).await;

//     assert_eq!(get_games_resp.status(), StatusCode::OK);

//     let body: Vec<Game> = test::read_body_json(get_games_resp).await;
//     assert_eq!(body.len(), 0);
// }
