// use actix_web::{delete, Error, get, HttpResponse, HttpRequest, post, put, Result, web::{Data, Json, Path, Query}};
// use crate::models::roominfo::{RoomInfoData,get_roominfo};
// use crate::models::common::BigId;
// use chrono::{ Utc, TimeZone };
// use crate::models::apicalllog::{apicalllog};
// use utoipa::{ToSchema,OpenApi};

// #[derive(OpenApi)]
// #[openapi(paths(index))]
// pub struct RoomInfoDoc;

// #[utoipa::path(
//         get,
//         path = "/roominfo",
//         responses(
//             (status = 200, description = "RoomInfo data found successfully", body = RoomInfoData),
//             (status = 404, description = "RoomInfo data not found")
//         ),
//         params(
//             ("page" = Option<u64>, Query, description = "Page to read"),
//             ("page_size" = Option<u64>, Query, description = "How many RoomInfo data rows to return")
//         )
//     )
// ]
// #[get("")]
// async fn index(
// //    Query(info): Query<PaginationParams>,
// ) -> HttpResponse {

//     let result = get_roominfo();

//     if result.is_ok() {
//         HttpResponse::Ok().json(result.unwrap())
//     } else {
//         HttpResponse::InternalServerError().finish()
//     }
// }

// #[get("/{id}")]
// async fn read(
//     item_id: Path<BigId>,
// ) -> HttpResponse {
// //    let result = models::ament::read(&mut db, item_id.into_inner());
// let result = get_roominfo();
// //    if result.is_ok() {
// //        HttpResponse::Ok().json(result.unwrap())
// //    } else {
// //        HttpResponse::NotFound().finish()
// //    }
//     HttpResponse::Ok().json(result.unwrap())
// }

// pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
//     return scope
//         .service(index);
// }
