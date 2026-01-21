use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::{database::Database, models::division::Division};
use crate::models::{self, common::PaginationParams, round::{NewRound, Round, RoundChangeset}, tournament::Tournament};
use crate::schema::rounds::dsl::rounds as rounds_table;
use crate::schema::divisions::dsl::{did as division_did, divisions as divisions_table};
use crate::services::common::{EntityResponse, process_response};
use utoipa::OpenApi;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, dsl::{exists,select}};
use uuid::Uuid;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct RoundDoc;

// #[utoipa::path(
//         get,
//         path = "/rounds",
//         responses(
//             (status = 200, description = "Rounds found successfully", body = Round),
//             (status = 404, description = "Round not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many Rounds to return")
//         )
//     )
// ]
#[get("")]
async fn index(
    db: Data<Database>,
    Query(url_params): Query<PaginationParams>,
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");
    
    match models::round::read_all(&mut db, &url_params) {
        Ok(round) => HttpResponse::Ok().json(round),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<Uuid>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::round::read(&mut conn, item_id.into_inner()) {
        Ok(round) => HttpResponse::Ok().json(round),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewRound>,
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

    tracing::debug!("{} Round model create {:?}", line!(), item);
    
    let result: QueryResult<Round> = models::round::create(&mut conn, &item);

    let response: EntityResponse<Round> = process_response(result, "post");
    
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
    Json(item): Json<RoundChangeset>,
) -> Result<HttpResponse, Error> {

    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Round model update {:?} {:?}", line!(), item_id, item); 

    let result = models::round::update(&mut db, item_id.into_inner(), &item);

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

//     tracing::debug!("{} Round model delete {:?}", line!(), item_id);

//     let result = models::round::delete(&mut db, item_id.into_inner());

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
