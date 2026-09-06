mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::tournamentgroup::{TournamentGroupRow, TournamentGroupBuilder};
use backend::models::tournamentgroup_tournament::TournamentGroupTournamentBuilder;
use backend::models::user::UserBuilder;
use backend::models::tournament::TournamentBuilder;
use backend::routes::configure_routes;
use backend::services::common::PagedResponse;
use crate::common::{TEST_DB_URL, clean_database};

/// Display name produced by `read_display_names` for a user built with
/// `UserBuilder::new_default(fname)` (which sets mname="Maurice", lname="Den").
fn display_name(fname: &str) -> String {
    format!("{} Maurice Den", fname)
}

/// Seeds one tournament with three tournament groups linked to it (names sorting out of insertion
/// order, so the endpoint's `name ASC` ordering is observable), plus a fourth group that is NOT
/// linked to this tournament to prove the endpoint filters by the bridge table. Two linked groups
/// are attributed to the tournament owner; one is explicitly attributed to a second "editor" user
/// so the per-row `last_modified_user_name` enrichment is exercised. Returns the tournament id.
fn seed(conn: &mut backend::database::Connection) -> uuid::Uuid {
    let owner = UserBuilder::new_default("Group Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(conn)
        .unwrap();
    let editor = UserBuilder::new_default("Edith")
        .set_email("editor@fakeemail.com")
        .set_hash_password("EditorPwd123!")
        .build_and_insert(conn)
        .unwrap();
    let tournament = TournamentBuilder::new_default("Group Rows Tour")
        .set_owner_id(owner.id)
        .build_and_insert(conn)
        .unwrap();

    // Insert out of alphabetical order to prove the endpoint sorts by name.
    let charlie = TournamentGroupBuilder::new_default("Charlie Group")
        .set_creator_id(owner.id)
        .set_owner_id(owner.id)
        .build_and_insert(conn)
        .unwrap();
    // Explicitly attributed to the editor — its enriched row should show the editor's name.
    let alpha = TournamentGroupBuilder::new_default("Alpha Group")
        .set_creator_id(owner.id)
        .set_owner_id(owner.id)
        .set_last_modified_user(editor.id)
        .build_and_insert(conn)
        .unwrap();
    let bravo = TournamentGroupBuilder::new_default("Bravo Group")
        .set_creator_id(owner.id)
        .set_owner_id(owner.id)
        .build_and_insert(conn)
        .unwrap();
    // Linked to no tournament — must be excluded from this tournament's rows.
    let _unlinked = TournamentGroupBuilder::new_default("Zzz Unlinked Group")
        .set_creator_id(owner.id)
        .set_owner_id(owner.id)
        .build_and_insert(conn)
        .unwrap();

    for group in [&charlie, &alpha, &bravo] {
        TournamentGroupTournamentBuilder::new_default(group.tgid, tournament.tid)
            .build_and_insert(conn)
            .unwrap();
    }

    tournament.tid
}

#[actix_web::test]
async fn tournament_tournamentgroup_rows_returns_enriched_sorted_paginated_rows() {
    // Arrange
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let tid = seed(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes),
    )
    .await;

    // Act — full first page.
    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/tournamentgroup-rows?page=0&page_size=100", tid))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert — only the three linked groups, ordered by name, with per-row enrichment.
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<TournamentGroupRow> = test::read_body_json(resp).await;
    assert_eq!(body.count, 3);
    assert_eq!(body.items.len(), 3);

    let names: Vec<&str> = body.items.iter().map(|g| g.name.as_str()).collect();
    assert_eq!(names, vec!["Alpha Group", "Bravo Group", "Charlie Group"]);

    // Alpha was explicitly attributed to the editor; the others default to the owner.
    assert_eq!(body.items[0].last_modified_user_name, display_name("Edith"));
    assert_eq!(body.items[1].last_modified_user_name, display_name("Group Owner"));
    assert_eq!(body.items[2].last_modified_user_name, display_name("Group Owner"));

    // Act — second page (page_size 2, page index 1 → the last group only).
    let req_page = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/tournamentgroup-rows?page=1&page_size=2", tid))
        .to_request();
    let resp_page = test::call_service(&app, req_page).await;

    // Assert — count is still the total; the page returns just the remaining group.
    assert_eq!(resp_page.status(), StatusCode::OK);
    let page_body: PagedResponse<TournamentGroupRow> = test::read_body_json(resp_page).await;
    assert_eq!(page_body.count, 3);
    let page_names: Vec<&str> = page_body.items.iter().map(|g| g.name.as_str()).collect();
    assert_eq!(page_names, vec!["Charlie Group"]);
}

#[actix_web::test]
async fn tournament_tournamentgroup_rows_no_linked_groups_returns_zero() {
    // Arrange — a tournament with no linked groups.
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let owner = UserBuilder::new_default("Lonely Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(&mut conn)
        .unwrap();
    let tournament = TournamentBuilder::new_default("Empty Tour")
        .set_owner_id(owner.id)
        .build_and_insert(&mut conn)
        .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes),
    )
    .await;

    // Act
    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/tournamentgroup-rows?page=0&page_size=100", tournament.tid))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<TournamentGroupRow> = test::read_body_json(resp).await;
    assert_eq!(body.count, 0);
    assert!(body.items.is_empty());
}
