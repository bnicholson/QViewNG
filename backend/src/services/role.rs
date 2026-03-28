use actix_web::{delete, get, post, put, Error, HttpRequest, HttpResponse, Result,
    web::{Data, Json, Path}};
use crate::{
    database::Database,
    models::{self, role::{NewRole, RoleChangeset}},
};
use crate::services::common::{EntityResponse, PagedResponse, process_response};
use diesel::QueryResult;
use uuid::Uuid;

#[get("")]
async fn index(db: Data<Database>, req: HttpRequest) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut db, &req);
    match (models::role::read_all(&mut db), models::role::count(&mut db)) {
        (Ok(items), Ok(count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn read(db: Data<Database>, item_id: Path<Uuid>, req: HttpRequest) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    match models::role::read(&mut db, item_id.into_inner()) {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/{id}/permissions")]
async fn read_permissions(db: Data<Database>, item_id: Path<Uuid>, req: HttpRequest) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    match models::role_permission::read_all_for_role(&mut db, item_id.into_inner()) {
        Ok(perms) => HttpResponse::Ok().json(perms),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewRole>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut db, &req);
    let result: QueryResult<_> = models::role::create(&mut db, item);
    let response: EntityResponse<_> = process_response(result, "post");
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
    Json(item): Json<RoleChangeset>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    let result = models::role::update(&mut db, item_id.into_inner(), &item);
    let response = process_response(result, "put");
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response)),
    }
}

#[delete("/{id}")]
async fn destroy(db: Data<Database>, item_id: Path<Uuid>, req: HttpRequest) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    if models::role::delete(&mut db, item_id.into_inner()).is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

/// POST /roles/{role_id}/permissions/{permission_id}
#[post("/{role_id}/permissions/{permission_id}")]
async fn add_permission(
    db: Data<Database>,
    path: Path<(Uuid, Uuid)>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut db, &req);
    let (role_id, permission_id) = path.into_inner();
    let result: QueryResult<_> = models::role_permission::create(
        &mut db,
        models::role_permission::NewRolePermission { role_id, permission_id },
    );
    let response: EntityResponse<_> = process_response(result, "post");
    match response.code {
        409 => Ok(HttpResponse::Conflict().json(response)),
        201 => Ok(HttpResponse::Created().json(response)),
        200 => Ok(HttpResponse::Ok().json(response)),
        _ => Ok(HttpResponse::InternalServerError().json(response)),
    }
}

/// DELETE /roles/{role_id}/permissions/{permission_id}
#[delete("/{role_id}/permissions/{permission_id}")]
async fn remove_permission(
    db: Data<Database>,
    path: Path<(Uuid, Uuid)>,
    req: HttpRequest,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    let (role_id, permission_id) = path.into_inner();
    if models::role_permission::delete(&mut db, role_id, permission_id).is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope
        .service(index)
        .service(read)
        .service(read_permissions)
        .service(create)
        .service(update)
        .service(destroy)
        .service(add_permission)
        .service(remove_permission)
}
