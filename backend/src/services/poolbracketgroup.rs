use actix_web::{delete, Error, get, HttpMessage, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path}};
use serde_json::json;
use crate::{auth::{is_rbac_and_abac_authorized, policies::{division::DivisionPolicyResource, PolicyContext, UserContext}}, models::{self, poolbracketgroup::{NewPoolBracketGroup, PoolBracketGroup, PoolBracketGroupChangeset}, permission::{AppAction, AppResource}}, services::common::{EntityResponse, PagedResponse, process_response}};
use crate::database::Database;
use diesel::QueryResult;
use uuid::Uuid;

// Poolbracketgroups are children of divisions and are authorized with the parent Division's
// permissions/policy (a tournament owner or admin may manage them).

/// Resolve the division + tournament for a group and check the caller holds `division:<action>` on it.
/// Returns Ok(()) when authorized, or the HttpResponse to return otherwise.
fn authorize_via_division(
    conn: &mut crate::database::Connection,
    user_ctx: &UserContext,
    division_id: Uuid,
    action: &str,
) -> Result<(), HttpResponse> {
    let division = match models::division::read(conn, division_id) {
        Ok(d) => d,
        Err(_) => return Err(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Division with ID {} does not exist", division_id)
        }))),
    };
    let tournament = match models::tournament::read(conn, division.tid) {
        Ok(t) => t,
        Err(_) => return Err(HttpResponse::InternalServerError().finish()),
    };
    let user_is_admin = models::tournament_admin::is_admin(conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: DivisionPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let permission = format!("{}:{}", AppResource::Division.as_str(), action);
    if is_rbac_and_abac_authorized(&policy_ctx, &permission, AppResource::Division.as_str()).is_err() {
        return Err(HttpResponse::Unauthorized().finish());
    }
    Ok(())
}

#[get("")]
async fn index(
    db: Data<Database>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);
    match models::poolbracketgroup::read_all(&mut conn) {
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
    models::apicalllog::create(&mut conn, &req);
    match models::poolbracketgroup::read(&mut conn, item_id.into_inner()) {
        Ok(group) => HttpResponse::Ok().json(group),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// The pool brackets belonging to this group.
#[get("/{id}/poolbrackets")]
async fn read_poolbrackets(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);
    match models::pool_bracket::read_all_of_poolbracketgroup(&mut conn, item_id.into_inner()) {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(mut item): Json<NewPoolBracketGroup>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    if let Err(resp) = authorize_via_division(&mut conn, user_ctx, item.divisionid, AppAction::Create.as_str()) {
        return Ok(resp);
    }

    item.creator_userid = user_ctx.user_id;
    item.last_modified_userid = user_ctx.user_id;
    let result: QueryResult<PoolBracketGroup> = models::poolbracketgroup::create(&mut conn, &item);
    let response: EntityResponse<PoolBracketGroup> = process_response(result, "post");

    match response.code {
        400 => Ok(HttpResponse::BadRequest().json(response)),
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response)),
    }
}

#[put("/{id}")]
async fn update(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Json(item): Json<PoolBracketGroupChangeset>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let group_id = item_id.into_inner();
    let group = match models::poolbracketgroup::read(&mut conn, group_id) {
        Ok(g) => g,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    if let Err(resp) = authorize_via_division(&mut conn, user_ctx, group.divisionid, AppAction::Update.as_str()) {
        return Ok(resp);
    }

    let result = models::poolbracketgroup::update(&mut conn, group_id, &item, user_ctx.user_id);
    let response = process_response(result, "put");

    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response)),
    }
}

#[delete("/{id}")]
async fn destroy(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let group_id = item_id.into_inner();
    let group = match models::poolbracketgroup::read(&mut conn, group_id) {
        Ok(g) => g,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    if let Err(resp) = authorize_via_division(&mut conn, user_ctx, group.divisionid, AppAction::Delete.as_str()) {
        return Ok(resp);
    }

    // A group that still has pools can't be deleted — its pools (and placements) would be orphaned.
    match models::poolbracketgroup::count_pool_brackets(&mut conn, group_id) {
        Ok(n) if n > 0 => return Ok(HttpResponse::Conflict().json(json!({
            "error": "This pool group cannot be deleted because it still has pools. Move or delete its pools first."
        }))),
        Ok(_) => {}
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    }

    match models::poolbracketgroup::delete(&mut conn, group_id) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Purge: permanently remove a poolbracketgroup (including soft-deleted ones). Not used by the frontend.
#[delete("/{id}/purge")]
async fn purge(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let group_id = item_id.into_inner();
    let group = match models::poolbracketgroup::read_including_deleted(&mut conn, group_id) {
        Ok(g) => g,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    if let Err(resp) = authorize_via_division(&mut conn, user_ctx, group.divisionid, AppAction::Delete.as_str()) {
        return Ok(resp);
    }

    match models::poolbracketgroup::purge(&mut conn, group_id) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(read)
        .service(read_poolbrackets)
        .service(create)
        .service(update)
        .service(purge)
        .service(destroy);
}
