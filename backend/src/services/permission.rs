use actix_web::{delete, get, post, put, Error, HttpRequest, HttpResponse, Result,
    web::{Data, Json, Path, Query}};
use uuid::Uuid;
use crate::{
    database::Database,
    models::{self, permission::{NewPermission, PermissionChangeset}},
};
use crate::services::common::{EntityResponse, PagedResponse, process_response};
use diesel::QueryResult;

#[derive(serde::Deserialize)]
struct ResourceQuery {
    resource: Option<String>,
}

#[get("")]
async fn index(
    db: Data<Database>,
    Query(params): Query<ResourceQuery>,
    req: HttpRequest,
) -> HttpResponse {
    let mut db = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut db, &req);
    match params.resource {
        Some(ref res) => match (models::permission::read_all_for_resource(&mut db, res), models::permission::count_for_resource(&mut db, res)) {
            (Ok(items), Ok(count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
            _ => HttpResponse::InternalServerError().finish(),
        },
        None => match (models::permission::read_all(&mut db), models::permission::count(&mut db)) {
            (Ok(items), Ok(count)) => HttpResponse::Ok().json(PagedResponse { count, items }),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

#[get("/{id}")]
async fn read(db: Data<Database>, item_id: Path<Uuid>, req: HttpRequest) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    match models::permission::read(&mut db, item_id.into_inner()) {
        Ok(perm) => HttpResponse::Ok().json(perm),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewPermission>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut db = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut db, &req);
    let result: QueryResult<_> = models::permission::create(&mut db, item);
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
    Json(item): Json<PermissionChangeset>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    let result = models::permission::update(&mut db, item_id.into_inner(), &item);
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
    if models::permission::delete(&mut db, item_id.into_inner()).is_ok() {
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
