
mod common;
mod fixtures;

use actix_web::{test, App, web::{self,Bytes}, http::StatusCode};
use chrono::{Duration, Local, NaiveDate, TimeZone, Utc};
use backend::{database::seed_data::system_default_data::insert_system_default_data, models::{self, apicalllog::ApiCalllog, equipmentregistration::EquipmentRegistration, game::Game, role::AppRole, room::Room, round::Round, team::TeamWithCoach, tournament_admin::{TournamentAdmin, TournamentAdminChangeset}, tournamentgroup::TournamentGroup, user::User}, routes::configure_routes, services::{common::{EntityResponse, PagedResponse}, tournament::TournamentWithRooms}};
use backend::models::{division::Division, tournament::Tournament};
use backend::database::Database;
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database};

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    fixtures::tournaments::seed_tournaments(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/tournaments?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: PagedResponse<Tournament> = test::read_body_json(resp).await;

    let len = 3;

    assert_eq!(body.items.len(), len);
    assert_eq!(body.count, len as i64);

    let mut tour_or_interest_idx = 10;
    for idx in 0..len {
        if body.items[idx].tname == "Tour 2" {
            tour_or_interest_idx = idx;
            break;
        }
    }

    let tour_of_interest = &body.items[tour_or_interest_idx];
    assert_eq!(tour_of_interest.organization,"Nazarene");
    assert_ne!(tour_of_interest.tid.to_string().as_str(),"");  // "ne" in "assert_ne!" means Not Equal
    assert_eq!(tour_of_interest.breadcrumb,"/test/bread/crumb");
    assert_eq!(tour_of_interest.fromdate, NaiveDate::from_ymd_opt(2025, 5, 23).unwrap());
    assert_eq!(tour_of_interest.todate, NaiveDate::from_ymd_opt(2025, 5, 27).unwrap());
    assert_eq!(tour_of_interest.venue,"Olivet Nazarene University");
    assert_eq!(tour_of_interest.city,"Bourbonnais");
    assert_eq!(tour_of_interest.region,"Central USA");
    assert_eq!(tour_of_interest.country,"USA");
    assert_eq!(tour_of_interest.contact,"Jason Morton");
    assert_eq!(tour_of_interest.contactemail,"jasonmorton@fakeemail.com");
    assert_eq!(tour_of_interest.is_public,false);
    assert_eq!(tour_of_interest.shortinfo,"NYI International quiz meet of 2025.");
    assert_eq!(tour_of_interest.info,"If I wanted a longer description I would have provided it here.");

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

    let tournaments = fixtures::tournaments::seed_tournaments(&mut conn);
    let user = fixtures::users::seed_user(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/tournaments/{}", &tournaments[0].tid);

    // Act + Assert: an unauthenticated request gets the tournament without pairing_code.

    let anon_req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    let anon_resp = test::call_service(&app, anon_req).await;
    assert_eq!(anon_resp.status(), StatusCode::OK);

    let anon_body: serde_json::Value = test::read_body_json(anon_resp).await;
    assert_eq!(anon_body.get("tname").and_then(|v| v.as_str()), Some("Q2025"));
    assert_eq!(
        anon_body.get("tid").and_then(|v| v.as_str()),
        Some(tournaments[0].tid.to_string().as_str())
    );
    assert_eq!(anon_body.get("organization").and_then(|v| v.as_str()), Some("Nazarene"));
    assert!(
        anon_body.get("pairing_code").is_none(),
        "pairing_code must be hidden from unauthenticated callers"
    );

    // Act + Assert: a `member`-only request also gets the tournament without pairing_code.

    let member_token = common::make_token(
        user.id,
        vec!["member".to_string()],
        vec!["tournament:read".to_string()],
    );
    let member_req = test::TestRequest::get()
        .uri(uri.as_str())
        .insert_header(("Authorization", format!("Bearer {}", member_token)))
        .to_request();

    let member_resp = test::call_service(&app, member_req).await;
    assert_eq!(member_resp.status(), StatusCode::OK);

    let member_body: serde_json::Value = test::read_body_json(member_resp).await;
    assert!(
        member_body.get("pairing_code").is_none(),
        "pairing_code must be hidden from `member` callers"
    );

    // Act + Assert: TournamentManager, TournamentAdmin, and SuperUser all see pairing_code.

    for role in [
        AppRole::TournamentManager.as_str(),
        AppRole::TournamentAdmin.as_str(),
        AppRole::SuperUser.as_str(),
    ] {
        let privileged_token = common::make_token(
            user.id,
            vec![role.to_string()],
            vec!["tournament:read".to_string()],
        );
        let privileged_req = test::TestRequest::get()
            .uri(uri.as_str())
            .insert_header(("Authorization", format!("Bearer {}", privileged_token)))
            .to_request();

        let privileged_resp = test::call_service(&app, privileged_req).await;
        assert_eq!(privileged_resp.status(), StatusCode::OK);

        let privileged_body: serde_json::Value = test::read_body_json(privileged_resp).await;
        assert_eq!(
            privileged_body.get("pairing_code").and_then(|v| v.as_str()),
            Some(tournaments[0].pairing_code.as_str()),
            "{} must receive pairing_code in the response",
            role
        );
    }

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    // 1 anon + 1 member + 3 privileged = 5 calls
    assert_eq!(apicalllog_records.iter().count(), 5);
    assert!(apicalllog_records.iter().all(|r| r.method.as_str() == "GET"));
    assert!(apicalllog_records.iter().all(|r| r.uri.as_str() == uri));
}

#[actix_web::test]
async fn get_all_in_date_range_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    fixtures::tournaments::seed_tournaments_for_get_all_in_date_range(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    // Act:
    let today: NaiveDate = Local::now().date_naive();
    let sub_ten_days: NaiveDate = today - Duration::days(10);
    let add_ten_days: NaiveDate = today + Duration::days(10);
    let sub_ten_days_millis: i64 = sub_ten_days.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_millis();
    let add_ten_days_millis: i64 = add_ten_days.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_millis();
    let uri = format!("/api/tournaments/filter?from_date={}&to_date={}", sub_ten_days_millis, add_ten_days_millis);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<Tournament> = test::read_body_json(resp).await;
    assert_eq!(body.iter().count(), 3);

    let sub_8_days_from_today: NaiveDate = today - Duration::days(8);
    let add_8_days_to_today: NaiveDate = today + Duration::days(8);
    let add_12_days_to_today: NaiveDate = today + Duration::days(12);  // outside of range

    let mut today_tour_idx = 10;
    let mut sub_8_tour_idx = 10;
    let mut add_8_tour_idx = 10;
    for idx in 0..3 {
        if body[idx].fromdate == today {
            today_tour_idx = idx;
        }
        else if body[idx].fromdate == sub_8_days_from_today {
            sub_8_tour_idx = idx;
        }
        else {
            add_8_tour_idx = idx;
        }
    }

    assert_eq!(body[today_tour_idx].tname, "Today Exactly");
    assert_eq!(body[today_tour_idx].fromdate, today);
    assert_eq!(body[today_tour_idx].todate, today);
    
    assert_eq!(body[sub_8_tour_idx].tname, "eight days past exactly");
    assert_eq!(body[sub_8_tour_idx].fromdate, sub_8_days_from_today);
    assert_eq!(body[sub_8_tour_idx].todate, sub_8_days_from_today);
    
    assert_eq!(body[add_8_tour_idx].tname, "eight to twelve days future");
    assert_eq!(body[add_8_tour_idx].fromdate, add_8_days_to_today);
    assert_eq!(body[add_8_tour_idx].todate, add_12_days_to_today);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

fn tournament_create_payload() -> serde_json::Value {
    serde_json::json!({
        "organization": "Nazarene",
        "tname": "Test Tour",
        "breadcrumb": "/test/post",
        "fromdate": "2025-05-23",
        "todate": "2025-05-27",
        "venue": "Vancouver University",
        "city": "Vancouver",
        "region": "North America",
        "country": "Canada",
        "contact": "primemin",
        "contactemail": "primemin@fakeemail.com",
        "shortinfo": "Winter Olympics",
        "info": "Shawn White did excellent in the halfpipe."
    })
}

#[actix_web::test]
async fn create_with_tournament_create_permission_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // A real user is required because the endpoint sets owner_id = user_ctx.user_id,
    // which is a FK to users.id.
    let user = fixtures::users::seed_user(&mut conn);
    let token = common::make_token(
        user.id,
        vec!["tournament_manager".to_string()],
        vec!["tournament:create".to_string()],
    );

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/tournaments")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&tournament_create_payload())
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Tournament> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let tournament = body.data.unwrap();
    assert_ne!(tournament.tid.to_string().as_str(), "");
    assert_eq!(tournament.organization.as_str(), "Nazarene");
    assert_eq!(tournament.tname.as_str(), "Test Tour");
    assert_eq!(tournament.owner_id, user.id);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let logs: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(logs.iter().count(), 1);
    assert_eq!(logs.first().unwrap().method.as_str(), "POST");
    assert_eq!(logs.first().unwrap().uri.as_str(), "/api/tournaments");
}

#[actix_web::test]
async fn create_as_super_user_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let user = fixtures::users::seed_user(&mut conn);
    let token = common::make_token(
        user.id,
        vec!["super_user".to_string()],
        vec!["tournament:create".to_string()],
    );

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/tournaments")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&tournament_create_payload())
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Tournament> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    let tournament = body.data.unwrap();
    assert_eq!(tournament.owner_id, user.id);
}

#[actix_web::test]
async fn create_with_insufficient_permissions_returns_401() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let user = fixtures::users::seed_user(&mut conn);
    // member role with only read access — no tournament:create permission.
    let token = common::make_token(
        user.id,
        vec!["member".to_string()],
        vec!["tournament:read".to_string()],
    );

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/tournaments")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&tournament_create_payload())
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (tournament, owner) = fixtures::tournaments::arrange_update_works_integration_test(&mut conn);
    let token = common::make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["tournament:update".to_string()],
    );

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let new_venue = "Albatross Academy".to_string();
    let new_todate = NaiveDate::from_ymd_opt(2025, 5, 30).unwrap();
    let new_info = "Sadly, Shawn White is retired from pro snowboarding now.".to_string();

    let put_payload = json!({
        "venue": &new_venue,
        "todate": new_todate,
        "info": &new_info
    });

    let put_uri = format!("/api/tournaments/{}", &tournament.tid);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<Tournament> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let new_tournament = put_resp_body.data.unwrap();
    assert_eq!(new_tournament.tid, tournament.tid);
    assert_eq!(new_tournament.organization.as_str(), "Nazarene");
    assert_eq!(new_tournament.tname.as_str(), "Test Tour");
    assert_eq!(new_tournament.venue.as_str(), new_venue);
    assert_eq!(new_tournament.todate, new_todate);
    assert_eq!(new_tournament.info.as_str(), new_info);
    assert_ne!(new_tournament.created_at, new_tournament.updated_at);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "PUT");
    assert_eq!(apicalllog_records.first().unwrap().uri.as_str(), format!("/api/tournaments/{}", tournament.tid));

    // A member without tournament:update permission should be rejected:
    let member_token = common::make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["tournament:read".to_string()],
    );
    let unauthorized_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", member_token)))
        .set_json(&put_payload)
        .to_request();
    let unauthorized_resp = test::call_service(&app, unauthorized_req).await;
    assert_eq!(unauthorized_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let delete_uri = format!("/api/tournaments/{}", tournament.tid);
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


    let get_by_id_uri = format!("/api/tournaments/{}", tournament.tid);
    let get_by_id_req = test::TestRequest::get()
        .uri(&get_by_id_uri)
        .to_request();

    let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

    assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 2);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "DELETE");
    assert_eq!(apicalllog_records.first().unwrap().uri.as_str(), format!("/api/tournaments/{}", tournament.tid));
}

#[actix_web::test]
async fn get_all_divisions_of_tournament_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");

    let tournament = fixtures::tournaments::seed_get_divisions_by_tournament(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/tournaments/{}/divisions?page={}&page_size={}", tournament.tid, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Division> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 3);

    let mut div_1_idx = 10;
    let mut div_2_idx = 10;
    let mut div_3_idx = 10;
    for idx in 0..3 {
        if body[idx].dname == "Test Div 9" {
            div_1_idx = idx;
        }
        if body[idx].dname == "Test Div 2" {
            div_2_idx = idx;
        }
        if body[idx].dname == "Test Div 7" {
            div_3_idx = idx;
        }
    }
    assert_ne!(div_1_idx, 10);
    assert_ne!(div_2_idx, 10);
    assert_ne!(div_3_idx, 10);
    // overkill, but thorough:
    assert_eq!(body[div_1_idx].dname, "Test Div 9");
    assert_eq!(body[div_2_idx].dname, "Test Div 2");
    assert_eq!(body[div_3_idx].dname, "Test Div 7");

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

#[actix_web::test]
async fn add_admin_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    insert_system_default_data(&mut conn);

    let (tournament, user_to_become_admin, payload) =
        fixtures::tournaments_admins::get_tour_admin_payload_singular(&mut conn);

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

    // Assert that the user was assigned the TournamentAdmin role:
    let tournament_admin_role = models::role::read_by_name(&mut conn, AppRole::TournamentAdmin.as_str())
        .expect("TournamentAdmin role should exist");
    let user_roles = models::users_roles::read_all_for_user(&mut conn, user_to_become_admin.id)
        .expect("Should be able to read user roles");
    assert!(
        user_roles.iter().any(|ur| ur.role_id == tournament_admin_role.id),
        "User should have been assigned the TournamentAdmin role"
    );

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

#[actix_web::test]
async fn get_all_admins_of_tournament_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (tour, admin_1, admin_2) = 
        fixtures::tournaments::arrange_get_all_admins_of_tournament_works_integration_test(&mut conn);
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/tournaments/{}/admins?page={}&page_size={}", tour.tid, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<User> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 2);

    let mut admin_1_idx = 10;
    let mut admin_2_idx = 10;
    for idx in 0..2 {
        if body[idx].id == admin_1.adminid {
            admin_1_idx = idx;
        }
        if body[idx].id == admin_2.adminid {
            admin_2_idx = idx;
        }
    }
    assert_ne!(admin_1_idx, 10);
    assert_ne!(admin_2_idx, 10);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records[0].method.as_str(), "GET");
    assert_eq!(apicalllog_records[0].uri, uri);
}

#[actix_web::test]
async fn update_admin_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (tour, user, _) = 
        fixtures::tournaments_admins::arrange_update_admin_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let new_role_desc = "diffrnt role";
    let new_access_lvl = 1;
    let put_payload = TournamentAdminChangeset {
        role_description: new_role_desc.to_string(),            
        access_lvl: new_access_lvl
    };
    
    let put_uri = format!("/api/tournaments/{}/admins/{}", tour.tid, user.id);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<TournamentAdmin> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let updated_admin = put_resp_body.data.unwrap();
    assert_eq!(updated_admin.adminid, user.id);
    assert_eq!(updated_admin.role_description.unwrap(), new_role_desc);
    assert_eq!(updated_admin.access_lvl, new_access_lvl);
    assert_ne!(updated_admin.created_at, updated_admin.updated_at);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records[0].method.as_str(), "PUT");
    assert_eq!(apicalllog_records[0].uri.as_str(), put_uri);
}

#[actix_web::test]
async fn delete_admin_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (tournament, user, _) = 
        fixtures::tournaments::arrange_delete_admin_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let delete_uri = format!("/api/tournaments/{}/admins/{}", tournament.tid, user.id);
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


    let get_admins_uri = format!("/api/tournaments/{}/admins?page={}&page_size={}", tournament.tid, PAGE_NUM, PAGE_SIZE);
    let get_admins_req = test::TestRequest::get()
        .uri(&get_admins_uri)
        .to_request();

    let get_admins_resp = test::call_service(&app, get_admins_req).await;

    assert_eq!(get_admins_resp.status(), StatusCode::OK);

    let body: Vec<User> = test::read_body_json(get_admins_resp).await;
    assert_eq!(body.len(), 0);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 2);
    assert_eq!(apicalllog_records[0].method.as_str(), "DELETE");
    assert_eq!(apicalllog_records[0].uri, delete_uri);
}

#[actix_web::test]
async fn get_all_rooms_of_tournament_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_get_rooms_by_tournament(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/tournaments/{}/rooms?page={}&page_size={}", tournament.tid, PAGE_NUM, PAGE_SIZE);
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
        if body[idx].name == "Test Room 2" {
            room_or_interest_idx = idx;
            break;
        }
    }

    let room_of_interest = &body[room_or_interest_idx];
    assert_eq!(room_of_interest.tid, tournament.tid);
    assert_eq!(room_of_interest.building.as_str(), "Bldng 2");
    assert_eq!(room_of_interest.comments.as_str(), "I thought I recognized this place.");

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records[0].method.as_str(), "GET");
    assert_eq!(apicalllog_records[0].uri, uri);
}

#[actix_web::test]
async fn get_all_rounds_of_tournament_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_get_rounds_by_tournament(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/tournaments/{}/rounds?page={}&page_size={}", tournament.tid, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Round> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 6);

    let mut round_1_idx = 10;
    let mut round_2_idx = 10;
    let mut round_3_idx = 10;
    let mut round_4_idx = 10;
    let mut round_5_idx = 10;
    let mut round_6_idx = 10;
    for idx in 0..6 {
        if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2058, 5, 23, 00, 00, 0).unwrap() {
            round_1_idx = idx;
        }
        if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2059, 5, 23, 00, 00, 0).unwrap() {
            round_2_idx = idx;
        }
        if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2060, 5, 23, 00, 00, 0).unwrap() {
            round_3_idx = idx;
        }
        if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2061, 5, 23, 00, 00, 0).unwrap() {
            round_4_idx = idx;
        }
        if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2062, 5, 23, 00, 00, 0).unwrap() {
            round_5_idx = idx;
        }
        if body[idx].scheduled_start_time.unwrap() == Utc.with_ymd_and_hms(2063, 5, 23, 00, 00, 0).unwrap() {
            round_6_idx = idx;
        }
    }

    // Tour 2 Div 1 Rounds:
    assert_ne!(round_1_idx, 10);
    assert_ne!(round_2_idx, 10);
    assert_ne!(round_3_idx, 10);

    // Tour 2 Div 2 Rounds:
    assert_ne!(round_4_idx, 10);
    assert_ne!(round_5_idx, 10);
    assert_ne!(round_6_idx, 10);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records[0].method.as_str(), "GET");
    assert_eq!(apicalllog_records[0].uri, uri);
}

#[actix_web::test]
async fn get_all_games_of_tournament_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (tour_2_id, game_1_of_tour_2, game_2_of_tour_2 ) = fixtures::games::seed_get_games_of_tournament(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/tournaments/{}/games?page={}&page_size={}", tour_2_id, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: PagedResponse<Game> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.items.len(), len);
    assert_eq!(body.count, len as i64);

    let mut game_1_idx = 10;
    let mut game_2_idx = 10;
    for idx in 0..len {
        if body.items[idx].gid == game_1_of_tour_2.gid {
            game_1_idx = idx;
        }
        if body.items[idx].gid == game_2_of_tour_2.gid {
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
    assert_eq!(apicalllog_records[0].method.as_str(), "GET");
    assert_eq!(apicalllog_records[0].uri, uri);
}

#[actix_web::test]
async fn get_all_tournamentgroups_of_tournament_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (tour, tg_1, tg_2) = fixtures::tournaments::arrange_get_all_tournamentgroups_of_tournament_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/tournaments/{}/tournamentgroups?page={}&page_size={}", tour.tid, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<TournamentGroup> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut tourgroup_1_idx = 10;
    let mut tourgroup_2_idx = 10;
    for idx in 0..len {
        if body[idx].tgid == tg_1.tgid {
            tourgroup_1_idx = idx;
        }
        if body[idx].tgid == tg_2.tgid {
            tourgroup_2_idx = idx;
        }
    }
    assert_ne!(tourgroup_1_idx, 10);
    assert_ne!(tourgroup_2_idx, 10);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records[0].method.as_str(), "GET");
    assert_eq!(apicalllog_records[0].uri, uri);
}

#[actix_web::test]
async fn get_all_equipmentregistrations_of_tournament_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (tour, er_computer, er_jumppad, er_interfacebox, er_monitor, er_microphonerecorder, er_projector, er_powerstrip, er_extensioncord) = 
        fixtures::tournaments::arrange_get_all_equipmentregistrations_of_tournament_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/tournaments/{}/equipmentregistrations?page={}&page_size={}", tour.tid, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<EquipmentRegistration> = test::read_body_json(resp).await;

    let len = 8;

    assert_eq!(body.len(), len);

    let mut equipmentregistration_1_idx = 10;
    let mut equipmentregistration_2_idx = 10;
    let mut equipmentregistration_3_idx = 10;
    let mut equipmentregistration_4_idx = 10;
    let mut equipmentregistration_5_idx = 10;
    let mut equipmentregistration_6_idx = 10;
    let mut equipmentregistration_7_idx = 10;
    let mut equipmentregistration_8_idx = 10;
    for idx in 0..len {
        if body[idx].id == er_computer.id {
            equipmentregistration_1_idx = idx;
        }
        if body[idx].id == er_jumppad.id {
            equipmentregistration_2_idx = idx;
        }
        if body[idx].id == er_interfacebox.id {
            equipmentregistration_3_idx = idx;
        }
        if body[idx].id == er_monitor.id {
            equipmentregistration_4_idx = idx;
        }
        if body[idx].id == er_microphonerecorder.id {
            equipmentregistration_5_idx = idx;
        }
        if body[idx].id == er_projector.id {
            equipmentregistration_6_idx = idx;
        }
        if body[idx].id == er_powerstrip.id {
            equipmentregistration_7_idx = idx;
        }
        if body[idx].id == er_extensioncord.id {
            equipmentregistration_8_idx = idx;
        }
    }
    assert_ne!(equipmentregistration_1_idx, 10);
    assert_ne!(equipmentregistration_2_idx, 10);
    assert_ne!(equipmentregistration_3_idx, 10);
    assert_ne!(equipmentregistration_4_idx, 10);
    assert_ne!(equipmentregistration_5_idx, 10);
    assert_ne!(equipmentregistration_6_idx, 10);
    assert_ne!(equipmentregistration_7_idx, 10);
    assert_ne!(equipmentregistration_8_idx, 10);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records[0].method.as_str(), "GET");
    assert_eq!(apicalllog_records[0].uri, uri);
}

#[actix_web::test]
async fn get_all_teams_of_tournament_works() {

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

    let uri = format!("/api/tournaments/{}/teams?page={}&page_size={}", tournament.tid, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let body: PagedResponse<TeamWithCoach> = test::read_body_json(resp).await;

    assert_eq!(body.items.len(), 3);
    assert_eq!(body.count, 3);

    let mut team_of_interest_idx = 10;
    for idx in 0..3 {
        if body.items[idx].name == "Luke Found a Frog" {
            team_of_interest_idx = idx;
            break;
        }
    }
    assert_ne!(team_of_interest_idx, 10);
    assert_eq!(body.items[team_of_interest_idx].did, division.did);
    assert_eq!(body.items[team_of_interest_idx].coach_name, "Kimberly Maurice Den");

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

#[actix_web::test]
async fn get_all_quizzers_of_tournament_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    // seed_teams creates 3 teams: Team 1 (0 quizzers), Come Get Some (2), Luke Found a Frog (6)
    fixtures::teams::seed_teams(&mut conn, division.did);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/tournaments/{}/quizzers", tournament.tid);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let body: PagedResponse<User> = test::read_body_json(resp).await;

    assert_eq!(body.count, 8);
    assert_eq!(body.items.len(), 8);

    // Verify a specific quizzer is present
    let tyler = body.items.iter().find(|u| u.fname == "Tyler");
    assert!(tyler.is_some());
    assert_eq!(tyler.unwrap().lname, "Den");

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

#[actix_web::test]
async fn get_today_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (tour_in_window, rooms, _tour_out_of_window) =
        fixtures::tournaments::arrange_today_max_100_works(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = "/api/tournaments/today";
    let req = test::TestRequest::get()
        .uri(uri)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: EntityResponse<Vec<TournamentWithRooms>> = test::read_body_json(resp).await;
    assert_eq!(body.code, 200);
    assert_eq!(body.message, "OK");
    let tournaments_with_rooms = body.data.expect("data missing from read_today payload");

    assert_eq!(tournaments_with_rooms.len(), 1);
    assert_eq!(tournaments_with_rooms[0].tournament.tid, tour_in_window.tid);
    assert_eq!(tournaments_with_rooms[0].tournament.tname, "Today Max 100 In Window");
    assert_eq!(tournaments_with_rooms[0].rooms.len(), 3);

    let mut room_alpha_idx = 10;
    for idx in 0..3 {
        if tournaments_with_rooms[0].rooms[idx].name == "Room Alpha" {
            room_alpha_idx = idx;
        }
    }
    assert_ne!(room_alpha_idx, 10);
    assert_eq!(tournaments_with_rooms[0].rooms[room_alpha_idx].tid, tour_in_window.tid);
    assert_eq!(tournaments_with_rooms[0].rooms[room_alpha_idx].roomid, rooms[0].roomid);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri.as_str(), uri);
}
