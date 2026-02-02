use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use crate::{database::Database, models::{self, common::PaginationParams, roster::{NewRoster, Roster, RosterChangeset}, roster_coach::{NewRosterCoach, RosterCoach}, roster_quizzer::{NewRosterQuizzer, RosterQuizzer}}};
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

// #[get("/{id}/quizzers")]
// async fn read_quizzers(
//     db: Data<Database>,
//     sg_id: Path<Uuid>,
//     Query(params): Query<PaginationParams>,
// ) -> HttpResponse {
//     let mut conn = db.pool.get().unwrap();

//     match models::quizzer::read_all_quizzers_of_roster(&mut conn, sg_id.into_inner(), &params) {
//         Ok(quizzers) => HttpResponse::Ok().json(quizzers),
//         Err(_) => HttpResponse::NotFound().finish(),
//     }
// }

#[get("/{coach_id}/coaches")]
async fn read_coaches(
    db: Data<Database>,
    coach_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::user::read_all_coaches_of_roster(&mut conn, coach_id.into_inner(), &params) {
        Ok(quizzers) => HttpResponse::Ok().json(quizzers),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/{sg_id}/quizzers/{quizzer_id}")]
async fn add_quizzer(
    db: Data<Database>,
    path_id: Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");

    // tracing::debug!("{} RosterQuizzer model create {:?}", line!(), item);

    let item_to_be_created = NewRosterQuizzer {
        rosterid: path_id.0,
        quizzerid: path_id.1,
    };
    
    let result : QueryResult<RosterQuizzer> = models::roster_quizzer::create(&mut db, &item_to_be_created);
    
    println!("Result from creating RosterQuizzer: {:?}", result);
    
    let response: EntityResponse<RosterQuizzer> = process_response(result, "post");
    
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

#[post("/{roster_id}/coaches")]
async fn add_coach(
    db: Data<Database>,
    path_id: Path<Uuid>,
    Json(item): Json<NewRosterCoach>    
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} NewRosterCoach model create {:?}", line!(), item);

    let item_to_be_created = NewRosterCoach {
        rosterid: path_id.into_inner(),
        ..item
    };
    
    let result : QueryResult<RosterCoach> = models::roster_coach::create(&mut db, item_to_be_created);
    
    println!("Result from creating RosterCoach: {:?}", result);
    
    let response: EntityResponse<RosterCoach> = process_response(result, "post");
    
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

#[delete("/{id}")]
async fn destroy(
    db: Data<Database>,
    item_id: Path<Uuid>,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Roster model delete {:?}", line!(), item_id);

    let result = models::roster::delete(&mut db, item_id.into_inner());

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[delete("/{sg_id}/coaches/{coach_id}")]
async fn remove_coach(
    db: Data<Database>,
    item_ids: Path<(Uuid, Uuid)>,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Roster model delete {:?}", line!(), item_ids);

    let roster_id = item_ids.0;
    let coach_id = item_ids.1;
    let result = models::roster_coach::delete(&mut db, roster_id, coach_id);

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

// #[delete("/{sg_id}/quizzers/{quizzer_id}")]
// async fn remove_quizzer(
//     db: Data<Database>,
//     item_ids: Path<(Uuid, Uuid)>,
// ) -> HttpResponse {
//     let mut db = db.pool.get().unwrap();

//     tracing::debug!("{} Roster model delete {:?}", line!(), item_ids);

//     let sg_id = item_ids.0;
//     let quizzer_id = item_ids.1;
//     let result = models::quizzer_roster::delete(&mut db, sg_id, quizzer_id);

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
        .service(read_coaches)
        // .service(read_quizzers)
        .service(add_quizzer)
        .service(add_coach)
        .service(update)
        .service(destroy)
        .service(remove_coach)
        // .service(remove_quizzer);
}
