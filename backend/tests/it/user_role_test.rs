use crate::fixtures;
use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::{
    database::Database,
    models::{self, apicalllog::ApiCalllog, users_roles::UsersRole},
    routes::configure_routes,
    services::common::EntityResponse,
};
use crate::common::{test_db};

// ── POST /api/usersroles/users/{user_id}/roles/{role_id} ────────────────────

// ── GET /api/usersroles/users/{user_id} ─────────────────────────────────────

// ── GET /api/usersroles/roles/{role_id} ─────────────────────────────────────

#[actix_web::test]
async fn get_users_for_role_works() {
    let db = test_db();
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

    let rows = models::users_roles::read_all_for_role(&mut conn, role.id).expect("read failed");
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.role_id == role.id));
}

// ── DELETE /api/usersroles/users/{user_id}/roles/{role_id} ─────────────────

#[actix_web::test]
async fn revoke_role_from_user_works() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let user = fixtures::users::seed_user(&mut conn);
    let role = fixtures::roles::seed_role(&mut conn, "admin");
    models::users_roles::create(
        &mut conn,
        backend::models::users_roles::NewUsersRole { user_id: user.id, role_id: role.id },
    ).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
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
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let user  = fixtures::users::seed_user(&mut conn);
    let roles = fixtures::roles::seed_roles(&mut conn);

    for r in &roles {
        models::users_roles::create(
            &mut conn,
            backend::models::users_roles::NewUsersRole { user_id: user.id, role_id: r.id },
        ).unwrap();
    }

    let _removed = models::users_roles::delete_all_for_user(&mut conn, user.id).expect("delete failed");
    let remaining = models::users_roles::read_all_for_user(&mut conn, user.id).unwrap();
    assert!(remaining.is_empty());
}

// ── GET/PUT /api/usersroles/users/{user_id} (Superuser Console role editor) ──

fn superuser_token(user_id: uuid::Uuid) -> String {
    crate::common::make_token(user_id, vec![], vec!["system:administer".to_string()])
}

#[actix_web::test]
async fn superuser_replaces_a_users_roles() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let users = fixtures::users::seed_users(&mut conn);
    let (caller, target) = (&users[0], &users[1]);
    let roles = fixtures::roles::seed_roles(&mut conn);
    // Start with admin + editor, and replace them with editor + viewer.
    for role in &roles[..2] {
        models::users_roles::create(
            &mut conn,
            backend::models::users_roles::NewUsersRole { user_id: target.id, role_id: role.id },
        ).unwrap();
    }

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/usersroles/users/{}", target.id);
    let resp = test::call_service(
        &app,
        test::TestRequest::put()
            .uri(&uri)
            .insert_header(("Authorization", format!("Bearer {}", superuser_token(caller.id))))
            .set_json(serde_json::json!({ "role_ids": [roles[1].id, roles[2].id] }))
            .to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let mut assigned: Vec<_> = models::users_roles::read_all_for_user(&mut conn, target.id)
        .unwrap().iter().map(|r| r.role_id).collect();
    assigned.sort();
    let mut expected = vec![roles[1].id, roles[2].id];
    expected.sort();
    assert_eq!(assigned, expected);

    // GET reads the same set back.
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&uri)
            .insert_header(("Authorization", format!("Bearer {}", superuser_token(caller.id))))
            .to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["role_ids"].as_array().unwrap().len(), 2);
}

#[actix_web::test]
async fn replacing_roles_requires_a_superuser() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let users = fixtures::users::seed_users(&mut conn);
    let (caller, target) = (&users[0], &users[1]);
    let role = fixtures::roles::seed_role(&mut conn, "viewer");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/usersroles/users/{}", target.id);
    let body = serde_json::json!({ "role_ids": [role.id] });

    let anonymous = test::call_service(
        &app,
        test::TestRequest::put().uri(&uri).set_json(&body).to_request(),
    ).await;
    assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);

    let member_token = crate::common::make_token(caller.id, vec![], vec!["user:update".to_string()]);
    let member = test::call_service(
        &app,
        test::TestRequest::put()
            .uri(&uri)
            .insert_header(("Authorization", format!("Bearer {}", member_token)))
            .set_json(&body)
            .to_request(),
    ).await;
    assert_eq!(member.status(), StatusCode::FORBIDDEN);

    let member_get = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&uri)
            .insert_header(("Authorization", format!("Bearer {}", member_token)))
            .to_request(),
    ).await;
    assert_eq!(member_get.status(), StatusCode::FORBIDDEN);

    assert!(models::users_roles::read_all_for_user(&mut conn, target.id).unwrap().is_empty());
}

#[actix_web::test]
async fn superuser_cannot_remove_their_own_super_user_role() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let caller = fixtures::users::seed_user(&mut conn);
    let super_user = models::role::read_by_name(&mut conn, "super_user")
        .unwrap_or_else(|_| fixtures::roles::seed_role(&mut conn, "super_user"));
    models::users_roles::create(
        &mut conn,
        backend::models::users_roles::NewUsersRole { user_id: caller.id, role_id: super_user.id },
    ).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(configure_routes)
    ).await;

    let resp = test::call_service(
        &app,
        test::TestRequest::put()
            .uri(&format!("/api/usersroles/users/{}", caller.id))
            .insert_header(("Authorization", format!("Bearer {}", superuser_token(caller.id))))
            .set_json(serde_json::json!({ "role_ids": [] }))
            .to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let assigned = models::users_roles::read_all_for_user(&mut conn, caller.id).unwrap();
    assert_eq!(assigned.len(), 1);
}

#[actix_web::test]
async fn replacing_roles_rejects_an_unknown_role() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let users = fixtures::users::seed_users(&mut conn);
    let (caller, target) = (&users[0], &users[1]);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(configure_routes)
    ).await;

    let resp = test::call_service(
        &app,
        test::TestRequest::put()
            .uri(&format!("/api/usersroles/users/{}", target.id))
            .insert_header(("Authorization", format!("Bearer {}", superuser_token(caller.id))))
            .set_json(serde_json::json!({ "role_ids": [uuid::Uuid::new_v4()] }))
            .to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ── New users get the default `member` role ──────────────────────────────────

/// The roles assigned to the user with `email`, by name.
fn role_names_for(conn: &mut backend::database::Connection, email: &str) -> Vec<String> {
    let user = models::user::find_by_email_or_username(conn, email).expect("user not created");
    models::users_roles::read_all_for_user(conn, user.id).unwrap()
        .iter()
        .map(|ur| models::role::read(conn, ur.role_id).unwrap().name)
        .collect()
}

#[actix_web::test]
async fn self_registration_assigns_the_member_role() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    fixtures::roles::seed_role(&mut conn, "member");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(configure_routes)
    ).await;

    let resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/auth/register")
            .set_json(serde_json::json!({
                "email": "newmember@example.com",
                "password": "NewUserPass!1",
                "fname": "New",
                "lname": "Member"
            }))
            .to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    assert_eq!(role_names_for(&mut conn, "newmember@example.com"), vec!["member".to_string()]);
}

#[actix_web::test]
async fn creating_a_user_assigns_the_member_role() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    fixtures::roles::seed_role(&mut conn, "member");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(configure_routes)
    ).await;

    let payload = fixtures::users::get_user_payload("FamouslySecure!23");
    let email = payload.email.clone();
    let resp = test::call_service(
        &app,
        test::TestRequest::post().uri("/api/users").set_json(&payload).to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    assert_eq!(role_names_for(&mut conn, &email), vec!["member".to_string()]);
}
