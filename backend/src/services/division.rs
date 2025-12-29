use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use crate::{models, models::division::{Division,NewDivision,DivisionChangeset}};
use crate::models::{tournament::Tournament,common::{PaginationParams,BigId}};
use crate::database::Database;
use utoipa::OpenApi;
use diesel::{QueryDsl,RunQueryDsl,dsl::{exists,select}};
use crate::schema::tournaments::dsl::{tournaments as tournaments_table,tid as tournament_tid};

#[derive(OpenApi)]
#[openapi(paths(index))]
pub struct DivisionDoc;

#[utoipa::path(
        get,
        path = "/divisions",
        responses(
            (status = 200, description = "Divisions found successfully", body = Division),
            (status = 404, description = "Division not found")
        ),
        params(
            ("page" = Option<u64>, Query, description = "Page to read"),
            ("page_size" = Option<u64>, Query, description = "How many Divisions to return")
        )
    )
]
#[get("")]
async fn index(
    db: Data<Database>,
    Query(info): Query<PaginationParams>,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    let result = models::division::read_all(&mut db, &info);
   
    println!("Divisions: {:?}",result);
    
    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<BigId>,
) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();

    match models::division::read(&mut conn, item_id.into_inner()) {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewDivision>,
) -> Result<HttpResponse, Error> {

    let mut conn = db.get_connection();

    let tournament_exists: bool = match tournaments_table
        .find(item.tid)
        .get_result::<Tournament>(&mut conn)
    {
        Ok(_) => true,
        Err(_) => false,
    };

    if !tournament_exists {
        println!("Could not find Tournament by ID={}", &item.tid);
        return Ok(HttpResponse::UnprocessableEntity().json(serde_json::json!({
            "error": format!("Tournament with ID {} does not exist", item.tid)
        })));
    }

    // tracing::debug!("{} Division model create {:?}", line!(), item);
    
    let result: Division = models::division::create(&mut conn, &item).expect("Creation error");

    Ok(HttpResponse::Created().json(result))
}

#[put("/{id}")]
async fn update(
    db: Data<Database>,
    item_id: Path<BigId>,
    Json(item): Json<DivisionChangeset>,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    let result = models::division::update(&mut db, item_id.into_inner(), &item);

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[delete("/{id}")]
async fn destroy(
    db: Data<Database>,
    item_id: Path<BigId>,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();

    let result = models::division::delete(&mut db, item_id.into_inner());

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(read)
        .service(create)
        .service(update)
        .service(destroy);
}
