
mod common;
mod fixtures;

use actix_web::{test, App, web::{self,Bytes}, http::StatusCode};
use chrono::{Duration, Local, NaiveDate, TimeZone, Utc};
use backend::{models::{game::Game, room::Room, round::Round, tournament_admin::{NewTournamentAdmin, TournamentAdmin}, tournamentgroup::TournamentGroup, user::User}, routes::configure_routes, services::common::EntityResponse};
use backend::models::{division::Division,tournament::Tournament};
use backend::database::Database;
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database};

// #[actix_web::test]
// async fn get_all_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     fixtures::tournaments::seed_tournaments(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/tournaments?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:

//     let body: Vec<Tournament> = test::read_body_json(resp).await;

//     assert_eq!(body.len(), 3);

//     let mut tour_or_interest_idx = 10;
//     for idx in 0..3 {
//         if body[idx].tname == "Tour 2" {
//             tour_or_interest_idx = idx;
//             break;
//         }
//     }

//     let tour_of_interest = &body[tour_or_interest_idx];
//     assert_eq!(tour_of_interest.organization,"Nazarene");
//     assert_ne!(tour_of_interest.tid.to_string().as_str(),"");  // "ne" in "assert_ne!" means Not Equal
//     assert_eq!(tour_of_interest.breadcrumb,"/test/bread/crumb");
//     assert_eq!(tour_of_interest.fromdate, NaiveDate::from_ymd_opt(2025, 5, 23).unwrap());
//     assert_eq!(tour_of_interest.todate, NaiveDate::from_ymd_opt(2025, 5, 27).unwrap());
//     assert_eq!(tour_of_interest.venue,"Olivet Nazarene University");
//     assert_eq!(tour_of_interest.city,"Bourbonnais");
//     assert_eq!(tour_of_interest.region,"Central USA");
//     assert_eq!(tour_of_interest.country,"USA");
//     assert_eq!(tour_of_interest.contact,"Jason Morton");
//     assert_eq!(tour_of_interest.contactemail,"jasonmorton@fakeemail.com");
//     assert_eq!(tour_of_interest.is_public,false);
//     assert_eq!(tour_of_interest.shortinfo,"NYI International quiz meet of 2025.");
//     assert_eq!(tour_of_interest.info,"If I wanted a longer description I would have provided it here.");
// }

// #[actix_web::test]
// async fn get_by_id_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let tournaments = fixtures::tournaments::seed_tournaments(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//         let uri = format!("/api/tournaments/{}", &tournaments[0].tid);
//         let req = test::TestRequest::get()
//             .uri(uri.as_str())
//             .to_request();

//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:
    
//     let body: Tournament = test::read_body_json(resp).await;
//     assert_eq!(body.tname, "Q2025");
//     assert_eq!(body.tid.to_string().as_str(), tournaments[0].tid.to_string().as_str());
//     assert_eq!(body.organization, "Nazarene");
// }

// #[actix_web::test]
// async fn get_todays_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     fixtures::tournaments::seed_tournaments_for_get_today(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     // Act:

//     let req = test::TestRequest::get()
//         .uri("/api/tournaments/today")
//         .to_request();
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:
    
//     let body: Vec<Tournament> = test::read_body_json(resp).await;
//     assert_eq!(body.iter().count(), 2);
    
//     let today: NaiveDate = Local::now().date_naive();

//     let mut tour_today_exactly: &Tournament = &body[0];
//     let mut tour_20_day_range_including_today: &Tournament = &body[1];

//     if &body[1].fromdate == &today {
//         tour_today_exactly = &body[1];
//         tour_20_day_range_including_today = &body[0];
//     }

//     assert_eq!(tour_today_exactly.tname, "Today Exactly");
//     assert_eq!(tour_today_exactly.fromdate, today);
//     assert_eq!(tour_today_exactly.todate, today);
    
//     let tour_min_ten: NaiveDate = today - Duration::days(10);
//     let tour_plus_ten: NaiveDate = today + Duration::days(10);
//     assert_eq!(tour_20_day_range_including_today.tname, "20 Days, Including Today");
//     assert_eq!(tour_20_day_range_including_today.fromdate, tour_min_ten);
//     assert_eq!(tour_20_day_range_including_today.todate, tour_plus_ten);
// }

// #[actix_web::test]
// async fn get_all_in_date_range_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     fixtures::tournaments::seed_tournaments_for_get_all_in_date_range(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     // Act:
//     let today: NaiveDate = Local::now().date_naive();
//     let sub_ten_days: NaiveDate = today - Duration::days(10);
//     let add_ten_days: NaiveDate = today + Duration::days(10);
//     let sub_ten_days_millis: i64 = sub_ten_days.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_millis();
//     let add_ten_days_millis: i64 = add_ten_days.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_millis();
//     let uri = format!("/api/tournaments/filter?from_date={}&to_date={}", sub_ten_days_millis, add_ten_days_millis);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     let resp = test::call_service(&app, req).await;
    
//     // Assert:
    
//     assert_eq!(resp.status(), StatusCode::OK);

//     let body: Vec<Tournament> = test::read_body_json(resp).await;
//     assert_eq!(body.iter().count(), 3);

//     let sub_8_days_from_today: NaiveDate = today - Duration::days(8);
//     let add_8_days_to_today: NaiveDate = today + Duration::days(8);
//     let add_12_days_to_today: NaiveDate = today + Duration::days(12);  // outside of range

//     let mut today_tour_idx = 10;
//     let mut sub_8_tour_idx = 10;
//     let mut add_8_tour_idx = 10;
//     for idx in 0..3 {
//         if body[idx].fromdate == today {
//             today_tour_idx = idx;
//         }
//         else if body[idx].fromdate == sub_8_days_from_today {
//             sub_8_tour_idx = idx;
//         }
//         else {
//             add_8_tour_idx = idx;
//         }
//     }

//     assert_eq!(body[today_tour_idx].tname, "Today Exactly");
//     assert_eq!(body[today_tour_idx].fromdate, today);
//     assert_eq!(body[today_tour_idx].todate, today);
    
//     assert_eq!(body[sub_8_tour_idx].tname, "eight days past exactly");
//     assert_eq!(body[sub_8_tour_idx].fromdate, sub_8_days_from_today);
//     assert_eq!(body[sub_8_tour_idx].todate, sub_8_days_from_today);
    
//     assert_eq!(body[add_8_tour_idx].tname, "eight to twelve days future");
//     assert_eq!(body[add_8_tour_idx].fromdate, add_8_days_to_today);
//     assert_eq!(body[add_8_tour_idx].todate, add_12_days_to_today);
// }

// #[actix_web::test]
// async fn create_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);

//     let payload = fixtures::tournaments::get_tournament_payload();

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//         let req = test::TestRequest::post()
//             .uri("/api/tournaments")
//             .set_json(&payload)
//             .to_request();
    
//     // Act:

//     let resp = test::call_service(&app, req).await;
    
//     // Assert:
    
//     assert_eq!(resp.status(), StatusCode::CREATED);

//     let body: EntityResponse<Tournament> = test::read_body_json(resp).await;
//     assert_eq!(body.code, 201);
//     assert_eq!(body.message, "");

//     let tournament = body.data.unwrap();
//     assert_ne!(tournament.tid.to_string().as_str(), "");
//     assert_eq!(tournament.organization.as_str(), "Nazarene");
//     assert_eq!(tournament.tname.as_str(), "Test Tour");
// }

// #[actix_web::test]
// async fn update_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");

//     let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     let new_venue = "Albatross Academy".to_string();
//     let new_todate = NaiveDate::from_ymd_opt(2025, 5, 30).unwrap();
//     let new_info = "Sadly, Shawn White is retired from pro snowboarding now.".to_string();

//     let put_payload = json!({
//         "venue": &new_venue,
//         "todate": new_todate,
//         "info": &new_info
//     });
    
//     let put_uri = format!("/api/tournaments/{}", tournament.tid);
//     let put_req = test::TestRequest::put()
//         .uri(&put_uri)
//         .set_json(&put_payload)
//         .to_request();

//     // Act:
    
//     let put_resp = test::call_service(&app, put_req).await;

//     // Assert:
    
//     assert_eq!(put_resp.status(), StatusCode::OK);

//     let put_resp_body: EntityResponse<Tournament> = test::read_body_json(put_resp).await;
//     assert_eq!(put_resp_body.code, 200);
//     assert_eq!(put_resp_body.message, "");

//     let new_tournament = put_resp_body.data.unwrap();
//     assert_eq!(new_tournament.tid, tournament.tid);
//     assert_eq!(new_tournament.organization.as_str(), "Nazarene");
//     assert_eq!(new_tournament.tname.as_str(), "Test Tour");
//     assert_eq!(new_tournament.venue.as_str(), new_venue);
//     assert_eq!(new_tournament.todate, new_todate);
//     assert_eq!(new_tournament.info.as_str(), new_info);
//     assert_ne!(new_tournament.created_at, new_tournament.updated_at);
// }

// #[actix_web::test]
// async fn delete_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");

//     let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let delete_uri = format!("/api/tournaments/{}", tournament.tid);
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


//     let get_by_id_uri = format!("/api/tournaments/{}", tournament.tid);
//     let get_by_id_req = test::TestRequest::get()
//         .uri(&get_by_id_uri)
//         .to_request();

//     let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

//     assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
// }

// #[actix_web::test]
// async fn get_all_divisions_of_tournament_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");

//     let tournament = fixtures::tournaments::seed_get_divisions_by_tournament(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/tournaments/{}/divisions?page={}&page_size={}", tournament.tid, PAGE_NUM, PAGE_SIZE);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:

//     let body: Vec<Division> = test::read_body_json(resp).await;

//     assert_eq!(body.len(), 3);

//     let mut div_1_idx = 10;
//     let mut div_2_idx = 10;
//     let mut div_3_idx = 10;
//     for idx in 0..3 {
//         if body[idx].dname == "Test Div 9" {
//             div_1_idx = idx;
//         }
//         if body[idx].dname == "Test Div 2" {
//             div_2_idx = idx;
//         }
//         if body[idx].dname == "Test Div 7" {
//             div_3_idx = idx;
//         }
//     }
//     assert_ne!(div_1_idx, 10);
//     assert_ne!(div_2_idx, 10);
//     assert_ne!(div_3_idx, 10);
//     // overkill, but thorough:
//     assert_eq!(body[div_1_idx].dname, "Test Div 9");
//     assert_eq!(body[div_2_idx].dname, "Test Div 2");
//     assert_eq!(body[div_3_idx].dname, "Test Div 7");
// }

#[actix_web::test]
async fn add_admin_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
    let user_to_become_admin = fixtures::users::seed_user(&mut conn);

    let payload = fixtures::tournaments_admins::get_tour_admin_payload_singular(tournament.tid, user_to_become_admin.id);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/tournaments/{}/admins", tournament.tid);
    let req = test::TestRequest::post()
        .uri(&uri)
        .set_json(payload)
        .to_request();
    
    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<TournamentAdmin> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let tournament_admin = body.data.unwrap();
    assert_eq!(tournament_admin.tournamentid, tournament.tid);
    assert_eq!(tournament_admin.adminid, user_to_become_admin.id);
    assert_eq!(tournament_admin.role_description.unwrap().as_str(), "default role (test id 334)");
}

// #[actix_web::test]
// async fn get_all_admins_of_tournament_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
//     let users = fixtures::users::seed_users_for_get_all_admins_of_tour(&mut conn);
    
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;

//     for user in users {
//         let uri = format!("/api/tournaments/{}/admins/{}", tournament.tid, user.id);
//         let payload = fixtures::tournaments_admins::get_tour_admin_payload_singular(tournament.tid, user.id);
//         let req = test::TestRequest::post()
//             .uri(&uri)
//             .set_json(&payload)
//             .to_request();
        
//         let resp = test::call_service(&app, req).await;
//         assert_eq!(resp.status(), StatusCode::CREATED);
//     }
    
//     let uri = format!("/api/tournaments/{}/admins?page={}&page_size={}", tournament.tid, PAGE_NUM, PAGE_SIZE);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:

//     let body: Vec<User> = test::read_body_json(resp).await;

//     assert_eq!(body.len(), 2);

//     let mut admin_1_idx = 10;
//     let mut admin_2_idx = 10;
//     for idx in 0..2 {
//         if body[idx].fname == "Test User 9" {
//             admin_1_idx = idx;
//         }
//         if body[idx].fname == "Test User 3" {
//             admin_2_idx = idx;
//         }
//     }
//     assert_ne!(admin_1_idx, 10);
//     assert_ne!(admin_2_idx, 10);
//     // overkill, but thorough:
//     assert_eq!(body[admin_1_idx].fname, "Test User 9");
//     assert_eq!(body[admin_2_idx].fname, "Test User 3");
// }

// #[actix_web::test]
// async fn update_admin_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
//     let user = fixtures::users::seed_user(&mut conn);

//     let post_payload = fixtures::tournaments_admins::get_tour_admin_payload_singular(tournament.tid, user.id);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let post_uri = format!("/api/tournaments/{}/admins/{}", tournament.tid, user.id);
//     let post_req = test::TestRequest::post()
//         .uri(&post_uri)
//         .set_json(&post_payload)
//         .to_request();
    
//     let post_resp = test::call_service(&app, post_req).await;
//     assert_eq!(post_resp.status(), StatusCode::CREATED);

//     let new_role_desc = "diffrnt role";
//     let new_access_lvl = 1;
//     let put_payload = NewTournamentAdmin {
//         role_description: new_role_desc.to_string(),            
//         access_lvl: new_access_lvl,
//         ..post_payload
//     };
    
//     let put_uri = format!("/api/tournaments/{}/admins/{}", tournament.tid, user.id);
//     let put_req = test::TestRequest::put()
//         .uri(&put_uri)
//         .set_json(&put_payload)
//         .to_request();

//     // Act:
    
//     let put_resp = test::call_service(&app, put_req).await;

//     // Assert:
    
//     assert_eq!(put_resp.status(), StatusCode::OK);

//     let put_resp_body: EntityResponse<TournamentAdmin> = test::read_body_json(put_resp).await;
//     assert_eq!(put_resp_body.code, 200);
//     assert_eq!(put_resp_body.message, "");

//     let updated_admin = put_resp_body.data.unwrap();
//     assert_eq!(updated_admin.adminid, user.id);
//     assert_eq!(updated_admin.role_description.unwrap(), new_role_desc);
//     assert_eq!(updated_admin.access_lvl, new_access_lvl);
//     assert_ne!(updated_admin.created_at, updated_admin.updated_at);
// }

// #[actix_web::test]
// async fn delete_admin_works() {

//     // Arrange:

//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");

//     let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
//     let user = fixtures::users::seed_user(&mut conn);

//     let payload = fixtures::tournaments_admins::get_tour_admin_payload_singular(tournament.tid, user.id);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let post_uri = format!("/api/tournaments/{}/admins/{}", tournament.tid, user.id);
//     let post_req = test::TestRequest::post()
//         .uri(&post_uri)
//         .set_json(payload)
//         .to_request();
    
//     let resp = test::call_service(&app, post_req).await;
    
//     assert_eq!(resp.status(), StatusCode::CREATED);
    
//     let delete_uri = format!("/api/tournaments/{}/admins/{}", tournament.tid, user.id);
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


//     let get_admins_uri = format!("/api/tournaments/{}/admins?page={}&page_size={}", tournament.tid, PAGE_NUM, PAGE_SIZE);
//     let get_admins_req = test::TestRequest::get()
//         .uri(&get_admins_uri)
//         .to_request();

//     let get_admins_resp = test::call_service(&app, get_admins_req).await;

//     assert_eq!(get_admins_resp.status(), StatusCode::OK);

//     let body: Vec<User> = test::read_body_json(get_admins_resp).await;
//     assert_eq!(body.len(), 0);
// }

// #[actix_web::test]
// async fn get_all_rooms_of_tournament_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let tournament = fixtures::tournaments::seed_get_rooms_by_tournament(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/tournaments/{}/rooms?page={}&page_size={}", tournament.tid, PAGE_NUM, PAGE_SIZE);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     // Act:
    
//     let resp = test::call_service(&app, req).await;
    
//     // Assert:
    
//     assert_eq!(resp.status(), StatusCode::OK);

//     let body: Vec<Room> = test::read_body_json(resp).await;

//     assert_eq!(body.len(), 3);

//     let mut room_or_interest_idx = 10;
//     for idx in 0..3 {
//         if body[idx].name == "Test Room 2" {
//             room_or_interest_idx = idx;
//             break;
//         }
//     }

//     let room_of_interest = &body[room_or_interest_idx];
//     assert_eq!(room_of_interest.tid, tournament.tid);
//     assert_eq!(room_of_interest.building.as_str(), "Bldng 2");
//     assert_eq!(room_of_interest.comments.as_str(), "I thought I recognized this place.");
// }

// #[actix_web::test]
// async fn get_all_rounds_of_tournament_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let tournament = fixtures::tournaments::seed_get_rounds_by_tournament(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/tournaments/{}/rounds?page={}&page_size={}", tournament.tid, PAGE_NUM, PAGE_SIZE);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:

//     let body: Vec<Round> = test::read_body_json(resp).await;

//     assert_eq!(body.len(), 6);

//     let mut round_1_idx = 10;
//     let mut round_2_idx = 10;
//     let mut round_3_idx = 10;
//     let mut round_4_idx = 10;
//     let mut round_5_idx = 10;
//     let mut round_6_idx = 10;
//     for idx in 0..6 {
//         if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2058, 5, 23, 00, 00, 0).unwrap() {
//             round_1_idx = idx;
//         }
//         if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2059, 5, 23, 00, 00, 0).unwrap() {
//             round_2_idx = idx;
//         }
//         if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2060, 5, 23, 00, 00, 0).unwrap() {
//             round_3_idx = idx;
//         }
//         if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2061, 5, 23, 00, 00, 0).unwrap() {
//             round_4_idx = idx;
//         }
//         if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2062, 5, 23, 00, 00, 0).unwrap() {
//             round_5_idx = idx;
//         }
//         if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2063, 5, 23, 00, 00, 0).unwrap() {
//             round_6_idx = idx;
//         }
//     }

//     // Tour 2 Div 1 Rounds:
//     assert_ne!(round_1_idx, 10);
//     assert_ne!(round_2_idx, 10);
//     assert_ne!(round_3_idx, 10);

//     // Tour 2 Div 2 Rounds:
//     assert_ne!(round_4_idx, 10);
//     assert_ne!(round_5_idx, 10);
//     assert_ne!(round_6_idx, 10);
// }

// #[actix_web::test]
// async fn get_all_games_of_tournament_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let (tour_2_id, game_1_of_tour_2, game_2_of_tour_2 ) = fixtures::games::seed_get_games_of_tournament(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/tournaments/{}/games?page={}&page_size={}", tour_2_id, PAGE_NUM, PAGE_SIZE);
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
//         if body[idx].gid == game_1_of_tour_2.gid {
//             game_1_idx = idx;
//         }
//         if body[idx].gid == game_2_of_tour_2.gid {
//             game_2_idx = idx;
//         }
//     }
//     assert_ne!(game_1_idx, 10);
//     assert_ne!(game_2_idx, 10);
// }

// #[actix_web::test]
// async fn get_all_tournamentgroups_of_tournament_works() {

//     // Arrange:
    
//     clean_database();
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
    
//     let (tour, tg_1, tg_2) = fixtures::tournaments::arrange_get_all_tournamentgroups_of_tournament_works_integration_test(&mut conn);

//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db))
//             .configure(configure_routes)
//     ).await;
    
//     let uri = format!("/api/tournaments/{}/tournamentgroups?page={}&page_size={}", tour.tid, PAGE_NUM, PAGE_SIZE);
//     let req = test::TestRequest::get()
//         .uri(&uri)
//         .to_request();
    
//     // Act:
    
//     let resp = test::call_service(&app, req).await;
//     assert_eq!(resp.status(), StatusCode::OK);

//     // Assert:

//     let body: Vec<TournamentGroup> = test::read_body_json(resp).await;

//     let len = 2;

//     assert_eq!(body.len(), len);

//     let mut tourgroup_1_idx = 10;
//     let mut tourgroup_2_idx = 10;
//     for idx in 0..len {
//         if body[idx].tgid == tg_1.tgid {
//             tourgroup_1_idx = idx;
//         }
//         if body[idx].tgid == tg_2.tgid {
//             tourgroup_2_idx = idx;
//         }
//     }
//     assert_ne!(tourgroup_1_idx, 10);
//     assert_ne!(tourgroup_2_idx, 10);
// }
