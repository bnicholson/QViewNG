use actix_web::{delete, Error, get, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use crate::{database::Database, models::tournamentgroup_tournament::{NewTournamentGroupTournament, TournamentGroupTournament}};
use crate::models::{self, common::PaginationParams, tournamentgroup::{NewTournamentGroup, TournamentGroup, TournamentGroupChangeset}};
use crate::services::common::{EntityResponse, process_response};
// use utoipa::OpenApi;
use diesel::QueryResult;
use uuid::Uuid;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct TournamentGroupDoc;

// #[utoipa::path(
//         get,
//         path = "/tournamentgroups",
//         responses(
//             (status = 200, description = "TournamentGroups found successfully", body = TournamentGroup),
//             (status = 404, description = "TournamentGroup not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many TournamentGroups to return")
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
    
    match models::tournamentgroup::read_all(&mut db, &url_params) {
        Ok(tournamentgroup) => HttpResponse::Ok().json(tournamentgroup),
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

    match models::tournamentgroup::read(&mut db, item_id.into_inner()) {
        Ok(tournamentgroup) => HttpResponse::Ok().json(tournamentgroup),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/tournaments")]
async fn read_tournaments(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::tournament::read_all_tournaments_of_tournamentgroup(&mut db, tour_id.into_inner(), &params) {
        Ok(rounds) => HttpResponse::Ok().json(rounds),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewTournamentGroup>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut db = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} TournamentGroup model create {:?}", line!(), item);

    // log this api call
    models::apicalllog::create(&mut db, &req);
    
    let result: QueryResult<TournamentGroup> = models::tournamentgroup::create(&mut db, &item);

    let response: EntityResponse<TournamentGroup> = process_response(result, "post");
    
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

#[post("/{tg_id}/tournaments")]
async fn add_tournament(
    db: Data<Database>,
    path_id: Path<Uuid>,
    Json(item): Json<NewTournamentGroupTournament>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} TournamentGroupTournament model create {:?}", line!(), item);

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let item_to_be_created = NewTournamentGroupTournament {
        tournamentgroupid: path_id.into_inner(),
        ..item
    };
    
    let result : QueryResult<TournamentGroupTournament> = models::tournamentgroup_tournament::create(&mut db, item_to_be_created);
    
    println!("Result from creating TournamentGroupTournament: {:?}", result);
    
    let response: EntityResponse<TournamentGroupTournament> = process_response(result, "post");
    
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
    Json(item): Json<TournamentGroupChangeset>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} TournamentGroup model update {:?} {:?}", line!(), item_id, item); 

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::tournamentgroup::update(&mut db, item_id.into_inner(), &item);

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

    tracing::debug!("{} TournamentGroup model delete {:?}", line!(), item_id);

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::tournamentgroup::delete(&mut db, item_id.into_inner());

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[delete("/{tg_id}/tournaments/{tour_id}")] 
async fn remove_tournament(
    db: Data<Database>,
    path_ids: Path<(Uuid,Uuid)>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    let tg_id = path_ids.0;
    let tour_id = path_ids.1;
    tracing::debug!("{} TournamentGroupTournament model delete, tg_id = {:?}, tour_id = {}", line!(), tg_id, tour_id);

    // log this api call
    models::apicalllog::create(&mut db, &req);
    
    let result = models::tournamentgroup_tournament::delete(&mut db, tg_id, tour_id);
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
        .service(read_tournaments)
        .service(create)
        .service(add_tournament)
        .service(update)
        .service(destroy)
        .service(remove_tournament);
}
