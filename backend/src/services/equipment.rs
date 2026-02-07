use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use crate::database::Database;
use crate::models::{self, common::PaginationParams, equipmentset::{NewEquipmentSet, EquipmentSet, EquipmentSetChangeset}};
use crate::services::common::{EntityResponse, process_response};
// use utoipa::OpenApi;
use diesel::QueryResult;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct EquipmentSetDoc;

// #[utoipa::path(
//         get,
//         path = "/equipmentsets",
//         responses(
//             (status = 200, description = "EquipmentSets found successfully", body = EquipmentSet),
//             (status = 404, description = "EquipmentSet not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many EquipmentSets to return")
//         )
//     )
// ]
// #[get("")]
// async fn index(
//     db: Data<Database>,
//     Query(url_params): Query<PaginationParams>,
// ) -> HttpResponse {
//     let mut db = db.get_connection().expect("Failed to get connection");
    
//     match models::equipmentset::read_all(&mut db, &url_params) {
//         Ok(equipmentset) => HttpResponse::Ok().json(equipmentset),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<i64>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::equipment::read(&mut conn, item_id.into_inner()) {
        Ok(equipment) => HttpResponse::Ok().json(equipment),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// #[post("")]
// async fn create(
//     db: Data<Database>,
//     Json(item): Json<NewEquipmentSet>,
// ) -> Result<HttpResponse, Error> {

//     let mut conn = db.get_connection().expect("Failed to get connection");

//     tracing::debug!("{} EquipmentSet model create {:?}", line!(), item);
    
//     let result: QueryResult<EquipmentSet> = models::equipmentset::create(&mut conn, &item);

//     let response: EntityResponse<EquipmentSet> = process_response(result, "post");
    
//     match response.code {
//         409 => Ok(HttpResponse::Conflict().json(response)),
//         201 => Ok(HttpResponse::Created().json(response)),
//         200 => Ok(HttpResponse::Ok().json(response)),
//         _ => Ok(HttpResponse::InternalServerError().json(response))
//     }
// }

// #[put("/{id}")]
// async fn update(
//     db: Data<Database>,
//     item_id: Path<i64>,
//     Json(item): Json<EquipmentSetChangeset>,
// ) -> Result<HttpResponse, Error> {

//     let mut db = db.pool.get().unwrap();

//     tracing::debug!("{} EquipmentSet model update {:?} {:?}", line!(), item_id, item); 

//     let result = models::equipmentset::update(&mut db, item_id.into_inner(), &item);

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
//     item_id: Path<i64>,
// ) -> HttpResponse {
//     let mut db = db.pool.get().unwrap();

//     tracing::debug!("{} EquipmentSet model delete {:?}", line!(), item_id);

//     let result = models::equipmentset::delete(&mut db, item_id.into_inner());

//     if result.is_ok() {
//         HttpResponse::Ok().finish()
//     } else {
//         HttpResponse::InternalServerError().finish()
//     }
// }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        // .service(index)
        .service(read)
        // .service(create)
        // .service(update)
        // .service(destroy);
}
