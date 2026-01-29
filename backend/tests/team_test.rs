
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::game::Game};
use backend::models::team::Team;
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

    let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    let payload = fixtures::teams::get_team_payload(&mut conn, division.did);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/teams")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Team> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let team = body.data.unwrap();
    assert_eq!(team.did, division.did);
    assert_eq!(team.name.as_str(), "Better Team than Last Year");
    assert_eq!(team.quizzer_two_id, payload.quizzer_two_id);
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    fixtures::teams::seed_teams(&mut conn, division.did);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/teams?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<Team> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 3);

    let mut team_of_interest_idx = 10;
    for idx in 0..3 {
        if body[idx].name == "Luke Found a Frog" {
            team_of_interest_idx = idx;
            break;
        }
    }
    assert_ne!(team_of_interest_idx, 10);
    assert_eq!(body[team_of_interest_idx].did, division.did);
}

#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    let teams: Vec<Team> = fixtures::teams::seed_teams(&mut conn, division.did);
    let team_of_interest_idx = 0;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/teams/{}", &teams[team_of_interest_idx].teamid);
    println!("Teams Get by ID URI: {}", &uri);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let team: Team = test::read_body_json(resp).await;
    assert_eq!(team.did, division.did);
    assert_eq!(team.name, teams[team_of_interest_idx].name);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    let team: Team = fixtures::teams::seed_team(&mut conn, division.did);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let new_scheduled_start_time = Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap();

    let put_payload = json!({
        "scheduled_start_time": &new_scheduled_start_time
    });
    
    let put_uri = format!("/api/teams/{}", team.teamid);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<Team> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let new_team = put_resp_body.data.unwrap();
    assert_eq!(new_team.did, division.did);
    assert_eq!(new_team.teamid, team.teamid);
    assert_eq!(new_team.name, team.name);
    assert_ne!(new_team.created_at, new_team.updated_at);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    let team: Team = fixtures::teams::seed_team(&mut conn, division.did);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let delete_uri = format!("/api/teams/{}", team.teamid);
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


    let get_by_id_uri = format!("/api/teams/{}", team.teamid);
    let get_by_id_req = test::TestRequest::get()
        .uri(&get_by_id_uri)
        .to_request();

    let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

    assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn get_all_games_of_team_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (team_4_id, game_2, game_3) = fixtures::games::seed_get_games_of_team(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/teams/{}/games?page={}&page_size={}", team_4_id, PAGE_NUM, PAGE_SIZE);
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
        if body[idx].gid == game_3.gid {
            game_2_idx = idx;
        }
    }
    assert_ne!(game_1_idx, 10);
    assert_ne!(game_2_idx, 10);
}
