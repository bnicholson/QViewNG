use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::{database::Database, models::division::Division};
use crate::models::{self, common::PaginationParams, game::{NewGame, Game, GameChangeset}, tournament::Tournament};
use crate::schema::games::dsl::games as games_table;
use crate::schema::divisions::dsl::{did as division_did, divisions as divisions_table};
use crate::services::common::{EntityResponse, process_response};
use utoipa::OpenApi;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, dsl::{exists,select}};
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
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");
    
    match models::game::read_all(&mut db, &url_params) {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<Uuid>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::game::read(&mut conn, item_id.into_inner()) {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewGame>,
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    if item.tournamentid.is_some() {
        if !models::tournament::exists(&mut conn, item.tournamentid.unwrap()) {
            println!("Could not find Tournament by ID={}", &item.tournamentid.unwrap());
            return Ok(HttpResponse::UnprocessableEntity().json(json!({
                "error": format!("Tournament with ID {} does not exist", item.tournamentid.unwrap())
            })));
        }
    }

    if item.divisionid.is_some() {
        if !models::division::exists(&mut conn, item.divisionid.unwrap()) {
            println!("Could not find Division by ID={}", &item.divisionid.unwrap());
            return Ok(HttpResponse::UnprocessableEntity().json(json!({
                "error": format!("Division with ID {} does not exist", item.divisionid.unwrap())
            })));
        }
    }

    if !models::room::exists(&mut conn, item.roomid) {
        println!("Could not find Room by ID={}", &item.roomid);
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Room with ID {} does not exist", item.roomid)
        })));
    }

    if !models::round::exists(&mut conn, item.roundid) {
        println!("Could not find Round by ID={}", &item.roundid);
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Round with ID {} does not exist", item.roundid)
        })));
    }

    if !models::team::exists(&mut conn, item.leftteamid) {
        println!("Could not find Team by ID={}", &item.leftteamid);
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Team with ID {} does not exist", item.leftteamid)
        })));
    }

    if !models::team::exists(&mut conn, item.rightteamid) {
        println!("Could not find Team by ID={}", &item.rightteamid);
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Team with ID {} does not exist", item.rightteamid)
        })));
    }

    if !models::user::exists(&mut conn, item.quizmasterid) {
        println!("Could not find Team by ID={}", &item.quizmasterid);
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Team with ID {} does not exist", item.quizmasterid)
        })));
    }
    
    tracing::debug!("{} Game model create {:?}", line!(), item);
    
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
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Game model update {:?} {:?}", line!(), item_id, item); 

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
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Game model delete {:?}", line!(), item_id);

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
        .service(create)
        .service(update)
        .service(destroy);
}
