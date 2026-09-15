use actix_web::{delete, Error, get, HttpMessage, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::{database::Database, models::division::Division};
use crate::auth::{is_rbac_and_abac_authorized, policies::{team::TeamPolicyResource, PolicyContext, UserContext}};
use crate::models::{self, common::PaginationParams, permission::{AppAction, AppResource}, role::AppRole, team::{NewTeam, Team, TeamChangeset}};
use crate::schema::divisions::dsl::{divisions as divisions_table};
use crate::services::common::{EntityResponse, PagedResponse, process_response};
// use utoipa::OpenApi;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

/// Builds the user-facing message listing every quizzer already on another team in the division.
/// `conflicts` is `(quizzer_id, other_team_name)` pairs.
fn quizzer_conflicts_message(
    conn: &mut crate::database::Connection,
    division_name: &str,
    conflicts: &[(Uuid, String)],
) -> String {
    let ids: Vec<Uuid> = conflicts.iter().map(|(id, _)| *id).collect();
    let names = models::user::read_display_names(conn, &ids).unwrap_or_default();
    let list = conflicts
        .iter()
        .map(|(id, team_name)| {
            let quizzer_name = names
                .get(id)
                .cloned()
                .filter(|n| !n.trim().is_empty())
                .unwrap_or_else(|| id.to_string());
            format!("{} (team \"{}\")", quizzer_name, team_name)
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "The following quizzers are already registered on other teams in division \"{}\": {}. Quizzers cannot be on multiple teams for the same division.",
        division_name, list
    )
}

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
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut db, &req);
    
    match (models::team::read_all(&mut db, &url_params), models::team::count(&mut db)) {
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
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::team::read(&mut db, item_id.into_inner()) {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// Fully-formed game data-table rows (game + division/room/team names, start time, and room
/// sequence number) for the games this team plays in any position, in a single paginated call.
#[get("/{id}/game-rows")]
async fn read_game_rows(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::game::read_game_rows_of_team(&mut db, item_id.into_inner(), &params) {
        Ok((items, count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(mut item): Json<NewTeam>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    // log this api call
    models::apicalllog::create(&mut conn, &req);
    
    tracing::debug!("{} Team model create: {:?}", line!(), item);
    
    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    
    tracing::debug!("{} UserCtx: {:?}", line!(), user_ctx);

    let division = match divisions_table
        .find(item.did)
        .get_result::<Division>(&mut conn)
    {
        Ok(d) => d,
        Err(_) => {
            println!("Could not find Division by ID={}", &item.did);
            return Ok(HttpResponse::UnprocessableEntity().json(json!({
                "error": format!("Division with ID {} does not exist", item.did)
            })));
        }
    };

    let tournament = match models::tournament::read(&mut conn, division.tid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let has_permission = user_ctx.permissions.contains(
        &format!("{}:{}", AppResource::Team.as_str(), AppAction::Create.as_str())
    );
    let is_owner = tournament.owner_id == user_ctx.user_id;
    let is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let is_super_user = user_ctx.roles.iter().any(|r| r == AppRole::SuperUser.as_str());
    let is_coach = item.coachid == user_ctx.user_id;

    let is_authorized = is_super_user || has_permission || is_owner || is_admin || is_coach;
    if !is_authorized {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let has_quizzer = item.quizzer_one_id.is_some()
        || item.quizzer_two_id.is_some()
        || item.quizzer_three_id.is_some()
        || item.quizzer_four_id.is_some()
        || item.quizzer_five_id.is_some()
        || item.quizzer_six_id.is_some();
    if !has_quizzer {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": "A team must have at least one quizzer."
        })));
    }

    // A quizzer may be on at most one team within the same division. Collect every conflict so the
    // error can name all quizzers already on another team, not just the first.
    let quizzer_ids: Vec<uuid::Uuid> = [
        item.quizzer_one_id, item.quizzer_two_id, item.quizzer_three_id,
        item.quizzer_four_id, item.quizzer_five_id, item.quizzer_six_id,
    ].into_iter().flatten().collect();
    let mut conflicts: Vec<(uuid::Uuid, String)> = Vec::new();
    for qid in &quizzer_ids {
        if conflicts.iter().any(|(id, _)| id == qid) { continue; }
        if let Ok(Some(existing_team)) = models::team::find_team_in_division_with_quizzer(&mut conn, item.did, *qid, None) {
            conflicts.push((*qid, existing_team.name));
        }
    }
    if !conflicts.is_empty() {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": quizzer_conflicts_message(&mut conn, &division.dname, &conflicts)
        })));
    }

    item.last_modified_user = user_ctx.user_id;
    item.creator_id = user_ctx.user_id;
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

    let team_id = item_id.into_inner();

    let team = match models::team::read(&mut conn, team_id) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let division = match divisions_table
        .find(team.did)
        .get_result::<Division>(&mut conn)
    {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, division.tid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let is_coach = team.coachid == user_ctx.user_id;
    let is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: TeamPolicyResource { tournament, user_is_tournament_admin: is_admin, user_is_team_coach: false },
    };
    let team_update_permission = format!("{}:{}", AppResource::Team.as_str(), AppAction::Update.as_str());
    
    let is_authorized = is_coach || is_rbac_and_abac_authorized(&policy_ctx, &team_update_permission, AppResource::Team.as_str()).is_ok();
    if !is_authorized {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let effective = |existing: Option<Uuid>, change: Option<Option<Uuid>>| -> Option<Uuid> {
        match change { Some(v) => v, None => existing }
    };
    let has_quizzer = effective(team.quizzer_one_id, item.quizzer_one_id).is_some()
        || effective(team.quizzer_two_id, item.quizzer_two_id).is_some()
        || effective(team.quizzer_three_id, item.quizzer_three_id).is_some()
        || effective(team.quizzer_four_id, item.quizzer_four_id).is_some()
        || effective(team.quizzer_five_id, item.quizzer_five_id).is_some()
        || effective(team.quizzer_six_id, item.quizzer_six_id).is_some();
    if !has_quizzer {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": "A team must have at least one quizzer."
        })));
    }

    // A quizzer may be on at most one team within the same division (excluding this team itself).
    // Collect every conflict so the error can name all quizzers already on another team.
    let effective_quizzers: Vec<uuid::Uuid> = [
        effective(team.quizzer_one_id, item.quizzer_one_id),
        effective(team.quizzer_two_id, item.quizzer_two_id),
        effective(team.quizzer_three_id, item.quizzer_three_id),
        effective(team.quizzer_four_id, item.quizzer_four_id),
        effective(team.quizzer_five_id, item.quizzer_five_id),
        effective(team.quizzer_six_id, item.quizzer_six_id),
    ].into_iter().flatten().collect();
    let mut conflicts: Vec<(uuid::Uuid, String)> = Vec::new();
    for qid in &effective_quizzers {
        if conflicts.iter().any(|(id, _)| id == qid) { continue; }
        if let Ok(Some(existing_team)) = models::team::find_team_in_division_with_quizzer(&mut conn, team.did, *qid, Some(team_id)) {
            conflicts.push((*qid, existing_team.name));
        }
    }
    if !conflicts.is_empty() {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": quizzer_conflicts_message(&mut conn, &division.dname, &conflicts)
        })));
    }

    tracing::debug!("{} Team model update {:?} {:?}", line!(), team_id, item);

    let result = models::team::update(&mut conn, team_id, &item, user_ctx.user_id);

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

    let team_id = item_id.into_inner();

    let team = match models::team::read(&mut conn, team_id) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let division = match divisions_table
        .find(team.did)
        .get_result::<Division>(&mut conn)
    {
        Ok(d) => d,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, division.tid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: TeamPolicyResource { tournament, user_is_tournament_admin: user_is_admin, user_is_team_coach: false },
    };
    let team_delete_permission = format!("{}:{}", AppResource::Team.as_str(), AppAction::Delete.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &team_delete_permission, AppResource::Team.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} Team model delete {:?}", line!(), team_id);

    let result = models::team::delete(&mut conn, team_id);

    if result.is_ok() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(read)
        .service(read_game_rows)
        .service(create)
        .service(update)
        .service(destroy);
}
