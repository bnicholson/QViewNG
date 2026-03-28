use actix_web::{delete, get, post, Error, HttpRequest, HttpResponse, Result,
    web::{Data, Path}};
use crate::{
    database::Database,
    models::{self, users_roles::NewUsersRole},
};
use crate::services::common::{EntityResponse, process_response};
use diesel::QueryResult;
use uuid::Uuid;

/// GET /usersroles/users/{user_id} — list all roles assigned to a user
#[get("/users/{user_id}")]
async fn read_for_user(
    db: Data<Database>,
    user_id: Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    match models::users_roles::read_all_for_user(&mut db, user_id.into_inner()) {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// GET /usersroles/roles/{role_id} — list all users assigned to a role
#[get("/roles/{role_id}")]
async fn read_for_role(
    db: Data<Database>,
    role_id: Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    match models::users_roles::read_all_for_role(&mut db, role_id.into_inner()) {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// POST /usersroles/users/{user_id}/roles/{role_id} — assign a role to a user
#[post("/users/{user_id}/roles/{role_id}")]
async fn assign(
    db: Data<Database>,
    path: Path<(Uuid, Uuid)>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut db, &req);
    let (user_id, role_id) = path.into_inner();
    let result: QueryResult<_> = models::users_roles::create(
        &mut db,
        NewUsersRole { user_id, role_id },
    );
    let response: EntityResponse<_> = process_response(result, "post");
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response)),
    }
}

/// DELETE /usersroles/users/{user_id}/roles/{role_id} — revoke a role from a user
#[delete("/users/{user_id}/roles/{role_id}")]
async fn revoke(
    db: Data<Database>,
    path: Path<(Uuid, Uuid)>,
    req: HttpRequest,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    let (user_id, role_id) = path.into_inner();
    if models::users_roles::delete(&mut db, user_id, role_id).is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

/// DELETE /usersroles/users/{user_id} — revoke all roles from a user
#[delete("/users/{user_id}")]
async fn revoke_all(
    db: Data<Database>,
    user_id: Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    if models::users_roles::delete_all_for_user(&mut db, user_id.into_inner()).is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope
        .service(read_for_user)
        .service(read_for_role)
        .service(assign)
        .service(revoke_all)
        .service(revoke)
}
