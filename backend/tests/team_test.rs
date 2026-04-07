
mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::{self, apicalllog::ApiCalllog, game::Game, team::TeamBuilder}, services::common::PagedResponse};
use backend::models::team::Team;
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
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
async fn get_all_games_of_team_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (team_4_id, game_2, game_3) = fixtures::games::seed_get_games_of_team(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/teams/{}/games?page={}&page_size={}", team_4_id, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<Game> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut game_1_idx = 10;
    let mut game_2_idx = 10;
    for idx in 0..len {
        if body[idx].gid == game_2.gid {
            game_1_idx = idx;
        }
        if body[idx].gid == game_3.gid {
            game_2_idx = idx;
        }
    }
    assert_ne!(game_1_idx, 10);
    assert_ne!(game_2_idx, 10);
    
    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}
