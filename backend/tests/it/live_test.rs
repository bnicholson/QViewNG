//! Live updates over Server-Sent Events: the Games in Progress and Server Monitor streams.
use crate::fixtures;

use std::pin::Pin;
use std::time::Duration;

use actix_http::StatusCode;
use actix_web::body::MessageBody;
use actix_web::{App, test, web};
use backend::cache::Cache;
use backend::live::{self, LiveHub};
use backend::models::permission::AppSpecialPermission;
use backend::models::room::{GamesInProgressRow, RoomMonitorRow};
use backend::routes::configure_routes;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::common::*;

/// A hub with test timings: quick to combine triggers, no minimum gap, and no sweep.
fn test_hub(db: &backend::database::Database) -> LiveHub {
    LiveHub::start_with(
        live::snapshot_builder(db.clone(), Cache::disabled()),
        Duration::from_millis(20),
        Duration::ZERO,
        Duration::from_secs(3600),
    )
}

fn bearer(token: String) -> (&'static str, String) {
    ("Authorization", format!("Bearer {token}"))
}

/// A system administrator's token; the Server Monitor is shown to them in every tournament.
fn system_admin() -> (&'static str, String) {
    bearer(make_token(Uuid::new_v4(), vec![], vec![AppSpecialPermission::SystemAdminister.as_str().to_string()]))
}

/// A system administrator's token that expires `secs` from now.
fn system_admin_expiring_in(secs: i64) -> (&'static str, String) {
    let exp = (chrono::Utc::now() + chrono::Duration::seconds(secs)).timestamp() as usize;
    let claims = serde_json::json!({
        "sub": Uuid::new_v4().to_string(),
        "exp": exp,
        "roles": [],
        "permissions": [AppSpecialPermission::SystemAdminister.as_str()],
    });
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"qview_dev_secret_changeme"))
        .expect("Failed to encode test JWT");
    bearer(token)
}

fn ping_uri(gid: Uuid, tournament: &str, division: &str, room: &str, round: &str, question: i32) -> String {
    format!(
        "/api/pingmsg?gid={}&bldgroom=bldg&key=test-client-key&tk=tk1&tn={}&dn={}&rm={}&rd={}&qn={}&qmv=6.2.2&ts=1786991939&jp=0",
        gid,
        query_encode(tournament),
        query_encode(division),
        query_encode(room),
        query_encode(round),
        question,
    )
}

/// Reads a streaming response body one Server-Sent Events frame at a time.
struct Frames<B: MessageBody> {
    body: Pin<Box<B>>,
    buffer: String,
}

impl<B: MessageBody> Frames<B> {
    fn new(body: B) -> Self {
        Frames { body: Box::pin(body), buffer: String::new() }
    }

    /// The next frame, without its trailing blank line. `None` once the stream has ended.
    async fn next(&mut self) -> Option<String> {
        loop {
            if let Some(end) = self.buffer.find("\n\n") {
                let frame = self.buffer[..end].to_string();
                self.buffer.drain(..end + 2);
                return Some(frame);
            }
            let chunk = tokio::time::timeout(
                Duration::from_secs(5),
                std::future::poll_fn(|cx| self.body.as_mut().poll_next(cx)),
            )
            .await
            .expect("timed out waiting for the stream");
            match chunk {
                Some(Ok(bytes)) => self.buffer.push_str(std::str::from_utf8(&bytes).expect("UTF-8 frame")),
                Some(Err(_)) => panic!("the stream body errored"),
                None => return None,
            }
        }
    }

    /// The next snapshot, skipping keepalives.
    async fn next_snapshot<T: DeserializeOwned>(&mut self) -> T {
        loop {
            let frame = self.next().await.expect("the stream ended before a snapshot");
            if let Some(json) = frame.strip_prefix("event: snapshot\ndata: ") {
                return serde_json::from_str(json).expect("a snapshot is the endpoint's JSON");
            }
        }
    }
}

#[actix_web::test]
async fn games_in_progress_stream_pushes_a_ping_as_it_happens() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (game, tournament, division, round, room, ..) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let hub = test_hub(&db);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(Cache::disabled()))
            .app_data(web::Data::new(hub.clone()))
            .configure(configure_routes),
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/gamesinprogress/stream", tournament.tid))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.headers().get("content-type").unwrap(), "text/event-stream");
    let mut frames = Frames::new(resp.into_body());

    // Nothing has checked in yet.
    let first: Vec<GamesInProgressRow> = frames.next_snapshot().await;
    assert!(first.is_empty(), "no room has pinged yet: {first:?}");
    assert_eq!(hub.viewers(live::Topic::GamesInProgress(tournament.tid)), 1);

    // The room checks in; the open stream gets the new state without being asked.
    let ping = test::TestRequest::get()
        .uri(&ping_uri(game.gid, &tournament.tname, &division.dname, &room.name, &round.name, 7))
        .to_request();
    assert_eq!(test::call_service(&app, ping).await.status(), StatusCode::OK);

    let after: Vec<GamesInProgressRow> = frames.next_snapshot().await;
    assert_eq!(after.len(), 1, "the room that pinged should be on the tickertape: {after:?}");
    assert_eq!(after[0].room.as_deref(), Some(room.name.as_str()));
    assert_eq!(after[0].question, Some(7));
}

#[actix_web::test]
async fn room_monitor_stream_pushes_a_game_event_as_it_happens() {
    use_test_gameevent_psk();
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (game, tournament, division, round, room, ..) =
        fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let hub = test_hub(&db);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(Cache::disabled()))
            .app_data(web::Data::new(hub.clone()))
            .configure(configure_routes),
    ).await;

    // The room is on question 1 of its game.
    let ping = test::TestRequest::get()
        .uri(&ping_uri(game.gid, &tournament.tname, &division.dname, &room.name, &round.name, 1))
        .to_request();
    assert_eq!(test::call_service(&app, ping).await.status(), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/roommonitor/stream", tournament.tid))
        .insert_header(system_admin())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let mut frames = Frames::new(resp.into_body());

    let first: Vec<RoomMonitorRow> = frames.next_snapshot().await;
    let row = first.iter().find(|r| r.roomid == room.roomid && r.is_room_row).expect("the room's row");
    assert_eq!(row.quarantine_count, 0);

    // An event for a game the server does not have is acknowledged and quarantined. That changes
    // only the Server Monitor's quarantine count, and must still reach the open stream.
    let ev = ClientGameEvent {
        gid: Uuid::new_v4().to_string(),
        tournament: tournament.tname.clone(),
        division: division.dname.clone(),
        room: room.name.clone(),
        round: round.name.clone(),
        question: 1,
        eventnum: 0,
        name: "Tournament".to_string(),
        team: 0,
        quizzer: 0,
        event: "RM".to_string(),
        parm1: String::new(),
        parm2: String::new(),
        nonce: "nonce-live-quarantine".to_string(),
    };
    let resp = test::call_service(&app, test::TestRequest::get().uri(&signed_gameevent_uri(&ev)).to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let after: Vec<RoomMonitorRow> = frames.next_snapshot().await;
    let row = after.iter().find(|r| r.roomid == room.roomid && r.is_room_row).expect("the room's row");
    assert_eq!(row.quarantine_count, 1, "the quarantined event should show on the live monitor");
}

#[actix_web::test]
async fn room_monitor_stream_is_refused_to_anyone_the_monitor_is_not_shown_to() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (_game, tournament, ..) = fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let hub = test_hub(&db);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(Cache::disabled()))
            .app_data(web::Data::new(hub.clone()))
            .configure(configure_routes),
    ).await;
    let uri = format!("/api/tournaments/{}/roommonitor/stream", tournament.tid);

    // Signed out.
    let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).to_request()).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // Can read the monitor in general, but is not this tournament's owner or one of its admins.
    let outsider = bearer(make_token(Uuid::new_v4(), vec![], vec!["roommonitor:read".to_string()]));
    let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).insert_header(outsider).to_request()).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // The tournament's owner may watch it.
    let owner = bearer(make_token(tournament.owner_id, vec![], vec!["roommonitor:read".to_string()]));
    let resp = test::call_service(&app, test::TestRequest::get().uri(&uri).insert_header(owner).to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);

    assert_eq!(hub.viewers(live::Topic::RoomMonitor(tournament.tid)), 1, "only the owner's stream is open");
}

#[actix_web::test]
async fn room_monitor_stream_ends_when_its_token_expires() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (_game, tournament, ..) = fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let hub = test_hub(&db);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(Cache::disabled()))
            .app_data(web::Data::new(hub.clone()))
            .configure(configure_routes),
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/roommonitor/stream", tournament.tid))
        .insert_header(system_admin_expiring_in(2))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let mut frames = Frames::new(resp.into_body());

    let _: Vec<RoomMonitorRow> = frames.next_snapshot().await;
    // Next comes the expiry notice (no keepalive is due within 2 s), then the end of the stream.
    assert_eq!(frames.next().await.as_deref(), Some("event: expired\ndata: {}"));
    assert_eq!(frames.next().await, None, "the stream should end after an expiry notice");
}

#[actix_web::test]
async fn streams_are_unavailable_without_a_hub_so_pages_poll() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let (_game, tournament, ..) = fixtures::games::seed_1_game_with_minimum_required_dependencies(&mut conn);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(Cache::disabled()))
            .configure(configure_routes),
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/gamesinprogress/stream", tournament.tid))
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), StatusCode::SERVICE_UNAVAILABLE);

    // The triggers are optional too: a ping into an app without a hub still succeeds.
    let resp = test::call_service(&app, test::TestRequest::get().uri("/api/pingmsg?tn=nope&rm=nope").to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn games_in_progress_stream_of_an_unknown_tournament_is_not_found() {
    let db = test_db();
    let hub = test_hub(&db);
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(Cache::disabled()))
            .app_data(web::Data::new(hub.clone()))
            .configure(configure_routes),
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/gamesinprogress/stream", Uuid::new_v4()))
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), StatusCode::NOT_FOUND);
    assert!(!hub.is_watching_anything());
}
