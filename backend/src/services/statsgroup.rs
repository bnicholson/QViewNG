use actix_web::{delete, Error, get, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use crate::{database::Database, models::game_statsgroup::{GameStatsGroup, NewGameStatsGroup}};
use crate::models::{self, common::PaginationParams, statsgroup::{NewStatsGroup, StatsGroup, StatsGroupChangeset}};
use crate::services::common::{EntityResponse, process_response};
use diesel::QueryResult;
use uuid::Uuid;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct StatsGroupDoc;

// #[utoipa::path(
//         get,
//         path = "/statsgroups",
//         responses(
//             (status = 200, description = "StatsGroups found successfully", body = StatsGroup),
//             (status = 404, description = "StatsGroup not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many StatsGroups to return")
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
    
    match models::statsgroup::read_all(&mut db, &url_params) {
        Ok(statsgroup) => HttpResponse::Ok().json(statsgroup),
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

    match models::statsgroup::read(&mut db, item_id.into_inner()) {
        Ok(statsgroup) => HttpResponse::Ok().json(statsgroup),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/games")]
async fn read_games(
    db: Data<Database>,
    sg_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::game::read_all_games_of_statsgroup(&mut db, sg_id.into_inner(), &params) {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewStatsGroup>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut db = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} StatsGroup model create {:?}", line!(), item);

    // log this api call
    models::apicalllog::create(&mut db, &req);
    
    let result: QueryResult<StatsGroup> = models::statsgroup::create(&mut db, &item);

    let response: EntityResponse<StatsGroup> = process_response(result, "post");
    
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

#[post("/{sg_id}/games")]
async fn add_game(
    db: Data<Database>,
    path_id: Path<Uuid>,
    Json(item): Json<NewGameStatsGroup>    ,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} GameStatsGroup model create {:?}", line!(), item);

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let item_to_be_created = NewGameStatsGroup {
        statsgroupid: path_id.into_inner(),
        ..item
    };
    
    let result : QueryResult<GameStatsGroup> = models::game_statsgroup::create(&mut db, &item_to_be_created);
    
    println!("Result from creating GameStatsGroup: {:?}", result);
    
    let response: EntityResponse<GameStatsGroup> = process_response(result, "post");
    
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
    Json(item): Json<StatsGroupChangeset>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} StatsGroup model update {:?} {:?}", line!(), item_id, item); 

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::statsgroup::update(&mut db, item_id.into_inner(), &item);

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

    tracing::debug!("{} StatsGroup model delete {:?}", line!(), item_id);

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::statsgroup::delete(&mut db, item_id.into_inner());

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[delete("/{sg_id}/games/{game_id}")]
async fn remove_game(
    db: Data<Database>,
    item_ids: Path<(Uuid, Uuid)>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} StatsGroup model delete {:?}", line!(), item_ids);

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let sg_id = item_ids.0;
    let game_id = item_ids.1;
    let result = models::game_statsgroup::delete(&mut db, sg_id, game_id);

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
        .service(create)
        .service(add_game)
        .service(update)
        .service(destroy)
        .service(remove_game);
}
