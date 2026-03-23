mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::{
    database::Database,
    models::{self, apicalllog::ApiCalllog, permission::Permission},
    routes::configure_routes,
    services::common::EntityResponse,
};
use serde_json::json;
use crate::common::{TEST_DB_URL, clean_database};

// ── POST /api/permissions ─────────────────────────────────────────────────────

#[actix_web::test]
async fn create_permission_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = "/api/permissions";
    let req = test::TestRequest::post()
        .uri(uri)
        .set_json(json!({ "name": "post:create", "resource": "post", "action": "create" }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Permission> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    let perm = body.data.unwrap();
    assert_eq!(perm.name, "post:create");
    assert_eq!(perm.resource.as_deref(), Some("post"));
    assert_eq!(perm.action.as_deref(), Some("create"));

    let logs: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].method.as_str(), "POST");
    assert_eq!(logs[0].uri, uri);
}

// ── GET /api/permissions ──────────────────────────────────────────────────────

#[actix_web::test]
async fn get_all_permissions_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    fixtures::permissions::seed_permissions(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = "/api/permissions";
    let req = test::TestRequest::get().uri(uri).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let perms: Vec<Permission> = test::read_body_json(resp).await;
    assert_eq!(perms.len(), 6);
}

// ── GET /api/permissions?resource=post ───────────────────────────────────────

#[actix_web::test]
async fn get_permissions_filtered_by_resource_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    fixtures::permissions::seed_permissions(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = "/api/permissions?resource=post";
    let req = test::TestRequest::get().uri(uri).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let perms: Vec<Permission> = test::read_body_json(resp).await;
    assert_eq!(perms.len(), 4);
    assert!(perms.iter().all(|p| p.resource.as_deref() == Some("post")));
}

// ── GET /api/permissions/{id} ─────────────────────────────────────────────────

#[actix_web::test]
async fn get_permission_by_id_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let perm = fixtures::permissions::seed_permission(&mut conn, "user:read", "user", "read");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/permissions/{}", perm.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&uri).to_request(),
    ).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Permission = test::read_body_json(resp).await;
    assert_eq!(body.id, perm.id);
    assert_eq!(body.name, "user:read");
}

#[actix_web::test]
async fn get_permission_by_id_not_found() {
    clean_database();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri("/api/permissions/99999").to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ── PUT /api/permissions/{id} ─────────────────────────────────────────────────

#[actix_web::test]
async fn update_permission_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let perm = fixtures::permissions::seed_permission(&mut conn, "post:read", "post", "read");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/permissions/{}", perm.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::put()
            .uri(&uri)
            .set_json(json!({ "name": "post:view", "resource": "post", "action": "view" }))
            .to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: EntityResponse<Permission> = test::read_body_json(resp).await;
    let updated = body.data.unwrap();
    assert_eq!(updated.name, "post:view");
    assert_eq!(updated.action.as_deref(), Some("view"));
    assert_ne!(updated.created_at, updated.updated_at);
}

// ── DELETE /api/permissions/{id} ──────────────────────────────────────────────

#[actix_web::test]
async fn delete_permission_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let perm = fixtures::permissions::seed_permission(&mut conn, "temp:perm", "temp", "perm");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let delete_uri = format!("/api/permissions/{}", perm.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::delete().uri(&delete_uri).to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let get_resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&delete_uri).to_request(),
    ).await;
    assert_eq!(get_resp.status(), StatusCode::NOT_FOUND);
}
