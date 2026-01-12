
mod common;
mod fixtures;

use actix_web::{test, App, web};
use diesel::prelude::*;
use backend::routes::configure_routes;
use backend::database::Database;
use backend::schema::tournaments::table;

#[actix_web::test]
async fn test_index_returns_all_accurate_data() {

    // Arrange:

    // Setup database
    let mut conn = common::establish_test_connection();

    // Clean slate
    diesel::delete(table)
        .execute(&mut conn)
        .expect("Failed to clean tournaments");
    
    // Insert test data
    let tournaments = fixtures::tournaments::seed_tournaments(&mut conn);
    
    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new("TEST_DATABASE_URL")))
            .configure(configure_routes)
    ).await;
    
    // Act:

    // Make request
    let req = test::TestRequest::get()
        .uri("/api/tournaments?page=0&page_size=10")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    // Read the body as bytes
    let body = test::read_body(resp).await;
    
    // Convert to string (example)
    // let body_str = std::str::from_utf8(&body).unwrap();

    // Or parse as JSON
    let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // let json_body = body_json.to_string();
    // println!("Body: {}", &body_json[1]);

    // Assert:

    // assert!(false);  // for viewing println! statement contents

    // There are 3 objects in the vec
    assert_eq!(body_json.as_array().unwrap().len(), 3);

    // Check data for the 2nd object (all properties):
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
    assert_eq!(object_two["shortinfo"],"This is your captain speaking.");
}