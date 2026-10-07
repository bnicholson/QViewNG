
use actix_web::{Error, HttpRequest, HttpResponse, Result, delete, get, post, put, web::{Data, Json, Path}};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::game::NewGame;
use chrono::{ DateTime, Utc, TimeZone };
use std::line;
use crate::models::apicalllog;
use crate::models::game;
use crate::models::game::GameChangeset;
use crate::database::Database;

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
                use crate::schema::{rounds, roundgroups};
                d::divisions
                    .filter(d::tid.eq(tid))
                    .filter(d::dname.eq(&dn))
                    .select(d::did)
                    .first::<Uuid>(&mut conn)
                    .ok()
                    .and_then(|div_id| {
                        // A round belongs to a division roundgroup; match by name across the division's roundgroups.
                        rounds::table
                            .inner_join(roundgroups::table.on(rounds::roundgroup_id.eq(roundgroups::roundgroup_id)))
                            .filter(roundgroups::did.eq(div_id))
                            .filter(rounds::name.eq(&rd))
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

            let _ = &game; // the current game is captured on the room above; resend is handled per-game below

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
                    let div_name = crate::models::game::read_division_of_game(&mut conn, &g)
                        .map(|d| d.dname)
                        .unwrap_or_default();
                    let round_name = crate::models::round::read(&mut conn, g.roundid)
                        .map(|r| r.name)
                        .unwrap_or_default();
                    let resend_line = format!(
                        "quizzes?cmd=Resend&tournament={}&division={}&room={}&round={}&gameid={}",
                        percent_encode(&tn),
                        percent_encode(&div_name),
                        percent_encode(&rm),
                        percent_encode(&round_name),
                        g.gid,
                    );
                    body.push_str(&resend_line);
                    body.push('\n');

                    let mut changes = GameChangeset::empty();
                    changes.resend_gameevents_response = Some(resend_line);
                    changes.resend_request_sent_ts = Some(Utc::now());
                    let _ = game::update(&mut conn, g.gid, &changes, g.last_modified_user);
                }
            }
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
    println!(
        "pingmsg full response:\n{:?} {}\n{}\n{}",
        response.head().version,
        response.status(),
        headers_dump,
        body
    );

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
