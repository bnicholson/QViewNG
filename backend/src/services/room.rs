use actix_http::ws::OpCode;
use actix_web::{delete, Error, get, HttpMessage, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use serde::{Deserialize, Serialize};
use serde_json::json;
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

    // Can assume all games for a given room belong to the same tournament.
    let tournament = match models::tournament::read(&mut db, games_list[0].tournamentid) {
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
        if !divisions_cache.contains_key(&g.divisionid) {
            match models::division::read(&mut db, g.divisionid) {
                Ok(d) => { divisions_cache.insert(g.divisionid, d); },
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
        let dname = divisions_cache.get(&g.divisionid).map(|d| d.dname.clone()).unwrap_or_default();

        items.push(RoomGame {
            seqnum: 0,
            gameid: g.gid,
            roundid: g.roundid,
            roundname,
            did: g.divisionid,
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

#[get("/{id}/equipmentregistrations")]
async fn read_equipmentregistrations(
    db: Data<Database>,
    room_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::equipmentregistration::read_all_equipmentregistrations_of_room(&mut db, room_id.into_inner(), &params) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewRoom>,
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

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let result = models::room::update(&mut conn, room_id, &item);

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

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(read)
        .service(read_games)
        .service(read_games_detailed)
        .service(read_equipmentregistrations)
        .service(create)
        .service(update)
        .service(destroy);
}
