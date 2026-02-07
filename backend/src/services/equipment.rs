use actix_web::{get, HttpResponse, web::{Data, Path}};
use crate::database::Database;
use crate::models;

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<i64>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::equipment::read(&mut conn, item_id.into_inner()) {
        Ok(equipment) => HttpResponse::Ok().json(equipment),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(read);
}
