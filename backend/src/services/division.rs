use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, Result, delete, get, post, put, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::{auth::{is_rbac_and_abac_authorized, policies::{division::DivisionPolicyResource, round::RoundPolicyResource, PolicyContext, UserContext}}, models::{self, division::{Division, DivisionChangeset, NewDivision}, permission::{AppAction, AppResource, AppSpecialPermission}, round::{QuestionTypesSaveError, RoundQuestionTypesUpdate}}, services::common::{EntityResponse, PagedResponse, process_response}};
use crate::models::common::PaginationParams;
use crate::database::Database;
use utoipa::OpenApi;
use diesel::QueryResult;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(paths(index))]
pub struct DivisionDoc;

#[utoipa::path(
        get,
        path = "/divisions",
        responses(
            (status = 200, description = "Divisions found successfully", body = Division),
            (status = 404, description = "Division not found")
        ),
        params(
            ("page" = Option<u64>, Query, description = "Page to read"),
            ("page_size" = Option<u64>, Query, description = "How many Divisions to return")
        )
    )
]
#[get("")]
async fn index(
    db: Data<Database>,
    Query(info): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match (models::division::read_all(&mut db, &info), models::division::count(&mut db)) {
        (Ok(items), Ok(count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::division::read(&mut conn, item_id.into_inner()) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/rounds")]
async fn read_rounds(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::round::read_all_rounds_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/pool-brackets")]
async fn read_pool_brackets(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::pool_bracket::read_all_of_division(&mut conn, item_id.into_inner()) {
        Ok(brackets) => HttpResponse::Ok().json(brackets),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// Query params for the pool-bracket rows endpoint: pagination plus the `type` filter
/// (e.g. "pool" or "bracket").
#[derive(serde::Deserialize)]
struct PoolBracketRowsParams {
    page: i64,
    page_size: i64,
    #[serde(rename = "type")]
    type_: String,
}

/// Returns fully-formed pool-bracket data-table rows (bracket + division name +
/// last-modified user name) for the division, filtered by `type`, in a single paginated call.
#[get("/{id}/pool-bracket-rows")]
async fn read_pool_bracket_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PoolBracketRowsParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let pagination = PaginationParams { page: params.page, page_size: params.page_size };
    match models::pool_bracket::read_pool_bracket_rows_of_division(&mut conn, item_id.into_inner(), &params.type_, &pagination) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns fully-formed poolbracketgroup data-table rows (group + division name + last-modified
/// user name) for the division in a single paginated call.
#[get("/{id}/poolbracketgroup-rows")]
async fn read_poolbracketgroup_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::poolbracketgroup::read_poolbracketgroup_rows_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// The division's poolbracketgroups (each groups the pools that run concurrently for team placement).
#[get("/{id}/poolbracketgroups")]
async fn read_poolbracketgroups(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::poolbracketgroup::read_all_of_division(&mut conn, item_id.into_inner()) {
        Ok(groups) => HttpResponse::Ok().json(groups),
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

    match models::team::read_all_teams_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// Returns fully-formed game data-table rows (game + division/room/team names, start time, and
/// room sequence number) for the division in a single paginated call.
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

    match models::game::read_game_rows_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns fully-formed round data-table rows (round + division name) for the division in a
/// single paginated call, so the rounds table needs only one request per page.
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

    match models::round::read_round_rows_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns fully-formed team data-table rows (team + division name + coach name) for the
/// division in a single call, so the teams table needs only one request.
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

    match models::team::read_team_rows_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Returns fully-formed quizzer data-table rows (user fields + divisions + teams) for the
/// division in a single call, so the quizzers table needs only one request.
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

    match models::team::read_quizzer_rows_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}/games")]
async fn read_games(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::game::read_all_games_of_division(&mut conn, item_id.into_inner(), &params) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(mut item): Json<NewDivision>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, item.tid) {
        Ok(t) => t,
        Err(_) => {
            println!("Could not find Tournament by ID={}", &item.tid);
            return Ok(HttpResponse::UnprocessableEntity().json(json!({
                "error": format!("Tournament with ID {} does not exist", item.tid)
            })));
        }
    };
    // Captured before `tournament` is moved into the policy context below; used to schedule the
    // default Round 1 at 8am on the tournament's start date.
    let tournament_start_date = tournament.fromdate;

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: DivisionPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let div_create_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Create.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &div_create_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    if !models::tournament::exists(&mut conn, item.tid) {
        println!("Could not find Tournament by ID={}", &item.tid);
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Tournament with ID {} does not exist", item.tid)
        })));
    }

    // Record who created/last-modified this row (server-derived, never client-supplied).
    let creator_id = user_ctx.user_id;
    item.last_modified_user = creator_id;

    tracing::debug!("{} Division model create {:?}", line!(), item);

    let result: QueryResult<Division> = models::division::create(&mut conn, &item);

    // On successful creation, also create the division's own stats group (division_id = the new
    // division). From then on it mirrors the division's games automatically: every game created in
    // or moved into the division joins it (see `statsgroup::sync_division_statsgroup`).
    if let Ok(ref division) = result {
        match models::statsgroup::StatsGroupBuilder::new_default(&division.dname, division.tid)
            .set_division_id(Some(division.did))
            .build_and_insert(&mut conn)
        {
            Ok(_) => {
                if let Err(e) = models::statsgroup::sync_division_statsgroup(&mut conn, division.did) {
                    tracing::error!("{} Failed to add division {}'s games to its statsgroup: {:?}", line!(), division.did, e);
                }
            }
            Err(e) => tracing::error!("{} Failed to create parallel statsgroup for division {}: {:?}", line!(), division.did, e),
        }

        // Seed the division with a starting structure: a "Round 1" scheduled for 8am on the
        // tournament's start date, plus a "Pool A" pool bracket.
        {
            use chrono::{TimeZone, Utc};
            let round_start = Utc.from_utc_datetime(&tournament_start_date.and_hms_opt(8, 0, 0).unwrap());
            if let Err(e) = models::round::RoundBuilder::new_default(division.did)
                .set_name("Round 1")
                .set_scheduled_start_time(round_start)
                .set_last_modified_user(creator_id)
                .build_and_insert(&mut conn)
            {
                tracing::error!("{} Failed to create default Round 1 for division {}: {:?}", line!(), division.did, e);
            }
        }

        // A "Group 1" poolbracketgroup holds the starting "Pool A" (team placement is per-group).
        match models::poolbracketgroup::PoolBracketGroupBuilder::new(division.did)
            .set_name("Group 1")
            .set_creator_userid(creator_id)
            .build_and_insert(&mut conn)
        {
            Ok(group) => {
                if let Err(e) = models::pool_bracket::PoolBracketBuilder::new(division.did)
                    .set_name("Pool A")
                    .set_type("pool")
                    .set_poolbracketgroupid(group.poolbracketgroupid)
                    .set_creator_userid(creator_id)
                    .build_and_insert(&mut conn)
                {
                    tracing::error!("{} Failed to create default Pool A for division {}: {:?}", line!(), division.did, e);
                }
            }
            Err(e) => tracing::error!("{} Failed to create default Group 1 for division {}: {:?}", line!(), division.did, e),
        }
    }

    let response: EntityResponse<Division> = process_response(result, "post");
    
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
    Json(item): Json<DivisionChangeset>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let division_id = item_id.into_inner();

    let division = match models::division::read(&mut conn, division_id) {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, division.tid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: DivisionPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let div_update_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Update.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &div_update_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} Division model update {:?} {:?}", line!(), division_id, item);

    let result = models::division::update(&mut conn, division_id, &item, user_ctx.user_id);

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
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let division_id = item_id.into_inner();

    let division = match models::division::read(&mut conn, division_id) {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, division.tid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: DivisionPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let div_delete_permission = format!("{}:{}", AppResource::Division.as_str(), AppAction::Delete.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &div_delete_permission, AppResource::Division.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} Division model delete {:?}", line!(), division_id);

    let result = models::division::delete(&mut conn, division_id);

    if result.is_ok() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}

// ─── Public team standings (LIVE Stats) ───────────────────────────────────────

/// The division's team standings: the rows of its own stats group, finished games only, with
/// the tie-breaker explanations.
///
/// Public by design, like Games in Progress -- it is what a spectator screen points at, and it
/// shows only what is already on a scoreboard. The full admin Stats Groups panel stays restricted.
/// Every round of the division with its question types, in schedule order, and whether the
/// division's question types are complete (every round has a type for every question).
#[get("/{id}/round-question-types")]
async fn read_round_question_types(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let division_id = item_id.into_inner();
    if models::division::read(&mut conn, division_id).is_err() {
        return HttpResponse::NotFound().finish();
    }

    match models::round::read_question_types_of_division(&mut conn, division_id) {
        Ok(question_types) => HttpResponse::Ok().json(question_types),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Importing question types takes `round:upload_question_types` on top of being allowed to update
/// the division's rounds (`round:update`, as the tournament's owner or one of its admins).
fn can_upload_question_types(
    conn: &mut crate::database::Connection,
    division: &Division,
    user_ctx: &UserContext,
) -> Result<bool, diesel::result::Error> {
    let tournament = models::tournament::read(conn, division.tid)?;
    let user_is_admin = models::tournament_admin::is_admin(conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: RoundPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let round_update_permission = format!("{}:{}", AppResource::Round.as_str(), AppAction::Update.as_str());
    Ok(is_rbac_and_abac_authorized(&policy_ctx, &round_update_permission, AppResource::Round.as_str()).is_ok()
        && user_ctx.can(AppSpecialPermission::RoundUploadQuestionTypes.as_str()))
}

#[derive(serde::Deserialize)]
struct QuestionSetFileUpload {
    /// The uploaded file's name; its extension picks the format.
    filename: String,
    /// The file's text.
    content: String,
}

/// Reads the question types out of an uploaded question set file (.rtf or .csv) without saving
/// anything, so the user can review them and choose which rounds they go to.
#[post("/{id}/round-question-types/parse")]
async fn parse_round_question_types(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Json(upload): Json<QuestionSetFileUpload>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let division = match models::division::read(&mut conn, item_id.into_inner()) {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    match can_upload_question_types(&mut conn, &division, user_ctx) {
        Ok(true) => {}
        Ok(false) => return Ok(HttpResponse::Unauthorized().finish()),
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    }

    Ok(HttpResponse::Ok().json(models::question_type::parse_question_set_file(&upload.filename, &upload.content)))
}

/// Saves question types for several of the division's rounds at once. Either every round is
/// saved or, if any round's question types are invalid, none is.
#[put("/{id}/round-question-types")]
async fn update_round_question_types(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Json(updates): Json<Vec<RoundQuestionTypesUpdate>>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let division = match models::division::read(&mut conn, item_id.into_inner()) {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    match can_upload_question_types(&mut conn, &division, user_ctx) {
        Ok(true) => {}
        Ok(false) => return Ok(HttpResponse::Unauthorized().finish()),
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    }

    match models::round::update_question_types_of_division(&mut conn, division.did, &updates, user_ctx.user_id) {
        Ok(rounds) => {
            crate::services::round::question_types_changed(&req, &mut conn, division.did).await;
            Ok(HttpResponse::Ok().json(rounds))
        }
        Err(QuestionTypesSaveError::Invalid(round_errors)) => Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": "Some rounds' question types are not valid; nothing was saved.",
            "rounds": round_errors,
        }))),
        Err(QuestionTypesSaveError::Database(_)) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[get("/{id}/teamstats")]
async fn read_team_stats(
    db: Data<Database>,
    cache: Data<crate::cache::Cache>,
    division_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    // Public for unauthenticated callers; an authenticated caller needs statsgroup:read.
    if !crate::services::common::authorized_to_read(&req, AppResource::StatsGroup) {
        return HttpResponse::Forbidden().finish();
    }

    let did = division_id.into_inner();
    if models::division::read(&mut conn, did).is_err() {
        return HttpResponse::NotFound().finish();
    }
    match models::statsgroup::read_team_stats_of_division(&cache, &mut conn, did).await {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Live division standings: the same rows as `/teamstats`, pushed as Server-Sent Events whenever
/// a game event lands in the division. See `crate::live`.
#[get("/{id}/teamstats/stream")]
async fn stream_team_stats(
    db: Data<Database>,
    division_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    if !crate::services::common::authorized_to_read(&req, AppResource::StatsGroup) {
        return HttpResponse::Forbidden().finish();
    }

    let did = division_id.into_inner();
    if models::division::read(&mut conn, did).is_err() {
        return HttpResponse::NotFound().finish();
    }
    let Some(hub) = req.app_data::<Data<crate::live::LiveHub>>() else {
        return HttpResponse::ServiceUnavailable().finish();
    };
    crate::live::stream_response(hub, crate::live::Topic::TeamStats(did), None)
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(read)
        .service(read_team_stats)
        .service(stream_team_stats)
        .service(read_rounds)
        .service(read_pool_brackets)
        .service(read_pool_bracket_rows)
        .service(read_poolbracketgroups)
        .service(read_poolbracketgroup_rows)
        .service(read_teams)
        .service(read_team_rows)
        .service(read_quizzer_rows)
        .service(read_round_rows)
        .service(read_round_question_types)
        .service(parse_round_question_types)
        .service(update_round_question_types)
        .service(read_game_rows)
        .service(read_games)
        .service(create)
        .service(update)
        .service(destroy);
}
