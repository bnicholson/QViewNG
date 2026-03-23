mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::{
    database::Database,
    models::{self, apicalllog::ApiCalllog, role::Role, role_permission::RolePermission},
    routes::configure_routes,
    services::common::EntityResponse,
};
use serde_json::json;
use crate::common::{TEST_DB_URL, clean_database};

// ── helpers ──────────────────────────────────────────────────────────────────

fn make_app_and_db() -> (Database, impl std::future::Future<Output = impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
>>) {
    let db = Database::new(TEST_DB_URL);
    let db_clone = Database::new(TEST_DB_URL);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db_clone))
            .configure(configure_routes)
    );
    (db, app)
}

// ── POST /api/roles ───────────────────────────────────────────────────────────

#[actix_web::test]
async fn create_role_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = "/api/roles";
    let req = test::TestRequest::post()
        .uri(uri)
        .set_json(json!({ "name": "admin", "description": "Full access" }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Role> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    let role = body.data.unwrap();
    assert_eq!(role.name, "admin");
    assert_eq!(role.description.as_deref(), Some("Full access"));

    let logs: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].method.as_str(), "POST");
    assert_eq!(logs[0].uri, uri);
}

// ── GET /api/roles ────────────────────────────────────────────────────────────

#[actix_web::test]
async fn get_all_roles_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    fixtures::roles::seed_roles(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = "/api/roles";
    let req = test::TestRequest::get().uri(uri).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let roles: Vec<Role> = test::read_body_json(resp).await;
    assert_eq!(roles.len(), 3);
    // Returned ordered by name asc
    assert_eq!(roles[0].name, "admin");
    assert_eq!(roles[1].name, "editor");
    assert_eq!(roles[2].name, "viewer");

    let logs: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].method.as_str(), "GET");
}

// ── GET /api/roles/{id} ───────────────────────────────────────────────────────

#[actix_web::test]
async fn get_role_by_id_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let role = fixtures::roles::seed_role(&mut conn, "editor");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/roles/{}", role.id);
    let req = test::TestRequest::get().uri(&uri).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Role = test::read_body_json(resp).await;
    assert_eq!(body.id, role.id);
    assert_eq!(body.name, "editor");
}

#[actix_web::test]
async fn get_role_by_id_not_found() {
    clean_database();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let req = test::TestRequest::get().uri("/api/roles/99999").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ── PUT /api/roles/{id} ───────────────────────────────────────────────────────

#[actix_web::test]
async fn update_role_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let role = fixtures::roles::seed_role(&mut conn, "viewer");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/roles/{}", role.id);
    let req = test::TestRequest::put()
        .uri(&uri)
        .set_json(json!({ "name": "super-viewer", "description": "Updated desc" }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: EntityResponse<Role> = test::read_body_json(resp).await;
    let updated = body.data.unwrap();
    assert_eq!(updated.name, "super-viewer");
    assert_eq!(updated.description.as_deref(), Some("Updated desc"));
    assert_ne!(updated.created_at, updated.updated_at);
}

// ── DELETE /api/roles/{id} ────────────────────────────────────────────────────

#[actix_web::test]
async fn delete_role_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let role = fixtures::roles::seed_role(&mut conn, "temp-role");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let delete_uri = format!("/api/roles/{}", role.id);
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

// ── POST /api/roles/{role_id}/permissions/{permission_id} ────────────────────

#[actix_web::test]
async fn add_permission_to_role_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let role = fixtures::roles::seed_role(&mut conn, "editor");
    let perm = fixtures::permissions::seed_permission(&mut conn, "post:read", "post", "read");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/roles/{}/permissions/{}", role.id, perm.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::post().uri(&uri).to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<RolePermission> = test::read_body_json(resp).await;
    let rp = body.data.unwrap();
    assert_eq!(rp.role_id, role.id);
    assert_eq!(rp.permission_id, perm.id);
}

// ── GET /api/roles/{id}/permissions ──────────────────────────────────────────

#[actix_web::test]
async fn get_role_permissions_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let role = fixtures::roles::seed_role(&mut conn, "admin");
    let perms = fixtures::permissions::seed_permissions(&mut conn);
    // Link first two permissions to the role directly via model
    models::role_permission::create(
        &mut conn,
        backend::models::role_permission::NewRolePermission { role_id: role.id, permission_id: perms[0].id },
    ).unwrap();
    models::role_permission::create(
        &mut conn,
        backend::models::role_permission::NewRolePermission { role_id: role.id, permission_id: perms[1].id },
    ).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/roles/{}/permissions", role.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&uri).to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<RolePermission> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 2);
}

// ── DELETE /api/roles/{role_id}/permissions/{permission_id} ──────────────────

#[actix_web::test]
async fn remove_permission_from_role_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let role = fixtures::roles::seed_role(&mut conn, "editor");
    let perm = fixtures::permissions::seed_permission(&mut conn, "post:write", "post", "write");
    models::role_permission::create(
        &mut conn,
        backend::models::role_permission::NewRolePermission { role_id: role.id, permission_id: perm.id },
    ).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/roles/{}/permissions/{}", role.id, perm.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::delete().uri(&uri).to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let remaining = models::role_permission::read_all_for_role(&mut conn, role.id).unwrap();
    assert!(remaining.is_empty());
}
