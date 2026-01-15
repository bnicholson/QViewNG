
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::database::Database;
use backend::models::division::Division;
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use diesel::prelude::*;

use crate::common::{TEST_DB_URL, clean_database};

// fn clean_database() {
//     let db = Database::new(TEST_DB_URL);
//     let mut conn = db.get_connection().expect("Failed to get connection.");
//     diesel::delete(divisions::table)
//         .execute(&mut conn)
//         .expect("Failed to clean tournaments");
// }

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
    assert_eq!(division.breadcrumb.as_str(), "/test/post/for/division");
    assert_eq!(division.is_public, false);
    assert_eq!(division.shortinfo.as_str(), "Experienced (but still young).");
}