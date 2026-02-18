use actix_web::{Error, get, HttpResponse, HttpRequest, post, Result, web::{Data, Json, Path, Query}};
use crate::database::Database;
use crate::models::{self, common::PaginationParams, gameevent::{NewGameEvent, GameEvent}};
use crate::services::common::{EntityResponse, process_response};
// use utoipa::OpenApi;
use diesel::QueryResult;
use uuid::Uuid;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct GameEventDoc;

// #[utoipa::path(
//         get,
//         path = "/gameevents",
//         responses(
//             (status = 200, description = "GameEvents found successfully", body = GameEvent),
//             (status = 404, description = "GameEvent not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many GameEvents to return")
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
    
    match models::gameevent::read_all(&mut db, &url_params) {
        Ok(gameevent) => HttpResponse::Ok().json(gameevent),
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

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::gameevent::read(&mut conn, item_id.into_inner()) {
        Ok(gameevent) => HttpResponse::Ok().json(gameevent),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewGameEvent>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");
    
    tracing::debug!("{} GameEvent model create {:?}", line!(), item);

    // log this api call
    models::apicalllog::create(&mut conn, &req);
    
    let result: QueryResult<GameEvent> = models::gameevent::create(&mut conn, &item);

    let response: EntityResponse<GameEvent> = process_response(result, "post");
    
    match response.code {
        400 => Ok(HttpResponse::BadRequest().json(response)),
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
//     Json(item): Json<GameEventChangeset>,
//     req: HttpRequest
// ) -> Result<HttpResponse, Error> {

//     let mut db = db.pool.get().unwrap();

//     tracing::debug!("{} GameEvent model update {:?} {:?}", line!(), item_id, item); 

//     // log this api call
//     models::apicalllog::create(&mut db, &req);

//     let result = models::gameevent::update(&mut db, item_id.into_inner(), &item);

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
//     req: HttpRequest
// ) -> HttpResponse {
//     let mut db = db.pool.get().unwrap();

//     tracing::debug!("{} GameEvent model delete {:?}", line!(), item_id);

//     // log this api call
//     models::apicalllog::create(&mut db, &req);

//     let result = models::gameevent::delete(&mut db, item_id.into_inner());

//     if result.is_ok() {
//         HttpResponse::Ok().finish()
//     } else {
//         HttpResponse::InternalServerError().finish()
//     }
// }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(read)
        .service(create);
        // .service(update)
        // .service(destroy);
}
