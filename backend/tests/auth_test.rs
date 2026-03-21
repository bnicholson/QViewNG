
mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::{database::Database, models::{self, apicalllog::ApiCalllog}, routes::configure_routes};
use crate::common::{TEST_DB_URL, clean_database};

#[actix_web::test]
async fn login_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // Create a user with a known password; UserBuilder/create will hash it
    let plain_password = "TestLogin!99";
    let user = backend::models::user::UserBuilder::new("Login")
        .set_email("logintest@example.com")
        .set_lname("Tester")
        .set_mname("Auth")
        .set_hash_password(plain_password)
        .set_activated(true)
        .build_and_insert(&mut conn)
        .expect("Failed to create test user");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = "/api/auth/login";
    let req = test::TestRequest::post()
        .uri(uri)
        .set_json(serde_json::json!({
            "email": user.email,
            "password": plain_password
        }))
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("access_token").is_some(), "Expected access_token in response");
    assert_ne!(body["access_token"].as_str().unwrap_or(""), "");

    // Check that ApiCalllog recorded the call:
    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri.as_str(), uri);
}

#[actix_web::test]
async fn login_with_wrong_password_fails() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let plain_password = "CorrectHorseBattery!1";
    let user = backend::models::user::UserBuilder::new("Wrong")
        .set_email("wrongpass@example.com")
        .set_lname("PassTest")
        .set_mname("Auth")
        .set_hash_password(plain_password)
        .set_activated(true)
        .build_and_insert(&mut conn)
        .expect("Failed to create test user");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = "/api/auth/login";
    let req = test::TestRequest::post()
        .uri(uri)
        .set_json(serde_json::json!({
            "email": user.email,
            "password": "WrongPassword!99"
        }))
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("error").is_some());

    // Check that ApiCalllog recorded the call:
    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri.as_str(), uri);
}

#[actix_web::test]
async fn register_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = "/api/auth/register";
    let req = test::TestRequest::post()
        .uri(uri)
        .set_json(serde_json::json!({
            "email": "newuser@example.com",
            "password": "NewUserPass!1",
            "fname": "New",
            "lname": "User"
        }))
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("id").is_some());
    assert_eq!(body["email"].as_str().unwrap_or(""), "newuser@example.com");

    // Check that ApiCalllog recorded the call:
    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri.as_str(), uri);
}

#[actix_web::test]
async fn logout_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let plain_password = "LogoutTest!1";
    backend::models::user::UserBuilder::new("Logout")
        .set_email("logouttest@example.com")
        .set_lname("Tester")
        .set_mname("Auth")
        .set_hash_password(plain_password)
        .set_activated(true)
        .build_and_insert(&mut conn)
        .expect("Failed to create test user");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    // Login first to get a session cookie
    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "logouttest@example.com",
            "password": plain_password
        }))
        .to_request();
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);

    // Extract only the "name=value" part from the Set-Cookie header (strip attributes)
    let cookie_value = login_resp
        .headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(';').next())
        .map(|s| s.to_string());

    let uri = "/api/auth/logout";
    let mut logout_req_builder = test::TestRequest::post().uri(uri);
    if let Some(cookie_val) = cookie_value {
        logout_req_builder = logout_req_builder.append_header(("Cookie", cookie_val));
    }
    let logout_req = logout_req_builder.to_request();

    // Act:

    let logout_resp = test::call_service(&app, logout_req).await;

    // Assert:

    assert_eq!(logout_resp.status(), StatusCode::OK);

    // Check that ApiCalllog recorded both calls:
    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(apicalllog_records.iter().count(), 2);
    let logout_record = apicalllog_records.iter().find(|r| r.uri == uri);
    assert!(logout_record.is_some());
    assert_eq!(logout_record.unwrap().method.as_str(), "POST");
}

#[actix_web::test]
async fn refresh_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let plain_password = "RefreshTest!1";
    backend::models::user::UserBuilder::new("Refresh")
        .set_email("refreshtest@example.com")
        .set_lname("Tester")
        .set_mname("Auth")
        .set_hash_password(plain_password)
        .set_activated(true)
        .build_and_insert(&mut conn)
        .expect("Failed to create test user");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    // Login to get the refresh_token cookie
    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "refreshtest@example.com",
            "password": plain_password
        }))
        .to_request();
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);

    // Extract only the "name=value" part from the Set-Cookie header (strip attributes)
    let cookie_value = login_resp
        .headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(';').next())
        .map(|s| s.to_string());

    let uri = "/api/auth/refresh";
    let mut refresh_req_builder = test::TestRequest::post().uri(uri);
    if let Some(cookie_val) = cookie_value {
        refresh_req_builder = refresh_req_builder.append_header(("Cookie", cookie_val));
    }
    let refresh_req = refresh_req_builder.to_request();

    // Act:

    let refresh_resp = test::call_service(&app, refresh_req).await;

    // Assert:

    assert_eq!(refresh_resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(refresh_resp).await;
    assert!(body.get("access_token").is_some());
    assert_ne!(body["access_token"].as_str().unwrap_or(""), "");

    // Check that ApiCalllog recorded both calls:
    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(apicalllog_records.iter().count(), 2);
    let refresh_record = apicalllog_records.iter().find(|r| r.uri == uri);
    assert!(refresh_record.is_some());
    assert_eq!(refresh_record.unwrap().method.as_str(), "POST");
}

#[actix_web::test]
async fn forgot_password_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    backend::models::user::UserBuilder::new("Forgot")
        .set_email("forgotpwd@example.com")
        .set_lname("Tester")
        .set_mname("Auth")
        .set_hash_password("ForgotTest!1")
        .set_activated(true)
        .build_and_insert(&mut conn)
        .expect("Failed to create test user");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = "/api/auth/forgot";
    let req = test::TestRequest::post()
        .uri(uri)
        .set_json(serde_json::json!({
            "email": "forgotpwd@example.com"
        }))
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("message").is_some());

    // Check that ApiCalllog recorded the call:
    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri.as_str(), uri);
}

#[actix_web::test]
async fn reset_password_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let user = backend::models::user::UserBuilder::new("Reset")
        .set_email("resetpwd@example.com")
        .set_lname("Tester")
        .set_mname("Auth")
        .set_hash_password("OldPassword!1")
        .set_activated(true)
        .build_and_insert(&mut conn)
        .expect("Failed to create test user");

    // Create a valid reset token directly in the DB
    let reset_token = "test-reset-token-abc123";
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);
    backend::models::password_reset::create(&mut conn, user.id, reset_token, expires_at)
        .expect("Failed to create reset token");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = "/api/auth/reset";
    let req = test::TestRequest::post()
        .uri(uri)
        .set_json(serde_json::json!({
            "reset_token": reset_token,
            "new_password": "NewSecurePassword!2"
        }))
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("message").is_some());

    // Check that ApiCalllog recorded the call:
    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri.as_str(), uri);
}

#[actix_web::test]
async fn change_password_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let plain_password = "ChangeMe!OldPwd1";
    backend::models::user::UserBuilder::new("Change")
        .set_email("changepwd@example.com")
        .set_lname("Tester")
        .set_mname("Auth")
        .set_hash_password(plain_password)
        .set_activated(true)
        .build_and_insert(&mut conn)
        .expect("Failed to create test user");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    // Login to get an access token
    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "changepwd@example.com",
            "password": plain_password
        }))
        .to_request();
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);

    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    let access_token = login_body["access_token"].as_str().unwrap().to_string();

    let uri = "/api/auth/change";
    let req = test::TestRequest::post()
        .uri(uri)
        .append_header(("Authorization", format!("Bearer {}", access_token)))
        .set_json(serde_json::json!({
            "old_password": plain_password,
            "new_password": "NewPassword!ChangedOk2"
        }))
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("message").is_some());

    // Check that ApiCalllog recorded both calls (login + change):
    let apicalllog_records: Vec<ApiCalllog> = models::apicalllog::read_all(&mut conn).unwrap();
    assert_eq!(apicalllog_records.iter().count(), 2);
    let change_record = apicalllog_records.iter().find(|r| r.uri == uri);
    assert!(change_record.is_some());
    assert_eq!(change_record.unwrap().method.as_str(), "POST");
}
