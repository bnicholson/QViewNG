use actix_web::{delete, Error, get, HttpMessage, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use crate::auth::policies::UserContext;
use crate::{database::Database, models::game_statsgroup::{GameStatsGroup, NewGameStatsGroup}};
use crate::models::{self, common::PaginationParams, statsgroup::{NewStatsGroup, StatsGroup, StatsGroupChangeset}};
use crate::services::common::{EntityResponse, PagedResponse, process_response};
use diesel::QueryResult;
use uuid::Uuid;

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct StatsGroupDoc;

/// Ensures the caller may view the stats group's data: allowed only for super users, the
/// owning tournament's owner, and its admins. Returns Err(response) to return directly
/// (404 if the group is unknown, 403 if the caller lacks access).
fn authorize_statsgroup_view(
    conn: &mut crate::database::Connection,
    sg_id: Uuid,
    req: &HttpRequest,
) -> core::result::Result<(), HttpResponse> {
    let tournament_id = match models::statsgroup::read(conn, sg_id) {
        Ok(sg) => sg.tournament_id,
        Err(_) => return Err(HttpResponse::NotFound().finish()),
    };
    let user_ctx = req.extensions().get::<UserContext>().cloned();
    if crate::auth::can_view_tournament_restricted_section(conn, tournament_id, user_ctx.as_ref()) {
        Ok(())
    } else {
        Err(HttpResponse::Forbidden().finish())
    }
}

#[get("/{id}/games")]
async fn read_games(
    db: Data<Database>,
    sg_id: Path<Uuid>,
    Query(params): Query<PaginationParams>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let sg_id = sg_id.into_inner();
    if let Err(resp) = authorize_statsgroup_view(&mut db, sg_id, &req) {
        return resp;
    }

    match models::game::read_all_games_of_statsgroup(&mut db, sg_id, &params) {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/teamstats")]
async fn read_team_stats(
    db: Data<Database>,
    sg_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let sg_id = sg_id.into_inner();
    if let Err(resp) = authorize_statsgroup_view(&mut db, sg_id, &req) {
        return resp;
    }

    match models::statsgroup::read_team_stats_of_statsgroup(&mut db, sg_id) {
        Ok(team_stats) => HttpResponse::Ok().json(team_stats),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/individualstats")]
async fn read_individual_stats(
    db: Data<Database>,
    sg_id: Path<Uuid>,
    req: HttpRequest
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    // log this api call
    models::apicalllog::create(&mut db, &req);

    let sg_id = sg_id.into_inner();
    if let Err(resp) = authorize_statsgroup_view(&mut db, sg_id, &req) {
        return resp;
    }

    match models::statsgroup::read_individual_stats_of_statsgroup(&mut db, sg_id) {
        Ok(individual_stats) => HttpResponse::Ok().json(individual_stats),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(read_games)
        .service(read_team_stats)
        .service(read_individual_stats)
}
