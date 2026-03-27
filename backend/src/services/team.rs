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

#[get("/{id}/games")]
async fn read_games(
    db: Data<Database>,
    tour_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    match models::game::read_all_games_of_team(&mut db, tour_id.into_inner(), &params) {
        Ok(rounds) => HttpResponse::Ok().json(rounds),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewTeam>,
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

    if !is_super_user && !has_permission && !is_owner && !is_admin {
        return Ok(HttpResponse::Unauthorized().finish());
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
        resource: TeamPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let team_update_permission = format!("{}:{}", AppResource::Team.as_str(), AppAction::Update.as_str());
    if is_rbac_and_abac_authorized(&policy_ctx, &team_update_permission, AppResource::Team.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} Team model update {:?} {:?}", line!(), team_id, item);

    let result = models::team::update(&mut conn, team_id, &item);

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
        resource: TeamPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
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
        .service(read_games)
        .service(create)
        .service(update)
        .service(destroy);
}
