
mod common;
mod fixtures;

use actix_http::StatusCode;
use backend::{models::{self, apicalllog::ApiCalllog}, routes::configure_routes};
use actix_web::{App, test, web::{self}};
use crate::common::{make_token, test_db};

#[actix_web::test]
async fn create_and_read_all_works() {

    // Arrange:
    
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    // Assumes apicallog model is being used for the create tournament endpoint:
    let (tour_1, tour_2) = 
        fixtures::apicalllogs::arrange_create_works_integration_test();

    // POST /api/tournaments requires an authenticated caller holding `tournament:create`; the
    // created tournament's owner is taken from the token, so the user must exist.
    let creator = fixtures::users::seed_user(&mut conn);
    let token = make_token(
        creator.id,
        vec![],
        vec!["tournament:create".to_string()],
    );
    let auth_header = ("Authorization", format!("Bearer {}", token));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri_1 = format!("/api/tournaments");
    let req_1 = test::TestRequest::post()
        .uri(&uri_1)
        .insert_header(auth_header.clone())
        .set_json(&tour_1)
        .to_request();
    
    let uri_2 = format!("/api/tournaments");
    let req_2 = test::TestRequest::post()
        .uri(&uri_2)
        .insert_header(auth_header)
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
