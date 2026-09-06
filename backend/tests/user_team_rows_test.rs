mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::team::{UserTeamRow, TeamBuilder};
use backend::models::user::UserBuilder;
use backend::models::tournament::TournamentBuilder;
use backend::models::division::DivisionBuilder;
use backend::routes::configure_routes;
use backend::services::common::PagedResponse;
use crate::common::{TEST_DB_URL, clean_database};

fn display_name(fname: &str) -> String { format!("{} Maurice Den", fname) }

/// Seeds a division with two teams: one the user coaches, one where the user is a rostered quizzer
/// (coached by someone else). Returns the user id whose /team-rows should list both.
fn seed(conn: &mut backend::database::Connection) -> uuid::Uuid {
    let user = UserBuilder::new_default("Dana").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    let other_coach = UserBuilder::new_default("Otto").set_email("otto@fakeemail.com").set_hash_password("Pwd123!").build_and_insert(conn).unwrap();
    let tournament = TournamentBuilder::new_default("Team Rows Tour").set_owner_id(user.id).build_and_insert(conn).unwrap();
    let division = DivisionBuilder::new_default("Div 1", tournament.tid).build_and_insert(conn).unwrap();

    // Team the user coaches (creator/modifier default to the coach = user).
    TeamBuilder::new_default(division.did).set_coachid(user.id).set_name("Team A").build_and_insert(conn).unwrap();
    // Team coached by someone else, on which the user is a quizzer.
    TeamBuilder::new_default(division.did).set_coachid(other_coach.id).set_name("Team B").set_quizzer_one_id(user.id).build_and_insert(conn).unwrap();
    // A team the user is not on at all — must be excluded.
    TeamBuilder::new_default(division.did).set_coachid(other_coach.id).set_name("Team Z").build_and_insert(conn).unwrap();

    user.id
}

#[actix_web::test]
async fn user_team_rows_returns_enriched_sorted_paginated_rows() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let uid = seed(&mut conn);

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/users/{}/team-rows?page=0&page_size=100", uid))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: PagedResponse<UserTeamRow> = test::read_body_json(resp).await;

    assert_eq!(body.count, 2);
    let names: Vec<&str> = body.items.iter().map(|t| t.name.as_str()).collect();
    assert_eq!(names, vec!["Team A", "Team B"]);
    for row in &body.items {
        assert_eq!(row.division_name, "Div 1");
    }
    // Team A: coached by the user → creator/modifier = the user.
    assert_eq!(body.items[0].coach_name, display_name("Dana"));
    assert_eq!(body.items[0].creator_name, display_name("Dana"));
    assert_eq!(body.items[0].last_modified_user_name, display_name("Dana"));
    // Team B: coached by Otto → creator/modifier = Otto.
    assert_eq!(body.items[1].coach_name, display_name("Otto"));
    assert_eq!(body.items[1].last_modified_user_name, display_name("Otto"));

    // Pagination: second page of size 1 → the second team only, count unchanged.
    let req2 = test::TestRequest::get()
        .uri(&format!("/api/users/{}/team-rows?page=1&page_size=1", uid))
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    let body2: PagedResponse<UserTeamRow> = test::read_body_json(resp2).await;
    assert_eq!(body2.count, 2);
    let names2: Vec<&str> = body2.items.iter().map(|t| t.name.as_str()).collect();
    assert_eq!(names2, vec!["Team B"]);
}
