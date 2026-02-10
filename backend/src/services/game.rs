use actix_web::{delete, Error, get, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use crate::database::Database;
use crate::models::{self, common::PaginationParams, game::{NewGame, Game, GameChangeset}};
use crate::services::common::{EntityResponse, process_response};
// use utoipa::OpenApi;
use diesel::QueryResult;
use uuid::Uuid;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct GameDoc;

// #[utoipa::path(
//         get,
//         path = "/games",
//         responses(
//             (status = 200, description = "Games found successfully", body = Game),
//             (status = 404, description = "Game not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many Games to return")
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
    
    match models::game::read_all(&mut db, &url_params) {
        Ok(game) => HttpResponse::Ok().json(game),
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

    match models::game::read(&mut conn, item_id.into_inner()) {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/statsgroups")]
async fn read_statsgroups(
    db: Data<Database>,
    game_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::statsgroup::read_all_statsgroups_of_game(&mut conn, game_id.into_inner(), &params) {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewGame>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");
    
    tracing::debug!("{} Game model create {:?}", line!(), item);

    // log this api call
    models::apicalllog::create(&mut conn, &req);
    
    let result: QueryResult<Game> = models::game::create(&mut conn, &item);

    let response: EntityResponse<Game> = process_response(result, "post");
    
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
    Json(item): Json<GameChangeset>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Game model update {:?} {:?}", line!(), item_id, item); 

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::game::update(&mut db, item_id.into_inner(), &item);

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

    tracing::debug!("{} Game model delete {:?}", line!(), item_id);

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::game::delete(&mut db, item_id.into_inner());

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
        .service(read_statsgroups)
        .service(create)
        .service(update)
        .service(destroy);
}
