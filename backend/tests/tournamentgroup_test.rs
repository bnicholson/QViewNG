
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::game::Game};
use backend::models::tournamentgroup::TournamentGroup;
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let payload = fixtures::tournamentgroups::get_tournamentgroup_payload();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/tournamentgroups")
        .set_json(&payload)
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: EntityResponse<TournamentGroup> = test::read_body_json(resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let tournamentgroup = body.data.unwrap();
    assert_ne!(tournamentgroup.tgid, uuid::Uuid::nil());
    assert_eq!(tournamentgroup.name.as_str(), "Test TourGroup 1");
    assert_eq!(tournamentgroup.description.unwrap().as_str(), "This is Tour 1's payload.");
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (tg_1, tg_2) = fixtures::tournamentgroups::arrange_get_all_works_intergration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/tournamentgroups?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<TournamentGroup> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut tg_of_interest_1_idx = 10;
    let mut tg_of_interest_2_idx = 10;
    for idx in 0..len {
        if body[idx].name == "Test TourGroup 1".to_string() {
            tg_of_interest_1_idx = idx;
            continue;
        }
        if body[idx].name == "Test TourGroup 2".to_string() {
            tg_of_interest_2_idx = idx;
            continue;
        }
    }

    let tg_of_interest_1 = &body[tg_of_interest_1_idx];
    assert_eq!(tg_of_interest_1.tgid, tg_1.tgid);
    assert_eq!(tg_of_interest_1.name.as_str(), "Test TourGroup 1");
    assert_eq!(tg_of_interest_1.description.as_ref().unwrap().as_str(), "This is Tour 1's payload.");

    let tg_of_interest_2 = &body[tg_of_interest_2_idx];
    assert_eq!(tg_of_interest_2.tgid, tg_2.tgid);
    assert_eq!(tg_of_interest_2.name.as_str(), "Test TourGroup 2");
    assert_eq!(tg_of_interest_2.description.as_ref().unwrap().as_str(), "This is Tour 2's payload.");
}

#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tg_2 = fixtures::tournamentgroups::arrange_get_by_id_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/tournamentgroups/{}", tg_2.tgid);
    println!("TournamentGroups Get by ID URI: {}", &uri);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let tournamentgroup_2: TournamentGroup = test::read_body_json(resp).await;
    assert_eq!(tournamentgroup_2.tgid, tg_2.tgid);
    assert_eq!(tournamentgroup_2.name, tg_2.name);
    assert_eq!(tournamentgroup_2.description, tg_2.description);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournamentgroup = fixtures::tournamentgroups::arrange_update_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let new_name = "Some other name for this TG.".to_string();
    let new_description = "Here is the updated description.".to_string();

    let put_payload = json!({
        "name": &new_name,
        "description": &new_description
    });
    
    let put_uri = format!("/api/tournamentgroups/{}", tournamentgroup.tgid);
    let put_req = test::TestRequest::put()
        .uri(&put_uri)
        .set_json(&put_payload)
        .to_request();

    // Act:
    
    let put_resp = test::call_service(&app, put_req).await;

    // Assert:
    
    assert_eq!(put_resp.status(), StatusCode::OK);

    let put_resp_body: EntityResponse<TournamentGroup> = test::read_body_json(put_resp).await;
    assert_eq!(put_resp_body.code, 200);
    assert_eq!(put_resp_body.message, "");

    let new_tournamentgroup = put_resp_body.data.unwrap();
    assert_eq!(new_tournamentgroup.tgid, tournamentgroup.tgid);
    assert_eq!(new_tournamentgroup.name.as_str(), new_name);
    assert_eq!(new_tournamentgroup.description.as_ref().unwrap().as_str(), new_description);
    assert_ne!(new_tournamentgroup.created_at, new_tournamentgroup.updated_at);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournamentgroup = fixtures::tournamentgroups::arrange_delete_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let delete_uri = format!("/api/tournamentgroups/{}", tournamentgroup.tgid);
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


    let get_by_id_uri = format!("/api/tournamentgroups/{}", tournamentgroup.tgid);
    let get_by_id_req = test::TestRequest::get()
        .uri(&get_by_id_uri)
        .to_request();

    let get_by_id_resp = test::call_service(&app, get_by_id_req).await;

    assert_eq!(get_by_id_resp.status(), StatusCode::NOT_FOUND);
}