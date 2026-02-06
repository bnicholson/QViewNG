use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use crate::{database::Database, models::projector::{Projector, ProjectorChangeSet, NewProjector}};
use crate::models::{self, common::PaginationParams};
use crate::services::common::{EntityResponse, process_response};
// use utoipa::OpenApi;
use diesel::QueryResult;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct ProjectorDoc;

// #[utoipa::path(
//         get,
//         path = "/projectors",
//         responses(
//             (status = 200, description = "Projectors found successfully", body = Projector),
//             (status = 404, description = "Projector not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many Projectors to return")
//         )
//     )
// ]
// #[get("")]
// async fn index(
//     db: Data<Database>,
//     Query(url_params): Query<PaginationParams>,
// ) -> HttpResponse {
//     let mut db = db.get_connection().expect("Failed to get connection");
    
//     match models::projector::read_all(&mut db, &url_params) {
//         Ok(projector) => HttpResponse::Ok().json(projector),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

// #[get("/{id}")]
// async fn read(
//     db: Data<Database>,
//     item_id: Path<i64>,
// ) -> HttpResponse {
//     let mut conn = db.pool.get().unwrap();

//     match models::projector::read(&mut conn, item_id.into_inner()) {
//         Ok(projector) => HttpResponse::Ok().json(projector),
//         Err(_) => HttpResponse::NotFound().finish(),
//     }
// }

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewProjector>,
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} Projector model create {:?}", line!(), item);
    
    let result: QueryResult<Projector> = models::projector::create(&mut conn, &item);

    let response: EntityResponse<Projector> = process_response(result, "post");
    
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
//     item_id: Path<i64>,
//     Json(item): Json<ProjectorChangeSet>,
// ) -> Result<HttpResponse, Error> {

//     let mut db = db.pool.get().unwrap();

//     tracing::debug!("{} Projector model update {:?} {:?}", line!(), item_id, item); 

//     let result = models::projector::update(&mut db, item_id.into_inner(), &item);

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

//     tracing::debug!("{} Projector model delete {:?}", line!(), item_id);

//     let result = models::projector::delete(&mut db, item_id.into_inner());

//     if result.is_ok() {
//         HttpResponse::Ok().finish()
//     } else {
//         println!("Errored delete result: {:?}",result);
//         HttpResponse::InternalServerError().finish()
//     }
// }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        // .service(index)
        // .service(read)
        .service(create)
        // .service(update)
        // .service(destroy);
}
