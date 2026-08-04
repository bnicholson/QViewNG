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
use crate::models::{gameeventlog, roominfo};
// use crate::models::gameevent::{self,GameEvent};
use crate::models::game::{self,GameChangeset};
use crate::database::{self,Database};
// use utoipa::OpenApi;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct GameEventDoc;

pub async fn write(
    mdb: &mut database::Connection,
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
    let mut roominfo_entry = roominfo::empty();

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
                roominfo_entry.bldgroom = tmp;
                field_count += 1;    
            },
            "key" => {  // key4server - uniquely identifies a particular client
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.clientkey = (&tmp).to_string();
                roominfo_entry.clientkey = (&tmp).to_string();
                game_entry.clientkey = Some(tmp);
                field_count += 1;
            },
            "gid" => { // UUID of Game
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.gid = tmp.clone();
                roominfo_entry.gid = tmp.clone();
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
                roominfo_entry.tournament = tmp;
                field_count += 1;
            },
            "dn" => { // Division Name
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.division = tmp.clone();
                roominfo_entry.division = tmp;
                field_count += 1;
            },
            "rm" => { // Room Name
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.room = tmp.clone();
                roominfo_entry.room = tmp.clone();

                field_count += 1;
            },
            "rd" => { // Round Name (Note: QuizMachine sequential lookup requires that this is an integer with no characters (1, 2, 3, 4, etc.); there should be no letters being received for this query parameter)
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.round = tmp.clone();
                roominfo_entry.round = tmp;
                field_count += 1;
            },
            "qn" => { // Question #
                qn_str = pair.1.replace("+"," ");
                let qn = pair.1.trim().parse().unwrap(); 
                gameeventlog_entry.question = qn;
                gameevent_entry.question = qn;
                roominfo_entry.question = qn;
                field_count += 1;
            },
            "e" => { // event number
                e_str = pair.1.replace("+"," ");
                let e = pair.1.trim().parse().unwrap();
                gameeventlog_entry.eventnum = e;
                gameevent_entry.eventnum = e;
                field_count +=1;
            },
            "n" => { // quizzer or team name
                let tmp = pair.1.replace("+"," ");
                gameevent_entry.name = (&tmp).to_string();
                gameeventlog_entry.name = tmp;
                field_count +=1;
            },
            "t" => { // team # (0-2)
                t_str = pair.1.replace("+"," ");
                let t = pair.1.trim().parse().unwrap();
                gameevent_entry.team = t;
                gameeventlog_entry.team = t;
                field_count +=1;
            },
            "q" => { // quizzer # (0-4)
                q_str = pair.1.replace("+"," "); 
                let q = pair.1.trim().parse().unwrap();
                gameevent_entry.quizzer = q;
                gameeventlog_entry.quizzer = q;
                field_count +=1;
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
                log::debug!("{}:{} - Parsed query param 'p1' = {}", module_path!(),line!(), gameeventlog_entry.parm1);
                field_count += 1;
            }, 
            "p2" => { // parameter 2 - depends upon what ec is
                let tmp = pair.1.replace("+"," ");
                gameevent_entry.parm2 = (&tmp).to_string();
                gameeventlog_entry.parm2 = tmp;
                field_count += 1;
            }, 
            "ts" => { // timestamp from the client
                println!("TS Input = {:?}",pair.1);
                let secs : i64 = pair.1.trim().parse().unwrap();
                ts = Utc.timestamp_opt(secs,0).unwrap();
                gameevent_entry.clientts = ts;
                gameeventlog_entry.ts = pair.1.to_string();
                roominfo_entry.client_time = ts;
                field_count += 1;
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
                log::debug!("{}:{} - Parsed query param 's3s' = {}", module_path!(),line!(), gameeventlog_entry.s3s);
                field_count += 1;
            },
            "myip" => {
                // this is optional should only be there sometimes.
                let tmp = pair.1.replace("+"," ");
                gameeventlog_entry.clientip = (&tmp).to_string();
                roominfo_entry.clientip = tmp;
            }
            _ => {
                log::error!("{:?} {:?} Invalid parameter received in /gameevent api call {:?} ",module_path!(),line!(),
                    pair);
            }
        }
    }

    // Check to make sure we got all the parameters
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
    // Grab the HOST:PORT the web server should run on.
    let gameevent_psk = match std::env::var("GAMEEVENT_PSK") {
        Ok(gameevent_psk) => {
            gameevent_psk
        },
        Err(e) => {
            log::error!("{:?} {:?} Invalid QUIZEVENT_PSK",module_path!(),line!());
            "this won't work but fail".to_string()
        }
    };

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
	log::info!("{:?} {:?} GameEvent: Org: {} BldgRoom: {}, Key: {}, Tk: {}, TN: {}, DN: {}, Room: {}, Round: {}, Question: {}, EventNumber: {} Name: {} Team: {} Quizzer: {}, EC: {}, Parm1: {} Parm2: {}, Timestamp: {}, Host: {}, MD5: {}, Nonce: {} {}, Sha3-512 sum: {} Calculated sha3-512 sum: {}",
        module_path!(),line!(), org, &gameeventlog_entry.bldgroom, &gameeventlog_entry.clientkey, tk, &gameeventlog_entry.tournament, &gameeventlog_entry.division, 
        &gameeventlog_entry.room, &gameeventlog_entry.round, &gameeventlog_entry.question, &gameeventlog_entry.eventnum, &gameeventlog_entry.name,
        &gameeventlog_entry.team, &gameeventlog_entry.quizzer, &gameeventlog_entry.event, &gameeventlog_entry.parm1, &gameeventlog_entry.parm2, 
        ts, &gameeventlog_entry.clientip, &gameeventlog_entry.md5digest, &gameeventlog_entry.nonce, &gameeventlog_entry.nonce.len(), &gameeventlog_entry.s3s, rsltbase64 );   
    
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
            let error_content = format!("Eventlog write failure {}", e);
            return Ok(
                HttpResponse::BadRequest()
                    .content_type("text/html; charset=utf-8")
                    .body(error_content)
            )
        }
    }

    // send an update to the cache for this room.  Rounds in  Progress (tickertape)
    // roominfo::update_roominfo(&mut roominfo_entry);

    // now let's write an entry in the quizzes event table
    // Handle errors while we create the entry - this is a database insert or update
    match gameevent::create_update_game_event(mdb, &gameevent_entry) {
        Ok(output) => {
            log::info!("Inserted/Updated a GameEvent: {:?}",output)
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

    let mut sha3_512_hasher_for_response_header = Sha3_512::new();
    sha3_512_hasher_for_response_header.update(&gameeventlog_entry.nonce);
    sha3_512_hasher_for_response_header.update(&gameevent_psk);
    let rslt_for_response_header = sha3_512_hasher_for_response_header.finalize();
    // base64-encode the raw digest bytes into a String suitable for a header value
    let sha3_512_sum_for_response_header = base64::engine::general_purpose::STANDARD.encode(rslt_for_response_header);

    log::debug!("{}:{} Generated sha3-512 sum for response header = {}", module_path!(), line!(), &sha3_512_sum_for_response_header);
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .insert_header(("sha3512sum", sha3_512_sum_for_response_header))
            .body("Inserted/Updated")
    )
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

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
    //Query(url_params): Query<GameEventParams>,
    req: HttpRequest
) -> HttpResponse {
    // This endpoint is intentionally NOT ReST compliant; while being a GET endpoint it is used to persist GameEvents
    // to the DB and then lets the client know the request has been successfully received

    let mut db = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match write(&mut db, req).await {
        Ok(response) => {
            // write() succeeded at the Result level; it may still carry a non-2xx
            // status (e.g. a validation BadRequest). Log those but pass them through
            // so the client sees the real status and body.
            if !response.status().is_success() {
                log::error!("{} {} write() returned a non-OK response: {}", module_path!(), line!(), response.status());
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

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewGameEvent>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");
    
    tracing::debug!("{} GameEvent model create {:?}", line!(), item);

    // log this api call
    models::apicalllog::create(&mut conn, &req);
    
    let result: QueryResult<GameEvent> = models::gameevent::create(&mut conn, &item);

    let response: EntityResponse<GameEvent> = process_response(result, "post");
    
    match response.code {
        400 => Ok(HttpResponse::BadRequest().json(response)),
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}


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
// }


pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(create);
}

// pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
//     return scope
//         .service(index)
//         .service(read)
//         .service(create)
//         .service(update)
//         .service(destroy);
// }
