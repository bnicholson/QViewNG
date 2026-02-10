use actix_web::{delete, Error, get, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::database::Database;
use crate::models::{self, common::PaginationParams, room::{NewRoom, Room, RoomChangeset}, tournament::Tournament};
use crate::schema::tournaments::dsl::{tournaments as tournaments_table};
use crate::services::common::{EntityResponse, process_response};
// use utoipa::OpenApi;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};
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
    
    match models::room::read_all(&mut db, &url_params) {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(_) => HttpResponse::InternalServerError().finish(),
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

    let mut db = db.get_connection().expect("Failed to get connection");

    let tournament_exists: bool = tournaments_table
        .find(item.tid)
        .get_result::<Tournament>(&mut db)
        .is_ok();
    
    if !tournament_exists {
        println!("Could not find Tournament by ID={}", &item.tid);
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Tournament with ID {} does not exist", item.tid)
        })));
    }

    tracing::debug!("{} Room model create {:?}", line!(), item);

    // log this api call
    models::apicalllog::create(&mut db, &req);
    
    let result: QueryResult<Room> = models::room::create(&mut db, &item);

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

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Room model update {:?} {:?}", line!(), item_id, item); 

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::room::update(&mut db, item_id.into_inner(), &item);

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
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Room model delete {:?}", line!(), item_id);

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::room::delete(&mut db, item_id.into_inner());

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
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
