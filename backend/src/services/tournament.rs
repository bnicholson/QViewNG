use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, Result, delete, get, post, put, web::{Data, Json, Path, Query}};
use serde::{Deserialize, Serialize};
use crate::{auth::{is_rbac_and_abac_authorized, policies::{PolicyContext, UserContext}}, models::{self, permission::{AppAction, AppResource}, role::AppRole, room::Room, tournament_admin::{NewTournamentAdmin, TournamentAdmin}, users_roles::NewUsersRole}};
use crate::models::tournament::{NewTournament, NewTournamentPayload, Tournament, TournamentChangeset};
use crate::models::tournament_admin::TournamentAdminChangeset;
use crate::models::common::{PaginationParams,SearchDateParams};
use crate::services::common::{EntityResponse, PagedResponse, process_response};
use chrono::Utc;
use utoipa::OpenApi;
use diesel::{QueryResult};
use crate::database::Database;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TournamentWithRooms {
    pub tournament: Tournament,
    pub rooms: Vec<Room>,
}
impl TournamentWithRooms {
    pub fn new(tournament: Tournament, rooms: Vec<Room>) -> Self {
        Self {
            tournament,
            rooms
        }
    }
}

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
    models::apicalllog::create(&mut db, &req);

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
    req: HttpRequest  
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut db, &req);
    
    match (models::tournament::read_all(&mut db, &url_params), models::tournament::count(&mut db)) {
        (Ok(items), Ok(count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        _ => HttpResponse::InternalServerError().finish(),
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
    req: HttpRequest  
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

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
    models::apicalllog::create(&mut db, &req);

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

#[get("/today-max-100")]
async fn read_today_max_100(
    db: Data<Database>,
    req: HttpRequest,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    println!("Inside /api/tournaments/today-max-100");
    // log this api call
    models::apicalllog::create(&mut db, &req);

    // use 1 month (31 days) as the base for obtaining tournaments from server, into the past and into the future
    let days_before_and_after: i64 = 31;
    let days_before_and_after_in_milliseconds: i64 = (days_before_and_after*24*3600)*1000;
    let today = Utc::now();
    let from_dt = today.timestamp_millis() - days_before_and_after_in_milliseconds;
    let to_dt   = today.timestamp_millis() + days_before_and_after_in_milliseconds;

    tracing::debug!("{} /api/tournaments/today-max-100 {:?} {:?} {:?}",line!(), today, from_dt, to_dt);

    let result_tournaments = models::tournament::read_between_dates(&mut db, from_dt, to_dt);
    println!("Tournaments Result: {:?} {:?} {:?}", from_dt, to_dt, result_tournaments);

    if !result_tournaments.is_ok() {
        return HttpResponse::InternalServerError().finish();
    }

    let mut tournaments = result_tournaments.unwrap();
    let max_tournaments_to_send_in_response = 100;
    while tournaments.len() > max_tournaments_to_send_in_response {
        tournaments.remove(0);
        tournaments.pop();
    }

    let mut tournaments_with_rooms = Vec::<TournamentWithRooms>::new();
    let pagination_params_vals = PaginationParams {
        page: 0,
        page_size: 100,
    };
    for tournament in tournaments {
        let rooms_result = models::room::read_all_rooms_of_tournament(&mut db, tournament.tid, &pagination_params_vals);
        println!("Tournament Rooms Result: {:?}", rooms_result);
        if !rooms_result.is_ok() {
            return HttpResponse::NotFound().finish();
        }
        let rooms = rooms_result.unwrap();

        // Construct the object to be returned in the response
        let tournament_with_rooms = TournamentWithRooms::new(
            tournament.clone(),
            rooms.clone()
        );

        tournaments_with_rooms.push(tournament_with_rooms);
    }

    return HttpResponse::Ok().json(tournaments_with_rooms);
}

#[get("/{id}/divisions")]
async fn read_divisions(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::division::read_all_divisions_of_tournament(&mut db, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/admins")]
async fn read_admins(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::user::read_all_admins_of_tournament(&mut db, item_id.into_inner(), &params) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/equipmentregistrations")]
async fn read_equipmentregistrations(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::equipmentregistration::read_all_equipmentregistrations_of_tournament(&mut db, tour_id.into_inner(), &params) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/rooms")]
async fn read_rooms(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::room::read_all_rooms_of_tournament(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/rounds")]
async fn read_rounds(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::round::read_all_rounds_of_tournament(&mut conn, tour_id.into_inner(), &params) {
        Ok(rounds) => HttpResponse::Ok().json(rounds),
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

    let tid = item_id.into_inner();
    match (
        models::team::read_all_teams_of_tournament(&mut conn, tid, &params),
        models::team::count_by_tournament(&mut conn, tid),
    ) {
        (Ok(items), Ok(count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}/quizzers")]
async fn read_quizzers(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let tid = item_id.into_inner();
    match models::team::read_all_quizzers_of_tournament(&mut conn, tid) {
        Ok(items) => {
            let count = items.len() as i64;
            HttpResponse::Ok().json(PagedResponse { count, items })
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}/games")]
async fn read_games(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let tid = tour_id.into_inner();
    match (
        models::game::read_all_games_of_tournament(&mut conn, tid, &params),
        models::game::count_by_tournament(&mut conn, tid),
    ) {
        (Ok(items), Ok(count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}/tournamentgroups")]
async fn read_tournamentgroups(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::tournamentgroup::read_all_tournamentgroups_of_tournament(&mut conn, tour_id.into_inner(), &params) {
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
    Json(payload): Json<NewTournamentPayload>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");

    tracing::debug!("Line {}, Tournament model create: {:?}", line!(), payload);
    
    // log this api call
    models::apicalllog::create(&mut db, &req);
    
    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish())
    };

    tracing::debug!("Line {}, User Context: {:?}", line!(), user_ctx);
    
    let tour_create_permission = format!["{}:{}", AppResource::Tournament.as_str(), AppAction::Create.as_str()];
    if !user_ctx.permissions.contains(&tour_create_permission) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let item = NewTournament {
        organization: payload.organization,
        tname: payload.tname,
        breadcrumb: payload.breadcrumb,
        fromdate: payload.fromdate,
        todate: payload.todate,
        venue: payload.venue,
        city: payload.city,
        region: payload.region,
        country: payload.country,
        contact: payload.contact,
        contactemail: payload.contactemail,
        shortinfo: payload.shortinfo,
        info: payload.info,
        owner_id: user_ctx.user_id,
        creator_id: user_ctx.user_id,
    };

    let result : QueryResult<Tournament> = models::tournament::create(&mut db, &item);

    let response: EntityResponse<Tournament> = process_response(result, "post");
    
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

#[post("/{tour_id}/admins")]
async fn add_admin(
    db: Data<Database>,
    path_ids: Path<Uuid>,
    Json(item): Json<NewTournamentAdmin>,
    req: HttpRequest  
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut db, &req);

    tracing::debug!("{} Tournament model create {:?}", line!(), item);

    let item_to_be_created = NewTournamentAdmin {
        tournamentid: path_ids.into_inner(),
        ..item
    };
    
    let result : QueryResult<TournamentAdmin> = models::tournament_admin::create(&mut db, &item_to_be_created);

    let response: EntityResponse<TournamentAdmin> = process_response(result, "post");

    if response.code == 201 {
        if let Ok(role) = models::role::read_by_name(&mut db, AppRole::TournamentAdmin.as_str()) {
            let existing_roles = models::users_roles::read_all_for_user(&mut db, item_to_be_created.adminid)
                .unwrap_or_default();
            if !existing_roles.iter().any(|ur| ur.role_id == role.id) {
                let _ = models::users_roles::create(&mut db, NewUsersRole {
                    user_id: item_to_be_created.adminid,
                    role_id: role.id,
                });
            }
        }
    }

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
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    tracing::debug!("{} Tournement model update {:?} {:?}", line!(), item_id, item); 

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish())
    };
    let tour_update_permission = format!["{}:{}", AppResource::Tournament.as_str(), AppAction::Update.as_str()];
    let resource_name = "tournament";

    let tournament = match models::tournament::read(&mut db, *item_id) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: tournament
    };
    if is_rbac_and_abac_authorized(&policy_ctx, tour_update_permission.as_str(), resource_name).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let result = models::tournament::update(&mut db, item_id.into_inner(), &item);

    let response = process_response(result, "put");
    
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response))
    }
}

#[put("/{tour_id}/admins/{user_id}")]
async fn update_admin(
    db: Data<Database>,
    item_id: Path<(Uuid,Uuid)>,
    Json(item): Json<TournamentAdminChangeset>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    tracing::debug!("{} Tournement model update {:?} {:?}", line!(), item_id, item); 

    let tour_id = item_id.0;
    let admin_id = item_id.1;
    let result = models::tournament_admin::update(&mut db, tour_id, admin_id, &item);

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
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

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
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

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
        .service(read_today_max_100)
        .service(read)
        .service(read_rooms)
        .service(read_rounds)
        .service(read_divisions)
        .service(read_teams)
        .service(read_quizzers)
        .service(read_games)
        .service(read_admins)
        .service(read_tournamentgroups)
        .service(read_equipmentregistrations)
        .service(create)
        .service(add_admin)
        .service(update)
        .service(update_admin)
        .service(destroy)
        .service(remove_admin);
}
