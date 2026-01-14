
mod common;
mod fixtures;

use actix_web::{test, App, web::{self,Bytes}, http::StatusCode};
use diesel::prelude::*;
use backend::routes::configure_routes;
use backend::database::Database;
use backend::schema::tournaments;

fn clean_database() {
    let db = Database::new("TEST_DATABASE_URL");
    let mut conn = db.get_connection().expect("Failed to get connection.");
    diesel::delete(tournaments::table)
        .execute(&mut conn)
        .expect("Failed to clean tournaments");
}

#[actix_web::test]
async fn test_index_returns_all_accurate_data() {

    // Arrange:
    
    clean_database();
    let db = Database::new("TEST_DATABASE_URL");
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    fixtures::tournaments::seed_tournaments(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    // Act:

    let req = test::TestRequest::get()
        .uri("/api/tournaments?page=0&page_size=10")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let body: Bytes = test::read_body(resp).await;
    let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body_json.as_array().unwrap().len(), 3);

    let object_two = &body_json[1];
    assert_eq!(object_two["organization"],"Nazarene");
    assert_ne!(object_two["tid"],"");  // "ne" in "assert_ne!" means Not Equal
    assert_eq!(object_two["tname"],"Tour 2");
    assert_eq!(object_two["breadcrumb"],"/test/bread/crumb");
    assert_eq!(object_two["fromdate"],"2025-05-23");
    assert_eq!(object_two["todate"],"2025-05-27");
    assert_eq!(object_two["venue"],"Olivet Nazarene University");
    assert_eq!(object_two["city"],"Bourbonnais");
    assert_eq!(object_two["region"],"Central USA");
    assert_eq!(object_two["country"],"USA");
    assert_eq!(object_two["contact"],"Jason Morton");
    assert_eq!(object_two["contactemail"],"jasonmorton@fakeemail.com");
    assert_eq!(object_two["is_public"],false);
    assert_eq!(object_two["shortinfo"],"NYI International quiz meet of 2025.");
    assert_eq!(object_two["info"],"If I wanted a longer description I would have provided it here.");
}

#[actix_web::test]
async fn test_create_inserts_tournament() {

    // Arrange:

    clean_database();
    let db = Database::new("TEST_DATABASE_URL");

    let payload = fixtures::tournaments::get_tournament_payload();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    // Act:

    let req = test::TestRequest::post()
        .uri("/api/tournaments")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body = test::read_body(resp).await;
    let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(body_json["code"].as_i64().unwrap(), 201);
    assert_eq!(body_json["message"].as_str().unwrap(), "");
    assert_ne!(body_json["data"]["tid"].as_str().unwrap(), "");
    assert_eq!(body_json["data"]["organization"].as_str().unwrap(), "Nazarene");
    assert_eq!(body_json["data"]["tname"].as_str().unwrap(), "Test Post");
}