
mod common;
mod fixtures;

use actix_http::StatusCode;
use backend::{database::Database, models::{self, apicalllog::ApiCalllog}, routes::configure_routes};
use actix_web::{App, test, web::{self}};
use crate::common::{TEST_DB_URL, clean_database};

#[actix_web::test]
async fn create_and_read_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    // Assumes apicallog model is being used for the create tournament endpoint:
    let (tour_1, tour_2) = 
        fixtures::apicalllogs::arrange_create_works_integration_test();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri_1 = format!("/api/tournaments");
    let req_1 = test::TestRequest::post()
        .uri(&uri_1)
        .set_json(&tour_1)
        .to_request();
    
    let uri_2 = format!("/api/tournaments");
    let req_2 = test::TestRequest::post()
        .uri(&uri_2)
        .set_json(&tour_2)
        .to_request();
    
    // Act:
    let resp_1 = test::call_service(&app, req_1).await;
    let resp_2 = test::call_service(&app, req_2).await;
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);

    // Assert:

    assert_eq!(resp_1.status(), StatusCode::CREATED);
    assert_eq!(resp_2.status(), StatusCode::CREATED);
    assert!(apicalllog_get_result.is_ok());

    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 2);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri.as_str(), "/api/tournaments");
}
