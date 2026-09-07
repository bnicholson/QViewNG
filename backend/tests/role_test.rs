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

// fn make_app_and_db() -> (Database, impl std::future::Future<Output = impl actix_web::dev::Service<
//     actix_http::Request,
//     Response = actix_web::dev::ServiceResponse,
//     Error = actix_web::Error,
// >>) {
//     let db = Database::new(TEST_DB_URL);
//     let db_clone = Database::new(TEST_DB_URL);
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(db_clone))
//             .configure(configure_routes)
//     );
//     (db, app)
// }

// ── POST /api/roles ───────────────────────────────────────────────────────────

#[actix_web::test]
async fn create_role_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let created = models::role::create(&mut conn, backend::models::role::NewRole { name: "admin".to_string(), description: Some("Full access".to_string()) }).expect("create failed");
    assert_eq!(created.name, "admin");
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

// ── PUT /api/roles/{id} ───────────────────────────────────────────────────────

#[actix_web::test]
async fn update_role_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let role = fixtures::roles::seed_role(&mut conn, "viewer");

    let changeset = backend::models::role::RoleChangeset { name: Some("super-viewer".to_string()), description: Some("Updated desc".to_string()) };
    let updated = models::role::update(&mut conn, role.id, &changeset).expect("update failed");
    assert_eq!(updated.name, "super-viewer");
}

// ── DELETE /api/roles/{id} ────────────────────────────────────────────────────

#[actix_web::test]
async fn delete_role_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let role = fixtures::roles::seed_role(&mut conn, "temp-role");

    let count = models::role::delete(&mut conn, role.id).expect("delete failed");
    assert_eq!(count, 1);
}

// ── POST /api/roles/{role_id}/permissions/{permission_id} ────────────────────

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

    let rows = models::role_permission::read_all_for_role(&mut conn, role.id).expect("read failed");
    assert_eq!(rows.len(), 2);
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

    let removed = models::role_permission::delete(&mut conn, role.id, perm.id).expect("delete failed");
    assert_eq!(removed, 1);
}
