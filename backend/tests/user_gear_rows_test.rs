mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::user::UserBuilder;
use backend::models::equipmentset::EquipmentSetBuilder;
use backend::models::computer::ComputerBuilder;
use backend::models::equipment_dbo::UserGearRow;
use backend::routes::configure_routes;
use backend::services::common::PagedResponse;
use crate::common::{TEST_DB_URL, clean_database};

fn display_name(fname: &str) -> String { format!("{} Maurice Den", fname) }

/// Seeds an owner with one equipment set holding three gear items. Returns the owner id.
fn seed(conn: &mut backend::database::Connection) -> uuid::Uuid {
    let owner = UserBuilder::new_default("Gary").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    let set = EquipmentSetBuilder::new_default(owner.id).set_name("Gear Bag").build_and_insert(conn).unwrap();

    // Creating a computer in the set also creates its wrapping gear (equipment) row.
    for _ in 0..3 {
        ComputerBuilder::new_default(set.id).build_and_insert(conn).unwrap();
    }
    owner.id
}

#[actix_web::test]
async fn user_gear_rows_returns_enriched_paginated_rows() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let uid = seed(&mut conn);

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/users/{}/gear-rows?page=0&page_size=100", uid))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<UserGearRow> = test::read_body_json(resp).await;

    assert_eq!(body.count, 3);
    assert_eq!(body.items.len(), 3);
    for row in &body.items {
        assert_eq!(row.set_name, "Gear Bag");
        // Gear defaults its creator/modifier to the owner of the parent set.
        assert_eq!(row.creator_name, display_name("Gary"));
        assert_eq!(row.last_modified_user_name, display_name("Gary"));
    }

    // Pagination: second page of size 2 → one remaining item, count unchanged.
    let req2 = test::TestRequest::get()
        .uri(&format!("/api/users/{}/gear-rows?page=1&page_size=2", uid))
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    let body2: PagedResponse<UserGearRow> = test::read_body_json(resp2).await;
    assert_eq!(body2.count, 3);
    assert_eq!(body2.items.len(), 1);
}
