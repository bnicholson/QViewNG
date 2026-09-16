use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, Result, delete, get, post, put, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::{auth::{is_rbac_and_abac_authorized, policies::{division::DivisionPolicyResource, PolicyContext, UserContext}}, models::{self, division::{Division, DivisionChangeset, NewDivision}, permission::{AppAction, AppResource}}, services::common::{EntityResponse, PagedResponse, process_response}};
use crate::models::common::PaginationParams;
use crate::database::Database;
use utoipa::OpenApi;
use diesel::QueryResult;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(paths(index))]
pub struct DivisionDoc;

#[utoipa::path(
        get,
        path = "/divisions",
        responses(
            (status = 200, description = "Divisions found successfully", body = Division),
            (status = 404, description = "Division not found")
        ),
        params(
            ("page" = Option<u64>, Query, description = "Page to read"),
            ("page_size" = Option<u64>, Query, description = "How many Divisions to return")
        )
    )
]
#[get("")]
async fn index(
    db: Data<Database>,
    Query(info): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match (models::division::read_all(&mut db, &info), models::division::count(&mut db)) {
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
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::division::read(&mut conn, item_id.into_inner()) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/rounds")]
async fn read_rounds(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::round::read_all_rounds_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/pool-brackets")]
async fn read_pool_brackets(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::pool_bracket::read_all_of_division(&mut conn, item_id.into_inner()) {
        Ok(brackets) => HttpResponse::Ok().json(brackets),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// Query params for the pool-bracket rows endpoint: pagination plus the `type` filter
/// (e.g. "pool" or "bracket").
#[derive(serde::Deserialize)]
struct PoolBracketRowsParams {
    page: i64,
    page_size: i64,
    #[serde(rename = "type")]
    type_: String,
}

/// Returns fully-formed pool-bracket data-table rows (bracket + roundgroup/division names +
/// last-modified user name) for the division, filtered by `type`, in a single paginated call.
#[get("/{id}/pool-bracket-rows")]
async fn read_pool_bracket_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PoolBracketRowsParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let pagination = PaginationParams { page: params.page, page_size: params.page_size };
    match models::pool_bracket::read_pool_bracket_rows_of_division(&mut conn, item_id.into_inner(), &params.type_, &pagination) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}/roundgroups")]
async fn read_roundgroups(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::roundgroup::read_all_of_division(&mut conn, item_id.into_inner()) {
        Ok(roundgroups) => HttpResponse::Ok().json(roundgroups),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// Returns fully-formed roundgroup data-table rows (roundgroup + division name + last-modified user name)
/// for the division in a single paginated call.
#[get("/{id}/roundgroup-rows")]
async fn read_roundgroup_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::roundgroup::read_roundgroup_rows_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}/teams")]
async fn read_teams(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::team::read_all_teams_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// Returns fully-formed game data-table rows (game + division/room/team names, start time, and
/// room sequence number) for the division in a single paginated call.
#[get("/{id}/game-rows")]
async fn read_game_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::game::read_game_rows_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns fully-formed round data-table rows (round + division name) for the division in a
/// single paginated call, so the rounds table needs only one request per page.
#[get("/{id}/round-rows")]
async fn read_round_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::round::read_round_rows_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns fully-formed team data-table rows (team + division name + coach name) for the
/// division in a single call, so the teams table needs only one request.
#[get("/{id}/team-rows")]
async fn read_team_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::team::read_team_rows_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns fully-formed quizzer data-table rows (user fields + divisions + teams) for the
/// division in a single call, so the quizzers table needs only one request.
#[get("/{id}/quizzer-rows")]
async fn read_quizzer_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::team::read_quizzer_rows_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}/games")]
async fn read_games(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::game::read_all_games_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(mut item): Json<NewDivision>,
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
        resource: DivisionPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let div_create_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Create.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &div_create_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    if !models::tournament::exists(&mut conn, item.tid) {
        println!("Could not find Tournament by ID={}", &item.tid);
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Tournament with ID {} does not exist", item.tid)
        })));
    }

    // Record who created/last-modified this row (server-derived, never client-supplied).
    item.last_modified_user = user_ctx.user_id;

    tracing::debug!("{} Division model create {:?}", line!(), item);

    let result: QueryResult<Division> = models::division::create(&mut conn, &item);

    // On successful creation, also create a parallel statsgroup scoped to this division
    // (division_id = the new division; tournament_id = its tournament), then add every game
    // currently in the division to that statsgroup via games_statsgroups.
    if let Ok(ref division) = result {
        match models::statsgroup::StatsGroupBuilder::new_default(&division.dname, division.tid)
            .set_division_id(Some(division.did))
            .build_and_insert(&mut conn)
        {
            Ok(statsgroup) => {
                let all_games = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
                match models::game::read_all_games_of_division(&mut conn, division.did, &all_games) {
                    Ok(games) => {
                        for game in games {
                            if let Err(e) = models::game_statsgroup::GameStatsGroupBuilder::new(game.gid, statsgroup.sgid)
                                .build_and_insert(&mut conn)
                            {
                                tracing::error!("{} Failed to add game {} to statsgroup {}: {:?}", line!(), game.gid, statsgroup.sgid, e);
                            }
                        }
                    }
                    Err(e) => tracing::error!("{} Failed to read games of division {}: {:?}", line!(), division.did, e),
                }
            }
            Err(e) => tracing::error!("{} Failed to create parallel statsgroup for division {}: {:?}", line!(), division.did, e),
        }
    }

    let response: EntityResponse<Division> = process_response(result, "post");
    
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
    Json(item): Json<DivisionChangeset>,
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

    let division_id = item_id.into_inner();

    let division = match models::division::read(&mut conn, division_id) {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, division.tid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: DivisionPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let div_update_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Update.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &div_update_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} Division model update {:?} {:?}", line!(), division_id, item);

    let result = models::division::update(&mut conn, division_id, &item, user_ctx.user_id);

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

    let division_id = item_id.into_inner();

    let division = match models::division::read(&mut conn, division_id) {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, division.tid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: DivisionPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let div_delete_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Delete.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &div_delete_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} Division model delete {:?}", line!(), division_id);

    let result = models::division::delete(&mut conn, division_id);

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
        .service(read_rounds)
        .service(read_pool_brackets)
        .service(read_pool_bracket_rows)
        .service(read_roundgroups)
        .service(read_roundgroup_rows)
        .service(read_teams)
        .service(read_team_rows)
        .service(read_quizzer_rows)
        .service(read_round_rows)
        .service(read_game_rows)
        .service(read_games)
        .service(create)
        .service(update)
        .service(destroy);
}
