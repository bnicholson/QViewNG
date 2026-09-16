use actix_web::{delete, Error, get, HttpMessage, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::{auth::{is_rbac_and_abac_authorized, policies::{division::DivisionPolicyResource, PolicyContext, UserContext}}, models::{self, common::PaginationParams, roundgroup::{NewRoundGroup, RoundGroup, RoundGroupChangeset}, permission::{AppAction, AppResource}}, services::common::{EntityResponse, PagedResponse, process_response}};
use crate::database::Database;
use diesel::QueryResult;
use uuid::Uuid;

// Division roundgroups are children of divisions and are authorized with the parent Division's
// permissions/policy (a tournament owner or admin may manage them).

#[get("")]
async fn index(
    db: Data<Database>,
    Query(url_params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::roundgroup::read_all(&mut db) {
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

    match models::roundgroup::read(&mut conn, item_id.into_inner()) {
        Ok(roundgroup) => HttpResponse::Ok().json(roundgroup),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// Enriched game rows for the roundgroup (games whose round belongs to the roundgroup), in one paginated
/// call.
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

    match models::game::read_game_rows_of_roundgroup(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// The roundgroup's rounds (a round belongs to a division roundgroup), ordered by scheduled start time.
#[get("/{id}/rounds")]
async fn read_rounds(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(url_params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::round::read_all_rounds_of_roundgroup(&mut conn, item_id.into_inner(), &url_params) {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Enriched round-table rows for the roundgroup (round + roundgroup/division names), plus total count.
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

    match models::round::read_round_rows_of_roundgroup(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(mut item): Json<NewRoundGroup>,
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

    let division = match models::division::read(&mut conn, item.did) {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Division with ID {} does not exist", item.did)
        }))),
    };

    let tournament = match models::tournament::read(&mut conn, division.tid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Tournament with ID {} does not exist", division.tid)
        }))),
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: DivisionPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let create_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Create.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &create_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} RoundGroup model create {:?}", line!(), item);

    item.creator_userid = user_ctx.user_id;
    item.last_modified_userid = user_ctx.user_id;
    let result: QueryResult<RoundGroup> = models::roundgroup::create(&mut conn, &item);

    let response: EntityResponse<RoundGroup> = process_response(result, "post");

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
    Json(item): Json<RoundGroupChangeset>,
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

    let roundgroup_id = item_id.into_inner();

    let roundgroup = match models::roundgroup::read(&mut conn, roundgroup_id) {
        Ok(s) => s,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let division = match models::division::read(&mut conn, roundgroup.did) {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
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
    let update_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Update.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &update_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} RoundGroup model update {:?} {:?}", line!(), roundgroup_id, item);

    let result = models::roundgroup::update(&mut conn, roundgroup_id, &item, user_ctx.user_id);

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

    let roundgroup_id = item_id.into_inner();

    let roundgroup = match models::roundgroup::read(&mut conn, roundgroup_id) {
        Ok(s) => s,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let division = match models::division::read(&mut conn, roundgroup.did) {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
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
    let delete_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Delete.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &delete_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} RoundGroup model delete {:?}", line!(), roundgroup_id);

    let result = models::roundgroup::delete(&mut conn, roundgroup_id);

    if result.is_ok() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}

/// Purge: permanently remove a division roundgroup (including soft-deleted ones). Not used by the
/// frontend — the app deletes via the soft-delete `destroy` endpoint.
#[delete("/{id}/purge")]
async fn purge(
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

    let roundgroup_id = item_id.into_inner();

    // read_including_deleted so an already soft-deleted roundgroup can still be purged.
    let roundgroup = match models::roundgroup::read_including_deleted(&mut conn, roundgroup_id) {
        Ok(s) => s,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let division = match models::division::read(&mut conn, roundgroup.did) {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
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
    let delete_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Delete.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &delete_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    match models::roundgroup::purge(&mut conn, roundgroup_id) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(read)
        .service(read_game_rows)
        .service(read_rounds)
        .service(read_round_rows)
        .service(create)
        .service(update)
        .service(purge)
        .service(destroy);
}
