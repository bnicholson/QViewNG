
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::{database::Database, models::{self, apicalllog::ApiCalllog, create_tournament_applicant::CreateTournamentApplicant, role::AppRole}, services::common::PagedResponse};
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

    let (payload, user) = fixtures::create_tournament_applicants::arrange_create_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = "/api/createtournamentapplicants";
    let req = test::TestRequest::post()
        .uri(uri)
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<CreateTournamentApplicant> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let item = body.data.unwrap();
    assert_ne!(item.id, uuid::Uuid::nil());
    assert_eq!(item.user_id, user.id);
    assert_eq!(item.status.as_str(), "pending");
    assert_eq!(item.request_context.unwrap().as_str(), "I would like to create a tournament for my region.");
    assert_eq!(item.last_modified_user_id, user.id);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    fixtures::create_tournament_applicants::arrange_get_all_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/createtournamentapplicants?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let body: PagedResponse<CreateTournamentApplicant> = test::read_body_json(resp).await;
    assert_eq!(body.items.len(), 2);
    assert_eq!(body.count, 2);

    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
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

    let item = fixtures::create_tournament_applicants::arrange_get_by_id_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/createtournamentapplicants/{}", item.id);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let body: CreateTournamentApplicant = test::read_body_json(resp).await;
    assert_eq!(body.id, item.id);
    assert_eq!(body.user_id, item.user_id);
    assert_eq!(body.status, item.status);

    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
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

    let (item, user) = fixtures::create_tournament_applicants::arrange_update_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/createtournamentapplicants/{}", item.id);
    let payload = json!({
        "status": "approved",
        "last_modified_user_id": user.id
    });
    let req = test::TestRequest::put()
        .uri(&uri)
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let body: EntityResponse<CreateTournamentApplicant> = test::read_body_json(resp).await;
    assert_eq!(body.code, 200);

    let updated = body.data.unwrap();
    assert_eq!(updated.id, item.id);
    assert_eq!(updated.status.as_str(), "approved");
    assert_eq!(updated.last_modified_user_id, user.id);
    assert_ne!(updated.modified_at, item.modified_at);

    // When status is approved, the applicant's user should be assigned the tournament_manager role:
    let tournament_manager_role = models::role::read_by_name(&mut conn, AppRole::TournamentManager.as_str())
        .expect("tournament_owner role should exist");
    let users_roles = models::users_roles::read_all_for_user(&mut conn, item.user_id)
        .expect("Should be able to read users_roles");
    assert!(
        users_roles.iter().any(|ur| ur.role_id == tournament_manager_role.id),
        "Applicant user should have been assigned the tournament_owner role on approval"
    );

    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "PUT");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let item = fixtures::create_tournament_applicants::arrange_delete_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/createtournamentapplicants/{}", item.id);
    let req = test::TestRequest::delete()
        .uri(&uri)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    // Confirm it no longer exists:
    let get_req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    let get_resp = test::call_service(
        &test::init_service(
            App::new()
                .app_data(web::Data::new(Database::new(TEST_DB_URL)))
                .configure(configure_routes)
        ).await,
        get_req
    ).await;
    assert_eq!(get_resp.status(), StatusCode::NOT_FOUND);

    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "DELETE");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}
