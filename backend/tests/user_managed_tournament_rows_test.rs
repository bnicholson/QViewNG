mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::user::UserBuilder;
use backend::models::tournament::{UserManagedTournamentRow, TournamentBuilder};
use backend::routes::configure_routes;
use backend::services::common::PagedResponse;
use crate::common::{TEST_DB_URL, clean_database};

fn display_name(fname: &str) -> String { format!("{} Maurice Den", fname) }

/// Seeds two tournaments owned by the user and one owned by someone else (which must be excluded).
fn seed(conn: &mut backend::database::Connection) -> uuid::Uuid {
    let owner = UserBuilder::new_default("Ophelia").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    let other = UserBuilder::new_default("Nate").set_email("nate@fakeemail.com").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    TournamentBuilder::new_default("Owned One").set_owner_id(owner.id).build_and_insert(conn).unwrap();
    TournamentBuilder::new_default("Owned Two").set_owner_id(owner.id).build_and_insert(conn).unwrap();
    TournamentBuilder::new_default("Not Owned").set_owner_id(other.id).build_and_insert(conn).unwrap();
    owner.id
}

#[actix_web::test]
async fn user_managed_tournament_rows_returns_owned_enriched_paginated_rows() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let uid = seed(&mut conn);

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/users/{}/managed-tournament-rows?page=0&page_size=100", uid))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<UserManagedTournamentRow> = test::read_body_json(resp).await;

    // Only the two owned tournaments, each attributed to the owner.
    assert_eq!(body.count, 2);
    assert_eq!(body.items.len(), 2);
    let names: Vec<&str> = body.items.iter().map(|t| t.tname.as_str()).collect();
    assert!(names.contains(&"Owned One") && names.contains(&"Owned Two"));
    assert!(!names.contains(&"Not Owned"));
    for row in &body.items {
        assert_eq!(row.creator_name, display_name("Ophelia"));
        assert_eq!(row.last_modified_user_name, display_name("Ophelia"));
    }

    // Pagination: page size 1 still reports the full count of 2.
    let req2 = test::TestRequest::get()
        .uri(&format!("/api/users/{}/managed-tournament-rows?page=0&page_size=1", uid))
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    let body2: PagedResponse<UserManagedTournamentRow> = test::read_body_json(resp2).await;
    assert_eq!(body2.count, 2);
    assert_eq!(body2.items.len(), 1);
}
