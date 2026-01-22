
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::user::User};
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use bcrypt::verify;
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);

    let unhashed_pwd = "FamouslySecure!23";
    let payload = fixtures::users::get_user_payload(unhashed_pwd);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<User> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    
    let user = body.data.unwrap();
    assert_ne!(user.id.to_string().as_str(), "");
    assert_eq!(user.fname.as_str(), "Test User 3276");
    assert_eq!(user.mname.as_str(), "Maurice");
    assert!(user.activated);

    let pwd_is_valid = verify(unhashed_pwd, &user.hash_password).expect("Password verification failed");
    assert!(pwd_is_valid);
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    fixtures::users::seed_users(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/users?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<User> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 3);

    let mut user_or_interest_idx = 10;
    for idx in 0..3 {
        if body[idx].fname == "Test User 9078" {
            user_or_interest_idx = idx;
            break;
        }
    }

    let user_or_interest = &body[user_or_interest_idx];
    assert_ne!(user_or_interest.id.to_string().as_str(),"");  // "ne" in "assert_ne!" means Not Equal
    assert_eq!(user_or_interest.mname, "Eugene");
    assert_eq!(user_or_interest.username, "edbashful");
}


#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let users: Vec<User> = fixtures::users::seed_users(&mut conn);
    let user_of_interest_idx = 0;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/users/{}", &users[user_of_interest_idx].id);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let user: User = test::read_body_json(resp).await;
    assert_eq!(user.fname, users[user_of_interest_idx].fname);
    assert_eq!(user.username, users[user_of_interest_idx].username);
    assert_eq!(user.lname, users[user_of_interest_idx].lname);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let user: User = fixtures::users::seed_user(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let new_fname = "Test User NEW".to_string();
    let new_mname = "Flemming".to_string();
    let new_activated = false;

    let put_payload = json!({
        "fname": new_fname,
        "mname": new_mname,
        "activated": new_activated
    });
    
    let put_uri = format!("/api/users/{}", user.id);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<User> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let new_user = put_resp_body.data.unwrap();
    assert_eq!(new_user.id, user.id);
    assert_eq!(new_user.fname.as_str(), new_fname);
    assert_eq!(new_user.mname.as_str(), new_mname);
    assert_eq!(new_user.activated, new_activated);
    assert_ne!(new_user.created_at, new_user.updated_at);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let user: User = fixtures::users::seed_user(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let delete_uri = format!("/api/users/{}", user.id);
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


    let get_by_id_uri = format!("/api/users/{}", user.id);
    let get_by_id_req = test::TestRequest::get()
        .uri(&get_by_id_uri)
        .to_request();

    let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

    assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
}
