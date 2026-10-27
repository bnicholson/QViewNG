use crate::fixtures;
use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::models::{self, division::DivisionBuilder, question_type::ParsedQuestionSetFile, round::{DivisionQuestionTypes, Round, RoundBuilder}};
use backend::routes::configure_routes;
use backend::services::common::EntityResponse;
use chrono::{TimeZone, Utc};
use serde_json::json;
use crate::common::{make_token, test_db};

const SET_1: &str = "GARGGVGAGGQGAGXGSAGG";
const SET_2: &str = "AGVGGQGAGGRAGXSGGAGG";

const RTF_FILE: &str = include_str!("fixtures/question_sets/experienced_sets_1_2.rtf");
const CSV_FILE: &str = include_str!("fixtures/question_sets/experienced_sets_1_2.csv");

fn uploader_permissions() -> Vec<String> {
    vec!["round:update".to_string(), "round:upload_question_types".to_string()]
}

// ── PUT /api/rounds/{id} with question types ──────────────────────────────────

#[actix_web::test]
async fn update_round_question_types_validates_on_save() {

    // Arrange:

    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_, _, round, owner, _, _) =
        fixtures::rounds::arrange_round_update_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let put_uri = format!("/api/rounds/{}", round.roundid);
    let token = make_token(owner.id, vec!["tournament_manager".to_string()], vec!["round:update".to_string()]);

    let put = |payload: serde_json::Value| test::TestRequest::put()
        .uri(&put_uri)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(payload)
        .to_request();

    // ── Success: a standard set is saved ──────────────────────────────────────

    let resp = test::call_service(&app, put(json!({ "question_types": SET_1 }))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: EntityResponse<Round> = test::read_body_json(resp).await;
    let saved = body.data.unwrap();
    assert_eq!(saved.question_types, SET_1);
    assert!(saved.using_standard_question_types);

    // ── Fail: a standard set with the wrong counts ────────────────────────────

    let all_g = "G".repeat(20);
    let resp = test::call_service(&app, put(json!({ "question_types": all_g }))).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(models::round::read(&mut conn, round.roundid).unwrap().question_types, SET_1);

    // ── Fail: not every question has a type ───────────────────────────────────

    let resp = test::call_service(&app, put(json!({ "question_types": "GARGG" }))).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // ── Fail: M is inferred and never stored ──────────────────────────────────

    let with_m = SET_1.replacen('Q', "M", 1);
    let resp = test::call_service(&app, put(json!({ "question_types": with_m, "using_standard_question_types": false }))).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // ── Success: any mix once the standard check is off ───────────────────────

    let resp = test::call_service(&app, put(json!({ "question_types": all_g, "using_standard_question_types": false }))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let saved = models::round::read(&mut conn, round.roundid).unwrap();
    assert_eq!(saved.question_types, all_g);
    assert!(!saved.using_standard_question_types);

    // ── Fail: turning the standard check back on with non-standard types ──────

    let resp = test::call_service(&app, put(json!({ "using_standard_question_types": true }))).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // ── Fail: S and I in the same standard set ────────────────────────────────

    let s_and_i = SET_1.replacen('G', "I", 1);
    let resp = test::call_service(&app, put(json!({ "question_types": s_and_i, "using_standard_question_types": true }))).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // ── Success: S and I together once the standard check is off ──────────────

    let resp = test::call_service(&app, put(json!({ "question_types": s_and_i, "using_standard_question_types": false }))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(models::round::read(&mut conn, round.roundid).unwrap().question_types, s_and_i);

    // ── Success: an empty string clears the round's question types ────────────

    let resp = test::call_service(&app, put(json!({ "question_types": "", "using_standard_question_types": true }))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(models::round::read(&mut conn, round.roundid).unwrap().question_types, "");
}

// ── POST /api/divisions/{id}/round-question-types/parse ───────────────────────

#[actix_web::test]
async fn parse_round_question_types_reads_rtf_and_csv() {

    // Arrange:

    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_, division, _, owner, admin_user, unrelated_user) =
        fixtures::rounds::arrange_round_update_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/divisions/{}/round-question-types/parse", division.did);
    let parse = |token: &str, filename: &str, content: &str| test::TestRequest::post()
        .uri(&uri)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({ "filename": filename, "content": content }))
        .to_request();

    let owner_token = make_token(owner.id, vec!["tournament_manager".to_string()], uploader_permissions());

    // ── Success: RTF ──────────────────────────────────────────────────────────

    let resp = test::call_service(&app, parse(&owner_token, "Experienced QView Questions.rtf", RTF_FILE)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let parsed: ParsedQuestionSetFile = test::read_body_json(resp).await;
    assert!(parsed.errors.is_empty());
    assert_eq!(parsed.sets.len(), 2);
    assert_eq!(parsed.sets[0].name, "Set #1 Experienced QView Questions");
    assert_eq!(parsed.sets[0].question_types, SET_1);
    assert_eq!(parsed.sets[1].question_types, SET_2);
    assert!(parsed.sets.iter().all(|s| s.errors.is_empty()));

    // ── Success: CSV, as a tournament admin ───────────────────────────────────

    let admin_token = make_token(admin_user.id, vec!["tournament_admin".to_string()], uploader_permissions());
    let resp = test::call_service(&app, parse(&admin_token, "Experienced QView Questions.csv", CSV_FILE)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let parsed: ParsedQuestionSetFile = test::read_body_json(resp).await;
    assert_eq!(parsed.sets.len(), 2);
    assert_eq!(parsed.sets[0].name, "Set #1 Experienced QView Questions");
    assert_eq!(parsed.sets[0].question_types, SET_1);
    assert_eq!(parsed.sets[1].question_types, SET_2);

    // ── Unsupported file type is reported, not an error status ────────────────

    let resp = test::call_service(&app, parse(&owner_token, "questions.txt", "hello")).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let parsed: ParsedQuestionSetFile = test::read_body_json(resp).await;
    assert!(parsed.sets.is_empty());
    assert!(!parsed.errors.is_empty());

    // ── Fail: round:update without round:upload_question_types ────────────────

    let no_upload_token = make_token(owner.id, vec!["tournament_manager".to_string()], vec!["round:update".to_string()]);
    let resp = test::call_service(&app, parse(&no_upload_token, "q.csv", CSV_FILE)).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: has both permissions but is neither owner nor tournament admin ──

    let unrelated_token = make_token(unrelated_user.id, vec!["tournament_manager".to_string()], uploader_permissions());
    let resp = test::call_service(&app, parse(&unrelated_token, "q.csv", CSV_FILE)).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

// ── PUT + GET /api/divisions/{id}/round-question-types ────────────────────────

#[actix_web::test]
async fn update_division_round_question_types_is_all_or_nothing() {

    // Arrange:

    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_, division, round_1, owner, _, unrelated_user) =
        fixtures::rounds::arrange_round_update_works_integration_test(&mut conn);
    let round_2 = RoundBuilder::new_default(division.did)
        .set_name("2")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2050, 1, 1, 1, 0, 0).unwrap())
        .build_and_insert(&mut conn)
        .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    let uri = format!("/api/divisions/{}/round-question-types", division.did);
    let owner_token = make_token(owner.id, vec!["tournament_manager".to_string()], uploader_permissions());
    let put = |token: &str, payload: serde_json::Value| test::TestRequest::put()
        .uri(&uri)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(payload)
        .to_request();

    let read_all = || test::TestRequest::get().uri(&uri).to_request();

    // ── Nothing saved yet: incomplete ─────────────────────────────────────────

    let resp = test::call_service(&app, read_all()).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let listed: DivisionQuestionTypes = test::read_body_json(resp).await;
    assert!(!listed.complete);
    assert_eq!(listed.rounds.iter().map(|r| r.roundid).collect::<Vec<_>>(), vec![round_1.roundid, round_2.roundid]);

    // ── Fail: one invalid round means neither round is saved ──────────────────

    let resp = test::call_service(&app, put(&owner_token, json!([
        { "roundid": round_1.roundid, "question_types": SET_1 },
        { "roundid": round_2.roundid, "question_types": "G".repeat(20) },
    ]))).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(models::round::read(&mut conn, round_1.roundid).unwrap().question_types, "");
    assert_eq!(models::round::read(&mut conn, round_2.roundid).unwrap().question_types, "");

    // ── Fail: a round from another division ───────────────────────────────────

    let other_division = DivisionBuilder::new_default("Other Div", division.tid)
        .build_and_insert(&mut conn)
        .unwrap();
    let other_round = fixtures::rounds::seed_round(&mut conn, other_division.did);
    let resp = test::call_service(&app, put(&owner_token, json!([
        { "roundid": other_round.roundid, "question_types": SET_1 },
    ]))).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(models::round::read(&mut conn, other_round.roundid).unwrap().question_types, "");

    // ── Fail: not an owner or admin of the tournament ─────────────────────────

    let unrelated_token = make_token(unrelated_user.id, vec!["tournament_manager".to_string()], uploader_permissions());
    let resp = test::call_service(&app, put(&unrelated_token, json!([
        { "roundid": round_1.roundid, "question_types": SET_1 },
    ]))).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // ── Success: one round only, still incomplete ─────────────────────────────

    let resp = test::call_service(&app, put(&owner_token, json!([
        { "roundid": round_1.roundid, "question_types": SET_1 },
    ]))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let listed: DivisionQuestionTypes = test::read_body_json(test::call_service(&app, read_all()).await).await;
    assert!(!listed.complete);

    // ── Success: both rounds, now complete ────────────────────────────────────

    let resp = test::call_service(&app, put(&owner_token, json!([
        { "roundid": round_1.roundid, "question_types": SET_1 },
        { "roundid": round_2.roundid, "question_types": "G".repeat(20), "using_standard_question_types": false },
    ]))).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let saved: Vec<Round> = test::read_body_json(resp).await;
    assert_eq!(saved.len(), 2);

    let listed: DivisionQuestionTypes = test::read_body_json(test::call_service(&app, read_all()).await).await;
    assert!(listed.complete);
    assert_eq!(listed.rounds[0].question_types, SET_1);
    assert_eq!(listed.rounds[1].question_types, "G".repeat(20));
    assert!(!listed.rounds[1].using_standard_question_types);
    assert!(models::round::division_has_complete_question_types(&mut conn, division.did).unwrap());
}
