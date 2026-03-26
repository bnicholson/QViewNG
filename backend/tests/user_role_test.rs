mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::{
    database::Database,
    models::{self, apicalllog::ApiCalllog, users_roles::UsersRole},
    routes::configure_routes,
    services::common::EntityResponse,
};
use crate::common::{TEST_DB_URL, clean_database};

// ── POST /api/usersroles/users/{user_id}/roles/{role_id} ────────────────────

#[actix_web::test]
async fn assign_role_to_user_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let user = fixtures::users::seed_user(&mut conn);
    let role = fixtures::roles::seed_role(&mut conn, "editor");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/usersroles/users/{}/roles/{}", user.id, role.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::post().uri(&uri).to_request(),
    ).await;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<UsersRole> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    let record = body.data.unwrap();
    assert_eq!(record.user_id, user.id);
    assert_eq!(record.role_id, role.id);

    let logs: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].method.as_str(), "POST");
    assert_eq!(logs[0].uri, uri);
}

#[actix_web::test]
async fn assign_same_role_twice_returns_conflict() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let user = fixtures::users::seed_user(&mut conn);
    let role = fixtures::roles::seed_role(&mut conn, "admin");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/usersroles/users/{}/roles/{}", user.id, role.id);
    test::call_service(&app, test::TestRequest::post().uri(&uri).to_request()).await;
    let resp = test::call_service(&app, test::TestRequest::post().uri(&uri).to_request()).await;

    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

// ── GET /api/usersroles/users/{user_id} ─────────────────────────────────────

#[actix_web::test]
async fn get_roles_for_user_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let user = fixtures::users::seed_user(&mut conn);
    let roles = fixtures::roles::seed_roles(&mut conn);

    // Assign two of the three roles to the user
    models::users_roles::create(
        &mut conn,
        backend::models::users_roles::NewUsersRole { user_id: user.id, role_id: roles[0].id },
    ).unwrap();
    models::users_roles::create(
        &mut conn,
        backend::models::users_roles::NewUsersRole { user_id: user.id, role_id: roles[1].id },
    ).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/usersroles/users/{}", user.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&uri).to_request(),
    ).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<UsersRole> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 2);
    assert!(body.iter().all(|r| r.user_id == user.id));
}

// ── GET /api/usersroles/roles/{role_id} ─────────────────────────────────────

#[actix_web::test]
async fn get_users_for_role_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let users = fixtures::users::seed_users(&mut conn);
    let role  = fixtures::roles::seed_role(&mut conn, "viewer");

    // Assign the role to user[0] and user[1], not user[2]
    models::users_roles::create(
        &mut conn,
        backend::models::users_roles::NewUsersRole { user_id: users[0].id, role_id: role.id },
    ).unwrap();
    models::users_roles::create(
        &mut conn,
        backend::models::users_roles::NewUsersRole { user_id: users[1].id, role_id: role.id },
    ).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/usersroles/roles/{}", role.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&uri).to_request(),
    ).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<UsersRole> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 2);
    assert!(body.iter().all(|r| r.role_id == role.id));
}

// ── DELETE /api/usersroles/users/{user_id}/roles/{role_id} ─────────────────

#[actix_web::test]
async fn revoke_role_from_user_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let user = fixtures::users::seed_user(&mut conn);
    let role = fixtures::roles::seed_role(&mut conn, "admin");
    models::users_roles::create(
        &mut conn,
        backend::models::users_roles::NewUsersRole { user_id: user.id, role_id: role.id },
    ).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/usersroles/users/{}/roles/{}", user.id, role.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::delete().uri(&uri).to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let remaining = models::users_roles::read_all_for_user(&mut conn, user.id).unwrap();
    assert!(remaining.is_empty());
}

// ── DELETE /api/usersroles/users/{user_id} ───────────────────────────────────

#[actix_web::test]
async fn revoke_all_roles_from_user_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let user  = fixtures::users::seed_user(&mut conn);
    let roles = fixtures::roles::seed_roles(&mut conn);

    for r in &roles {
        models::users_roles::create(
            &mut conn,
            backend::models::users_roles::NewUsersRole { user_id: user.id, role_id: r.id },
        ).unwrap();
    }

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/usersroles/users/{}", user.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::delete().uri(&uri).to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let remaining = models::users_roles::read_all_for_user(&mut conn, user.id).unwrap();
    assert!(remaining.is_empty());
}
