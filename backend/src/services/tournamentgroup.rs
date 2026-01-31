use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::database::Database;
use crate::models::{self, common::PaginationParams, tournamentgroup::{NewTournamentGroup, TournamentGroup, TournamentGroupChangeset}, tournament::Tournament};
use crate::schema::tournaments::dsl::{tid as tournament_tid, tournaments as tournaments_table};
use crate::services::common::{EntityResponse, process_response};
use utoipa::OpenApi;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, dsl::{exists,select}};
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
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");
    
    match models::tournamentgroup::read_all(&mut db, &url_params) {
        Ok(tournamentgroup) => HttpResponse::Ok().json(tournamentgroup),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<Uuid>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::tournamentgroup::read(&mut conn, item_id.into_inner()) {
        Ok(tournamentgroup) => HttpResponse::Ok().json(tournamentgroup),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// #[get("/{id}/games")]
// async fn read_games(
//     db: Data<Database>,
//     tour_id: Path<Uuid>,
//     Query(params): Query<PaginationParams>,
// ) -> HttpResponse {
//     let mut conn = db.pool.get().unwrap();

//     match models::game::read_all_games_of_tournamentgroup(&mut conn, tour_id.into_inner(), &params) {
//         Ok(rounds) => HttpResponse::Ok().json(rounds),
//         Err(_) => HttpResponse::NotFound().finish(),
//     }
// }

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewTournamentGroup>,
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} TournamentGroup model create {:?}", line!(), item);
    
    let result: QueryResult<TournamentGroup> = models::tournamentgroup::create(&mut conn, &item);

    let response: EntityResponse<TournamentGroup> = process_response(result, "post");
    
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
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} TournamentGroup model update {:?} {:?}", line!(), item_id, item); 

    let result = models::tournamentgroup::update(&mut db, item_id.into_inner(), &item);

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

//     tracing::debug!("{} TournamentGroup model delete {:?}", line!(), item_id);

//     let result = models::tournamentgroup::delete(&mut db, item_id.into_inner());

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
        .service(update)
        // .service(destroy);
}
