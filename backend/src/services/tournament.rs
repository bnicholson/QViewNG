use actix_web::{delete, Error, get, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use crate::models::{self, tournament_admin::{NewTournamentAdmin, TournamentAdmin}};
use crate::models::tournament::{NewTournament, Tournament, TournamentChangeset};
use crate::models::common::{PaginationParams,SearchDateParams};
use crate::services::common::{EntityResponse, process_response};
use chrono::{ Utc, TimeZone };
use crate::models::apicalllog::{apicalllog};
use utoipa::OpenApi;
use diesel::{QueryResult};
use crate::database::Database;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(paths(
    index,
    read,
    read_today,
    // create,
    // update,
    destroy
))]
pub struct TournamentDoc;

#[get("filter")]
async fn get_between_dates(
    db: Data<Database>,
    req: HttpRequest,
    Query(dinfo): Query<SearchDateParams>,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    apicalllog(&req);

    let result = models::tournament::read_between_dates(&mut db, dinfo.from_date, dinfo.to_date);

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[utoipa::path(
        get,
        path = "/tournaments",
        responses(
            (status = 200, description = "Tournaments found successfully", body = Tournament),
            (status = 404, description = "Tournament not found")
        ),
        params(
            ("page" = Option<u64>, Query, description = "Page to read"),
            ("page_size" = Option<u64>, Query, description = "How many Tournaments to return")
        )
    )
]
#[get("")]
async fn index(
    db: Data<Database>,
    Query(url_params): Query<PaginationParams>,
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");
    
    match models::tournament::read_all(&mut db, &url_params) {
        Ok(tournament) => HttpResponse::Ok().json(tournament),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
        get,
        path = "/tournaments/{id}",
        responses(
            (status = 200, description = "Tournament found successfully", body = Tournament),
            (status = 404, description = "Tournament not found")
        )
    )
]
#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<Uuid>,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    match models::tournament::read(&mut db, item_id.into_inner()) {
        Ok(tournament) => HttpResponse::Ok().json(tournament),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[utoipa::path(
        get,
        path = "/tournaments/today",
        responses(
            (status = 200, description = "Tournaments found successfully", body = Tournament),
            (status = 404, description = "Tournament not found")
        )
    )
]
#[get("/today")]
async fn read_today(
    db: Data<Database>,
    req: HttpRequest,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    println!("Inside /api/tournaments/today");
    // log this api call
    apicalllog(&req);

    // convert the query from the api call from timestamps in millis since 1970
    // to an actual 
    let now = Utc::now();
    let from_dt = (now.timestamp()-(7*24*3600))*1000;
    let to_dt = (now.timestamp() + (7*24*3600))*1000;

    tracing::debug!("{} /api/tournaments/today {:?} {:?} {:?}",line!(), now, from_dt, to_dt);

    let result = models::tournament::read_between_dates(&mut db, from_dt, to_dt);
    println!("Results: {:?} {:?} {:?}", from_dt, to_dt, result);

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/{id}/divisions")]
async fn read_divisions(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::tournament::read_divisions(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/admins")]
async fn read_admins(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::tournament::read_users(&mut conn, item_id.into_inner(), &params) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/rooms")]
async fn read_rooms(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::tournament::read_rooms(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/rounds")]
async fn read_rounds(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::tournament::read_rounds(&mut conn, tour_id.into_inner(), &params) {
        Ok(rounds) => HttpResponse::Ok().json(rounds),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// #[utoipa::path(
//         post,
//         path = "/tournaments",
//         responses(
//             (status = 200, description = "Tournament created successfully", body = Tournament)
//         )
//     )
// ]
#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewTournament>    
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} Tournament model create {:?}", line!(), item);
    
    let result : QueryResult<Tournament> = models::tournament::create(&mut db, &item);

    let response: EntityResponse<Tournament> = process_response(result, "post");
    
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

#[post("/{tour_id}/admins/{user_id}")]
async fn add_admin(
    db: Data<Database>,
    path_ids: Path<(Uuid,Uuid)>,
    Json(item): Json<NewTournamentAdmin>    
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} Tournament model create {:?}", line!(), item);

    let item_to_be_created = NewTournamentAdmin {
        tournamentid: path_ids.0,
        adminid: path_ids.1,
        ..item
    };
    
    let result : QueryResult<TournamentAdmin> = models::tournament_admin::create(&mut db, &item_to_be_created);

    let response: EntityResponse<TournamentAdmin> = process_response(result, "post");
    
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

// #[utoipa::path(
//         put,
//         path = "/tournaments/{id}",
//         responses(
//             (status = 200, description = "Tournament updated successfully", body = Tournament),
//             (status = 404, description = "Tournament not found")
//         )
//     )
// ]
#[put("/{id}")]
async fn update(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Json(item): Json<TournamentChangeset>,
) -> Result<HttpResponse, Error> {
    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Tournement model update {:?} {:?}", line!(), item_id, item); 

    let result = models::tournament::update(&mut db, item_id.into_inner(), &item);

    let response = process_response(result, "put");
    
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

#[utoipa::path(
        delete,
        path = "/tournaments/{id}",
        responses(
            (status = 200, description = "Tournament deleted successfully", body = Tournament),
            (status = 404, description = "Tournament not found")
        )
    )
]
#[delete("/{id}")]
async fn destroy(
    db: Data<Database>,
    item_id: Path<Uuid>,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    tracing::debug!("{} Tournament model delete {:?}", line!(), item_id);

    let result = models::tournament::delete(&mut db, item_id.into_inner());

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[delete("/{tour_id}/admins/{user_id}")]
async fn remove_admin(
    db: Data<Database>,
    path_ids: Path<(Uuid,Uuid)>,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    let tour_id = path_ids.0;
    let admin_id = path_ids.1;
    tracing::debug!("{} Tournament model delete, tour_id = {:?}, admin_id = {}", line!(), tour_id, admin_id);
    
    let result = models::tournament_admin::delete(&mut db, tour_id, admin_id);

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(get_between_dates)
        .service(read_today)
        .service(read)
        .service(read_rooms)
        .service(read_rounds)
        .service(read_divisions)
        .service(read_admins)
        .service(add_admin)
        .service(create)
        .service(update)
        .service(destroy)
        .service(remove_admin);
}
