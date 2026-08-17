
use actix_web::{Error, HttpRequest, HttpResponse, Result, delete, get, post, put, web::{Data, Json, Path}};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::game::NewGame;
use chrono::{ DateTime, Utc, TimeZone };
use std::line;
use crate::models::apicalllog;
use crate::models::roominfo;
use crate::models::game;
use crate::models::game::GameChangeset;
use crate::database::Database;

pub async fn write(
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    // let db = req.app_data::<Data<Database>>().unwrap();
    // let mut mdb = db.pool.get().unwrap();

    log::info!("Inside pingmsg");

    // log this api call (*Actually let's do this in one place: Let the endpoint fn do the logging.)
    // let mut db = db.get_connection().expect("Failed to get connection");
    // apicalllog::create(&mut db, &req);

    // let's create the roominfo structure and start filling it in
    let mut roominfo_entry = roominfo::empty();

    // Okay, it's now time to search all the parameters and set the associated 
    // variables set in all the data that we will write to the cache
    // or to the database
    let qs = qstring::QString::from(req.query_string());
    let ps = qs.to_pairs();
    let psiter = ps.iter();
    let mut tk=String::new();
    let mut org = "Nazarene".to_string();
    // let mut qn_str = String::new();
    // let mut e_str = String::new();
    // let mut t_str = String::new();
    // let mut q_str = String::new();
    let mut ts = Utc::now();
    // let mut gid: i64  = 0;
    let mut field_count = 0;
    for pair in psiter {
        let s = String::from(pair.0);
        match s.as_str() {
            "bldgroom" => {
                roominfo_entry.bldgroom = pair.1.replace("+"," ");
                field_count += 1;               
            },
            "key" => {  // key4server - uniquely identifies a particular client
                roominfo_entry.clientkey = pair.1.replace("+"," ");
                field_count += 1;
            },
            "tk" => {   // tournament key - short id for a particular tournament
                tk = pair.1.replace("+"," ");   // currently used except to ensure we don't have corruption
                field_count += 1;                
            },
            "org" => {
                org = pair.1.replace("+"," ");   // don't bump the field count because it's not sent by client
            },
            "tn" => { // Tournament Name
                roominfo_entry.tournament = pair.1.replace("+"," ");
                field_count += 1;
            },
            "dn" => { // Division Name
                roominfo_entry.division = pair.1.replace("+"," ");
                field_count += 1;
            },
            "rm" => { // Room 
                roominfo_entry.room = pair.1.replace("+"," ");
                field_count += 1;
            },
            "rd" => { // Round
                roominfo_entry.round = pair.1.replace("+"," ");
                field_count += 1;
            }, 
            "qn" => { // Question #
                roominfo_entry.question = pair.1.trim().parse().unwrap();
                field_count += 1;
            },
            "ts" => { // timestamp from the client
                let secs : i64 = pair.1.trim().parse().unwrap();
                ts = Utc.timestamp_opt(secs, 0).unwrap();
                field_count += 1;
            }, 
            "qmv" => {
                roominfo_entry.qm_version = pair.1.trim().to_string();
                field_count += 1;
            },
            "jp" => {
                roominfo_entry.jobs_pending = pair.1.trim().parse().unwrap();
                field_count += 1;
            },
            "myip" => {
                // this is optional should only be there sometimes.
                let tmp = pair.1.replace("+"," ");
                roominfo_entry.clientip = tmp.to_string();
            }
            _ => {
                log::error!("{:?} {:?} Invalid parameter received in /pingmsg api call {:?} ",module_path!(),line!(),
                    pair);
            }
        }
    }

    // Check to make sure we got all the parameters
    let content = format!("bad parameters {:}",field_count);
    if field_count != 11 {
        return Ok(
            HttpResponse::BadRequest()
                .content_type("text/html; charset=utf-8")
                .body(content)
        )
    }

    // now printout all that we received
	log::info!("{:?} {:?} PingMsg: Org: {} BldgRoom: {}, Key: {}, Tk: {}, TN: {}, DN: {}, Room: {}, Round: {}, Question: {}, Timestamp: {}, Client IP: {}",
        module_path!(),line!(), org, &roominfo_entry.bldgroom, &roominfo_entry.clientkey, tk, &roominfo_entry.tournament, &roominfo_entry.division, 
        &roominfo_entry.room, &roominfo_entry.round, &roominfo_entry.question, ts, &roominfo_entry.clientip);   
    

    // Find out the tournament id using the the tk (tournament key) or the name of the tournament.
       
    // send an update to the cache for this room.  Rounds in  Progress (tickertape)
    // roominfo::update_roominfo(&mut roominfo_entry);  // ***will reintroduce Redis later

    Ok(HttpResponse::Ok().finish())
}

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
                use crate::schema::rounds::dsl as r;
                d::divisions
                    .filter(d::tid.eq(tid))
                    .filter(d::dname.eq(&dn))
                    .select(d::did)
                    .first::<Uuid>(&mut conn)
                    .ok()
                    .and_then(|did| {
                        r::rounds
                            .filter(r::did.eq(did))
                            .filter(r::name.eq(&rd))
                            .select(r::roundid)
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
            let _ = crate::models::room::update(&mut conn, room.roomid, &room_changes);

            // If a resend has been requested for the game, emit the command and record it.
            if let Some(game) = game {
                if game.resend_gameevents_request_ts.is_some() {
                    let resend_line = format!(
                        "quizzes?cmd=Resend&tournament={}&division={}&room={}&round={}",
                        percent_encode(&tn),
                        percent_encode(&dn),
                        percent_encode(&rm),
                        percent_encode(&rd),
                    );
                    body.push_str(&resend_line);
                    body.push('\n');

                    let mut changes = GameChangeset::empty();
                    changes.resend_gameevents_response = Some(resend_line);
                    let _ = game::update(&mut conn, game.gid, &changes);
                }
            }
        }
    }

    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(body)
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
