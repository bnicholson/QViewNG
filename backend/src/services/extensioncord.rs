use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use crate::{database::Database, models::extensioncord::{ExtensionCord, ExtensionCordChangeSet, NewExtensionCord}};
use crate::models::{self, common::PaginationParams};
use crate::services::common::{EntityResponse, process_response};
// use utoipa::OpenApi;
use diesel::QueryResult;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct ExtensionCordDoc;

// #[utoipa::path(
//         get,
//         path = "/extensioncords",
//         responses(
//             (status = 200, description = "ExtensionCords found successfully", body = ExtensionCord),
//             (status = 404, description = "ExtensionCord not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many ExtensionCords to return")
//         )
//     )
// ]
#[get("")]
async fn index(
    db: Data<Database>,
    Query(url_params): Query<PaginationParams>,
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");
    
    match models::extensioncord::read_all(&mut db, &url_params) {
        Ok(extensioncord) => HttpResponse::Ok().json(extensioncord),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// #[get("/{id}")]
// async fn read(
//     db: Data<Database>,
//     item_id: Path<i64>,
// ) -> HttpResponse {
//     let mut conn = db.pool.get().unwrap();

//     match models::extensioncord::read(&mut conn, item_id.into_inner()) {
//         Ok(extensioncord) => HttpResponse::Ok().json(extensioncord),
//         Err(_) => HttpResponse::NotFound().finish(),
//     }
// }

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewExtensionCord>,
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} ExtensionCord model create {:?}", line!(), item);
    
    let result: QueryResult<ExtensionCord> = models::extensioncord::create(&mut conn, &item);

    let response: EntityResponse<ExtensionCord> = process_response(result, "post");
    
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
//     Json(item): Json<ExtensionCordChangeSet>,
// ) -> Result<HttpResponse, Error> {

//     let mut db = db.pool.get().unwrap();

//     tracing::debug!("{} ExtensionCord model update {:?} {:?}", line!(), item_id, item); 

//     let result = models::extensioncord::update(&mut db, item_id.into_inner(), &item);

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

//     tracing::debug!("{} ExtensionCord model delete {:?}", line!(), item_id);

//     let result = models::extensioncord::delete(&mut db, item_id.into_inner());

//     if result.is_ok() {
//         HttpResponse::Ok().finish()
//     } else {
//         println!("Errored delete result: {:?}",result);
//         HttpResponse::InternalServerError().finish()
//     }
// }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        // .service(read)
        .service(create)
        // .service(update)
        // .service(destroy);
}
