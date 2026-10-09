//! Keeping a QuizMachine client's event queue moving: quarantining events the server would refuse
//! on every retry, and the operator's Clear Pending + Resend.

mod common;
mod fixtures;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::cache::Cache;
use backend::models::{self, permission::AppSpecialPermission, room_queue_clear};
use backend::routes::configure_routes;
use uuid::Uuid;

use crate::common::*;

/// A token for an existing user. The clear records who started it, so the user must be real.
fn token_for(user_id: Uuid, permissions: &[&str]) -> (&'static str, String) {
    let token = make_token(user_id, vec![], permissions.iter().map(|p| p.to_string()).collect());
    ("Authorization", format!("Bearer {}", token))
}

fn system_admin(user_id: Uuid) -> (&'static str, String) {
    token_for(user_id, &[AppSpecialPermission::SystemAdminister.as_str()])
}

fn event(
    gid: &str,
    tournament: &str,
    division: &str,
    round: &str,
    room: &str,
    question: i32,
    eventnum: i32,
    nonce: &str,
) -> ClientGameEvent {
    ClientGameEvent {
        gid: gid.to_string(),
        tournament: tournament.to_string(),
        division: division.to_string(),
        room: room.to_string(),
        round: round.to_string(),
        question,
        eventnum,
        name: "Tournament".to_string(),
        team: 0,
        quizzer: 0,
        event: "RM".to_string(),
        parm1: String::new(),
        parm2: String::new(),
        nonce: nonce.to_string(),
    }
}

fn ping_uri(gid: Uuid, tournament: &str, division: &str, room: &str, round: &str, pending: i32) -> String {
    format!(
        "/api/pingmsg?gid={}&bldgroom=bldg&key=test-client-key&tk=tk1&tn={}&dn={}&rm={}&rd={}&qn=3&qmv=6.2.2&ts=1786991939&jp={}",
        gid,
        query_encode(tournament),
        query_encode(division),
        query_encode(room),
        query_encode(round),
        pending,
    )
}

fn stored(conn: &mut backend::database::Connection, gid: Uuid, question: i32, eventnum: i32) -> bool {
    models::gameevent::exists(conn, gid, question, eventnum)
}

// ─── Ingest ───────────────────────────────────────────────────────────────────

/// Baseline: a valid signed event is stored and acknowledged. Also proves the test signer matches
/// the server, which every other test here depends on.
#[actix_web::test]
async fn a_valid_signed_event_is_stored_and_acknowledged() {
    use_test_gameevent_psk();
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (game, tournament, division, round, room, ..) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).app_data(web::Data::new(Cache::disabled())).configure(configure_routes)).await;

    let ev = event(&game.gid.to_string(), &tournament.tname, &division.dname, &round.name, &room.name, 1, 0, "nonce-valid");
    let resp = test::call_service(&app, test::TestRequest::get().uri(&signed_gameevent_uri(&ev)).to_request()).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.headers().get("sha3512sum").unwrap().to_str().unwrap(), expected_ack_header("nonce-valid"));
    assert!(stored(&mut conn, game.gid, 1, 0));
    assert_eq!(models::gameevent_quarantine::count_of_room(&mut conn, room.roomid).unwrap(), 0);
}

/// An event for a game the server does not have used to be refused on every retry, holding up the
/// client's whole queue. It is now acknowledged — with the same response a stored event gets — and
/// kept in the quarantine with the reason.
#[actix_web::test]
async fn an_event_for_an_unknown_game_is_acknowledged_and_quarantined() {
    use_test_gameevent_psk();
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (_game, tournament, division, round, room, ..) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).app_data(web::Data::new(Cache::disabled())).configure(configure_routes)).await;

    let missing = Uuid::new_v4();
    let ev = event(&missing.to_string(), &tournament.tname, &division.dname, &round.name, &room.name, 4, 1, "nonce-unknown");
    let resp = test::call_service(&app, test::TestRequest::get().uri(&signed_gameevent_uri(&ev)).to_request()).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.headers().get("sha3512sum").unwrap().to_str().unwrap(), expected_ack_header("nonce-unknown"));
    let body = test::read_body(resp).await;
    assert_eq!(body, "Inserted/Updated", "indistinguishable from a stored event, so the client moves on");

    let rows = models::gameevent_quarantine::read_all_of_room(&mut conn, room.roomid).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].kind, "rejected");
    assert!(rows[0].reason.contains("does not exist"), "reason was {:?}", rows[0].reason);
    assert_eq!(rows[0].gid, Some(missing));
    assert_eq!((rows[0].question, rows[0].eventnum), (4, 1));
    assert!(rows[0].query_string.contains("nonce=nonce-unknown"), "the raw request is kept for replay");
    assert!(!stored(&mut conn, missing, 4, 1));
}

/// A request whose checksum does not match cannot be shown to come from a client, so it is still
/// refused and nothing is written — acknowledging it would let anyone add rows.
#[actix_web::test]
async fn a_bad_checksum_is_still_refused_and_writes_nothing() {
    use_test_gameevent_psk();
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (game, tournament, division, round, room, ..) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).app_data(web::Data::new(Cache::disabled())).configure(configure_routes)).await;

    let ev = event(&game.gid.to_string(), &tournament.tname, &division.dname, &round.name, &room.name, 1, 0, "nonce-bad");
    let uri = gameevent_uri_with_checksum(&ev, "not-the-checksum");
    let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).to_request()).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert!(resp.headers().get("sha3512sum").is_none());
    assert_eq!(models::gameevent_quarantine::count_of_room(&mut conn, room.roomid).unwrap(), 0);
    assert!(!stored(&mut conn, game.gid, 1, 0));
}

/// A non-numeric number used to panic the handler, leaving the client with no response at all.
#[actix_web::test]
async fn a_malformed_number_is_refused_rather_than_panicking() {
    use_test_gameevent_psk();
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (game, tournament, division, round, room, ..) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).app_data(web::Data::new(Cache::disabled())).configure(configure_routes)).await;

    let ev = event(&game.gid.to_string(), &tournament.tname, &division.dname, &round.name, &room.name, 1, 0, "nonce-nan");
    let uri = signed_gameevent_uri(&ev).replace("&qn=1&", "&qn=abc&");
    let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).to_request()).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(test::read_body(resp).await, "bad parameters");
}

// ─── Clear Pending + Resend ───────────────────────────────────────────────────

/// The whole run: while clearing, events are acknowledged but set aside; the ping that reports an
/// empty queue gets the Resend; the resent events are stored normally; the next ping completes it.
#[actix_web::test]
async fn clear_pending_sets_events_aside_then_resends_until_they_are_back() {
    use_test_gameevent_psk();
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (game, tournament, division, round, room, ..) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let operator = fixtures::users::seed_user(&mut conn);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).app_data(web::Data::new(Cache::disabled())).configure(configure_routes)).await;

    // Start the clear.
    let start = test::TestRequest::post()
        .uri(&format!("/api/rooms/{}/clear-pending", room.roomid))
        .insert_header(system_admin(operator.id))
        .to_request();
    assert_eq!(test::call_service(&app, start).await.status(), StatusCode::CREATED);

    // While clearing, the room's queued events are acknowledged but not applied.
    let gid = game.gid.to_string();
    for (q, e) in [(1, 0), (1, 1), (2, 0)] {
        let ev = event(&gid, &tournament.tname, &division.dname, &round.name, &room.name, q, e, &format!("clr-{q}-{e}"));
        let resp = test::call_service(&app, test::TestRequest::get().uri(&signed_gameevent_uri(&ev)).to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(!stored(&mut conn, game.gid, q, e), "set aside, not applied");
    }
    let clear = room_queue_clear::active_for_room(&mut conn, room.roomid).unwrap().unwrap();
    let set_aside = models::gameevent_quarantine::read_cleared_of_clear(&mut conn, clear.room_clear_id).unwrap();
    assert_eq!(set_aside.len(), 3);
    assert!(set_aside.iter().all(|r| r.kind == "cleared"));

    // A ping still reporting pending events changes nothing.
    let ping = ping_uri(game.gid, &tournament.tname, &division.dname, &room.name, &round.name, 2);
    let resp = test::call_service(&app, test::TestRequest::get().uri(&ping).to_request()).await;
    assert!(!std::str::from_utf8(&test::read_body(resp).await).unwrap().contains("cmd=Resend"));
    assert_eq!(room_queue_clear::active_for_room(&mut conn, room.roomid).unwrap().unwrap().status, "clearing");

    // The ping reporting an empty queue gets the Resend in that same response.
    let ping = ping_uri(game.gid, &tournament.tname, &division.dname, &room.name, &round.name, 0);
    let resp = test::call_service(&app, test::TestRequest::get().uri(&ping).to_request()).await;
    let body = String::from_utf8(test::read_body(resp).await.to_vec()).unwrap();
    assert!(body.contains("cmd=Resend") && body.contains(&format!("gameid={}", game.gid)), "body was {body:?}");
    let clear = room_queue_clear::active_for_room(&mut conn, room.roomid).unwrap().unwrap();
    assert_eq!(clear.status, "resending");
    assert_eq!(clear.resend_attempts, 1);
    assert!(clear.drained_at.is_some());

    // No longer clearing: the room's resend is stored normally.
    for (q, e) in [(1, 0), (1, 1), (2, 0)] {
        let ev = event(&gid, &tournament.tname, &division.dname, &round.name, &room.name, q, e, &format!("rs-{q}-{e}"));
        let resp = test::call_service(&app, test::TestRequest::get().uri(&signed_gameevent_uri(&ev)).to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(stored(&mut conn, game.gid, q, e));
    }

    // Everything set aside is back, so the next ping finishes the clear.
    let ping = ping_uri(game.gid, &tournament.tname, &division.dname, &room.name, &round.name, 0);
    test::call_service(&app, test::TestRequest::get().uri(&ping).to_request()).await;
    assert!(room_queue_clear::active_for_room(&mut conn, room.roomid).unwrap().is_none());
    let latest = room_queue_clear::latest_to_show_for_room(&mut conn, room.roomid).unwrap().unwrap();
    assert_eq!(latest.status, "complete");
}

/// Starting and cancelling a clear: signed in, `game:update`, and someone the Server Monitor is
/// shown to. One clear per room at a time; cancelling before anything was set aside ends it.
#[actix_web::test]
async fn clearing_requires_permission_and_one_runs_per_room() {
    use_test_gameevent_psk();
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (_game, _tournament, _division, _round, room, ..) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let stranger = fixtures::users::seed_user(&mut conn);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).app_data(web::Data::new(Cache::disabled())).configure(configure_routes)).await;
    let uri = format!("/api/rooms/{}/clear-pending", room.roomid);

    let anon = test::call_service(&app, test::TestRequest::post().uri(&uri).to_request()).await;
    assert_eq!(anon.status(), StatusCode::UNAUTHORIZED);

    // Can update games, but is not the owner, an admin, or a system administrator.
    let not_staff = test::TestRequest::post().uri(&uri).insert_header(token_for(stranger.id, &["game:update"])).to_request();
    assert_eq!(test::call_service(&app, not_staff).await.status(), StatusCode::FORBIDDEN);

    let ok = test::TestRequest::post().uri(&uri).insert_header(system_admin(stranger.id)).to_request();
    assert_eq!(test::call_service(&app, ok).await.status(), StatusCode::CREATED);

    let again = test::TestRequest::post().uri(&uri).insert_header(system_admin(stranger.id)).to_request();
    assert_eq!(test::call_service(&app, again).await.status(), StatusCode::CONFLICT);

    // Nothing has been set aside yet, so cancelling simply ends it.
    let cancel = test::TestRequest::delete().uri(&uri).insert_header(system_admin(stranger.id)).to_request();
    assert_eq!(test::call_service(&app, cancel).await.status(), StatusCode::OK);
    let latest = room_queue_clear::latest_to_show_for_room(&mut conn, room.roomid).unwrap().unwrap();
    assert_eq!(latest.status, "cancelled");
    assert_eq!(latest.finished_by, Some(stranger.id));
}

/// The quarantine carries raw client requests, so it is not public.
#[actix_web::test]
async fn the_quarantine_list_is_for_monitor_staff_only() {
    use_test_gameevent_psk();
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (_game, tournament, division, round, room, ..) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let operator = fixtures::users::seed_user(&mut conn);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).app_data(web::Data::new(Cache::disabled())).configure(configure_routes)).await;

    let ev = event(&Uuid::new_v4().to_string(), &tournament.tname, &division.dname, &round.name, &room.name, 2, 0, "nonce-q");
    test::call_service(&app, test::TestRequest::get().uri(&signed_gameevent_uri(&ev)).to_request()).await;

    let uri = format!("/api/rooms/{}/quarantine", room.roomid);
    let anon = test::call_service(&app, test::TestRequest::get().uri(&uri).to_request()).await;
    assert_eq!(anon.status(), StatusCode::UNAUTHORIZED);

    let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).insert_header(system_admin(operator.id)).to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let rows: Vec<serde_json::Value> = test::read_body_json(resp).await;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["kind"], "rejected");
}

/// The Server Monitor row says whether the room looks stuck, how many events it has quarantined,
/// and how its clear is going.
#[actix_web::test]
async fn the_monitor_reports_a_stalled_queue_the_quarantine_count_and_the_clear() {
    use_test_gameevent_psk();
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (game, tournament, division, round, room, ..) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let operator = fixtures::users::seed_user(&mut conn);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).app_data(web::Data::new(Cache::disabled())).configure(configure_routes)).await;

    // Two pings with the same pending count start the stall clock; backdate it past the threshold.
    for _ in 0..2 {
        let ping = ping_uri(game.gid, &tournament.tname, &division.dname, &room.name, &round.name, 5);
        test::call_service(&app, test::TestRequest::get().uri(&ping).to_request()).await;
    }
    let since = models::room::read(&mut conn, room.roomid).unwrap().ping_pending_stalled_since;
    assert!(since.is_some(), "a held pending count starts the clock");
    models::room::set_pending_stalled_since(
        &mut conn, room.roomid,
        Some(chrono::Utc::now() - models::room::PENDING_STALL_THRESHOLD - chrono::Duration::seconds(1)),
    ).unwrap();

    let ev = event(&Uuid::new_v4().to_string(), &tournament.tname, &division.dname, &round.name, &room.name, 2, 0, "nonce-m");
    test::call_service(&app, test::TestRequest::get().uri(&signed_gameevent_uri(&ev)).to_request()).await;
    room_queue_clear::start(&mut conn, room.roomid, operator.id).unwrap();

    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/api/tournaments/{}/roommonitor", tournament.tid))
            // The Server Monitor is restricted to owners, admins and system administrators.
            .insert_header(system_admin(operator.id))
            .to_request(),
    ).await;
    let rows: Vec<serde_json::Value> = test::read_body_json(resp).await;
    let row = rows.iter().find(|r| r["roomid"] == room.roomid.to_string() && r["is_room_row"] == true).unwrap();
    assert_eq!(row["pending_stalled"], true);
    assert_eq!(row["quarantine_count"], 1);
    assert_eq!(row["clear_status"], "clearing");

    // A falling count is a queue draining, not a stuck one.
    let ping = ping_uri(game.gid, &tournament.tname, &division.dname, &room.name, &round.name, 2);
    test::call_service(&app, test::TestRequest::get().uri(&ping).to_request()).await;
    let resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/api/tournaments/{}/roommonitor", tournament.tid))
            // The Server Monitor is restricted to owners, admins and system administrators.
            .insert_header(system_admin(operator.id))
            .to_request(),
    ).await;
    let rows: Vec<serde_json::Value> = test::read_body_json(resp).await;
    let row = rows.iter().find(|r| r["roomid"] == room.roomid.to_string() && r["is_room_row"] == true).unwrap();
    assert_eq!(row["pending_stalled"], false);
}

/// `request-resend` had no permission check at all.
#[actix_web::test]
async fn request_resend_now_requires_permission() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (game, ..) = fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let operator = fixtures::users::seed_user(&mut conn);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).app_data(web::Data::new(Cache::disabled())).configure(configure_routes)).await;
    let uri = format!("/api/games/{}/request-resend", game.gid);

    let anon = test::call_service(&app, test::TestRequest::post().uri(&uri).to_request()).await;
    assert_eq!(anon.status(), StatusCode::UNAUTHORIZED);

    let ok = test::TestRequest::post().uri(&uri).insert_header(system_admin(operator.id)).to_request();
    assert_eq!(test::call_service(&app, ok).await.status(), StatusCode::OK);
}
