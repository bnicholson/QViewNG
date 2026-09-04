mod common;
mod fixtures;

use actix_web::{test, App, web, http::StatusCode};
use backend::database::Database;
use backend::models::team::QuizzerRow;
use backend::models::user::UserBuilder;
use backend::models::tournament::TournamentBuilder;
use backend::models::division::DivisionBuilder;
use backend::routes::configure_routes;
use crate::common::{TEST_DB_URL, clean_database};

/// Seeds one tournament → one division → three teams (with 0, 2, and 6 quizzers = 8 distinct
/// quizzers) and returns (tid, did) plus the division name.
fn seed(conn: &mut backend::database::Connection) -> (uuid::Uuid, uuid::Uuid) {
    let owner = UserBuilder::new_default("Quizzer Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(conn)
        .unwrap();
    let tournament = TournamentBuilder::new_default("Quizzer Rows Tour")
        .set_owner_id(owner.id)
        .build_and_insert(conn)
        .unwrap();
    let division = DivisionBuilder::new_default("Test Div", tournament.tid)
        .build_and_insert(conn)
        .unwrap();
    // team 1: no quizzers, team 2: Trishell + David, team 3: Tyler/Taylor/Tiffany/Sam/John/Lucas
    fixtures::teams::seed_teams(conn, division.did);
    (tournament.tid, division.did)
}

/// The 8 distinct quizzer first names seeded by `fixtures::teams::seed_teams`, sorted the way
/// the endpoint orders them (by fname, then mname, then lname — the defaults are shared, so
/// this reduces to fname-ascending).
fn expected_sorted_fnames() -> Vec<&'static str> {
    vec!["David", "John", "Lucas", "Sam", "Taylor", "Tiffany", "Trishell", "Tyler"]
}

#[actix_web::test]
async fn tournament_quizzer_rows_returns_enriched_sorted_rows() {
    // Arrange
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (tid, _did) = seed(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes),
    )
    .await;

    // Act
    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/quizzer-rows", tid))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::OK);
    let rows: Vec<QuizzerRow> = test::read_body_json(resp).await;

    // 8 distinct quizzers, ordered alphabetically by first name.
    assert_eq!(rows.len(), 8);
    let fnames: Vec<&str> = rows.iter().map(|r| r.fname.as_str()).collect();
    assert_eq!(fnames, expected_sorted_fnames());

    // Every row is scoped to the single seeded division and is on at least one team.
    for row in &rows {
        assert_eq!(row.divisions.len(), 1, "each quizzer belongs to exactly one division here");
        assert_eq!(row.divisions[0].name, "Test Div");
        assert!(!row.teams.is_empty(), "each quizzer is on at least one team");
        // Sensitive user fields are never part of the DTO — only safe columns are present.
        assert!(!row.email.is_empty());
    }

    // Spot-check specific team membership.
    let david = rows.iter().find(|r| r.fname == "David").expect("David present");
    assert!(david.teams.iter().any(|t| t.name == "Come Get Some"));

    let tyler = rows.iter().find(|r| r.fname == "Tyler").expect("Tyler present");
    assert!(tyler.teams.iter().any(|t| t.name == "Luke Found a Frog"));
}

#[actix_web::test]
async fn division_quizzer_rows_returns_enriched_sorted_rows() {
    // Arrange
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (_tid, did) = seed(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes),
    )
    .await;

    // Act
    let req = test::TestRequest::get()
        .uri(&format!("/api/divisions/{}/quizzer-rows", did))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), StatusCode::OK);
    let rows: Vec<QuizzerRow> = test::read_body_json(resp).await;

    assert_eq!(rows.len(), 8);
    let fnames: Vec<&str> = rows.iter().map(|r| r.fname.as_str()).collect();
    assert_eq!(fnames, expected_sorted_fnames());
    for row in &rows {
        assert_eq!(row.divisions[0].name, "Test Div");
    }
}
