mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::{
    database::{Database, seed_data::system_default_data::insert_system_default_data},
    models::{self, apicalllog::ApiCalllog, permission::Permission, role::AppRole, users_roles::UsersRolesBuilder},
    routes::configure_routes,
    services::common::{EntityResponse, PagedResponse},
};
use serde_json::json;
use crate::common::{TEST_DB_URL, clean_database};

#[derive(serde::Deserialize)]
struct RolesAndPermissions {
    roles: Vec<String>,
    permissions: Vec<String>,
}

// ── POST /api/permissions ─────────────────────────────────────────────────────

#[actix_web::test]
async fn create_permission_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let created = models::permission::create(&mut conn, backend::models::permission::NewPermission { name: "post:create".to_string(), resource: Some("post".to_string()), action: Some("create".to_string()) }).expect("create failed");
    assert_eq!(created.name, "post:create");
}

// ── GET /api/permissions ──────────────────────────────────────────────────────

#[actix_web::test]
async fn get_all_permissions_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    fixtures::permissions::seed_permissions(&mut conn);

    let all = models::permission::read_all(&mut conn).expect("read_all failed");
    assert!(!all.is_empty());
}

// ── GET /api/permissions?resource=post ───────────────────────────────────────

#[actix_web::test]
async fn get_permissions_filtered_by_resource_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    fixtures::permissions::seed_permissions(&mut conn);

    let filtered = models::permission::read_all_for_resource(&mut conn, "post").expect("read_all_for_resource failed");
    assert!(!filtered.is_empty());
    assert!(filtered.iter().all(|p| p.resource.as_deref() == Some("post")));
}

// ── GET /api/permissions/{id} ─────────────────────────────────────────────────

// ── PUT /api/permissions/{id} ─────────────────────────────────────────────────

#[actix_web::test]
async fn update_permission_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let perm = fixtures::permissions::seed_permission(&mut conn, "post:read", "post", "read");

    let changeset = backend::models::permission::PermissionChangeset { name: Some("post:view".to_string()), resource: Some("post".to_string()), action: Some("view".to_string()) };
    let updated = models::permission::update(&mut conn, perm.id, &changeset).expect("update failed");
    assert_eq!(updated.name, "post:view");
}

// ── DELETE /api/permissions/{id} ──────────────────────────────────────────────

#[actix_web::test]
async fn delete_permission_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let perm = fixtures::permissions::seed_permission(&mut conn, "temp:perm", "temp", "perm");

    let count = models::permission::delete(&mut conn, perm.id).expect("delete failed");
    assert_eq!(count, 1);
}

// ── GET /api/users/{id}/roles-and-permissions ─────────────────────────────────

#[actix_web::test]
async fn get_user_roles_and_permissions_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    insert_system_default_data(&mut conn);
    let user = fixtures::users::seed_user(&mut conn);
    let tour_admin_role = models::role::read_by_name(&mut conn, AppRole::TournamentAdmin.as_str())
        .expect("TournamentAdmin role should exist");
    UsersRolesBuilder::new(user.id)
        .assign(tour_admin_role.id)
        .build_and_insert(&mut conn)
        .expect("Failed to assign role");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/users/{}/roles-and-permissions", user.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&uri).to_request(),
    ).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: RolesAndPermissions = test::read_body_json(resp).await;
    assert_eq!(body.roles, vec![AppRole::TournamentAdmin.as_str()]);
    assert!(!body.permissions.is_empty());
    assert!(body.permissions.iter().any(|p| p == "tournament:update"));
    assert!(!body.permissions.iter().any(|p| p == "tournament:create"),
        "TournamentAdmin should not have tournament:create");

    let logs: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].method.as_str(), "GET");
    assert_eq!(logs[0].uri, uri);
}

#[actix_web::test]
async fn get_user_roles_and_permissions_empty_for_user_with_no_roles() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let user = fixtures::users::seed_user(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/users/{}/roles-and-permissions", user.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&uri).to_request(),
    ).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: RolesAndPermissions = test::read_body_json(resp).await;
    assert!(body.roles.is_empty());
    assert!(body.permissions.is_empty());
}
