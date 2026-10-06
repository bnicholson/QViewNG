
mod common;
mod fixtures;

use backend::models::permission::AppSpecialPermission;
use actix_http::StatusCode;
use actix_web::{App, test, web::{self,Bytes}};
use backend::{database::Database, models::{self, apicalllog::ApiCalllog, game::Game, game_statsgroup::GameStatsGroup}};
use backend::models::statsgroup::StatsGroup;
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use diesel::prelude::*;
use serde_json::json;
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database};

// The stats view endpoints (games/teamstats/individualstats) are restricted to holders of
// `system:administer`, the tournament owner, and its admins. Tests authenticate with that
// permission, which bypasses the owner/admin check regardless of the tournament.
fn system_admin_auth_header() -> (&'static str, String) {
    let token = common::make_token(
        uuid::Uuid::new_v4(),
        vec![],
        vec![AppSpecialPermission::SystemAdminister.as_str().to_string()],
    );
    ("Authorization", format!("Bearer {}", token))
}

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let payload = fixtures::statsgroups::arrange_create_works_integration_test(&mut conn);

    let created = models::statsgroup::create(&mut conn, &payload).expect("create failed");
    assert_eq!(created.name, payload.name);
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    fixtures::statsgroups::arrange_get_all_works_integration_test(&mut conn);

    // The list endpoint was removed; this now covers models::statsgroup::read_all directly.
    let pagination = backend::models::common::PaginationParams { page: PAGE_NUM, page_size: PAGE_SIZE };
    let result = models::statsgroup::read_all(&mut conn, &pagination).expect("read_all failed");
    assert_eq!(result.len(), 2);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let statsgroup = 
        fixtures::statsgroups::arrange_update_works_integration_test(&mut conn);

    let changeset = backend::models::statsgroup::StatsGroupChangeset { name: "My NEW name".to_string(), description: Some("NEW description".to_string()) };
    let updated = models::statsgroup::update(&mut conn, statsgroup.sgid, &changeset).expect("update failed");
    assert_eq!(updated.name, "My NEW name");
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let statsgroup = fixtures::statsgroups::arrange_delete_works_integration_test(&mut conn);

    let count = models::statsgroup::delete(&mut conn, statsgroup.sgid).expect("delete failed");
    assert_eq!(count, 1);
}

#[actix_web::test]
async fn delete_soft_deletes_and_purge_removes_row() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let statsgroup = fixtures::statsgroups::arrange_delete_works_integration_test(&mut conn);

    // Act + Assert: delete() is a soft delete — the row is hidden from reads but still present.
    let affected = models::statsgroup::delete(&mut conn, statsgroup.sgid).unwrap();
    assert_eq!(affected, 1);
    assert!(models::statsgroup::read(&mut conn, statsgroup.sgid).is_err());

    // The underlying row still exists with del_fl = true (raw query that ignores the flag).
    use backend::schema::statsgroups::dsl as sg;
    let raw_count: i64 = sg::statsgroups.filter(sg::sgid.eq(statsgroup.sgid)).count().get_result(&mut conn).unwrap();
    assert_eq!(raw_count, 1);
    let flag: bool = sg::statsgroups.filter(sg::sgid.eq(statsgroup.sgid)).select(sg::del_fl).first(&mut conn).unwrap();
    assert!(flag);

    // Act + Assert: purge() permanently removes the row.
    let purged = models::statsgroup::purge(&mut conn, statsgroup.sgid).unwrap();
    assert_eq!(purged, 1);
    let raw_count_after: i64 = sg::statsgroups.filter(sg::sgid.eq(statsgroup.sgid)).count().get_result(&mut conn).unwrap();
    assert_eq!(raw_count_after, 0);
}

#[actix_web::test]
async fn add_game_to_statsgroup_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (statsgroup, game, new_gsg) = fixtures::statsgroups::arrange_add_game_to_statsgroup_works_integration_test(&mut conn);

    let _created = models::game_statsgroup::create(&mut conn, &new_gsg).expect("create failed");
    let _ = game;
}

#[actix_web::test]
async fn get_all_games_of_statsgroup_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (statsgroup, game_1, game_2) = 
        fixtures::statsgroups::arrange_get_all_games_of_statsgroup_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/statsgroups/{}/games?page={}&page_size={}", statsgroup.sgid, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .insert_header(system_admin_auth_header())
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
        if body[idx].gid == game_1.gid {
            game_1_idx = idx;
        }
        if body[idx].gid == game_2.gid {
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

#[actix_web::test]
async fn remove_game_from_statsgroup_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (statsgroup, game, _) = 
        fixtures::statsgroups::arrange_remove_game_from_statsgroup_works_integration_test(&mut conn);

    let removed = models::game_statsgroup::delete(&mut conn, statsgroup.sgid, game.gid).expect("delete failed");
    assert_eq!(removed, 1);
}

#[actix_web::test]
async fn team_stats_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let statsgroup = fixtures::statsgroups::arrange_team_stats_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/statsgroups/{}/teamstats", statsgroup.sgid);
    let req = test::TestRequest::get()
        .uri(&uri)
        .insert_header(system_admin_auth_header())
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<backend::models::statsgroup::TeamStat> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 2);

    // Standings are ordered by wins desc, then total points desc.
    // Red answered 4 tossups (80 pts) and wins; Blue answered 2 (40 pts).
    //
    // Olympic points follow §12.4 for a 2-team round: 8 to the winner, 3 to the runner-up.
    // Modified Olympic (§12.5) adds 0.75 per 10 points above a threshold — 100 for 1st, 60 for
    // 2nd — so with 80 and 40 points neither team clears its threshold and both match the base.
    let red = &body[0];
    assert_eq!(red.name.as_str(), "Red Team");
    assert_eq!(red.place, 1);
    assert_eq!(red.games, 1);
    assert_eq!(red.wins, 1);
    assert_eq!(red.losses, 0);
    assert_eq!(red.total_points, 80);
    assert_eq!(red.olympic_points, 8);
    assert_eq!(red.mod_olympic_points, 8);

    let blue = &body[1];
    assert_eq!(blue.name.as_str(), "Blue Team");
    assert_eq!(blue.place, 2);
    assert_eq!(blue.games, 1);
    assert_eq!(blue.wins, 0);
    assert_eq!(blue.losses, 1);
    assert_eq!(blue.total_points, 40);
    assert_eq!(blue.olympic_points, 3);
    assert_eq!(blue.mod_olympic_points, 3);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

#[actix_web::test]
async fn individual_stats_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // Reuses the same example game: Red #1 answers 4 tossups (80 pts), Blue #1
    // answers 2 (40 pts); the other eight rostered quizzers score nothing.
    let statsgroup = fixtures::statsgroups::arrange_team_stats_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/statsgroups/{}/individualstats", statsgroup.sgid);
    let req = test::TestRequest::get()
        .uri(&uri)
        .insert_header(system_admin_auth_header())
        .to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Vec<backend::models::statsgroup::IndividualStat> = test::read_body_json(resp).await;

    // Both teams' full five-quizzer rosters appear (10 quizzers total).
    assert_eq!(body.len(), 10);

    let find = |team: &str, individual: &str| {
        body.iter().find(|s| s.team_name == team && s.individual == individual).unwrap().clone()
    };

    // Top scorer: Red #1.
    let red1 = find("Red Team", "Red #1");
    assert_eq!(red1.place, 1);
    assert_eq!(red1.games, 1);
    assert_eq!(red1.score, 80);
    assert!((red1.avg - 80.0).abs() < 1e-9);
    assert_eq!(red1.correct, 4);
    assert_eq!(red1.errors, 0);
    assert_eq!(red1.bonus_pts, 0);
    assert_eq!(red1.bonus_attempts, 0);

    // Second: Blue #1.
    let blue1 = find("Blue Team", "Blue #1");
    assert_eq!(blue1.place, 2);
    assert_eq!(blue1.games, 1);
    assert_eq!(blue1.score, 40);
    assert!((blue1.avg - 40.0).abs() < 1e-9);
    assert_eq!(blue1.correct, 2);

    // A non-scoring rostered quizzer still appears with a game played and zero score.
    let red2 = find("Red Team", "Red #2");
    assert_eq!(red2.games, 1);
    assert_eq!(red2.score, 0);
    assert_eq!(red2.correct, 0);

    // Check that ApiCalllog is recording API calls for this endpoint:
    let apicalllog_get_result = models::apicalllog::read_all(&mut conn);
    assert!(apicalllog_get_result.is_ok());
    let apicalllog_records: Vec<ApiCalllog> = apicalllog_get_result.unwrap();
    assert_eq!(apicalllog_records.iter().count(), 1);
    assert_eq!(apicalllog_records.first().unwrap().method.as_str(), "GET");
    assert_eq!(apicalllog_records.first().unwrap().uri, uri);
}

// ─── Tie-breaking rules ───────────────────────────────────────────────────────

#[derive(serde::Deserialize, Debug)]
struct TieBreakerSettingBody {
    rule: String,
    enabled: bool,
}

/// GET /api/statsgroups/{id}/tiebreaker-rules starts at the documented default order.
#[actix_web::test]
async fn tiebreaker_rules_default_to_the_documented_order() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let statsgroup = fixtures::statsgroups::arrange_team_stats_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/statsgroups/{}/tiebreaker-rules", statsgroup.sgid);
    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&uri).insert_header(system_admin_auth_header()).to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let rules: Vec<TieBreakerSettingBody> = test::read_body_json(resp).await;
    let keys: Vec<&str> = rules.iter().map(|r| r.rule.as_str()).collect();
    assert_eq!(
        keys,
        vec!["team_wl", "head_to_head", "total_points", "avg_points", "olympic", "mod_olympic"]
    );
    assert!(rules.iter().all(|r| r.enabled), "every criterion starts selected");
}

/// PUT replaces the order, and an unchecked criterion keeps its slot across a round trip.
#[actix_web::test]
async fn tiebreaker_rules_can_be_reordered_and_disabled() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let statsgroup = fixtures::statsgroups::arrange_team_stats_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/statsgroups/{}/tiebreaker-rules", statsgroup.sgid);
    let payload = json!({ "rules": [
        { "rule": "total_points",  "enabled": true  },
        { "rule": "team_wl",       "enabled": false },
        { "rule": "head_to_head",  "enabled": true  },
        { "rule": "avg_points",    "enabled": true  },
        { "rule": "olympic",       "enabled": false },
        { "rule": "mod_olympic",   "enabled": true  },
    ]});

    let put = test::call_service(
        &app,
        test::TestRequest::put()
            .uri(&uri)
            .insert_header(system_admin_auth_header())
            .set_json(&payload)
            .to_request(),
    ).await;
    assert_eq!(put.status(), StatusCode::OK);

    // Read it back: the order and the unchecked slots survive.
    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&uri).insert_header(system_admin_auth_header()).to_request(),
    ).await;
    let rules: Vec<TieBreakerSettingBody> = test::read_body_json(resp).await;
    let keys: Vec<&str> = rules.iter().map(|r| r.rule.as_str()).collect();
    assert_eq!(
        keys,
        vec!["total_points", "team_wl", "head_to_head", "avg_points", "olympic", "mod_olympic"]
    );
    assert!(!rules[1].enabled, "Team W-L stays in position 2, unchecked");
    assert!(!rules[4].enabled, "Olympic Pts stays in position 5, unchecked");
    assert!(rules[0].enabled && rules[2].enabled && rules[3].enabled && rules[5].enabled);
}

/// Saving the rules changes a stats group, so it needs `statsgroup:update`.
#[actix_web::test]
async fn saving_tiebreaker_rules_requires_statsgroup_update() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let statsgroup = fixtures::statsgroups::arrange_team_stats_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/statsgroups/{}/tiebreaker-rules", statsgroup.sgid);
    let payload = json!({ "rules": [{ "rule": "total_points", "enabled": true }] });

    // Unauthenticated.
    let anon = test::TestRequest::put().uri(&uri).set_json(&payload).to_request();
    assert_eq!(test::call_service(&app, anon).await.status(), StatusCode::FORBIDDEN);

    // Authenticated, but read-only on stats groups.
    let read_only = common::make_token(uuid::Uuid::new_v4(), vec![], vec!["statsgroup:read".to_string()]);
    let req = test::TestRequest::put()
        .uri(&uri)
        .insert_header(("Authorization", format!("Bearer {}", read_only)))
        .set_json(&payload)
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), StatusCode::FORBIDDEN);

    // With the permission.
    let permitted = common::make_token(uuid::Uuid::new_v4(), vec![], vec!["statsgroup:update".to_string()]);
    let req = test::TestRequest::put()
        .uri(&uri)
        .insert_header(("Authorization", format!("Bearer {}", permitted)))
        .set_json(&payload)
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), StatusCode::OK);
}

/// Each standings row says which criterion separated it from its neighbours, with that
/// criterion's number in the configured sequence.
#[actix_web::test]
async fn team_stats_explain_the_rule_that_set_each_rank() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let statsgroup = fixtures::statsgroups::arrange_team_stats_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/statsgroups/{}/teamstats", statsgroup.sgid);
    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&uri).insert_header(system_admin_auth_header()).to_request(),
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let rows = body.as_array().expect("teamstats returns an array");
    assert_eq!(rows.len(), 2);

    // Red (80 pts) beat Blue (40 pts) in their one game, so Team W-L — rule 1 — separates them.
    let red = &rows[0];
    assert_eq!(red["name"], "Red Team");
    assert_eq!(red["avg_points"], 80.0);
    assert!(red["rank_rule_above"].is_null(), "the leader has no team above it");
    assert_eq!(red["rank_rule_below"]["number"], 1);
    assert_eq!(red["rank_rule_below"]["rule"], "team_wl");
    assert_eq!(red["rank_rule_below"]["other_team"], "Blue Team");
    assert_eq!(red["rank_rule_below"]["detail"], "1-0 vs 0-1");

    let blue = &rows[1];
    assert_eq!(blue["name"], "Blue Team");
    assert_eq!(blue["avg_points"], 40.0);
    assert_eq!(blue["rank_rule_above"]["number"], 1);
    assert_eq!(blue["rank_rule_above"]["other_team"], "Red Team");
    assert_eq!(blue["rank_rule_above"]["detail"], "0-1 vs 1-0");
    assert!(blue["rank_rule_below"].is_null(), "the last team has none below it");
}

/// Reordering the criteria reorders the standings on the next read.
#[actix_web::test]
async fn reordering_the_rules_reorders_the_standings() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let statsgroup = fixtures::statsgroups::arrange_team_stats_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Database::new(TEST_DB_URL)))
            .configure(configure_routes)
    ).await;

    let rules_uri = format!("/api/statsgroups/{}/tiebreaker-rules", statsgroup.sgid);
    let stats_uri = format!("/api/statsgroups/{}/teamstats", statsgroup.sgid);

    // With every criterion unchecked, level teams fall back to name order — so Blue leads Red.
    let payload = json!({ "rules": [
        { "rule": "team_wl",      "enabled": false },
        { "rule": "head_to_head", "enabled": false },
        { "rule": "total_points", "enabled": false },
        { "rule": "avg_points",   "enabled": false },
        { "rule": "olympic",      "enabled": false },
        { "rule": "mod_olympic",  "enabled": false },
    ]});
    let put = test::call_service(
        &app,
        test::TestRequest::put()
            .uri(&rules_uri)
            .insert_header(system_admin_auth_header())
            .set_json(&payload)
            .to_request(),
    ).await;
    assert_eq!(put.status(), StatusCode::OK);

    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&stats_uri).insert_header(system_admin_auth_header()).to_request(),
    ).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let rows = body.as_array().unwrap();
    assert_eq!(rows[0]["name"], "Blue Team", "with no criteria enabled, name order decides");
    assert_eq!(rows[0]["place"], 1);
    assert!(rows[0]["rank_rule_below"].is_null(), "nothing separated them, so no rule to report");

    // Re-enable Team W-L and Red leads again.
    let payload = json!({ "rules": [{ "rule": "team_wl", "enabled": true }] });
    test::call_service(
        &app,
        test::TestRequest::put()
            .uri(&rules_uri)
            .insert_header(system_admin_auth_header())
            .set_json(&payload)
            .to_request(),
    ).await;

    let resp = test::call_service(
        &app,
        test::TestRequest::get().uri(&stats_uri).insert_header(system_admin_auth_header()).to_request(),
    ).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let rows = body.as_array().unwrap();
    assert_eq!(rows[0]["name"], "Red Team");
    assert_eq!(rows[0]["rank_rule_below"]["rule"], "team_wl");
}

