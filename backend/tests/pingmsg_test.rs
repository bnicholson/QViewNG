mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::database::Database;
use backend::routes::configure_routes;
use crate::common::{TEST_DB_URL, clean_database};

// Mirrors the handler's encoder (RFC 3986 unreserved kept; everything else -> %XX).
fn percent_encode(s: &str) -> String {
    let mut out = String::new();
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

// Seeds events with a question gap (1 and 3 present, 2 missing) so the game reads as
// "incomplete" — required for the resend command to be issued.
fn seed_gap_events(conn: &mut backend::database::Connection, gid: uuid::Uuid) {
    use backend::models::gameevent::{GameEventBuilder, GameEventCode};
    for q in [1, 3] {
        GameEventBuilder::new_default(gid)
            .set_question(Some(q))
            .set_eventnum(Some(0))
            .set_name(Some("Team A".to_string()))
            .set_team(Some(0))
            .set_quizzer(Some(0))
            .set_event(Some(GameEventCode::TC))
            .build_and_insert(conn)
            .unwrap();
    }
}

#[actix_web::test]
async fn pingmsg_records_checkin_and_resend() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (game, tournament, division, round, room, _, _, _, _, _) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);

    // Flag a resend request on the game so the ping should emit a Resend command.
    let mut req_changes = backend::models::game::GameChangeset::empty();
    req_changes.resend_gameevents_request_ts = Some(chrono::Utc::now());
    backend::models::game::update(&mut conn, game.gid, &req_changes, game.last_modified_user).unwrap();

    // The game's events are incomplete (a resend is only issued while data has gaps).
    seed_gap_events(&mut conn, game.gid);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    // The client sends compound-key + ping data (values percent-encoded, e.g. "Tour 1" -> "Tour%201").
    let uri = format!(
        "/api/pingmsg?gid={}&bldgroom=bawl&key=abc123&tk=tk1&tn={}&dn={}&rm={}&rd={}&qn=9&qmv=6.2.2-b15&ts=1786991939&jp=8",
        game.gid,
        percent_encode(&tournament.tname),
        percent_encode(&division.dname),
        percent_encode(&room.name),
        percent_encode(&round.name),
    );
    let req = test::TestRequest::get().uri(&uri).to_request();

    // Act:

    let resp = test::call_service(&app, req).await;

    // Assert:

    assert_eq!(resp.status(), StatusCode::OK);

    let expected_resend = format!(
        "quizzes?cmd=Resend&tournament={}&division={}&room={}&round={}",
        percent_encode(&tournament.tname),
        percent_encode(&division.dname),
        percent_encode(&room.name),
        percent_encode(&round.name),
    );

    // The Resend command appears (URL-encoded) in the response body.
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains(&expected_resend), "response body was: {:?}", body_str);

    // The command is stored on the game in its encoded form.
    let updated_game = backend::models::game::read(&mut conn, game.gid).unwrap();
    assert_eq!(updated_game.resend_gameevents_response.as_deref(), Some(expected_resend.as_str()));

    // The room's ping_* fields are populated from the ping data.
    let updated_room = backend::models::room::read(&mut conn, room.roomid).unwrap();
    assert!(updated_room.ping_last_checkin_ts.is_some());
    assert!(updated_room.ping_client_ts.is_some());
    assert_eq!(updated_room.ping_question_number, Some(9));
    assert_eq!(updated_room.ping_qm_version.as_deref(), Some("6.2.2-b15"));
    assert_eq!(updated_room.ping_jobspending, Some(8));
    assert_eq!(updated_room.ping_room.as_deref(), Some(room.name.as_str()));
    assert_eq!(updated_room.ping_round.as_deref(), Some(round.name.as_str()));
    assert_eq!(updated_room.ping_game_id, Some(game.gid));
}

#[actix_web::test]
async fn request_resend_endpoint_flags_game_then_ping_returns_command() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (game, tournament, division, round, room, _, _, _, _, _) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);

    // No resend requested yet.
    let before = backend::models::game::read(&mut conn, game.gid).unwrap();
    assert!(before.resend_gameevents_request_ts.is_none());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    // Act 1 — click "Resend": POST the request-resend endpoint.

    let resend_req = test::TestRequest::post()
        .uri(&format!("/api/games/{}/request-resend", game.gid))
        .to_request();
    let resend_resp = test::call_service(&app, resend_req).await;

    // Assert 1 — endpoint succeeds and the game is now flagged for resend.

    assert_eq!(resend_resp.status(), StatusCode::OK);
    let flagged = backend::models::game::read(&mut conn, game.gid).unwrap();
    assert!(flagged.resend_gameevents_request_ts.is_some());
    // The request is live: it hasn't been sent yet.
    assert!(flagged.resend_request_sent_ts.is_none());

    // The game's events are incomplete (a resend is only issued while data has gaps).
    seed_gap_events(&mut conn, game.gid);

    // Act 2 — the room's next ping.

    let uri = format!(
        "/api/pingmsg?gid={}&bldgroom=bawl&key=abc123&tk=tk1&tn={}&dn={}&rm={}&rd={}&qn=9&qmv=6.2.2-b15&ts=1786991939&jp=8",
        game.gid,
        percent_encode(&tournament.tname),
        percent_encode(&division.dname),
        percent_encode(&room.name),
        percent_encode(&round.name),
    );
    let ping_req = test::TestRequest::get().uri(&uri).to_request();
    let ping_resp = test::call_service(&app, ping_req).await;

    // Assert 2 — the ping returns the resend command, stored (encoded) on the game.

    assert_eq!(ping_resp.status(), StatusCode::OK);

    let expected_resend = format!(
        "quizzes?cmd=Resend&tournament={}&division={}&room={}&round={}",
        percent_encode(&tournament.tname),
        percent_encode(&division.dname),
        percent_encode(&room.name),
        percent_encode(&round.name),
    );
    let body = test::read_body(ping_resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains(&expected_resend), "response body was: {:?}", body_str);

    let after = backend::models::game::read(&mut conn, game.gid).unwrap();
    assert_eq!(after.resend_gameevents_response.as_deref(), Some(expected_resend.as_str()));
    // The resend has now been marked as sent.
    assert!(after.resend_request_sent_ts.is_some());

    // Act 3 — a second ping (data still incomplete, resend still requested).

    let ping_req_2 = test::TestRequest::get().uri(&uri).to_request();
    let ping_resp_2 = test::call_service(&app, ping_req_2).await;

    // Assert 3 — the resend command is NOT sent again (it is issued exactly once).

    assert_eq!(ping_resp_2.status(), StatusCode::OK);
    let body_2 = test::read_body(ping_resp_2).await;
    let body_2_str = std::str::from_utf8(&body_2).unwrap();
    assert!(
        !body_2_str.contains(&expected_resend),
        "second ping should not resend the command, but body was: {:?}",
        body_2_str
    );
}
