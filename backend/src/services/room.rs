use actix_web::{delete, Error, get, HttpMessage, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::auth::{is_abac_authorized, policies::{room::RoomPolicyResource, PolicyContext, UserContext}};
use crate::database::Database;
use crate::models::{self, common::PaginationParams, permission::{AppAction, AppResource}, room::{NewRoom, Room, RoomChangeset}};
use crate::services::common::{EntityResponse, PagedResponse, process_response};
// use utoipa::OpenApi;
use diesel::QueryResult;
use uuid::Uuid;

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
    tour_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::game::read_all_games_of_room(&mut db, tour_id.into_inner(), &params) {
        Ok(rounds) => HttpResponse::Ok().json(rounds),
        Err(_) => HttpResponse::NotFound().finish(),
    }
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
    if is_abac_authorized(&policy_ctx, &room_create_permission, AppResource::Room.as_str()).is_err() {
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
    if is_abac_authorized(&policy_ctx, &room_update_permission, AppResource::Room.as_str()).is_err() {
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
    if is_abac_authorized(&policy_ctx, &room_delete_permission, AppResource::Room.as_str()).is_err() {
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
        .service(read_equipmentregistrations)
        .service(create)
        .service(update)
        .service(destroy);
}
