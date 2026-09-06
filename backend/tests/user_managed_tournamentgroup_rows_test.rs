mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::user::UserBuilder;
use backend::models::tournamentgroup::{UserManagedTournamentGroupRow, TournamentGroupBuilder};
use backend::routes::configure_routes;
use backend::services::common::PagedResponse;
use crate::common::{TEST_DB_URL, clean_database};

fn display_name(fname: &str) -> String { format!("{} Maurice Den", fname) }

/// Seeds two groups owned by the user and one owned by someone else (which must be excluded).
fn seed(conn: &mut backend::database::Connection) -> uuid::Uuid {
    let owner = UserBuilder::new_default("Gwen").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    let other = UserBuilder::new_default("Hank").set_email("hank@fakeemail.com").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    TournamentGroupBuilder::new_default("Alpha Group").set_creator_id(owner.id).set_owner_id(owner.id).build_and_insert(conn).unwrap();
    TournamentGroupBuilder::new_default("Bravo Group").set_creator_id(owner.id).set_owner_id(owner.id).build_and_insert(conn).unwrap();
    TournamentGroupBuilder::new_default("Not Mine").set_creator_id(other.id).set_owner_id(other.id).build_and_insert(conn).unwrap();
    owner.id
}

#[actix_web::test]
async fn user_managed_tournamentgroup_rows_returns_owned_enriched_paginated_rows() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let uid = seed(&mut conn);

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/users/{}/managed-tournamentgroup-rows?page=0&page_size=100", uid))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<UserManagedTournamentGroupRow> = test::read_body_json(resp).await;

    assert_eq!(body.count, 2);
    let names: Vec<&str> = body.items.iter().map(|g| g.name.as_str()).collect();
    assert_eq!(names, vec!["Alpha Group", "Bravo Group"]);
    for row in &body.items {
        assert_eq!(row.creator_name, display_name("Gwen"));
        assert_eq!(row.last_modified_user_name, display_name("Gwen"));
    }

    // Pagination: second page of size 1 → the second group, count unchanged.
    let req2 = test::TestRequest::get()
        .uri(&format!("/api/users/{}/managed-tournamentgroup-rows?page=1&page_size=1", uid))
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    let body2: PagedResponse<UserManagedTournamentGroupRow> = test::read_body_json(resp2).await;
    assert_eq!(body2.count, 2);
    let names2: Vec<&str> = body2.items.iter().map(|g| g.name.as_str()).collect();
    assert_eq!(names2, vec!["Bravo Group"]);
}
