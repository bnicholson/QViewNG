use actix_web::{delete, Error, get, HttpMessage, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::{auth::{is_rbac_and_abac_authorized, policies::{division::DivisionPolicyResource, PolicyContext, UserContext}}, models::{self, common::PaginationParams, pool_bracket::{NewPoolBracket, PoolBracket, PoolBracketChangeset}, permission::{AppAction, AppResource}}, services::common::{EntityResponse, PagedResponse, process_response}};
use crate::database::Database;
use diesel::QueryResult;
use uuid::Uuid;

// Pool brackets are grandchildren of divisions (division → division_session → pool_bracket) and are
// authorized with the parent Division's permissions/policy (a tournament owner or admin may manage
// them). The `type` field distinguishes "pool" from "bracket".

/// Resolves the tournament that owns a division session (session → division → tournament) plus
/// whether the user is that tournament's admin, for ABAC. Returns None if any link is missing.
fn resolve_policy(
    conn: &mut crate::database::Connection,
    session_id: Uuid,
    user_id: Uuid,
) -> Option<DivisionPolicyResource> {
    let session = models::division_session::read(conn, session_id).ok()?;
    let division = models::division::read(conn, session.did).ok()?;
    let tournament = models::tournament::read(conn, division.tid).ok()?;
    let user_is_tournament_admin = models::tournament_admin::is_admin(conn, tournament.tid, user_id);
    Some(DivisionPolicyResource { tournament, user_is_tournament_admin })
}

#[get("")]
async fn index(
    db: Data<Database>,
    Query(_url_params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::pool_bracket::read_all(&mut db) {
        Ok(items) => {
            let count = items.len() as i64;
            HttpResponse::Ok().json(PagedResponse { count, items })
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
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

    match models::pool_bracket::read(&mut conn, item_id.into_inner()) {
        Ok(bracket) => HttpResponse::Ok().json(bracket),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// Enriched team rows for the pool bracket, resolved via its teamgroup → team_teamgroups.
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

    match models::team::read_team_rows_of_pool_bracket(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Enriched game rows for games whose `poolbracket_id` is this bracket.
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

    match models::game::read_game_rows_of_pool_bracket(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(mut item): Json<NewPoolBracket>,
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

    let resource = match resolve_policy(&mut conn, item.division_session_id, user_ctx.user_id) {
        Some(r) => r,
        None => return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Division session with ID {} does not exist", item.division_session_id)
        }))),
    };

    let policy_ctx = PolicyContext { user_ctx: user_ctx.clone(), resource };
    let create_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Create.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &create_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} PoolBracket model create {:?}", line!(), item);

    item.creator_userid = user_ctx.user_id;
    item.last_modified_userid = user_ctx.user_id;
    let result: QueryResult<PoolBracket> = models::pool_bracket::create(&mut conn, &item);

    let response: EntityResponse<PoolBracket> = process_response(result, "post");

    match response.code {
        400 => Ok(HttpResponse::BadRequest().json(response)),
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
    Json(item): Json<PoolBracketChangeset>,
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

    let bracket_id = item_id.into_inner();

    let bracket = match models::pool_bracket::read(&mut conn, bracket_id) {
        Ok(b) => b,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let resource = match resolve_policy(&mut conn, bracket.division_session_id, user_ctx.user_id) {
        Some(r) => r,
        None => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let policy_ctx = PolicyContext { user_ctx: user_ctx.clone(), resource };
    let update_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Update.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &update_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} PoolBracket model update {:?} {:?}", line!(), bracket_id, item);

    let result = models::pool_bracket::update(&mut conn, bracket_id, &item, user_ctx.user_id);

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

    let bracket_id = item_id.into_inner();

    let bracket = match models::pool_bracket::read(&mut conn, bracket_id) {
        Ok(b) => b,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let resource = match resolve_policy(&mut conn, bracket.division_session_id, user_ctx.user_id) {
        Some(r) => r,
        None => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let policy_ctx = PolicyContext { user_ctx: user_ctx.clone(), resource };
    let delete_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Delete.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &delete_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} PoolBracket model delete {:?}", line!(), bracket_id);

    let result = models::pool_bracket::delete(&mut conn, bracket_id);

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
        .service(read_team_rows)
        .service(read_game_rows)
        .service(create)
        .service(update)
        .service(destroy);
}
