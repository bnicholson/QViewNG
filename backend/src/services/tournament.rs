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
    // create,
    // update,
    destroy
))]
pub struct TournamentDoc;

#[derive(serde::Deserialize)]
struct VisibilityQuery {
    visibility: Option<String>,
}

#[get("filter")]
async fn get_between_dates(
    db: Data<Database>,
    req: HttpRequest,
    Query(dinfo): Query<SearchDateParams>,
    Query(vis): Query<VisibilityQuery>,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let visibility = models::tournament::VisibilityFilter::from_param(vis.visibility.as_deref());
    let result = models::tournament::read_between_dates(&mut db, dinfo.from_date, dinfo.to_date, visibility);

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

    let tournament = match models::tournament::read(&mut db, item_id.into_inner()) {
        Ok(t) => t,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let can_view_pairing_code = req
        .extensions()
        .get::<UserContext>()
        .map(|ctx| ctx.roles.iter().any(|r| {
            r == AppRole::SuperUser.as_str()
                || r == AppRole::TournamentManager.as_str()
                || r == AppRole::TournamentAdmin.as_str()
        }))
        .unwrap_or(false);

    let mut body = serde_json::to_value(&tournament).unwrap();
    if !can_view_pairing_code {
        if let Some(obj) = body.as_object_mut() {
            obj.remove("pairing_code");
        }
    }

    HttpResponse::Ok().json(body)
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

/// Enriched, paginated division rows (division + last-modified user name) — one call per page.
#[get("/{id}/division-rows")]
async fn read_division_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    match models::division::read_division_rows_of_tournament(&mut db, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Enriched, paginated room rows (room + last-modified user name) — one call per page.
#[get("/{id}/room-rows")]
async fn read_room_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    match models::room::read_room_rows_of_tournament(&mut db, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}/statsgroups")]
async fn read_statsgroups(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let tid = item_id.into_inner();

    // Restricted to super users, the tournament owner, and tournament admins.
    let user_ctx = req.extensions().get::<UserContext>().cloned();
    if !crate::auth::can_view_tournament_restricted_section(&mut db, tid, user_ctx.as_ref()) {
        return HttpResponse::Forbidden().finish();
    }

    match models::statsgroup::read_all_statsgroups_of_tournament(&mut db, tid, &params) {
        Ok(statsgroups) => HttpResponse::Ok().json(statsgroups),
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

/// Returns fully-formed game data-table rows (game + division/room/team names, start time, and
/// room sequence number) for the whole tournament in a single paginated call.
#[get("/{id}/game-rows")]
async fn read_game_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::game::read_game_rows_of_tournament(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns fully-formed round data-table rows (round + division name) for the whole tournament
/// in a single paginated call, so the rounds table needs only one request per page.
#[get("/{id}/round-rows")]
async fn read_round_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::round::read_round_rows_of_tournament(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns fully-formed division-session rows (session + division name + last-modified user name)
/// across every division in the tournament, in a single paginated call.
#[get("/{id}/session-rows")]
async fn read_session_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::division_session::read_session_rows_of_tournament(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns fully-formed team data-table rows (team + division name + coach name) for the
/// whole tournament in a single call, so the teams table needs only one request.
#[get("/{id}/team-rows")]
async fn read_team_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::team::read_team_rows_of_tournament(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns fully-formed quizzer data-table rows (user fields + divisions + teams) for the
/// whole tournament in a single call, so the quizzers table needs only one request.
#[get("/{id}/quizzer-rows")]
async fn read_quizzer_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let tid = item_id.into_inner();
    match models::team::read_quizzer_rows_of_tournament(&mut conn, tid, &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
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

#[get("/{id}/roommonitor")]
async fn read_room_monitor(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let tid = tour_id.into_inner();

    // Restricted to super users, the tournament owner, and tournament admins.
    let user_ctx = req.extensions().get::<UserContext>().cloned();
    if !crate::auth::can_view_tournament_restricted_section(&mut conn, tid, user_ctx.as_ref()) {
        return HttpResponse::Forbidden().finish();
    }

    match models::room::read_room_monitor_of_tournament(&mut conn, tid) {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}/gamestatuses")]
async fn read_game_statuses(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::game::read_game_statuses_of_tournament(&mut conn, tour_id.into_inner()) {
        Ok(statuses) => HttpResponse::Ok().json(statuses),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Debug, Deserialize)]
struct GameEventImportRequest {
    csv: String,
}

#[post("/{id}/gameevents/import/preview")]
async fn import_gameevents_preview(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Json(body): Json<GameEventImportRequest>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    // Dry run: parse, match and validate only — no writes.
    let preview = models::gameevent_import::preview(&mut conn, tour_id.into_inner(), &body.csv);
    HttpResponse::Ok().json(preview)
}

#[post("/{id}/gameevents/import/commit")]
async fn import_gameevents_commit(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Json(body): Json<GameEventImportRequest>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::gameevent_import::commit(&mut conn, tour_id.into_inner(), &body.csv) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish(),
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

/// Enriched, paginated tournament-group rows (group + last-modified user name) — one call per page.
#[get("/{id}/tournamentgroup-rows")]
async fn read_tournamentgroup_rows(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);
    match models::tournamentgroup::read_tournamentgroup_rows_of_tournament(&mut conn, tour_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
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

    let pairing_code = format!("{:06}", rand::random_range(0..=999_999u32));

    let item = NewTournament {
        organization: payload.organization,
        tname: payload.tname,
        breadcrumb: payload.breadcrumb,
        fromdate: payload.fromdate,
        todate: payload.todate,
        venue: payload.venue,
        city: payload.city,
        country: payload.country,
        contact: payload.contact,
        contactemail: payload.contactemail,
        shortinfo: payload.shortinfo,
        info: payload.info,
        owner_id: user_ctx.user_id,
        creator_id: user_ctx.user_id,
        pairing_code,
        address_line_1: payload.address_line_1,
        address_line_2: payload.address_line_2,
        state: payload.state,
        zip_code: payload.zip_code,
        is_public: payload.is_public,
        registration_open_date: payload.registration_open_date,
        registration_close_date: payload.registration_close_date,
        last_modified_user: user_ctx.user_id,
        use_team_registration: payload.use_team_registration,
        use_gear_registration: payload.use_gear_registration,
        use_volunteer_registration: payload.use_volunteer_registration,
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

    let modified_by = user_ctx.user_id;
    let result = models::tournament::update(&mut db, item_id.into_inner(), &item, modified_by);

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
        .service(read)
        .service(read_rooms)
        .service(read_division_rows)
        .service(read_room_rows)
        .service(read_rounds)
        .service(read_divisions)
        .service(read_statsgroups)
        .service(read_teams)
        .service(read_quizzers)
        .service(read_quizzer_rows)
        .service(read_team_rows)
        .service(read_round_rows)
        .service(read_session_rows)
        .service(read_game_rows)
        .service(read_games)
        .service(read_game_statuses)
        .service(read_room_monitor)
        .service(import_gameevents_preview)
        .service(import_gameevents_commit)
        .service(read_admins)
        .service(read_tournamentgroups)
        .service(read_tournamentgroup_rows)
        .service(read_equipmentregistrations)
        .service(create)
        .service(add_admin)
        .service(update)
        .service(destroy)
        .service(remove_admin);
}
