use actix_web::{HttpResponse, get, web::{Data, Path, Query}};
use crate::{database::Database, models::common::PaginationParams};
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

#[get("/{id}/equipmentregistrations")]
async fn read_equipmentregistrations(
    db: Data<Database>,
    equipment_id: Path<i64>,
    Query(params): Query<PaginationParams>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::equipmentregistration::read_all_equipmentregistrations_of_equipment_piece(&mut conn, equipment_id.into_inner(), &params) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(read)
        .service(read_equipmentregistrations);
}
