use actix_web::{delete, Error, get, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::{models::{self, division::{Division, DivisionChangeset, NewDivision}}, services::common::{EntityResponse, process_response}};
use crate::models::common::PaginationParams;
use crate::database::Database;
use utoipa::OpenApi;
use diesel::QueryResult;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(paths(index))]
pub struct DivisionDoc;

#[utoipa::path(
        get,
        path = "/divisions",
        responses(
            (status = 200, description = "Divisions found successfully", body = Division),
            (status = 404, description = "Division not found")
        ),
        params(
            ("page" = Option<u64>, Query, description = "Page to read"),
            ("page_size" = Option<u64>, Query, description = "How many Divisions to return")
        )
    )
]
#[get("")]
async fn index(
    db: Data<Database>,
    Query(info): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::division::read_all(&mut db, &info);
   
    println!("Divisions: {:?}",result);
    
    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::InternalServerError().finish()
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

    match models::division::read(&mut conn, item_id.into_inner()) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/rounds")]
async fn read_rounds(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::round::read_all_rounds_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/teams")]
async fn read_teams(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::team::read_all_teams_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/games")]
async fn read_games(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::game::read_all_games_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewDivision>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    if !models::tournament::exists(&mut conn, item.tid) {
        println!("Could not find Tournament by ID={}", &item.tid);
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Tournament with ID {} does not exist", item.tid)
        })));
    }

    tracing::debug!("{} Division model create {:?}", line!(), item);
    
    let result: QueryResult<Division> = models::division::create(&mut conn, &item);

    let response: EntityResponse<Division> = process_response(result, "post");
    
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
    Json(item): Json<DivisionChangeset>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    tracing::debug!("{} Division model update {:?} {:?}", line!(), item_id, item); 

    let result = models::division::update(&mut db, item_id.into_inner(), &item);

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

    tracing::debug!("{} Division model delete {:?}", line!(), item_id);

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let result = models::division::delete(&mut db, item_id.into_inner());

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
        .service(read_rounds)
        .service(read_teams)
        .service(read_games)
        .service(create)
        .service(update)
        .service(destroy);
}
