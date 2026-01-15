
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::database::Database;
use backend::models::division::Division;
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn);

    let payload = fixtures::divisions::get_division_payload(parent_tournament.tid);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/divisions")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Division> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let division = body.data.unwrap();
    assert_ne!(division.did.to_string().as_str(), "");
    assert_eq!(division.tid, parent_tournament.tid);
    assert_eq!(division.dname.as_str(), "Test Div 3276");
    assert_eq!(division.breadcrumb.as_str(), "/test/post/for/division/1");
    assert_eq!(division.is_public, false);
    assert_eq!(division.shortinfo.as_str(), "Experienced (but still young).");
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn);

    fixtures::divisions::seed_divisions(&mut conn, parent_tournament.tid);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/divisions?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Division> = test::read_body_json(resp).await;

    assert_eq!(body.len(), 3);

    let object_two = &body[1];
    assert_eq!(object_two.tid, parent_tournament.tid);
    assert_ne!(object_two.did.to_string().as_str(),"");  // "ne" in "assert_ne!" means Not Equal
    assert_eq!(object_two.dname,"Test Div 9078");
    assert_eq!(object_two.breadcrumb,"/test/post/for/division/2");
    assert!(!object_two.is_public);
    assert_eq!(object_two.shortinfo, "Novice");
}


#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let parent_tournament = fixtures::tournaments::seed_tournament(&mut conn);

    let divisions: Vec<Division> = fixtures::divisions::seed_divisions(&mut conn, parent_tournament.tid);
    let division_of_interest_idx = 0;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/divisions/{}", &divisions[division_of_interest_idx].did);
    println!("Divisions Get by ID URI: {}", &uri);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let division: Division = test::read_body_json(resp).await;
    assert_eq!(division.dname, divisions[division_of_interest_idx].dname);
    assert_eq!(division.shortinfo, divisions[division_of_interest_idx].shortinfo);
    assert_eq!(division.breadcrumb, divisions[division_of_interest_idx].breadcrumb);
}
