use actix_web::{delete, Error, get, HttpMessage, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use serde_json::json;
use crate::{auth::{is_abac_authorized, policies::{game::GamePolicyResource, PolicyContext, UserContext}}, models::{self, common::PaginationParams, game::{NewGame, Game, GameChangeset}, permission::{AppAction, AppResource}}};
use crate::database::Database;
use crate::services::common::{EntityResponse, PagedResponse, process_response};
// use utoipa::OpenApi;
use diesel::QueryResult;
use uuid::Uuid;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct GameDoc;

// #[utoipa::path(
//         get,
//         path = "/games",
//         responses(
//             (status = 200, description = "Games found successfully", body = Game),
//             (status = 404, description = "Game not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many Games to return")
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
    
    match (models::game::read_all(&mut db, &url_params), models::game::count(&mut db)) {
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

    match models::game::read(&mut conn, item_id.into_inner()) {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/statsgroups")]
async fn read_statsgroups(
    db: Data<Database>,
    game_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::statsgroup::read_all_statsgroups_of_game(&mut conn, game_id.into_inner(), &params) {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/gameevents")]
async fn read_gameevents(
    db: Data<Database>,
    game_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    match models::gameevent::read_all_gameevents_of_game(&mut conn, game_id.into_inner(), &params) {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewGame>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection().expect("Failed to get connection");

    tracing::debug!("{} Game model create {:?}", line!(), item);

    // log this api call
    models::apicalllog::create(&mut conn, &req);

    let extensions = req.extensions();
    let user_ctx = match extensions.get::<UserContext>() {
        Some(u_ctx) => u_ctx,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let tournament_id = match item.tournamentid {
        Some(tid) => tid,
        None => {
            let round = match models::round::read(&mut conn, item.roundid) {
                Ok(r) => r,
                Err(_) => return Ok(HttpResponse::UnprocessableEntity().json(json!({
                    "error": format!("Round with ID {} does not exist", item.roundid)
                }))),
            };
            let division = match models::division::read(&mut conn, round.did) {
                Ok(d) => d,
                Err(_) => return Ok(HttpResponse::UnprocessableEntity().json(json!({
                    "error": format!("Division with ID {} does not exist", round.did)
                }))),
            };
            division.tid
        }
    };

    let tournament = match models::tournament::read(&mut conn, tournament_id) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Tournament with ID {} does not exist", tournament_id)
        }))),
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: GamePolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let game_create_permission = format!("{}:{}", AppResource::Game.as_str(), AppAction::Create.as_str());
    if is_abac_authorized(&policy_ctx, &game_create_permission, AppResource::Game.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let result: QueryResult<Game> = models::game::create(&mut conn, &item);

    let response: EntityResponse<Game> = process_response(result, "post");
    
    match response.code {
        400 => Ok(HttpResponse::BadRequest().json(response)),
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
    Json(item): Json<GameChangeset>,
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

    let game_id = item_id.into_inner();

    let game = match models::game::read(&mut conn, game_id) {
        Ok(g) => g,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, game.tournamentid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: GamePolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let game_update_permission = format!("{}:{}", AppResource::Game.as_str(), AppAction::Update.as_str());
    if is_abac_authorized(&policy_ctx, &game_update_permission, AppResource::Game.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} Game model update {:?} {:?}", line!(), game_id, item);

    let result = models::game::update(&mut conn, game_id, &item);

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

    let game_id = item_id.into_inner();

    let game = match models::game::read(&mut conn, game_id) {
        Ok(g) => g,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let tournament = match models::tournament::read(&mut conn, game.tournamentid) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let user_is_admin = models::tournament_admin::is_admin(&mut conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: GamePolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let game_delete_permission = format!("{}:{}", AppResource::Game.as_str(), AppAction::Delete.as_str());
    if is_abac_authorized(&policy_ctx, &game_delete_permission, AppResource::Game.as_str()).is_err() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    tracing::debug!("{} Game model delete {:?}", line!(), game_id);

    let result = models::game::delete(&mut conn, game_id);

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
        .service(read_statsgroups)
        .service(read_gameevents)
        .service(create)
        .service(update)
        .service(destroy);
}
