
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

    let deps = fixtures::games::seed_game_payload_dependencies(&mut conn, "Tour 1");
    let payload = fixtures::games::get_game_payload(deps.0,deps.1,deps.2,deps.3,deps.4,Some(deps.5),deps.6,deps.7);

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
    assert_eq!(game.quizmasterid, deps.7);
    assert_eq!(game.leftteamid, deps.4);
}

#[actix_web::test]
async fn create_errors_when_team_is_found_more_than_once_in_a_game() {

    // Arrange 1:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let payload_one = fixtures::games::duplicate_team_in_game_case_one_payload(&mut conn);
    
    let req_one = test::TestRequest::post()
        .uri("/api/games")
        .set_json(&payload_one)
        .to_request();

    // Act 1:

    let resp_one = test::call_service(&app, req_one).await;
    
    // Assert 1:
    
    assert_eq!(resp_one.status(), StatusCode::BAD_REQUEST);

    // Arrange 2:

    clean_database();

    let payload_two = fixtures::games::duplicate_team_in_game_case_two_payload(&mut conn);
    
    let req_two = test::TestRequest::post()
        .uri("/api/games")
        .set_json(&payload_two)
        .to_request();

    // Act 2:

    let resp_two = test::call_service(&app, req_two).await;
    
    // Assert 2:
    
    assert_eq!(resp_two.status(), StatusCode::BAD_REQUEST);

    // Arrange 3:

    clean_database();

    let payload_three = fixtures::games::duplicate_team_in_game_case_three_payload(&mut conn);
    
    let req_three = test::TestRequest::post()
        .uri("/api/games")
        .set_json(&payload_three)
        .to_request();

    // Act 3:

    let resp_three = test::call_service(&app, req_three).await;
    
    // Assert 3:
    
    assert_eq!(resp_three.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    fixtures::games::seed_games(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/games?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<Game> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 2);
}

#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let games: Vec<Game> = fixtures::games::seed_games(&mut conn);
    let game_of_interest_idx = 0;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/games/{}", &games[game_of_interest_idx].gid);
    println!("Games Get by ID URI: {}", &uri);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    assert_eq!(resp.status(), StatusCode::OK);
    
    let game: Game = test::read_body_json(resp).await;
    assert_eq!(game.divisionid.unwrap(), games[game_of_interest_idx].divisionid.unwrap());
    assert_eq!(game.rightteamid, games[game_of_interest_idx].rightteamid);
    assert_eq!(game.centerteamid.unwrap(), games[game_of_interest_idx].centerteamid.unwrap());
    assert_eq!(game.leftteamid, games[game_of_interest_idx].leftteamid);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let game = fixtures::games::seed_game(&mut conn);
    
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(db))
        .configure(configure_routes)
    ).await;
    
    let new_left_team = fixtures::teams::seed_team(&mut conn, game.divisionid.unwrap());

    let put_payload = json!({
        "leftteamid": &new_left_team.teamid
    });
    
    let put_uri = format!("/api/games/{}", game.gid);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<Game> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let updated_game = put_resp_body.data.unwrap();
    assert_eq!(updated_game.divisionid.unwrap(), game.divisionid.unwrap());
    assert_eq!(updated_game.leftteamid, new_left_team.teamid);
    assert_ne!(updated_game.created_at, updated_game.updated_at);
}

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
