
use actix_web::{Error, HttpRequest, HttpResponse, Result, delete, get, post, put, web::{Data, Json, Path}};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::game::NewGame;
use chrono::{ DateTime, Utc, TimeZone };
use std::line;
use crate::models::apicalllog;
use crate::models::game;
use crate::models::game::GameChangeset;
use crate::database::{self, Database};
use crate::models::{gameevent_quarantine, room_queue_clear::{self, ClearAction}};
use std::collections::HashSet;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

/// Percent-encodes a value (RFC 3986 unreserved kept, everything else -> %XX, so " " -> %20).
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

/// The Resend command for one game, as the client expects it on its own line.
fn resend_line(conn: &mut database::Connection, tournament_name: &str, room_name: &str, g: &game::Game) -> String {
    let div_name = crate::models::game::read_division_of_game(conn, g).map(|d| d.dname).unwrap_or_default();
    let round_name = crate::models::round::read(conn, g.roundid).map(|r| r.name).unwrap_or_default();
    format!(
        "quizzes?cmd=Resend&tournament={}&division={}&room={}&round={}&gameid={}",
        percent_encode(tournament_name),
        percent_encode(&div_name),
        percent_encode(room_name),
        percent_encode(&round_name),
        g.gid,
    )
}

/// The events a clear set aside that can come back and have not yet.
///
/// An event can come back only if it names a game the server has and a real question (>= 1);
/// anything else stays in the quarantine for an operator. It has come back once it is in
/// `gameevents`, or once it was quarantined again as rejected after the queue drained — it
/// arrived, and was set aside with a reason.
fn outstanding(
    conn: &mut database::Connection,
    clear: &room_queue_clear::RoomQueueClear,
    cleared: &[gameevent_quarantine::QuarantinedGameEvent],
) -> Vec<gameevent_quarantine::QuarantinedGameEvent> {
    let since = clear.drained_at.unwrap_or(clear.requested_at);
    let mut out = Vec::new();
    for e in cleared {
        let Some(gid) = e.gid else { continue };
        if e.question < 1 {
            continue; // cannot come back
        }
        match game::row_exists(conn, gid) {
            Ok(false) => continue, // cannot come back
            Ok(true) => {}
            // Unsure, so keep waiting: an error must never let a clear finish early.
            Err(_) => { out.push(e.clone()); continue; }
        }
        let stored = crate::models::gameevent::exists(conn, gid, e.question, e.eventnum);
        let rejected_again =
            gameevent_quarantine::rejected_since(conn, gid, e.question, e.eventnum, since).unwrap_or(false);
        if !(stored || rejected_again) {
            out.push(e.clone());
        }
    }
    out
}

/// Moves the room's Clear Pending + Resend along, appending any Resend lines to the response.
///
/// See `room_queue_clear::decide` for the rules; this applies them. A ping that reports the
/// queue drained both starts the resend and sends it, so the room is asked in that same response.
#[allow(clippy::too_many_arguments)]
fn advance_room_clear(
    conn: &mut database::Connection,
    room_id: Uuid,
    jobs_pending: Option<i32>,
    current_game: Option<Uuid>,
    tournament_name: &str,
    room_name: &str,
    now: DateTime<Utc>,
    emitted: &mut HashSet<Uuid>,
    body: &mut String,
) {
    let Ok(Some(mut clear)) = room_queue_clear::active_for_room(conn, room_id) else { return };

    // At most two steps per ping: StartResending, then the first SendResend.
    for _ in 0..2 {
        let cleared = gameevent_quarantine::read_cleared_of_clear(conn, clear.room_clear_id).unwrap_or_default();
        let still_out = outstanding(conn, &clear, &cleared);
        let action = room_queue_clear::decide(&clear, jobs_pending, cleared.len(), still_out.is_empty(), now);
        log::info!("{} {} Room {} clear {} ({}): {:?}", module_path!(), line!(), room_id, clear.room_clear_id, clear.status, action);

        match action {
            ClearAction::Wait => return,
            ClearAction::Finish(status) => {
                let _ = room_queue_clear::finish(conn, clear.room_clear_id, status, None);
                return;
            }
            ClearAction::StartResending => match room_queue_clear::mark_resending(conn, clear.room_clear_id, now, current_game) {
                Ok(updated) => clear = updated, // then send, in this same response
                Err(e) => {
                    log::error!("{} {} Could not start resending for clear {}: {:?}", module_path!(), line!(), clear.room_clear_id, e);
                    return;
                }
            },
            ClearAction::SendResend => {
                // First time: every game the clear set events aside for, plus the game the room is
                // on. After that: only the games still missing something.
                let mut games: Vec<Uuid> = if clear.resend_attempts == 0 {
                    cleared.iter().filter_map(|e| e.gid).collect()
                } else {
                    still_out.iter().filter_map(|e| e.gid).collect()
                };
                if clear.resend_attempts == 0 {
                    games.extend(clear.resend_game_id);
                }
                let mut seen = HashSet::new();
                games.retain(|g| seen.insert(*g));

                for gid in games {
                    if emitted.contains(&gid) {
                        continue;
                    }
                    if let Ok(g) = game::read(conn, gid) {
                        body.push_str(&resend_line(conn, tournament_name, room_name, &g));
                        body.push('\n');
                        emitted.insert(gid);
                    }
                }
                let _ = room_queue_clear::record_resend_sent(conn, clear.room_clear_id, now);
                return;
            }
        }
    }
}

#[get("")]
async fn index(
    db: Data<Database>,
    req: HttpRequest,
) -> HttpResponse {
    let mut conn = db.get_connection().expect("Failed to get connection");

    // log this api call
    apicalllog::create(&mut conn, &req);

    // Pull the compound-key parameters from the ping.
    let qs = qstring::QString::from(req.query_string());
    let param = |key: &str| qs.get(key).unwrap_or("").replace("+", " ");
    let tn = param("tn"); // tournament name
    let dn = param("dn"); // division name
    let rm = param("rm"); // room name
    let rd = param("rd"); // round number/name

    let mut body = String::new();

    // Resolve the tournament (by name), then the room within it.
    let tid: Option<Uuid> = {
        use crate::schema::tournaments::dsl as t;
        t::tournaments.filter(t::tname.eq(&tn)).select(t::tid).first::<Uuid>(&mut conn).ok()
    };

    if let Some(tid) = tid {
        if let Ok(room) = crate::models::room::find_by_name_in_tournament(&mut conn, &rm, tid) {
            // Resolve the round (via division) for the composite-key game lookup.
            let round_id: Option<Uuid> = {
                use crate::schema::divisions::dsl as d;
                use crate::schema::rounds;
                d::divisions
                    .filter(d::tid.eq(tid))
                    .filter(d::dname.eq(&dn))
                    .select(d::did)
                    .first::<Uuid>(&mut conn)
                    .ok()
                    .and_then(|div_id| {
                        rounds::table
                            .filter(rounds::did.eq(div_id))
                            .filter(rounds::name.eq(&rd))
                            .filter(rounds::del_fl.eq(false))
                            .select(rounds::roundid)
                            .first::<Uuid>(&mut conn)
                            .ok()
                    })
            };

            // Find the game: prefer the 'gid' query param, else fall back to the composite key.
            let game = qs.get("gid")
                .and_then(|s| Uuid::parse_str(s.trim()).ok())
                .and_then(|gid| game::read(&mut conn, gid).ok())
                .or_else(|| round_id.and_then(|rid| {
                    use crate::schema::games::dsl as g;
                    g::games
                        .filter(g::roomid.eq(room.roomid))
                        .filter(g::roundid.eq(rid))
                        .first::<crate::models::game::Game>(&mut conn)
                        .ok()
                }));

            // Record the ping data on the room (including this check-in timestamp).
            let mut room_changes = crate::models::room::RoomChangeset::empty();
            room_changes.ping_question_number = qs.get("qn").and_then(|s| s.trim().parse::<i32>().ok());
            room_changes.ping_qm_version = qs.get("qmv").map(|s| s.trim().to_string());
            room_changes.ping_client_ts = qs.get("ts")
                .and_then(|s| s.trim().parse::<i64>().ok())
                .and_then(|secs| DateTime::from_timestamp(secs, 0));
            room_changes.ping_jobspending = qs.get("jp").and_then(|s| s.trim().parse::<i32>().ok());
            room_changes.ping_room = Some(rm.clone());
            room_changes.ping_round = Some(rd.clone());
            room_changes.ping_host_ip = qs.get("myip").map(|s| s.replace("+", " "));
            room_changes.ping_game_id = game.as_ref().map(|g| g.gid);
            room_changes.ping_last_checkin_ts = Some(Utc::now());
            let _ = crate::models::room::update(&mut conn, room.roomid, &room_changes, room.last_modified_user);

            // Whether the room's queue is stuck is judged here, from consecutive pings, rather
            // than in each browser watching the Server Monitor. `room` still holds the previous
            // ping, which is what the comparison needs.
            let now = Utc::now();
            let stalled_since = crate::models::room::next_pending_stalled_since(
                room.ping_jobspending,
                room.ping_pending_stalled_since,
                room_changes.ping_jobspending,
                now,
            );
            if stalled_since != room.ping_pending_stalled_since {
                let _ = crate::models::room::set_pending_stalled_since(&mut conn, room.roomid, stalled_since);
            }

            // Games a resend line has gone out for in this response, so no game is asked twice.
            let mut emitted: HashSet<Uuid> = HashSet::new();

            // Process resend state for EVERY game in this room that has an active resend
            // request or a stored resend response — not just the current ping game. This lets
            // the operator resend a non-current game while the room is reporting, and clears a
            // game's response once its data is whole again (so it drops off the Room Monitor).
            let pagination = crate::models::common::PaginationParams {
                page: 0,
                page_size: crate::models::common::PaginationParams::MAX_PAGE_SIZE as i64,
            };
            let resend_games: Vec<crate::models::game::Game> = {
                use crate::schema::games::dsl as g;
                g::games
                    .filter(g::roomid.eq(room.roomid))
                    .filter(
                        g::resend_gameevents_request_ts.is_not_null()
                            .or(g::resend_gameevents_response.is_not_null()),
                    )
                    .load::<crate::models::game::Game>(&mut conn)
                    .unwrap_or_default()
            };

            for g in resend_games {
                let events = crate::models::gameevent::read_all_gameevents_of_game(&mut conn, g.gid, &pagination)
                    .unwrap_or_default();
                let complete = !crate::models::gameevent::events_have_gaps(&events);

                if complete {
                    // All events accounted for — the resend request is fulfilled; blank the response.
                    if g.resend_gameevents_response.as_deref().map_or(false, |s| !s.is_empty()) {
                        let mut changes = GameChangeset::empty();
                        changes.resend_gameevents_response = Some(String::new());
                        let _ = game::update(&mut conn, g.gid, &changes, g.last_modified_user);
                    }
                } else if g.resend_gameevents_request_ts.is_some() && g.resend_request_sent_ts.is_none() {
                    // Incomplete + a fresh resend request not yet sent: issue the command once for
                    // THIS game's division/round (which may differ from the current ping), then
                    // stamp resend_request_sent_ts so future pings won't repeat it.
                    let resend_line = resend_line(&mut conn, &tn, &rm, &g);
                    body.push_str(&resend_line);
                    body.push('\n');
                    emitted.insert(g.gid);

                    let mut changes = GameChangeset::empty();
                    changes.resend_gameevents_response = Some(resend_line);
                    changes.resend_request_sent_ts = Some(Utc::now());
                    let _ = game::update(&mut conn, g.gid, &changes, g.last_modified_user);
                }
            }

            advance_room_clear(
                &mut conn,
                room.roomid,
                room_changes.ping_jobspending,
                game.as_ref().map(|g| g.gid),
                &tn,
                &rm,
                now,
                &mut emitted,
                &mut body,
            );

            // Last, after every write above: the question, check-in, pending count, stalled flag,
            // resend and clear state all show on the live pages.
            crate::live::tournament_changed(&req, tid);
        }
    }

    // Send uncompressed: the QuizMachine client can't decode br/gzip, so opt this
    // endpoint out of the app-wide Compress middleware by declaring identity encoding.
    let response = HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .insert_header(actix_web::http::header::ContentEncoding::Identity)
        .body(body.clone());

    // Log the entire response being sent (status line, headers, and body).
    let mut headers_dump = String::new();
    for (name, value) in response.headers() {
        headers_dump.push_str(&format!("{}: {}\n", name, value.to_str().unwrap_or("<non-utf8>")));
    }
    // println!(
    //     "pingmsg full response:\n{:?} {}\n{}\n{}",
    //     response.head().version,
    //     response.status(),
    //     headers_dump,
    //     body
    // );

    response
}

// #[get("/{id}")]
// async fn read(
//     db: Data<Database>,
//     item_id: Path<Uuid>,
//     req: HttpRequest,
// ) -> HttpResponse {
//     println!("read endpoint");
//     // let mut db = db.pool.get().unwrap();
//     let mut db = db.get_connection().expect("Failed to get connection");
    
//     // log this api call
//     apicalllog::create(&mut db, &req);

//     let result = game::read(&mut db, item_id.into_inner());

//     if result.is_ok() {
//         HttpResponse::Ok().json(result.unwrap())
//     } else {
//         HttpResponse::NotFound().finish()
//     }
// }

// #[post("")]
// async fn create(
//     db: Data<Database>,
//     Json(item): Json<NewGame>,
//     req: HttpRequest,
// ) -> Result<HttpResponse, Error> {
//     println!("create endpoint");

//     let mut db = db.pool.get().unwrap();

//     // log this api call
//     apicalllog::create(&mut db, &req);

//     Ok(HttpResponse::Ok().json(item))

//     // let result: Game = game::create(&mut db, &item).expect("Creation error");

//     // Ok(HttpResponse::Created().json(result))
// }

// #[put("/{id}")]
// async fn update(
//     db: Data<Database>,
//     item_id: Path<Uuid>,
//     Json(item): Json<GameChangeset>,
//     req: HttpRequest,
// ) -> HttpResponse {
//     println!("update endpoint");
//     let mut db = db.pool.get().unwrap();

//     // log this api call
//     apicalllog::create(&mut db, &req);

//     let result = game::update(&mut db, item_id.into_inner(), &item);

//     if result.is_ok() {
//         HttpResponse::Ok().finish()
//     } else {
//         HttpResponse::InternalServerError().finish()
//     }
// }

// #[delete("/{id}")]
// async fn destroy(
//     db: Data<Database>,
//     item_id: Path<Uuid>,
//     req: HttpRequest,
// ) -> HttpResponse {
//     println!("destroy endpoint");
//     let mut db = db.pool.get().unwrap();

//     // log this api call
//     apicalllog::create(&mut db, &req);

//     let result = game::delete(&mut db, item_id.into_inner());

//     if result.is_ok() {
//         HttpResponse::Ok().finish()
//     } else {
//         HttpResponse::InternalServerError().finish()
//     }
// }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        // .service(read)
        // .service(create)
        // .service(update)
        // .service(destroy);
}
