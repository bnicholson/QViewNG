
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::database::Database;
use backend::models::game::Game;
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use chrono::{TimeZone, Utc};
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let deps = fixtures::games::seed_game_payload_dependencies(&mut conn);
    let payload = fixtures::games::get_game_payload(deps.0,deps.1,deps.2,deps.3,deps.4,deps.5,deps.6);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/games")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Game> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let game = body.data.unwrap();
    assert_eq!(game.divisionid.unwrap(), deps.1);
    assert_eq!(game.quizmasterid, deps.6);
    assert_eq!(game.leftteamid, deps.4);
}

// #[actix_web::test]
// async fn get_all_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let tournament = fixtures::tournaments::seed_tournament(&mut conn);
//     let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

//     fixtures::games::seed_games(&mut conn, division.did);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/games?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     // Act:
    
//     let resp = test::call_service(&app, req).await;
    
//     // Assert:
    
//     assert_eq!(resp.status(), StatusCode::OK);

//     let body: Vec<Game> = test::read_body_json(resp).await;

//     assert_eq!(body.len(), 3);

//     let mut game_or_interest_idx = 10;
//     for idx in 0..3 {
//         if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2045, 5, 23, 00, 00, 0).unwrap() {
//             game_or_interest_idx = idx;
//             break;
//         }
//     }
//     assert_ne!(game_or_interest_idx, 10);
// }

// #[actix_web::test]
// async fn get_by_id_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let tournament = fixtures::tournaments::seed_tournament(&mut conn);
//     let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

//     let games: Vec<Game> = fixtures::games::seed_games(&mut conn, division.did);
//     let game_of_interest_idx = 0;

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     let uri = format!("/api/games/{}", &games[game_of_interest_idx].gameid);
//     println!("Games Get by ID URI: {}", &uri);
//     let req = test::TestRequest::get()
//         .uri(uri.as_str())
//         .to_request();

//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:
    
//     let game: Game = test::read_body_json(resp).await;
//     assert_eq!(game.did, division.did);
//     assert_eq!(game.scheduled_start_time.unwrap(), Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap());
// }

// #[actix_web::test]
// async fn update_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let tournament = fixtures::tournaments::seed_tournament(&mut conn);
//     let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

//     let game: Game = fixtures::games::seed_game(&mut conn, division.did);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     let new_scheduled_start_time = Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap();

//     let put_payload = json!({
//         "scheduled_start_time": &new_scheduled_start_time
//     });
    
//     let put_uri = format!("/api/games/{}", game.gameid);
//     let put_req = test::TestRequest::put()
//         .uri(&put_uri)
//         .set_json(&put_payload)
//         .to_request();

//     // Act:
    
//     let put_resp = test::call_service(&app, put_req).await;

//     // Assert:
    
//     assert_eq!(put_resp.status(), StatusCode::OK);

//     let put_resp_body: EntityResponse<Game> = test::read_body_json(put_resp).await;
//     assert_eq!(put_resp_body.code, 200);
//     assert_eq!(put_resp_body.message, "");

//     let new_game = put_resp_body.data.unwrap();
//     assert_eq!(new_game.did, division.did);
//     assert_eq!(new_game.gameid, game.gameid);
//     assert_eq!(new_game.scheduled_start_time.unwrap(), new_scheduled_start_time);
//     assert_ne!(new_game.created_at, new_game.updated_at);
// }

// #[actix_web::test]
// async fn delete_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let tournament = fixtures::tournaments::seed_tournament(&mut conn);
//     let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

//     let game: Game = fixtures::games::seed_game(&mut conn, division.did);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let delete_uri = format!("/api/games/{}", game.gameid);
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


//     let get_by_id_uri = format!("/api/games/{}", game.gameid);
//     let get_by_id_req = test::TestRequest::get()
//         .uri(&get_by_id_uri)
//         .to_request();

//     let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

//     assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
// }
