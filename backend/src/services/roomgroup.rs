use actix_web::{delete, Error, get, HttpMessage, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use crate::{auth::policies::UserContext, models::{self, common::PaginationParams, roomgroup::{NewRoomGroup, RoomGroup, RoomGroupChangeset}, permission::{AppAction, AppResource}}, services::common::{EntityResponse, PagedResponse, process_response}};
use crate::database::Database;
use diesel::QueryResult;
use uuid::Uuid;

// Roomgroups (e.g. buildings) are standalone: rooms and tournaments reference one. They aren't owned
// by a single tournament, so they're authorized with the plain Room RBAC permission (no tournament
// ABAC) — the same permission that guards room management.
fn has_room_permission(user_ctx: &UserContext, action: AppAction) -> bool {
    let permission = format!("{}:{}", AppResource::Room.as_str(), action.as_str());
    user_ctx.permissions.contains(&permission)
}

#[get("")]
async fn index(
    db: Data<Database>,
    Query(_url_params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);

    match models::roomgroup::read_all(&mut conn) {
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

    match models::roomgroup::read(&mut conn, item_id.into_inner()) {
        Ok(roomgroup) => HttpResponse::Ok().json(roomgroup),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// Enriched room data-table rows for the rooms in this roomgroup (building), in one paginated call.
#[get("/{id}/room-rows")]
async fn read_room_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    match models::room::read_room_rows_of_roomgroup(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(mut item): Json<NewRoomGroup>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    if !has_room_permission(user_ctx, AppAction::Create) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    // A roomgroup belongs to a tournament; make sure the parent exists.
    if models::tournament::read(&mut conn, item.tournamentid).is_err() {
        return Ok(HttpResponse::UnprocessableEntity().json(serde_json::json!({
            "error": format!("Tournament with ID {} does not exist", item.tournamentid)
        })));
    }

    item.creator_userid = user_ctx.user_id;
    item.last_modified_userid = user_ctx.user_id;
    let result: QueryResult<RoomGroup> = models::roomgroup::create(&mut conn, &item);

    let response: EntityResponse<RoomGroup> = process_response(result, "post");
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
    Json(item): Json<RoomGroupChangeset>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    if !has_room_permission(user_ctx, AppAction::Update) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let roomgroup_id = item_id.into_inner();
    if models::roomgroup::read(&mut conn, roomgroup_id).is_err() {
        return Ok(HttpResponse::NotFound().finish());
    }

    let result = models::roomgroup::update(&mut conn, roomgroup_id, &item, user_ctx.user_id);
    let response: EntityResponse<RoomGroup> = process_response(result, "put");
    match response.code {
        200 | 201 => Ok(HttpResponse::Ok().json(response)),
        409 => Ok(HttpResponse::Conflict().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

#[delete("/{id}")]
async fn destroy(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    if !has_room_permission(user_ctx, AppAction::Delete) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let roomgroup_id = item_id.into_inner();
    if models::roomgroup::read(&mut conn, roomgroup_id).is_err() {
        return Ok(HttpResponse::NotFound().finish());
    }

    match models::roomgroup::delete(&mut conn, roomgroup_id) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Purge: permanently remove a roomgroup (including soft-deleted ones).
#[delete("/{id}/purge")]
async fn purge(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    if !has_room_permission(user_ctx, AppAction::Delete) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let roomgroup_id = item_id.into_inner();
    if models::roomgroup::read_including_deleted(&mut conn, roomgroup_id).is_err() {
        return Ok(HttpResponse::NotFound().finish());
    }

    match models::roomgroup::purge(&mut conn, roomgroup_id) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope
        .service(index)
        .service(read)
        .service(read_room_rows)
        .service(create)
        .service(update)
        .service(purge)
        .service(destroy)
}
