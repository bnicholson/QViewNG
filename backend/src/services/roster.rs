use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use crate::{database::Database, models::{self, common::PaginationParams, roster::{NewRoster, Roster, RosterChangeset}}};
use crate::services::common::{EntityResponse, process_response};
use diesel::QueryResult;
use uuid::Uuid;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct RosterDoc;

// #[utoipa::path(
//         get,
//         path = "/rosters",
//         responses(
//             (status = 200, description = "Rosters found successfully", body = Roster),
//             (status = 404, description = "Roster not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many Rosters to return")
//         )
//     )
// ]
#[get("")]
async fn index(
    db: Data<Database>,
    Query(url_params): Query<PaginationParams>,
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");
    
    match models::roster::read_all(&mut db, &url_params) {
        Ok(roster) => HttpResponse::Ok().json(roster),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<Uuid>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::roster::read(&mut conn, item_id.into_inner()) {
        Ok(roster) => HttpResponse::Ok().json(roster),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// #[get("/{id}/games")]
// async fn read_games(
//     db: Data<Database>,
//     sg_id: Path<Uuid>,
//     Query(params): Query<PaginationParams>,
// ) -> HttpResponse {
//     let mut conn = db.pool.get().unwrap();

//     match models::game::read_all_games_of_roster(&mut conn, sg_id.into_inner(), &params) {
//         Ok(games) => HttpResponse::Ok().json(games),
//         Err(_) => HttpResponse::NotFound().finish(),
//     }
// }

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewRoster>,
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} Roster model create {:?}", line!(), item);
    
    let result: QueryResult<Roster> = models::roster::create(&mut conn, &item);

    let response: EntityResponse<Roster> = process_response(result, "post");
    
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

// #[post("/{sg_id}/games")]
// async fn add_game(
//     db: Data<Database>,
//     path_id: Path<Uuid>,
//     Json(item): Json<NewGameRoster>    
// ) -> Result<HttpResponse, Error> {
//     let mut db = db.get_connection().expect("Failed to get connection");

//     tracing::debug!("{} GameRoster model create {:?}", line!(), item);

//     let item_to_be_created = NewGameRoster {
//         rosterid: path_id.into_inner(),
//         ..item
//     };
    
//     let result : QueryResult<GameRoster> = models::game_roster::create(&mut db, &item_to_be_created);
    
//     println!("Result from creating GameRoster: {:?}", result);
    
//     let response: EntityResponse<GameRoster> = process_response(result, "post");
    
//     match response.code {
//         409 => Ok(HttpResponse::Conflict().json(response)),
//         201 => Ok(HttpResponse::Created().json(response)),
//         200 => Ok(HttpResponse::Ok().json(response)),
//         _ => Ok(HttpResponse::InternalServerError().json(response))
//     }
// }

#[put("/{id}")]
async fn update(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Json(item): Json<RosterChangeset>,
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Roster model update {:?} {:?}", line!(), item_id, item); 

    let result = models::roster::update(&mut db, item_id.into_inner(), &item);

    let response = process_response(result, "put");
    
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

// #[delete("/{id}")]
// async fn destroy(
//     db: Data<Database>,
//     item_id: Path<Uuid>,
// ) -> HttpResponse {
//     let mut db = db.pool.get().unwrap();

//     tracing::debug!("{} Roster model delete {:?}", line!(), item_id);

//     let result = models::roster::delete(&mut db, item_id.into_inner());

//     if result.is_ok() {
//         HttpResponse::Ok().finish()
//     } else {
//         HttpResponse::InternalServerError().finish()
//     }
// }

// #[delete("/{sg_id}/games/{game_id}")]
// async fn remove_game(
//     db: Data<Database>,
//     item_ids: Path<(Uuid, Uuid)>,
// ) -> HttpResponse {
//     let mut db = db.pool.get().unwrap();

//     tracing::debug!("{} Roster model delete {:?}", line!(), item_ids);

//     let sg_id = item_ids.0;
//     let game_id = item_ids.1;
//     let result = models::game_roster::delete(&mut db, sg_id, game_id);

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
        // .service(read_games)
        .service(create)
        // .service(add_game)
        .service(update)
        // .service(destroy)
        // .service(remove_game);
}
