
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::{game::Game, roster::Roster, roster_coach::RosterCoach, team::Team, tournament::Tournament, user::User}};
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use bcrypt::verify;
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);

    let unhashed_pwd = "FamouslySecure!23";
    let payload = fixtures::users::get_user_payload(unhashed_pwd);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<User> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    
    let user = body.data.unwrap();
    assert_ne!(user.id.to_string().as_str(), "");
    assert_eq!(user.fname.as_str(), "Test User 3276");
    assert_eq!(user.mname.as_str(), "Maurice");
    assert!(user.activated);

    let pwd_is_valid = verify(unhashed_pwd, &user.hash_password).expect("Password verification failed");
    assert!(pwd_is_valid);
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    fixtures::users::seed_users(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/users?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<User> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 3);

    let mut user_or_interest_idx = 10;
    for idx in 0..3 {
        if body[idx].fname == "Test User 9078" {
            user_or_interest_idx = idx;
            break;
        }
    }

    let user_or_interest = &body[user_or_interest_idx];
    assert_ne!(user_or_interest.id.to_string().as_str(),"");  // "ne" in "assert_ne!" means Not Equal
    assert_eq!(user_or_interest.mname, "Eugene");
    assert_eq!(user_or_interest.username, "edbashful");
}


#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let users: Vec<User> = fixtures::users::seed_users(&mut conn);
    let user_of_interest_idx = 0;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/users/{}", &users[user_of_interest_idx].id);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let user: User = test::read_body_json(resp).await;
    assert_eq!(user.fname, users[user_of_interest_idx].fname);
    assert_eq!(user.username, users[user_of_interest_idx].username);
    assert_eq!(user.lname, users[user_of_interest_idx].lname);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let user: User = fixtures::users::seed_user(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let new_fname = "Test User NEW".to_string();
    let new_mname = "Flemming".to_string();
    let new_activated = false;

    let put_payload = json!({
        "fname": new_fname,
        "mname": new_mname,
        "activated": new_activated
    });
    
    let put_uri = format!("/api/users/{}", user.id);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<User> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let new_user = put_resp_body.data.unwrap();
    assert_eq!(new_user.id, user.id);
    assert_eq!(new_user.fname.as_str(), new_fname);
    assert_eq!(new_user.mname.as_str(), new_mname);
    assert_eq!(new_user.activated, new_activated);
    assert_ne!(new_user.created_at, new_user.updated_at);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let user: User = fixtures::users::seed_user(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let delete_uri = format!("/api/users/{}", user.id);
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


    let get_by_id_uri = format!("/api/users/{}", user.id);
    let get_by_id_req = test::TestRequest::get()
        .uri(&get_by_id_uri)
        .to_request();

    let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

    assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn get_all_games_where_user_is_quizmaster_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (qm_id, contenjudge_id, game_1, game_3) = fixtures::games::seed_get_games_where_user_is_quizmaster_or_contentjudge(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/users/{}/games-where-quizmaster?page={}&page_size={}", qm_id, PAGE_NUM, PAGE_SIZE);
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
        if body[idx].gid == game_1.gid {
            game_1_idx = idx;
        }
        if body[idx].gid == game_3.gid {
            game_2_idx = idx;
        }
    }
    assert_ne!(game_1_idx, 10);
    assert_ne!(game_2_idx, 10);
}

#[actix_web::test]
async fn get_all_games_where_user_is_contentjudge_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (qm_id, contenjudge_id, game_1, game_3) = fixtures::games::seed_get_games_where_user_is_quizmaster_or_contentjudge(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/users/{}/games-where-contentjudge?page={}&page_size={}", contenjudge_id, PAGE_NUM, PAGE_SIZE);
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
        if body[idx].gid == game_1.gid {
            game_1_idx = idx;
        }
        if body[idx].gid == game_3.gid {
            game_2_idx = idx;
        }
    }
    assert_ne!(game_1_idx, 10);
    assert_ne!(game_2_idx, 10);
}

#[actix_web::test]
async fn get_all_tournaments_where_user_is_admin_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (admin, tour_1_id, tour_2_id) = fixtures::users::seed_get_tournaments_where_user_is_admin(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/users/{}/tournaments-where-admin?page={}&page_size={}", admin.id, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Tournament> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut tour_1_idx = 10;
    let mut tour_2_idx = 10;
    for idx in 0..len {
        if body[idx].tid == tour_1_id {
            tour_1_idx = idx;
        }
        if body[idx].tid == tour_2_id {
            tour_2_idx = idx;
        }
    }
    assert_ne!(tour_1_idx, 10);
    assert_ne!(tour_2_idx, 10);
}

#[actix_web::test]
async fn get_all_teams_where_user_is_coach_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (coach, team_1, team_2) = 
        fixtures::users::arrange_get_all_teams_where_user_is_coach_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/users/{}/teams-where-coach?page={}&page_size={}", coach.id, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Team> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut team_1_idx = 10;
    let mut team_2_idx = 10;
    for idx in 0..len {
        if body[idx].teamid == team_1.teamid {
            team_1_idx = idx;
        }
        if body[idx].teamid == team_2.teamid {
            team_2_idx = idx;
        }
    }
    assert_ne!(team_1_idx, 10);
    assert_ne!(team_2_idx, 10);
}

#[actix_web::test]
async fn get_all_teams_where_user_is_quizzer_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (quizzer, team_1, team_2, team_3, team_4, team_5, team_6) = 
        fixtures::users::arrange_get_all_teams_where_user_is_quizzer_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/users/{}/teams-where-quizzer?page={}&page_size={}", quizzer.id, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Team> = test::read_body_json(resp).await;

    let len = 6;

    assert_eq!(body.len(), len);

    let mut team_1_idx = 10;
    let mut team_2_idx = 10;
    let mut team_3_idx = 10;
    let mut team_4_idx = 10;
    let mut team_5_idx = 10;
    let mut team_6_idx = 10;
    for idx in 0..len {
        if body[idx].teamid == team_1.teamid {
            team_1_idx = idx;
        }
        if body[idx].teamid == team_2.teamid {
            team_2_idx = idx;
        }
        if body[idx].teamid == team_3.teamid {
            team_3_idx = idx;
        }
        if body[idx].teamid == team_4.teamid {
            team_4_idx = idx;
        }
        if body[idx].teamid == team_5.teamid {
            team_5_idx = idx;
        }
        if body[idx].teamid == team_6.teamid {
            team_6_idx = idx;
        }
    }
    assert_ne!(team_1_idx, 10);
    assert_ne!(team_2_idx, 10);
    assert_ne!(team_3_idx, 10);
    assert_ne!(team_4_idx, 10);
    assert_ne!(team_5_idx, 10);
    assert_ne!(team_6_idx, 10);
}

#[actix_web::test]
async fn create_roster_works() {

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
    
    let uri = format!("/api/users/{}/rosters", payload.created_by_userid);
    let req = test::TestRequest::post()
        .uri(&uri)
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


    // test also that the coach was added to the roster via record inserted in the rosters_coaches table:

    let get_coaches_uri = format!("/api/rosters/{}/coaches?page={}&page_size={}", roster.rosterid, PAGE_NUM, PAGE_SIZE);
    let get_coaches_req = test::TestRequest::get()
        .uri(&get_coaches_uri)
        .to_request();

    let get_coaches_resp = test::call_service(&app, get_coaches_req).await;

    assert_eq!(get_coaches_resp.status(), StatusCode::OK);

    let body: Vec<User> = test::read_body_json(get_coaches_resp).await;
    assert_eq!(body.len(), 1);
}

#[actix_web::test]
async fn get_all_rosters_of_coach_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (coach_2, quizzer_2, roster_3, roster_4) = 
        fixtures::users::arrange_get_all_rosters_of_coach_or_quizzer_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/users/{}/rosters-of-coach?page={}&page_size={}", coach_2.id, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Roster> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut roster_1_idx = 10;
    let mut roster_2_idx = 10;
    for idx in 0..len {
        if body[idx].rosterid == roster_3.rosterid {
            roster_1_idx = idx;
        }
        if body[idx].rosterid == roster_4.rosterid {
            roster_2_idx = idx;
        }
    }
    assert_ne!(roster_1_idx, 10);
    assert_ne!(roster_2_idx, 10);
}

#[actix_web::test]
async fn get_all_rosters_of_quizzer_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (coach_2, quizzer_2, roster_3, roster_4) = 
        fixtures::users::arrange_get_all_rosters_of_coach_or_quizzer_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/users/{}/rosters-containing-quizzer?page={}&page_size={}", quizzer_2.id, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Roster> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut roster_1_idx = 10;
    let mut roster_2_idx = 10;
    for idx in 0..len {
        if body[idx].rosterid == roster_3.rosterid {
            roster_1_idx = idx;
        }
        if body[idx].rosterid == roster_4.rosterid {
            roster_2_idx = idx;
        }
    }
    assert_ne!(roster_1_idx, 10);
    assert_ne!(roster_2_idx, 10);
}

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

