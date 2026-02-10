use actix_web::{delete, Error, get, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use crate::database::Database;
use crate::models::{self, common::PaginationParams, equipmentregistration::{NewEquipmentRegistration, EquipmentRegistration, EquipmentRegistrationChangeset}};
use crate::services::common::{EntityResponse, process_response};
// use utoipa::OpenApi;
use diesel::QueryResult;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct EquipmentRegistrationDoc;

// #[utoipa::path(
//         get,
//         path = "/equipmentregistrations",
//         responses(
//             (status = 200, description = "EquipmentRegistrations found successfully", body = EquipmentRegistration),
//             (status = 404, description = "EquipmentRegistration not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many EquipmentRegistrations to return")
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
    
    match models::equipmentregistration::read_all(&mut db, &url_params) {
        Ok(equipmentregistration) => HttpResponse::Ok().json(equipmentregistration),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<i64>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::equipmentregistration::read(&mut conn, item_id.into_inner()) {
        Ok(equipmentregistration) => HttpResponse::Ok().json(equipmentregistration),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// #[get("/{id}/games")]
// async fn read_games(
//     db: Data<Database>,
//     tour_id: Path<Uuid>,
//     Query(params): Query<PaginationParams>,
// ) -> HttpResponse {
//     let mut conn = db.pool.get().unwrap();

//     match models::game::read_all_games_of_equipmentregistration(&mut conn, tour_id.into_inner(), &params) {
//         Ok(rounds) => HttpResponse::Ok().json(rounds),
//         Err(_) => HttpResponse::NotFound().finish(),
//     }
// }

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewEquipmentRegistration>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} EquipmentRegistration model create {:?}", line!(), item);

    // log this api call
    models::apicalllog::create(&mut conn, &req);
    
    let result: QueryResult<EquipmentRegistration> = models::equipmentregistration::create(&mut conn, &item);

    let response: EntityResponse<EquipmentRegistration> = process_response(result, "post");
    
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
    item_id: Path<i64>,
    Json(item): Json<EquipmentRegistrationChangeset>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} EquipmentRegistration model update {:?} {:?}", line!(), item_id, item); 

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::equipmentregistration::update(&mut db, item_id.into_inner(), &item);

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
    item_id: Path<i64>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} EquipmentRegistration model delete {:?}", line!(), item_id);

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::equipmentregistration::delete(&mut db, item_id.into_inner());

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
        // .service(read_games)
        .service(create)
        .service(update)
        .service(destroy);
}
