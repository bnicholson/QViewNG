use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::{database::Database, models::division::Division};
use crate::models::{self, common::PaginationParams, team::{NewTeam, Team, TeamChangeset}, tournament::Tournament};
use crate::schema::teams::dsl::{teams as teams_table,teamid};
use crate::schema::tournaments::dsl::{tid as tournament_tid, tournaments as tournaments_table};
use crate::schema::divisions::dsl::{did as division_did, divisions as divisions_table};
use crate::services::common::{EntityResponse, process_response};
use utoipa::OpenApi;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, dsl::{exists,select}};
use uuid::Uuid;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct TeamDoc;

// #[utoipa::path(
//         get,
//         path = "/teams",
//         responses(
//             (status = 200, description = "Teams found successfully", body = Team),
//             (status = 404, description = "Team not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many Teams to return")
//         )
//     )
// ]
#[get("")]
async fn index(
    db: Data<Database>,
    Query(url_params): Query<PaginationParams>,
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");
    
    match models::team::read_all(&mut db, &url_params) {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<Uuid>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::team::read(&mut conn, item_id.into_inner()) {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewTeam>,
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    let division_exists: bool = divisions_table
        .find(item.did)
        .get_result::<Division>(&mut conn)
        .is_ok();
    
    if !division_exists {
        println!("Could not find Division by ID={}", &item.did);
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Division with ID {} does not exist", item.did)
        })));
    }

    tracing::debug!("{} Team model create {:?}", line!(), item);
    
    let result: QueryResult<Team> = models::team::create(&mut conn, &item);

    let response: EntityResponse<Team> = process_response(result, "post");
    
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
    Json(item): Json<TeamChangeset>,
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Team model update {:?} {:?}", line!(), item_id, item); 

    let result = models::team::update(&mut db, item_id.into_inner(), &item);

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

//     tracing::debug!("{} Team model delete {:?}", line!(), item_id);

//     let result = models::team::delete(&mut db, item_id.into_inner());

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
        .service(create)
        .service(update)
        // .service(destroy);
}
