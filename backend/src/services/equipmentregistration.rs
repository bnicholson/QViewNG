use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::database::Database;
use crate::models::{self, common::PaginationParams, equipmentregistration::{NewEquipmentRegistration, EquipmentRegistration, EquipmentRegistrationChangeset}, tournament::Tournament};
use crate::schema::tournaments::dsl::{tournaments as tournaments_table};
use crate::services::common::{EntityResponse, process_response};
// use utoipa::OpenApi;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

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
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");
    
    match models::equipmentregistration::read_all(&mut db, &url_params) {
        Ok(equipmentregistration) => HttpResponse::Ok().json(equipmentregistration),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// #[get("/{id}")]
// async fn read(
//     db: Data<Database>,
//     item_id: Path<Uuid>,
// ) -> HttpResponse {
//     let mut conn = db.pool.get().unwrap();

//     match models::equipmentregistration::read(&mut conn, item_id.into_inner()) {
//         Ok(equipmentregistration) => HttpResponse::Ok().json(equipmentregistration),
//         Err(_) => HttpResponse::NotFound().finish(),
//     }
// }

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
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} EquipmentRegistration model create {:?}", line!(), item);
    
    let result: QueryResult<EquipmentRegistration> = models::equipmentregistration::create(&mut conn, &item);

    let response: EntityResponse<EquipmentRegistration> = process_response(result, "post");
    
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

// #[put("/{id}")]
// async fn update(
//     db: Data<Database>,
//     item_id: Path<Uuid>,
//     Json(item): Json<EquipmentRegistrationChangeset>,
// ) -> Result<HttpResponse, Error> {

//     let mut db = db.pool.get().unwrap();

//     tracing::debug!("{} EquipmentRegistration model update {:?} {:?}", line!(), item_id, item); 

//     let result = models::equipmentregistration::update(&mut db, item_id.into_inner(), &item);

//     let response = process_response(result, "put");
    
//     match response.code {
//         409 => Ok(HttpResponse::Conflict().json(response)),
//         200 => Ok(HttpResponse::Ok().json(response)),
//         _ => Ok(HttpResponse::InternalServerError().json(response))
//     }
// }

// #[delete("/{id}")]
// async fn destroy(
//     db: Data<Database>,
//     item_id: Path<Uuid>,
// ) -> HttpResponse {
//     let mut db = db.pool.get().unwrap();

//     tracing::debug!("{} EquipmentRegistration model delete {:?}", line!(), item_id);

//     let result = models::equipmentregistration::delete(&mut db, item_id.into_inner());

//     if result.is_ok() {
//         HttpResponse::Ok().finish()
//     } else {
//         HttpResponse::InternalServerError().finish()
//     }
// }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        // .service(read)
        // .service(read_games)
        .service(create)
        // .service(update)
        // .service(destroy);
}
