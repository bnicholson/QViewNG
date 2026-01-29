use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::{models::{self, user::{NewUser, User, UserChangeset}}, services::common::{EntityResponse, process_response}};
use crate::models::{tournament::Tournament,common::PaginationParams};
use crate::database::Database;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};
use crate::schema::tournaments::dsl::{tournaments as tournaments_table};
use uuid::Uuid;

#[get("")]
async fn index(
    db: Data<Database>,
    Query(info): Query<PaginationParams>,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    let result = models::user::read_all(&mut db, &info);
   
    println!("Users: {:?}",result);
    
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
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::user::read(&mut conn, item_id.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/games-where-quizmaster")]
async fn read_games_where_quizmaster(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::game::read_all_games_where_user_is_quizmaster(&mut conn, tour_id.into_inner(), &params) {
        Ok(rounds) => HttpResponse::Ok().json(rounds),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/games-where-contentjudge")]
async fn read_games_where_contentjudge(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::game::read_all_games_where_user_is_contentjudge(&mut conn, tour_id.into_inner(), &params) {
        Ok(rounds) => HttpResponse::Ok().json(rounds),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewUser>,
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} User model create {:?}", line!(), item);
    
    let result: QueryResult<User> = models::user::create(&mut conn, item);

    let response: EntityResponse<User> = process_response(result, "post");
    
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
    Json(item): Json<UserChangeset>,
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} User model update {:?} {:?}", line!(), item_id, item); 

    let result = models::user::update(&mut db, item_id.into_inner(), &item);

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
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} User model delete {:?}", line!(), item_id);

    let result = models::user::delete(&mut db, item_id.into_inner());

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
        .service(read_games_where_quizmaster)
        .service(read_games_where_contentjudge)
        .service(create)
        .service(update)
        .service(destroy);
}
