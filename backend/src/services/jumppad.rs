use actix_web::{delete, Error, get, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use crate::{database::Database, models::jumppad::{JumpPad, JumpPadChangeSet, NewJumpPad}};
use crate::models::{self, common::PaginationParams};
use crate::services::common::{EntityResponse, process_response};
// use utoipa::OpenApi;
use diesel::QueryResult;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct JumpPadDoc;

// #[utoipa::path(
//         get,
//         path = "/jumppads",
//         responses(
//             (status = 200, description = "JumpPads found successfully", body = JumpPad),
//             (status = 404, description = "JumpPad not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many JumpPads to return")
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
    
    match models::jumppad::read_all(&mut db, &url_params) {
        Ok(jumppad) => HttpResponse::Ok().json(jumppad),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<i64>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::jumppad::read(&mut db, item_id.into_inner()) {
        Ok(jumppad) => HttpResponse::Ok().json(jumppad),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewJumpPad>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut db = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} JumpPad model create {:?}", line!(), item);

    // log this api call
    models::apicalllog::create(&mut db, &req);
    
    let result: QueryResult<JumpPad> = models::jumppad::create(&mut db, &item);

    let response: EntityResponse<JumpPad> = process_response(result, "post");
    
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
    Json(item): Json<JumpPadChangeSet>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} JumpPad model update {:?} {:?}", line!(), item_id, item); 

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::jumppad::update(&mut db, item_id.into_inner(), &item);

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

    tracing::debug!("{} JumpPad model delete {:?}", line!(), item_id);

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::jumppad::delete(&mut db, item_id.into_inner());

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        println!("Errored delete result: {:?}",result);
        HttpResponse::InternalServerError().finish()
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(read)
        .service(create)
        .service(update)
        .service(destroy);
}
