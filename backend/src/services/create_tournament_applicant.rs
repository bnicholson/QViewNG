use actix_web::{delete, Error, get, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
use crate::database::Database;
use crate::models::{self, common::PaginationParams, create_tournament_applicant::{NewCreateTournamentApplicant, CreateTournamentApplicant, CreateTournamentApplicantChangeset}, role::AppRole, users_roles::NewUsersRole};
use crate::services::common::{EntityResponse, PagedResponse, process_response};
use diesel::QueryResult;
use uuid::Uuid;

#[get("")]
async fn index(
    db: Data<Database>,
    Query(url_params): Query<PaginationParams>,
    req: HttpRequest,
) -> HttpResponse {
    let mut conn = db.get_connection().expect("Failed to get connection");

    models::apicalllog::create(&mut conn, &req);

    match (
        models::create_tournament_applicant::read_all(&mut conn, &url_params),
        models::create_tournament_applicant::count(&mut conn),
    ) {
        (Ok(items), Ok(count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let mut conn = db.get_connection().expect("Failed to get connection");

    models::apicalllog::create(&mut conn, &req);

    match models::create_tournament_applicant::read(&mut conn, item_id.into_inner()) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewCreateTournamentApplicant>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut conn = db.get_connection().expect("Failed to get connection");

    models::apicalllog::create(&mut conn, &req);

    let result: QueryResult<CreateTournamentApplicant> =
        models::create_tournament_applicant::create(&mut conn, &item);

    let response: EntityResponse<CreateTournamentApplicant> = process_response(result, "post");

    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response)),
    }
}

#[put("/{id}")]
async fn update(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Json(item): Json<CreateTournamentApplicantChangeset>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut conn = db.get_connection().expect("Failed to get connection");

    models::apicalllog::create(&mut conn, &req);

    let result = models::create_tournament_applicant::update(&mut conn, item_id.into_inner(), &item);

    if item.status.as_deref() == Some("approved") {
        if let Ok(ref updated) = result {
            if let Ok(role) = models::role::read_by_name(&mut conn, AppRole::TournamentManager.as_str()) {
                let _ = models::users_roles::create(&mut conn, NewUsersRole { user_id: updated.user_id, role_id: role.id });
            }
        }
    }

    let response: EntityResponse<CreateTournamentApplicant> = process_response(result, "put");

    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response)),
    }
}

#[delete("/{id}")]
async fn destroy(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let mut conn = db.get_connection().expect("Failed to get connection");

    models::apicalllog::create(&mut conn, &req);

    let result = models::create_tournament_applicant::delete(&mut conn, item_id.into_inner());

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope
        .service(index)
        .service(read)
        .service(create)
        .service(update)
        .service(destroy)
}
