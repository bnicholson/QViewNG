
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::{self, apicalllog::ApiCalllog, game::Game, team::TeamBuilder}, services::common::PagedResponse};
use backend::models::team::Team;
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use diesel::prelude::*;
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database, make_token};

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_, division, owner, admin_user, unrelated_user) =
        fixtures::teams::arrange_team_create_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = "/api/teams";

    // ── Success: tournament owner with team:create ────────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["team:create".to_string()],
    );

    let owner_payload = fixtures::teams::get_team_payload(&mut conn, division.did);
    let owner_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&owner_payload)
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;

    assert_eq!(owner_resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Team> = test::read_body_json(owner_resp).await;
    assert_eq!(body.code, 201);
    assert_eq!(body.message, "");

    let team = body.data.unwrap();
    assert_eq!(team.did, division.did);
    assert_eq!(team.name.as_str(), "Better Team than Last Year");
    assert_eq!(team.quizzer_two_id, owner_payload.quizzer_two_id);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "POST");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);

    // ── Success: tournament admin with team:create ────────────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["team:create".to_string()],
    );

    let admin_payload = TeamBuilder::new_default(division.did)
        .set_name("Admin Created Team")
        .set_coachid(fixtures::users::create_and_insert_user(&mut conn, "AdminCoach", "CoachPwd123!").id)
        .set_quizzer_one_id(fixtures::users::create_and_insert_user(&mut conn, "AdminQuizzer", "QuizPwd123!").id)
        .build()
        .unwrap();
    let admin_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(&admin_payload)
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;

    assert_eq!(admin_resp.status(), StatusCode::CREATED);

    // ── Success: user with team:create permission only (not owner or admin) ───

    let perm_only_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["team:create".to_string()],
    );

    let perm_only_payload = TeamBuilder::new_default(division.did)
        .set_name("Permission Only Team")
        .set_coachid(fixtures::users::create_and_insert_user(&mut conn, "PermCoach", "PermPwd123!").id)
        .set_quizzer_one_id(fixtures::users::create_and_insert_user(&mut conn, "PermQuizzer", "QuizPwd123!").id)
        .build()
        .unwrap();
    let perm_only_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", perm_only_token)))
        .set_json(&perm_only_payload)
        .to_request();

    let perm_only_resp = test::call_service(&app, perm_only_req).await;

    assert_eq!(perm_only_resp.status(), StatusCode::CREATED);

    // ── Fail: no permission, not owner, not admin, not coach ─────────────────────────────

    let no_auth_token = make_token(
        unrelated_user.id,
        vec!["member".to_string()],
        vec![],
    );

    let no_auth_payload = TeamBuilder::new_default(division.did)
        .set_name("Unauthorized Team")
        .set_coachid(fixtures::users::create_and_insert_user(&mut conn, "NoAuthCoach", "NoPwd123!").id)
        .build()
        .unwrap();
    let no_auth_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", no_auth_token)))
        .set_json(&no_auth_payload)
        .to_request();

    let no_auth_resp = test::call_service(&app, no_auth_req).await;

    assert_eq!(no_auth_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: authorized but no quizzer on the team ───────────────────────────

    let no_quizzer_payload = TeamBuilder::new_default(division.did)
        .set_name("No Quizzer Team")
        .set_coachid(fixtures::users::create_and_insert_user(&mut conn, "NoQuizCoach", "NoQuizPwd123!").id)
        .build()
        .unwrap();
    let no_quizzer_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&no_quizzer_payload)
        .to_request();

    let no_quizzer_resp = test::call_service(&app, no_quizzer_req).await;

    assert_eq!(no_quizzer_resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // Reset DB:

    clean_database();

    let (_tournament, division, _owner, _admin_user, _unrelated_user, coach_user) =
        fixtures::teams::arrange_team_create_as_coach_works_integration_test(&mut conn);

    let uri = "/api/teams";

    // ── Success: user creates a team listing themselves as coach (no extra permission needed) ──

    let coach_token = make_token(
        coach_user.id,
        vec!["member".to_string()],
        vec![],
    );

    let coach_payload = TeamBuilder::new_default(division.did)
        .set_name("Coach Self Registered Team")
        .set_coachid(coach_user.id)
        .set_quizzer_one_id(fixtures::users::create_and_insert_user(&mut conn, "CoachQuizzer", "QuizPwd123!").id)
        .build()
        .unwrap();
    let coach_req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {}", coach_token)))
        .set_json(&coach_payload)
        .to_request();

    let coach_resp = test::call_service(&app, coach_req).await;

    assert_eq!(coach_resp.status(), StatusCode::CREATED);

    let body: EntityResponse<Team> = test::read_body_json(coach_resp).await;
    assert_eq!(body.code, 201);
    let created_team = body.data.unwrap();
    assert_eq!(created_team.coachid, coach_user.id);
    assert_eq!(created_team.name.as_str(), "Coach Self Registered Team");
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    fixtures::teams::seed_teams(&mut conn, division.did);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/teams?page={}&page_size={}", PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    
    // Assert:
    
    assert_eq!(resp.status(), StatusCode::OK);

    let body: PagedResponse<Team> = test::read_body_json(resp).await;

    assert_eq!(body.items.len(), 3);
    assert_eq!(body.count, 3);

    let mut team_of_interest_idx = 10;
    for idx in 0..3 {
        if body.items[idx].name == "Luke Found a Frog" {
            team_of_interest_idx = idx;
            break;
        }
    }
    assert_ne!(team_of_interest_idx, 10);
    assert_eq!(body.items[team_of_interest_idx].did, division.did);
    
    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let tournament = fixtures::tournaments::seed_tournament(&mut conn, "Test Tour");
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);

    let teams: Vec<Team> = fixtures::teams::seed_teams(&mut conn, division.did);
    let team_of_interest_idx = 0;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/teams/{}", &teams[team_of_interest_idx].teamid);
    println!("Teams Get by ID URI: {}", &uri);
    let req = test::TestRequest::get()
        .uri(uri.as_str())
        .to_request();

    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:
    
    let team: Team = test::read_body_json(resp).await;
    assert_eq!(team.did, division.did);
    assert_eq!(team.name, teams[team_of_interest_idx].name);
    
    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_, division, team, owner, admin_user, unrelated_user) =
        fixtures::teams::arrange_team_update_works_integration_test(&mut conn);

    // A spare quizzer used to prove roster management (adding a quizzer) is permission-gated.
    let added_quizzer = fixtures::users::create_and_insert_user(&mut conn, "AddedRosterQuizzer", "QuizPwd999!");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let put_uri = format!("/api/teams/{}", team.teamid);

    // ── Success: tournament owner with team:update ────────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["team:update".to_string()],
    );

    let owner_payload = json!({ "name": "Owner Updated Team" });
    let owner_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&owner_payload)
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;

    assert_eq!(owner_resp.status(), StatusCode::OK);

    let owner_resp_body: EntityResponse<Team> = test::read_body_json(owner_resp).await;
    assert_eq!(owner_resp_body.code, 200);
    assert_eq!(owner_resp_body.message, "");

    let updated_team = owner_resp_body.data.unwrap();
    assert_eq!(updated_team.did, division.did);
    assert_eq!(updated_team.teamid, team.teamid);
    assert_eq!(updated_team.name.as_str(), "Owner Updated Team");
    assert_ne!(updated_team.created_at, updated_team.updated_at);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "PUT");
    assert_eq!(apicalllog_records.first().unwrap().uri, put_uri);

    // ── Success: tournament admin with team:update ────────────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["team:update".to_string()],
    );

    let admin_payload = json!({ "name": "Admin Updated Team" });
    let admin_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .set_json(&admin_payload)
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;

    assert_eq!(admin_resp.status(), StatusCode::OK);

    // ── Success: an authorized user manages the roster (adds a quizzer) ──────────
    // Roster add/remove goes through this same team-update endpoint, so it is gated by the
    // same permission that the "Add Quizzers" / "Remove" buttons are shown for.

    let roster_payload = json!({ "quizzer_two_id": added_quizzer.id });
    let roster_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&roster_payload)
        .to_request();

    let roster_resp = test::call_service(&app, roster_req).await;

    assert_eq!(roster_resp.status(), StatusCode::OK);
    let roster_body: EntityResponse<Team> = test::read_body_json(roster_resp).await;
    assert_eq!(roster_body.data.unwrap().quizzer_two_id, Some(added_quizzer.id));

    // ── Fail: an unauthorized user cannot manage the roster ─────────────────────
    let unauth_roster_payload = json!({ "quizzer_three_id": added_quizzer.id });
    let unauth_roster_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", make_token(
            unrelated_user.id,
            vec!["member".to_string()],
            vec!["team:read".to_string()],
        ))))
        .set_json(&unauth_roster_payload)
        .to_request();

    let unauth_roster_resp = test::call_service(&app, unauth_roster_req).await;
    assert_eq!(unauth_roster_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: has team:update but is neither owner, tournament admin nor coach ────────

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["team:update".to_string()],
    );

    let unrelated_payload = json!({ "name": "Unauthorized Update" });
    let unrelated_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .set_json(&unrelated_payload)
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;

    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: no team:update permission at all and not coach ────────────────────────────────

    let no_perm_token = make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["team:read".to_string()],
    );

    let no_perm_payload = json!({ "name": "No Permission Update" });
    let no_perm_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", no_perm_token)))
        .set_json(&no_perm_payload)
        .to_request();

    let no_perm_resp = test::call_service(&app, no_perm_req).await;

    assert_eq!(no_perm_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: authorized but update would remove all quizzers ────────────────

    let remove_quizzers_payload = json!({
        "quizzer_one_id": null,
        "quizzer_two_id": null,
        "quizzer_three_id": null,
        "quizzer_four_id": null,
        "quizzer_five_id": null,
        "quizzer_six_id": null,
    });
    let remove_quizzers_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&remove_quizzers_payload)
        .to_request();

    let remove_quizzers_resp = test::call_service(&app, remove_quizzers_req).await;

    assert_eq!(remove_quizzers_resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // Reset DB:

    clean_database();

    let (_tournament, _division, team, coach_user, _) =
        fixtures::teams::arrange_team_update_as_coach_works_integration_test(&mut conn);

    let put_uri = format!("/api/teams/{}", team.teamid);

    // ── Success: coach updates their own team without needing any extra permission ──

    let coach_token = make_token(
        coach_user.id,
        vec!["member".to_string()],
        vec![],
    );

    let coach_payload = json!({ "name": "Coach Updated Team Name" });
    let coach_req = test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", coach_token)))
        .set_json(&coach_payload)
        .to_request();

    let coach_resp = test::call_service(&app, coach_req).await;

    assert_eq!(coach_resp.status(), StatusCode::OK);

    let coach_resp_body: EntityResponse<Team> = test::read_body_json(coach_resp).await;
    assert_eq!(coach_resp_body.code, 200);
    let updated_team = coach_resp_body.data.unwrap();
    assert_eq!(updated_team.teamid, team.teamid);
    assert_eq!(updated_team.coachid, coach_user.id);
    assert_eq!(updated_team.name.as_str(), "Coach Updated Team Name");
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, _division, team_1, team_2, owner, admin_user, unrelated_user) =
        fixtures::teams::arrange_team_delete_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let delete_uri_1 = format!("/api/teams/{}", team_1.teamid);
    let delete_uri_2 = format!("/api/teams/{}", team_2.teamid);

    // ── Fail: has team:delete but is neither owner nor tournament admin ────────

    let unrelated_token = make_token(
        unrelated_user.id,
        vec!["tournament_manager".to_string()],
        vec!["team:delete".to_string()],
    );

    let unrelated_req = test::TestRequest::delete()
        .uri(&delete_uri_1)
        .insert_header(("Authorization", format!("Bearer {}", unrelated_token)))
        .to_request();

    let unrelated_resp = test::call_service(&app, unrelated_req).await;

    assert_eq!(unrelated_resp.status(), StatusCode::UNAUTHORIZED);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "DELETE");
    assert_eq!(apicalllog_records.first().unwrap().uri, delete_uri_1);

    // ── Fail: no team:delete permission at all ────────────────────────────────

    let no_perm_token = make_token(
        owner.id,
        vec!["member".to_string()],
        vec!["team:read".to_string()],
    );

    let no_perm_req = test::TestRequest::delete()
        .uri(&delete_uri_1)
        .insert_header(("Authorization", format!("Bearer {}", no_perm_token)))
        .to_request();

    let no_perm_resp = test::call_service(&app, no_perm_req).await;

    assert_eq!(no_perm_resp.status(), StatusCode::UNAUTHORIZED);

    // ── Success: tournament owner with team:delete ────────────────────────────

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["team:delete".to_string()],
    );

    let owner_req = test::TestRequest::delete()
        .uri(&delete_uri_1)
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .to_request();

    let owner_resp = test::call_service(&app, owner_req).await;

    assert_eq!(owner_resp.status(), StatusCode::OK);

    let owner_resp_body_bytes: Bytes = test::read_body(owner_resp).await;
    let owner_resp_body_string = String::from_utf8(owner_resp_body_bytes.to_vec()).unwrap();
    assert_eq!(&owner_resp_body_string, "");

    let get_by_id_req_1 = test::TestRequest::get()
        .uri(&delete_uri_1)
        .to_request();
    let get_by_id_resp_1 = test::call_service(&app, get_by_id_req_1).await;
    assert_eq!(get_by_id_resp_1.status(), StatusCode::NOT_FOUND);

    // ── Success: tournament admin with team:delete ────────────────────────────

    let admin_token = make_token(
        admin_user.id,
        vec!["tournament_manager".to_string()],
        vec!["team:delete".to_string()],
    );

    let admin_req = test::TestRequest::delete()
        .uri(&delete_uri_2)
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();

    let admin_resp = test::call_service(&app, admin_req).await;

    assert_eq!(admin_resp.status(), StatusCode::OK);

    let get_by_id_req_2 = test::TestRequest::get()
        .uri(&delete_uri_2)
        .to_request();
    let get_by_id_resp_2 = test::call_service(&app, get_by_id_req_2).await;
    assert_eq!(get_by_id_resp_2.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn delete_soft_deletes_and_purge_removes_row() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let tournament = fixtures::tournaments::seed_tournament(&mut conn, "SoftDelete Team Tour");
    let division = fixtures::divisions::seed_division(&mut conn, tournament.tid);
    let team = fixtures::teams::seed_teams(&mut conn, division.did).remove(0);

    // Act + Assert: delete() is a soft delete — the row is hidden from reads but still present.
    let affected = models::team::delete(&mut conn, team.teamid).unwrap();
    assert_eq!(affected, 1);
    assert!(models::team::read(&mut conn, team.teamid).is_err());

    // The underlying row still exists with del_fl = true (raw query that ignores the flag).
    use backend::schema::teams::dsl as t;
    let raw_count: i64 = t::teams.filter(t::teamid.eq(team.teamid)).count().get_result(&mut conn).unwrap();
    assert_eq!(raw_count, 1);
    let flag: bool = t::teams.filter(t::teamid.eq(team.teamid)).select(t::del_fl).first(&mut conn).unwrap();
    assert!(flag);

    // Act + Assert: purge() permanently removes the row.
    let purged = models::team::purge(&mut conn, team.teamid).unwrap();
    assert_eq!(purged, 1);
    let raw_count_after: i64 = t::teams.filter(t::teamid.eq(team.teamid)).count().get_result(&mut conn).unwrap();
    assert_eq!(raw_count_after, 0);
}

#[actix_web::test]
async fn get_all_games_of_team_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (team_4_id, game_2, game_3) = fixtures::games::seed_get_games_of_team(&mut conn);

    let pagination = backend::models::common::PaginationParams { page: PAGE_NUM, page_size: PAGE_SIZE };
    let result = models::game::read_all_games_of_team(&mut conn, team_4_id, &pagination).expect("read failed");
    assert!(!result.is_empty());
}

#[actix_web::test]
async fn quizzer_cannot_be_on_two_teams_in_same_division() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (tournament, division, owner, _admin_user, _unrelated_user) =
        fixtures::teams::arrange_team_create_works_integration_test(&mut conn);

    let coach = fixtures::users::create_and_insert_user(&mut conn, "Casey", "CoachPwd123!");
    let quizzer = fixtures::users::create_and_insert_user(&mut conn, "Quinn", "QuizPwd123!");
    let quizzer_2 = fixtures::users::create_and_insert_user(&mut conn, "Quincy", "QuizPwd123!");

    let app = test::init_service(
        App::new().app_data(web::Data::new(db)).configure(configure_routes)
    ).await;

    let owner_token = make_token(
        owner.id,
        vec!["tournament_manager".to_string()],
        vec!["team:create".to_string()],
    );

    // First team in the division, with two quizzers.
    let team_1 = TeamBuilder::new_default(division.did)
        .set_name("Lightning")
        .set_coachid(coach.id)
        .set_quizzer_one_id(quizzer.id)
        .set_quizzer_two_id(quizzer_2.id)
        .build()
        .unwrap();
    let resp_1 = test::call_service(&app, test::TestRequest::post()
        .uri("/api/teams")
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&team_1)
        .to_request()).await;
    assert_eq!(resp_1.status(), StatusCode::CREATED);

    // A second team in the SAME division reusing both quizzers must be rejected, and the message
    // must list every conflicting quizzer (both) with their existing team.
    let team_2 = TeamBuilder::new_default(division.did)
        .set_name("Thunder")
        .set_coachid(coach.id)
        .set_quizzer_one_id(quizzer.id)
        .set_quizzer_two_id(quizzer_2.id)
        .build()
        .unwrap();
    let resp_2 = test::call_service(&app, test::TestRequest::post()
        .uri("/api/teams")
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&team_2)
        .to_request()).await;
    assert_eq!(resp_2.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let body: serde_json::Value = test::read_body_json(resp_2).await;
    let msg = body["error"].as_str().unwrap();
    assert_eq!(
        msg,
        "The following quizzers are already registered on other teams in division \"Test Div\": Quinn Maurice Den (team \"Lightning\"), Quincy Maurice Den (team \"Lightning\"). Quizzers cannot be on multiple teams for the same division."
    );

    // The same quizzer on a team in a DIFFERENT division is allowed.
    let other_division = backend::models::division::DivisionBuilder::new_default("Other Div", tournament.tid)
        .build_and_insert(&mut conn)
        .unwrap();
    let team_3 = TeamBuilder::new_default(other_division.did)
        .set_name("Comets")
        .set_coachid(coach.id)
        .set_quizzer_one_id(quizzer.id)
        .build()
        .unwrap();
    let resp_3 = test::call_service(&app, test::TestRequest::post()
        .uri("/api/teams")
        .insert_header(("Authorization", format!("Bearer {}", owner_token)))
        .set_json(&team_3)
        .to_request()).await;
    assert_eq!(resp_3.status(), StatusCode::CREATED);
}
