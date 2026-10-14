use actix_web::{Error, get, HttpResponse, HttpRequest, post, Result, web::{Data, Json, Query}};
use crate::models::{self, common::{GameEventParams, PaginationParams}, gameevent::{self, GameEvent, NewGameEvent}};
use crate::services::common::{EntityResponse, PagedResponse, process_response};
use diesel::QueryResult;
use chrono::{ DateTime, TimeZone, Utc };
use uuid::Uuid;
// use std::file;
use base64::{self, Engine};
use sha3::{Sha3_512, Digest};
use diesel::result::Error as DBError;
use crate::models::gameeventlog;
use crate::models::{gameevent_quarantine, room_queue_clear};
// use crate::models::gameevent::{self,GameEvent};
use crate::models::game::{self,GameChangeset};
use crate::cache::Cache;
use crate::database::{self,Database};
// use utoipa::OpenApi;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct GameEventDoc;

pub async fn write(
    mdb: &mut database::Connection,
    cache: &Cache,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    // let db = req.app_data::<Data<Database>>().unwrap();
    // let mut mdb = db.pool.get().unwrap();

    log::info!("Inside 'write' fn for endpoint '{}'", req.full_url());

    // First let's get an eventlog structure, a game structure, and
    // an empty quiz events structure
    let mut gameeventlog_entry: gameeventlog::GameEventlogChangeset = gameeventlog::empty_changeset();
    let mut game_entry: GameChangeset = GameChangeset::empty();
    let mut gameevent_entry = NewGameEvent::empty();
    gameevent_entry.source = Some("qview network".to_string());

    // Okay, it's now time to search all the parameters and set the associated 
    // variables set in all the data that we will write to the cache
    // or to the database
    let qs = qstring::QString::from(req.query_string());
    let ps = qs.to_pairs();
    let psiter = ps.iter();
    let mut tk=String::new();
    let mut org = "Nazarene".to_string();
    let mut qn_str = String::new();
    let mut e_str = String::new();
    let mut t_str = String::new();
    let mut q_str = String::new();
    let mut ts = Utc::now();
    let mut field_count = 0;
    for pair in psiter {

        let s = String::from(pair.0);
        match s.as_str() {
            "bldgroom" => {
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.bldgroom = (&tmp).to_string();
                field_count += 1;    
            },
            "key" => {  // key4server - uniquely identifies a particular client
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.clientkey = (&tmp).to_string();
                gameevent_entry.qm_registration_key = Some(tmp.clone());
                game_entry.clientkey = Some(tmp);
                field_count += 1;
            },
            "gid" => { // UUID of Game
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.gid = tmp.clone();
                match Uuid::parse_str(&tmp) {
                    Ok(uuid) => {
                        gameevent_entry.gid = uuid;
                        field_count += 1; 
                    },
                    Err(e) => log::error!("{:?} {:?} Failed to parse Game ID ('gid' query param) as UUID '{}': {:?}", module_path!(), line!(), tmp, e),
                }
            },
            "tk" => {   // tournament key - short id for a particular tournament
                tk = pair.1.replace("+"," ");   // currently used except to ensure we don't have corruption
                field_count += 1;                
            },
            "org" => {
                org = pair.1.replace("+"," ");   // don't bump the field count because it's not sent by client
            },
            "tn" => { // Tournament Name
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.tournament = tmp.clone();
                field_count += 1;
            },
            "dn" => { // Division Name
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.division = tmp.clone();
                field_count += 1;
            },
            "rm" => { // Room Name
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.room = tmp.clone();

                field_count += 1;
            },
            "rd" => { // Round Name (Note: QuizMachine sequential lookup requires that this is an integer with no characters (1, 2, 3, 4, etc.); there should be no letters being received for this query parameter)
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.round = tmp.clone();
                field_count += 1;
            },
            "qn" => { // Question #
                qn_str = pair.1.replace("+"," ");
                // A malformed number leaves the field uncounted, so the request fails the field
                // count with a 400 rather than panicking the handler (which leaves the client
                // with no response at all, to retry forever).
                match pair.1.trim().parse::<i32>() {
                    Ok(qn) => {
                        gameeventlog_entry.question = qn;
                        gameevent_entry.question = qn;
                        field_count += 1;
                    }
                    Err(_) => log::error!("{} {} Malformed 'qn' (question number): {:?}", module_path!(), line!(), pair.1),
                }
            },
            "e" => { // event number
                e_str = pair.1.replace("+"," ");
                match pair.1.trim().parse::<i32>() {
                    Ok(e) => {
                        gameeventlog_entry.eventnum = e;
                        gameevent_entry.eventnum = e;
                        field_count += 1;
                    }
                    Err(_) => log::error!("{} {} Malformed 'e' (event number): {:?}", module_path!(), line!(), pair.1),
                }
            },
            "n" => { // quizzer or team name
                let tmp = pair.1.replace("+"," ");
                gameevent_entry.name = (&tmp).to_string();
                gameeventlog_entry.name = tmp;
                field_count +=1;
            },
            "t" => { // team # (0-2)
                t_str = pair.1.replace("+"," ");
                match pair.1.trim().parse::<i32>() {
                    Ok(t) => {
                        gameevent_entry.team = t;
                        gameeventlog_entry.team = t;
                        field_count += 1;
                    }
                    Err(_) => log::error!("{} {} Malformed 't' (team number): {:?}", module_path!(), line!(), pair.1),
                }
            },
            "q" => { // quizzer # (0-4)
                q_str = pair.1.replace("+"," ");
                match pair.1.trim().parse::<i32>() {
                    Ok(q) => {
                        gameevent_entry.quizzer = q;
                        gameeventlog_entry.quizzer = q;
                        field_count += 1;
                    }
                    Err(_) => log::error!("{} {} Malformed 'q' (quizzer number): {:?}", module_path!(), line!(), pair.1),
                }
            }, 
            "ec" => { // Event type/class (TC, BE, QT, ...
                gameevent_entry.event = pair.1.to_string();
                gameeventlog_entry.event = pair.1.to_string();
                field_count += 1;
            }, 
            "p1" => { // parameter 1
                let tmp = pair.1.replace("+"," ");
                gameevent_entry.parm1 = (&tmp).to_string();
                gameeventlog_entry.parm1 = tmp;
                // log::debug!("{}:{} - Parsed query param 'p1' = {}", module_path!(),line!(), gameeventlog_entry.parm1);
                field_count += 1;
            }, 
            "p2" => { // parameter 2 - depends upon what ec is
                let tmp = pair.1.replace("+"," ");
                gameevent_entry.parm2 = (&tmp).to_string();
                gameeventlog_entry.parm2 = tmp;
                field_count += 1;
            }, 
            "ts" => { // timestamp from the client
                // Out-of-range seconds are as malformed as non-numeric ones; neither may panic.
                match pair.1.trim().parse::<i64>().ok().and_then(|secs| Utc.timestamp_opt(secs, 0).single()) {
                    Some(parsed) => {
                        ts = parsed;
                        gameevent_entry.clientts = ts;
                        gameeventlog_entry.ts = pair.1.to_string();
                        field_count += 1;
                    }
                    None => log::error!("{} {} Malformed 'ts' (client timestamp): {:?}", module_path!(), line!(), pair.1),
                }
            }, 
            "md5" => {  // md5 hashsum
                let tmp = pair.1.replace("+"," ");
                gameevent_entry.md5digest = (&tmp).to_string();
                gameeventlog_entry.md5digest = tmp;
                field_count += 1;
            },
            "nonce" => {
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.nonce = tmp;
                field_count += 1;
            },
            "s3s" => {
                let tmp = pair.1.replace("+","+");
                gameeventlog_entry.s3s = tmp;
                // log::debug!("{}:{} - Parsed query param 's3s' = {}", module_path!(),line!(), gameeventlog_entry.s3s);
                field_count += 1;
            },
            "myip" => {
                // this is optional should only be there sometimes.
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.clientip = (&tmp).to_string();
            }
            _ => {
                log::error!("{:?} {:?} Invalid parameter received in /gameevent api call {:?} ",module_path!(),line!(),
                    pair);
            }
        }
    }

    let gameevent_psk = gameevent_psk();

    // A room whose queue an operator is clearing: acknowledge the event without applying it and
    // set it aside, so the client's queue drains. This comes before every check on purpose — the
    // event holding the queue up is, by definition, one the checks below would refuse.
    if let Some(clear) = clearing_room_for(mdb, &gameeventlog_entry) {
        let reason = "Acknowledged and set aside while this room's pending queue was being cleared; \
                      it is recovered when the room resends the game.";
        match quarantine_event(mdb, gameevent_quarantine::KIND_CLEARED, reason, Some(clear.roomid),
                               Some(clear.room_clear_id), &gameeventlog_entry, req.query_string()) {
            Ok(_) => return Ok(acknowledged(&gameeventlog_entry.nonce, &gameevent_psk)),
            // Could not record it, so do not claim to have it: the client keeps it and retries.
            Err(e) => log::error!("{} {} Could not set aside a cleared event: {:?}", module_path!(), line!(), e),
        }
    }

    // Check to make sure we got all the parameters. A request that fails this or the checksum
    // below cannot be verified as coming from a client, so it is refused rather than quarantined:
    // acknowledging it would let anyone write rows and obtain signed responses.
    let content = "bad parameters";
    if field_count != 20 {
        log::error!("{} {} write() to return 400 BadRequest. Number of field_counted = {}", module_path!(), line!(), field_count);
        return Ok(
            HttpResponse::BadRequest()
                .content_type("text/html; charset=utf-8")
                .body(content)
        )
    }

    // create the sha3-512 object
    let mut sha3_512_hasher = Sha3_512::new();

    // the following code calculates and checks the sha3-512 sum of all the GET parameters.
    // we had issues with the network (firewalls, app firewalls, etc) corrupting or 
    // giving false 200s.  This avoids that.

    sha3_512_hasher.update(&&gameeventlog_entry.nonce);
    sha3_512_hasher.update(&gameevent_psk);
    sha3_512_hasher.update(&gameeventlog_entry.bldgroom);
	sha3_512_hasher.update(&gameeventlog_entry.clientkey);  // key4Server
	sha3_512_hasher.update(&tk);
	sha3_512_hasher.update(&gameeventlog_entry.tournament);
	sha3_512_hasher.update(&gameeventlog_entry.division);
    sha3_512_hasher.update(&gameeventlog_entry.room);
    sha3_512_hasher.update(&gameeventlog_entry.round);
	sha3_512_hasher.update(&qn_str);  // question number
    sha3_512_hasher.update(&e_str);
    sha3_512_hasher.update(&gameeventlog_entry.name);
    sha3_512_hasher.update(&t_str);
    sha3_512_hasher.update(&q_str);
    sha3_512_hasher.update(&gameeventlog_entry.event);
    sha3_512_hasher.update(&gameeventlog_entry.parm1);
    sha3_512_hasher.update(&gameeventlog_entry.parm2);
    let rslt = sha3_512_hasher.finalize();
    let rsltbase64 = base64::engine::general_purpose::STANDARD.encode(rslt);

    // now grab the result of the sha3-512 hashing
	// log::info!("{:?} {:?} GameEvent: Org: {} BldgRoom: {}, Key: {}, Tk: {}, TN: {}, DN: {}, Room: {}, Round: {}, Question: {}, EventNumber: {} Name: {} Team: {} Quizzer: {}, EC: {}, Parm1: {} Parm2: {}, Timestamp: {}, Host: {}, MD5: {}, Nonce: {} {}, Sha3-512 sum: {} Calculated sha3-512 sum: {}",
    //     module_path!(),line!(), org, &gameeventlog_entry.bldgroom, &gameeventlog_entry.clientkey, tk, &gameeventlog_entry.tournament, &gameeventlog_entry.division, 
    //     &gameeventlog_entry.room, &gameeventlog_entry.round, &gameeventlog_entry.question, &gameeventlog_entry.eventnum, &gameeventlog_entry.name,
    //     &gameeventlog_entry.team, &gameeventlog_entry.quizzer, &gameeventlog_entry.event, &gameeventlog_entry.parm1, &gameeventlog_entry.parm2, 
    //     ts, &gameeventlog_entry.clientip, &gameeventlog_entry.md5digest, &gameeventlog_entry.nonce, &gameeventlog_entry.nonce.len(), &gameeventlog_entry.s3s, rsltbase64 );   
    
    // now make sure we didn't have any corrupted data.  If so print an error and get out
    if !&gameeventlog_entry.s3s.eq(&rsltbase64) {
        // oh boy!!!
        log::error!("{} {} /api/gameevents/create Sha3-512 sums don't match {} {}",module_path!(), line!(), &gameeventlog_entry.s3s, rsltbase64);
        let error_content = format!("Sha3-512 sums don't match! {} {}",&gameeventlog_entry.s3s, &rsltbase64);
        return Ok(
            HttpResponse::BadRequest()
                .content_type("text/html; charset=utf-8")
                .body(error_content)
        )
    }

    // now lets log all this information to the eventlog table.
    // This is a file on disk in QMServer.  But we'll put it
    // on the database in the eventlog table for Qview
    match gameeventlog::write_gameeventlog(mdb, gameeventlog_entry.clone()) {
        Ok(_eventlog) => {
            // okay we wrote to eventlog - do nothing
        },
        Err(e) => {
            log::error!("{} {} Eventlog write failure: {}",module_path!(),line!(),e);
            if !is_transient(&e) {
                // e.g. a value too long for its column: it would fail on every retry.
                let reason = format!("Could not be logged: {}", e);
                if let Some(ack) = reject_to_quarantine(mdb, &reason, &gameeventlog_entry, req.query_string(), &gameevent_psk) {
                    return Ok(ack);
                }
            }
            let error_content = format!("Eventlog write failure {}", e);
            return Ok(
                HttpResponse::BadRequest()
                    .content_type("text/html; charset=utf-8")
                    .body(error_content)
            )
        }
    }

    // An event for a game the server does not have fails the gameevents foreign key on every
    // retry — a deleted or purged game, or a database re-seeded under a running client.
    // Only a definite "no": if the check itself fails, carry on and let the insert decide.
    if let Ok(false) = game::row_exists(mdb, gameevent_entry.gid) {
        let reason = format!(
            "Game {} does not exist on the server (deleted, purged, or the database was re-seeded).",
            gameevent_entry.gid
        );
        if let Some(ack) = reject_to_quarantine(mdb, &reason, &gameeventlog_entry, req.query_string(), &gameevent_psk) {
            return Ok(ack);
        }
    }

    // now let's write an entry in the quizzes event table
    // Handle errors while we create the entry - this is a database insert or update
    match gameevent::create_update_game_event(mdb, &gameevent_entry) {
        Ok(output) => {
            log::info!("Inserted/Updated a GameEvent: {:?}",output)
        },
        // Written by a concurrent copy of this same event (the upsert checks before it writes, so
        // two arriving together can both try to insert). The event is stored: that is success.
        Err(DBError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
            log::info!("{} {} GameEvent already written by a concurrent request: {:?}", module_path!(), line!(), gameevent_entry);
        },
        Err(err) if !is_transient(&err) => {
            log::error!("{} {} GameEvent write failure (quarantining): {:?} {:?}", module_path!(), line!(), err, gameevent_entry);
            let reason = format!("Could not be stored: {}", err);
            if let Some(ack) = reject_to_quarantine(mdb, &reason, &gameeventlog_entry, req.query_string(), &gameevent_psk) {
                return Ok(ack);
            }
            return Ok(
                HttpResponse::BadRequest()
                    .content_type("text/html; charset=utf-8")
                    .body(format!("GameEvent write failure {}", err))
            )
        },
        Err(err) => {
            let error_content = format!("GameEvent write failure {}", err);
            match err {
                // the most likely cause here is a Unique constraint - the row
                // already exists in the database.  We'll ignore those and
                // panic or log the others
                DBError::DatabaseError(dbek,info) => match dbek {
                    diesel::result::DatabaseErrorKind::UniqueViolation => {
                        // Okay we've written this one before.  this is some weird error
                         // since the upsert() in the gameevent model should have 
                        // handled it.
                    },
                    _ => {
                        // Okay this error is a database error but not a unique violation
                        log::error!("Line: {:?} DB Create error {:?} {:?} {:?}",line!(),dbek,info,gameevent_entry);
                    },
                },
                _ => {
                    // this is some error but not a database error
                    log::error!("Line: {:?} DB Create error {:?} {:?}",line!(),err,gameevent_entry);
                },
            };

            log::error!("{} {} write() to return 400 BadRequest", module_path!(), line!());
            return Ok(
                HttpResponse::BadRequest()
                    .content_type("text/html; charset=utf-8")
                    .body(error_content)
            )
        },
    
    }

    // The event is committed and Postgres is authoritative, so drop everything this event
    // makes stale: the game's calculated state, and the aggregated standings of every stats
    // group the game feeds.
    //
    // Order matters: write first, then invalidate. A lost invalidation (Valkey down, timeout)
    // then degrades to "stale until the TTL expires" rather than to wrong data being written.
    // `invalidate` cannot fail in a way the caller must handle -- see backend/src/cache.rs.
    //
    // Only reached on success: every error arm of the match above returns early.
    let mut stale_keys = vec![gameevent::CalculatedGame::cache_key(gameevent_entry.gid)];
    match models::game_statsgroup::read_statsgroup_ids_of_game(mdb, gameevent_entry.gid) {
        Ok(sg_ids) => {
            for sg_id in sg_ids {
                stale_keys.extend(models::statsgroup::aggregate_cache_keys(sg_id));
            }
        }
        Err(e) => {
            // We could not learn which stats groups to clear. Their rows carry a short TTL,
            // so this self-heals; log it rather than failing a write that already succeeded.
            log::warn!(
                "{} {} Could not read stats groups for game {} to invalidate standings ({:?}); \
                 relying on AGGREGATE_TTL.",
                module_path!(), line!(), gameevent_entry.gid, e
            );
        }
    }
    cache.invalidate(&stale_keys).await;

    Ok(acknowledged(&gameeventlog_entry.nonce, &gameevent_psk))
}

/// The pre-shared key that signs requests and responses.
fn gameevent_psk() -> String {
    std::env::var("GAMEEVENT_PSK").unwrap_or_else(|_| {
        log::error!("{:?} {:?} Invalid GAMEEVENT_PSK", module_path!(), line!());
        "this won't work but fail".to_string()
    })
}

/// The response that tells the client its event arrived, so it drops the event from its queue.
///
/// Every path that acknowledges an event uses this — applied, quarantined, or set aside while
/// clearing — so the client cannot tell them apart and treats each as a success.
fn acknowledged(nonce: &str, psk: &str) -> HttpResponse {
    let mut hasher = Sha3_512::new();
    hasher.update(nonce);
    hasher.update(psk);
    // base64-encode the raw digest bytes into a String suitable for a header value
    let sum = base64::engine::general_purpose::STANDARD.encode(hasher.finalize());
    log::debug!("{}:{} Generated sha3-512 sum for response header = {}", module_path!(), line!(), &sum);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .insert_header(("sha3512sum", sum))
        .body("Inserted/Updated")
}

/// Failures the next retry may not hit: the connection or transaction, not the event. These are
/// still refused so the client retries; everything else would fail on every retry and is
/// quarantined instead.
fn is_transient(err: &DBError) -> bool {
    use diesel::result::DatabaseErrorKind as K;
    matches!(
        err,
        DBError::DatabaseError(K::SerializationFailure | K::ReadOnlyTransaction | K::ClosedConnection | K::UnableToSendCommand, _)
            | DBError::BrokenTransactionManager
            | DBError::AlreadyInTransaction
            | DBError::NotInTransaction
    )
}

/// The clear acknowledging events for this event's room right now, if any.
fn clearing_room_for(
    db: &mut database::Connection,
    log: &gameeventlog::GameEventlogChangeset,
) -> Option<room_queue_clear::RoomQueueClear> {
    // Almost always false, which spares every other event resolving its room.
    if !room_queue_clear::any_clearing_now(db).unwrap_or(false) {
        return None;
    }
    let room_id = models::room::find_id_by_names(db, &log.tournament, &log.room)?;
    room_queue_clear::clearing_now_for_room(db, room_id).ok().flatten()
}

fn quarantine_event(
    db: &mut database::Connection,
    kind: &str,
    reason: &str,
    room_id: Option<Uuid>,
    clear_id: Option<i64>,
    log: &gameeventlog::GameEventlogChangeset,
    query_string: &str,
) -> QueryResult<gameevent_quarantine::QuarantinedGameEvent> {
    gameevent_quarantine::create(db, &gameevent_quarantine::NewQuarantinedGameEvent {
        kind: kind.to_string(),
        reason: reason.to_string(),
        roomid: room_id,
        room_clear_id: clear_id,
        gid_raw: log.gid.clone(),
        gid: Uuid::parse_str(log.gid.trim()).ok(),
        tournament: log.tournament.clone(),
        division: log.division.clone(),
        room: log.room.clone(),
        round: log.round.clone(),
        question: log.question,
        eventnum: log.eventnum,
        event: log.event.clone(),
        name: log.name.clone(),
        team: log.team,
        quizzer: log.quizzer,
        parm1: log.parm1.clone(),
        parm2: log.parm2.clone(),
        clientkey: log.clientkey.clone(),
        query_string: query_string.to_string(),
    })
}

/// Quarantines a verified event the server would refuse on every retry, and returns the
/// acknowledgement to send. None if it could not be recorded — then the caller refuses it as
/// before, so the client keeps the event rather than the server claiming one it does not have.
fn reject_to_quarantine(
    db: &mut database::Connection,
    reason: &str,
    log: &gameeventlog::GameEventlogChangeset,
    query_string: &str,
    psk: &str,
) -> Option<HttpResponse> {
    let room_id = models::room::find_id_by_names(db, &log.tournament, &log.room);
    match quarantine_event(db, gameevent_quarantine::KIND_REJECTED, reason, room_id, None, log, query_string) {
        Ok(_) => {
            log::warn!("{} {} Quarantined game event ({}): {:?}", module_path!(), line!(), reason, log);
            Some(acknowledged(&log.nonce, psk))
        }
        Err(e) => {
            log::error!("{} {} Could not quarantine game event: {:?}", module_path!(), line!(), e);
            None
        }
    }
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct GameEventDoc;

// #[utoipa::path(
//         get,
//         path = "/gameevents",
//         responses(
//             (status = 200, description = "GameEvents found successfully", body = GameEvent),
//             (status = 404, description = "GameEvent not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many GameEvents to return")
//         )
//     )
// ]
#[get("/create")]
async fn index(
    db: Data<Database>,
    cache: Data<Cache>,
    //Query(url_params): Query<GameEventParams>,
    req: HttpRequest
) -> HttpResponse {
    // This endpoint is intentionally NOT ReST compliant; while being a GET endpoint it is used to persist GameEvents
    // to the DB and then lets the client know the request has been successfully received
    // Resent gameevents are received here also.

    let mut db = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match write(&mut db, &cache, req.clone()).await {
        Ok(response) => {
            // write() succeeded at the Result level; it may still carry a non-2xx
            // status (e.g. a validation BadRequest). Log those but pass them through
            // so the client sees the real status and body.
            if !response.status().is_success() {
                log::error!("{} {} write() returned a non-OK response: {}", module_path!(), line!(), response.status());
            } else {
                // Every acknowledged event changed something a live page shows: an applied one the
                // scores, a quarantined or set-aside one the Server Monitor's quarantine count.
                // Notifying here, once, covers each of write()'s acknowledging paths.
                let query = qstring::QString::from(req.query_string());
                let tournament_name = query.get("tn").unwrap_or("").replace("+", " ");
                crate::live::tournament_named_changed(&req, &mut db, &tournament_name);
                // And the standings of the game's division, which an event can finish.
                crate::live::game_division_changed(&req, &mut db, query.get("gid").unwrap_or(""));
            }
            return response;
        }
        Err(e) => {
            // write() itself errored out; report a 500 to the client.
            log::error!("{} {} write() failed: {:?}", module_path!(), line!(), e);
            let internal_error_response_body = EntityResponse::<bool> {
                code: 500,
                message: "Internal Server Error".to_string(),
                data: None,
            };
            return HttpResponse::InternalServerError().json(internal_error_response_body);
        }
    }
}

// pub async fn index_playground(
//     req: HttpRequest,
//     Query(url_params): Query<PaginationParams>,
//     db: Data<Database>,
// ) -> actix_web::Result<HttpResponse> {
//     let mut db = db.get_connection().expect("Failed to get connection");

//     let content = std::fs::read_to_string("./.cargo/graphql-playground.html").unwrap();

//     let result = game::read_all(&mut db, &url_params);
//     println!("{:?}",result);

//     Ok(
//         HttpResponse::Ok()
//             .content_type("text/html; charset=utf-8")
//             // GraphQL Playground original source:
//             // .body(playground_source(
//             //     GraphQLPlaygroundConfig::new("/api/graphql")
//             //         .with_header("Authorization", "token")
//             //         .subscription_endpoint("/api/graphql/ws"),
//             // ))

//             // GraphQL Playground modified source to include authentication:
//             .body(content)
//     )
// }

// #[utoipa::path(
//         get,
//         path = "/gameevents",
//         responses(
//             (status = 200, description = "GameEvents found successfully", body = GameEvent),
//             (status = 404, description = "GameEvents not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many GameEvents to return")
//         )
//     )
// ]
// #[get("")]
// async fn index(
//     db: Data<Database>,
//     Query(info): Query<PaginationParams>,
// //    info: web::Path<Info>,
// //    path: web::Path<(String,String,String)>,
//     req: HttpRequest,
// ) -> HttpResponse {
//     let mut db = db.get_connection().expect("Failed to get connection");
    
//     print_type_of(&db); 

//     println!("Method: {:?}",req.method()); 
//     println!("URI: {:?}",req.uri()); 
//     println!("Version: {:?}",req.version());     
//     println!("URI: {:?}",req.uri()); 
//     println!("Path: {:?}",req.path()); 
//     println!("URI: {:?}",req.uri()); 
//     println!("Query_string: {:?}",req.query_string()); 

//     let result = game::read_all(&mut db, &info);

//     if result.is_ok() {
//         HttpResponse::Ok().json(result.unwrap())
//     } else {
//         HttpResponse::InternalServerError().finish()
//     }
// }

// #[get("/{id}")]
// async fn read(
//     db: Data<Database>,
//     item_id: Path<Uuid>,
// ) -> HttpResponse {
//     println!("read endpoint");
//     let mut db = db.get_connection().expect("Failed to get connection");

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
//     Json(item): Json<NewGameEvent>,
//     req: HttpRequest
// ) -> Result<HttpResponse, Error> {

//     let mut conn = db.get_connection().expect("Failed to get connection");
    
//     tracing::debug!("{} GameEvent model create {:?}", line!(), item);

//     // log this api call
//     models::apicalllog::create(&mut conn, &req);
    
//     let result: QueryResult<GameEvent> = models::gameevent::create(&mut conn, &item);

//     let response: EntityResponse<GameEvent> = process_response(result, "post");
    
//     match response.code {
//         400 => Ok(HttpResponse::BadRequest().json(response)),
//         409 => Ok(HttpResponse::Conflict().json(response)),
//         201 => Ok(HttpResponse::Created().json(response)),
//         200 => Ok(HttpResponse::Ok().json(response)),
//         _ => Ok(HttpResponse::InternalServerError().json(response))
//     }
// }


// #[post("")]
// async fn create(
//     db: Data<Database>,
//     Json(item): Json<NewGame>,
// ) -> Result<HttpResponse, Error> {
//     println!("create endpoint");
//     let mut db = db.get_connection().expect("Failed to get connection");

//     let result: Game = game::create(&mut db, &item).expect("Creation error");

//     Ok(HttpResponse::Created().json(result))
// }

// #[put("/{id}")]
// async fn update(
//     db: Data<Database>,
//     item_id: Path<Uuid>,
//     Json(item): Json<GameChangeset>,
// ) -> HttpResponse {
//     println!("update endpoint");
//     let mut db = db.get_connection().expect("Failed to get connection");

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
// ) -> HttpResponse {
//     println!("destroy endpoint");
//     let mut db = db.get_connection().expect("Failed to get connection");

//     let result = game::delete(&mut db, item_id.into_inner());

//     if result.is_ok() {
//         HttpResponse::Ok().finish()
//     } else {
//         HttpResponse::InternalServerError().finish()
//     }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
//         .service(read)
//         .service(create)
//         .service(update)
//         .service(destroy);
}
