use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use crate::{models::{self, roster::{NewRoster, Roster}, roster_coach::{RosterCoach, RosterCoachBuilder}, user::{NewUser, User, UserChangeset}}, services::common::{EntityResponse, process_response}};
use crate::models::common::PaginationParams;
use crate::database::Database;
use diesel::QueryResult;
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

#[get("/{id}/tournaments-where-admin")]
async fn read_tournaments_where_admin(
    db: Data<Database>,
    user_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::tournament::read_all_tournaments_where_user_is_admin(&mut conn, user_id.into_inner(), &params) {
        Ok(rounds) => HttpResponse::Ok().json(rounds),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/teams-where-coach")]
async fn read_teams_where_coach(
    db: Data<Database>,
    user_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::team::read_all_teams_where_user_is_coach(&mut conn, user_id.into_inner(), &params) {
        Ok(rounds) => HttpResponse::Ok().json(rounds),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/teams-where-quizzer")]
async fn read_teams_where_quizzer(
    db: Data<Database>,
    user_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::team::read_all_teams_where_user_is_quizzer(&mut conn, user_id.into_inner(), &params) {
        Ok(rounds) => HttpResponse::Ok().json(rounds),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/rosters-of-coach")]
async fn read_rosters_of_coach(
    db: Data<Database>,
    user_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::roster::read_all_rosters_of_coach(&mut conn, user_id.into_inner(), &params) {
        Ok(rounds) => HttpResponse::Ok().json(rounds),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/rosters-containing-quizzer")]
async fn read_rosters_containing_quizzer(
    db: Data<Database>,
    user_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::roster::read_all_rosters_containing_quizzer(&mut conn, user_id.into_inner(), &params) {
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

#[post("/{coach_id}/rosters")]
async fn create_roster(
    db: Data<Database>,
    Json(item): Json<NewRoster>,
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} Roster model create {:?}", line!(), item);

    // Create the Roster:
    
    let command_1_result: QueryResult<Roster> = models::roster::create(&mut conn, &item);

    if command_1_result.is_err() {
        println!("Error creating Roster: {:?}", command_1_result);
        let command_1_response: EntityResponse<Roster> = process_response(command_1_result, "post");
        return Ok(HttpResponse::InternalServerError().json(command_1_response));
    }

    // Create the RosterCoach record (*this is what gives the coach access to the Roster):

    let coach_id = item.created_by_userid;
    let roster_id = command_1_result.as_ref().unwrap().rosterid;
    let new_rostercoach = RosterCoachBuilder::new(coach_id, roster_id)
        .build()
        .unwrap();
    
    let command_2_result: QueryResult<RosterCoach> = models::roster_coach::create(&mut conn, new_rostercoach);

    if command_2_result.is_err() {
        println!("Error creating RosterCoach: {:?}", command_2_result);
        let command_2_response: EntityResponse<RosterCoach> = process_response(command_2_result, "post");
        return Ok(HttpResponse::InternalServerError().json(command_2_response));
    }
    
    let command_1_response: EntityResponse<Roster> = process_response(command_1_result, "post");    
    match command_1_response.code {
        409 => Ok(HttpResponse::Conflict().json(command_1_response)),
        201 => Ok(HttpResponse::Created().json(command_1_response)),
        200 => Ok(HttpResponse::Ok().json(command_1_response)),
        _ => Ok(HttpResponse::InternalServerError().json(command_1_response))
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
        .service(read_tournaments_where_admin)
        .service(read_teams_where_coach)
        .service(read_teams_where_quizzer)
        .service(read_rosters_of_coach)
        .service(read_rosters_containing_quizzer)
        .service(create)
        .service(create_roster)
        .service(update)
        .service(destroy);
}
