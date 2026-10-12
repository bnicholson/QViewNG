use actix_http::ws::OpCode;
use actix_web::{delete, Error, get, HttpMessage, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;
use crate::{auth::{is_rbac_and_abac_authorized, policies::{PolicyContext, UserContext, room::RoomPolicyResource}}, models::common::ReadGamesDetailedParams};
use crate::database::Database;
use crate::models::{self, common::PaginationParams, permission::{AppAction, AppResource}, room::{NewRoom, Room, RoomChangeset}};
use crate::services::common::{EntityResponse, PagedResponse, process_response};
// use utoipa::OpenApi;
use diesel::QueryResult;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoomGameTeam {
    pub name: String,
    pub quizzers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoomGame {
    pub seqnum: i64,
    pub gameid: Uuid,
    pub roundid: Uuid,
    pub roundname: String,
    pub did: Uuid,
    pub dname: String,
    pub tid: Uuid,
    pub tname: String,
    pub leftteam: RoomGameTeam,
    pub centerteam: RoomGameTeam,
    pub rightteam: RoomGameTeam,
}

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct RoomDoc;

// #[utoipa::path(
//         get,
//         path = "/rooms",
//         responses(
//             (status = 200, description = "Rooms found successfully", body = Room),
//             (status = 404, description = "Room not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many Rooms to return")
//         )
//     )
// ]
#[get("")]
async fn index(
    db: Data<Database>,
    Query(url_params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut db, &req);
    
    match (models::room::read_all(&mut db, &url_params), models::room::count(&mut db)) {
        (Ok(items), Ok(count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::room::read(&mut db, item_id.into_inner()) {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/games")]
async fn read_games(
    db: Data<Database>,
    room_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::game::read_all_games_of_room(&mut db, room_id.into_inner(), &params) {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// Returns fully-formed game data-table rows (game + division/room/team names, start time, and
/// room sequence number) for the room in a single paginated call.
/// Every game scheduled in a room, with each team's roster, ordered by the round's scheduled
/// start time. This is what the QuizMachine client reads to populate a room's schedule.
///
/// Restored from commit a419373, which removed it without mention while consolidating the
/// frontend's User-profile endpoints. The gate is the tournament's pairing code, passed as a query
/// parameter — the client has no account, so there is no token to check.
#[get("/{id}/games-detailed")]
async fn read_games_detailed(
    db: Data<Database>,
    room_id: Path<Uuid>,
    Query(params): Query<ReadGamesDetailedParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let pagination_params = PaginationParams {
        page: params.page,
        page_size: params.page_size
    };

    let games_list = match models::game::read_all_games_of_room(&mut db, room_id.into_inner(), &pagination_params) {
        Ok(g) => g,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    if games_list.is_empty() {
        return HttpResponse::Ok().json(EntityResponse::<Vec<RoomGame>> {
            code: 200,
            message: "OK".to_string(),
            data: Some(Vec::<RoomGame>::new()),
        });
    }

    let internal_server_error_payload = EntityResponse::<Vec<RoomGame>> {
        code: 500,
        message: "Internal Server Error".to_string(),
        data: None
    };

    // Can assume all games for a given room belong to the same tournament. Games no longer carry
    // a tournamentid (dropped in 2026-09-13-120000), so reach it through the game's round.
    let tournament = match models::game::read_tournament_of_game(&mut db, &games_list[0]) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().json(internal_server_error_payload),
    };

    if tournament.pairing_code != params.pairing_code {
        let unauthorized_payload = EntityResponse::<Vec<RoomGame>> {
            code: 401,
            message: "Unauthorized".to_string(),
            data: Some(Vec::<RoomGame>::new())
        };
        return HttpResponse::Unauthorized().json(unauthorized_payload);
    }

    let mut rounds_cache: HashMap<Uuid, models::round::Round> = HashMap::new();
    let mut divisions_cache: HashMap<Uuid, models::division::Division> = HashMap::new();
    let mut teams_cache: HashMap<Uuid, models::team::Team> = HashMap::new();
    let mut quizzers_cache: HashMap<Uuid, String> = HashMap::new();

    let mut items: Vec<RoomGame> = Vec::with_capacity(games_list.len());

    for g in games_list.into_iter() {
        if !rounds_cache.contains_key(&g.roundid) {
            match models::round::read(&mut db, g.roundid) {
                Ok(r) => { rounds_cache.insert(g.roundid, r); },
                Err(_) => return HttpResponse::InternalServerError().json(internal_server_error_payload),
            }
        }
        // Games no longer carry a divisionid (dropped in 2026-09-13-100000); a game's division is
        // reached through its round, so this caches per round rather than per division.
        if !divisions_cache.contains_key(&g.roundid) {
            match models::game::read_division_of_game(&mut db, &g) {
                Ok(d) => { divisions_cache.insert(g.roundid, d); },
                Err(_) => return HttpResponse::InternalServerError().json(internal_server_error_payload),
            }
        }

        let mut team_ids: Vec<Uuid> = vec![g.leftteamid, g.rightteamid];
        if let Some(ct) = g.centerteamid {
            team_ids.push(ct);
        }
        for team_id in team_ids {
            if !teams_cache.contains_key(&team_id) {
                let team = match models::team::read(&mut db, team_id) {
                    Ok(t) => t,
                    Err(_) => return HttpResponse::InternalServerError().json(internal_server_error_payload),
                };
                for qid in [
                    team.quizzer_one_id,
                    team.quizzer_two_id,
                    team.quizzer_three_id,
                    team.quizzer_four_id,
                    team.quizzer_five_id,
                    team.quizzer_six_id,
                ].into_iter().flatten() {
                    if !quizzers_cache.contains_key(&qid) {
                        match models::user::read(&mut db, qid) {
                            Ok(u) => { quizzers_cache.insert(qid, format!("{} {}", u.fname, u.lname)); },
                            Err(_) => return HttpResponse::InternalServerError().json(internal_server_error_payload),
                        }
                    }
                }
                teams_cache.insert(team_id, team);
            }
        }

        let build_team_payload = |team_id_opt: Option<Uuid>| -> RoomGameTeam {
            match team_id_opt.and_then(|team_id| teams_cache.get(&team_id)) {
                Some(team) => {
                    let quizzers: Vec<String> = [
                        team.quizzer_one_id,
                        team.quizzer_two_id,
                        team.quizzer_three_id,
                        team.quizzer_four_id,
                        team.quizzer_five_id,
                        team.quizzer_six_id,
                    ]
                    .into_iter()
                    .flatten()
                    .filter_map(|qid| quizzers_cache.get(&qid).cloned())
                    .collect();
                    RoomGameTeam { name: team.name.clone(), quizzers }
                }
                None => RoomGameTeam { name: String::new(), quizzers: Vec::new() },
            }
        };

        let roundname = rounds_cache.get(&g.roundid).map(|r| r.name.clone()).unwrap_or_default();
        let division = divisions_cache.get(&g.roundid);
        let did = division.map(|d| d.did).unwrap_or_default();
        let dname = division.map(|d| d.dname.clone()).unwrap_or_default();

        items.push(RoomGame {
            seqnum: 0,
            gameid: g.gid,
            roundid: g.roundid,
            roundname,
            did,
            dname,
            tid: tournament.tid,
            tname: tournament.tname.clone(),
            leftteam: build_team_payload(Some(g.leftteamid)),
            centerteam: build_team_payload(g.centerteamid),
            rightteam: build_team_payload(Some(g.rightteamid)),
        });
    }

    // Order by the round's scheduled start time (earliest first). Games whose
    // round has no scheduled start time sort last. Then assign seqnum starting at 1.
    items.sort_by_key(|item| {
        rounds_cache
            .get(&item.roundid)
            .and_then(|r| r.scheduled_start_time)
            .map(|dt| (0i8, dt.timestamp_nanos_opt().unwrap_or(i64::MAX)))
            .unwrap_or((1i8, i64::MAX))
    });
    for (idx, item) in items.iter_mut().enumerate() {
        item.seqnum = (idx as i64) + 1;
    }

    let payload = EntityResponse::<Vec<RoomGame>> {
        code: 200,
        message: "OK".to_string(),
        data: Some(items)
    };

    HttpResponse::Ok().json(payload)
}

#[get("/{id}/game-rows")]
async fn read_game_rows(
    db: Data<Database>,
    room_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::game::read_game_rows_of_room(&mut db, room_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(mut item): Json<NewRoom>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, item.tid) {
        Ok(t) => t,
        Err(_) => {
            println!("Could not find Tournament by ID={}", &item.tid);
            return Ok(HttpResponse::UnprocessableEntity().json(json!({
                "error": format!("Tournament with ID {} does not exist", item.tid)
            })));
        }
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: RoomPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let room_create_permission = format!("{}:{}", AppResource::Room.as_str(), AppAction::Create.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &room_create_permission, AppResource::Room.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} Room model create {:?}", line!(), item);

    item.last_modified_user = user_ctx.user_id;
    let result: QueryResult<Room> = models::room::create(&mut conn, &item);

    let response: EntityResponse<Room> = process_response(result, "post");

    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

#[put("/{id}")]
async fn update(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Json(item): Json<RoomChangeset>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    let room_id = item_id.into_inner();

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let room = match models::room::read(&mut conn, room_id) {
        Ok(r) => r,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, room.tid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: RoomPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let room_update_permission = format!("{}:{}", AppResource::Room.as_str(), AppAction::Update.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &room_update_permission, AppResource::Room.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} Room model update {:?} {:?}", line!(), room_id, item);

    // A room can only be placed in a building of its own tournament.
    if let Some(Some(group_id)) = item.roomgroupid {
        match models::roomgroup::read(&mut conn, group_id) {
            Ok(group) if group.tournamentid == room.tid => {}
            _ => return Ok(HttpResponse::BadRequest().body("The building is not part of this room's tournament.")),
        }
    }

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let result = models::room::update(&mut conn, room_id, &item, user_ctx.user_id);

    let response = process_response(result, "put");

    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

#[delete("/{id}")]
async fn destroy(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let room_id = item_id.into_inner();

    let room = match models::room::read(&mut conn, room_id) {
        Ok(r) => r,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, room.tid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: RoomPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let room_delete_permission = format!("{}:{}", AppResource::Room.as_str(), AppAction::Delete.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &room_delete_permission, AppResource::Room.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} Room model delete {:?}", line!(), room_id);

    let result = models::room::delete(&mut conn, room_id);

    if result.is_ok() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}

// ─── Server Monitor: quarantine and Clear Pending + Resend ─────────────────────

/// Server Monitor operations on a room. The caller must be signed in, hold `permission`, and be
/// someone the Server Monitor is shown to: a system administrator, the tournament's owner, or one
/// of its admins. Quarantined events carry raw client requests, so this is not public.
fn authorize_monitor_operation(
    conn: &mut crate::database::Connection,
    req: &HttpRequest,
    room: &Room,
    permission: &str,
) -> core::result::Result<UserContext, HttpResponse> {
    let ctx = match req.extensions().get::<UserContext>().cloned() {
        Some(c) => c,
        None => return Err(HttpResponse::Unauthorized().finish()),
    };
    if !ctx.can(permission) || !crate::auth::can_view_tournament_restricted_section(conn, room.tid, Some(&ctx)) {
        return Err(HttpResponse::Forbidden().finish());
    }
    Ok(ctx)
}

/// The events the server acknowledged to this room's client but did not apply, newest first.
#[get("/{id}/quarantine")]
async fn read_quarantine(db: Data<Database>, room_id: Path<Uuid>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);

    let room = match models::room::read(&mut conn, room_id.into_inner()) {
        Ok(r) => r,
        Err(_) => return HttpResponse::NotFound().finish(),
    };
    let read_permission = format!("{}:{}", AppResource::RoomMonitor.as_str(), AppAction::Read.as_str());
    if let Err(resp) = authorize_monitor_operation(&mut conn, &req, &room, &read_permission) {
        return resp;
    }
    match models::gameevent_quarantine::read_all_of_room(&mut conn, room.roomid) {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Starts a Clear Pending + Resend for the room. It discards nothing: every event acknowledged
/// while clearing is kept in the quarantine and the affected games are then resent in full.
#[post("/{id}/clear-pending")]
async fn start_clear_pending(db: Data<Database>, room_id: Path<Uuid>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);

    let room = match models::room::read(&mut conn, room_id.into_inner()) {
        Ok(r) => r,
        Err(_) => return HttpResponse::NotFound().finish(),
    };
    let update_permission = format!("{}:{}", AppResource::Game.as_str(), AppAction::Update.as_str());
    let ctx = match authorize_monitor_operation(&mut conn, &req, &room, &update_permission) {
        Ok(c) => c,
        Err(resp) => return resp,
    };

    match models::room_queue_clear::start(&mut conn, room.roomid, ctx.user_id) {
        Ok(clear) => {
            crate::live::tournament_changed(&req, room.tid);
            HttpResponse::Created().json(clear)
        }
        // The partial unique index allows one clear in flight per room.
        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
            HttpResponse::Conflict().json(json!({ "error": "This room already has a Clear Pending + Resend in progress." }))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Stops a clear that is still acknowledging events. Anything it already set aside is still owed
/// back, so in that case it moves straight on to resending rather than ending.
#[delete("/{id}/clear-pending")]
async fn cancel_clear_pending(db: Data<Database>, room_id: Path<Uuid>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);

    let room = match models::room::read(&mut conn, room_id.into_inner()) {
        Ok(r) => r,
        Err(_) => return HttpResponse::NotFound().finish(),
    };
    let update_permission = format!("{}:{}", AppResource::Game.as_str(), AppAction::Update.as_str());
    let ctx = match authorize_monitor_operation(&mut conn, &req, &room, &update_permission) {
        Ok(c) => c,
        Err(resp) => return resp,
    };

    let clear = match models::room_queue_clear::active_for_room(&mut conn, room.roomid) {
        Ok(Some(c)) => c,
        Ok(None) => return HttpResponse::NotFound().json(json!({ "error": "This room has no Clear Pending + Resend in progress." })),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    if clear.status != models::room_queue_clear::STATUS_CLEARING {
        return HttpResponse::Conflict().json(json!({
            "error": "The room's events are already being resent; that cannot be cancelled without losing them."
        }));
    }

    let set_aside = models::gameevent_quarantine::read_cleared_of_clear(&mut conn, clear.room_clear_id)
        .map(|rows| rows.len())
        .unwrap_or(0);
    let result = if set_aside > 0 {
        models::room_queue_clear::mark_resending(&mut conn, clear.room_clear_id, Utc::now(), room.ping_game_id)
    } else {
        models::room_queue_clear::finish(&mut conn, clear.room_clear_id, models::room_queue_clear::STATUS_CANCELLED, Some(ctx.user_id))
    };
    match result {
        Ok(updated) => {
            crate::live::tournament_changed(&req, room.tid);
            HttpResponse::Ok().json(updated)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(read)
        .service(read_games)
        .service(read_game_rows)
        .service(read_games_detailed)
        .service(read_quarantine)
        .service(start_clear_pending)
        .service(cancel_clear_pending)
        .service(create)
        .service(update)
        .service(destroy);
}
