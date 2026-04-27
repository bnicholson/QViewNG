
use actix_web::{Error, HttpRequest, HttpResponse, Result, delete, get, post, put, web::{Data, Json, Path, Query}};
use uuid::Uuid;
use crate::models::game::NewGame;
use crate::models::apicalllog;
use crate::models::game;
use crate::models::game::GameChangeset;
use crate::database::Database;

#[get("")]
async fn index(
    db: Data<Database>,
    // Query(info): Query<PaginationParams>,
    // info: web::Path<Info>,
    // path: web::Path<(String,String,String)>,
    req: HttpRequest,
) -> HttpResponse {
    // let mut db = db.pool.get().unwrap();
    let mut db = db.get_connection().expect("Failed to get connection");
    
    // log this api call
    apicalllog::create(&mut db, &req);

    HttpResponse::Ok().finish()

    // let result = game::read_all(&mut db, &info);

    // if result.is_ok() {
    //     HttpResponse::Ok().json(result.unwrap())
    // } else {
    //     HttpResponse::InternalServerError().finish()
    // }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    println!("read endpoint");
    // let mut db = db.pool.get().unwrap();
    let mut db = db.get_connection().expect("Failed to get connection");

    // log this api call
    apicalllog::create(&mut db, &req);

    let result = game::read(&mut db, item_id.into_inner());

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<NewGame>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    println!("create endpoint");

    let mut db = db.get_connection().expect("Failed to get connection");

    // log this api call
    apicalllog::create(&mut db, &req);

    println!("payload: {:?}", &item);

    Ok(HttpResponse::Ok().json(item))

    // let mut db = db.pool.get().unwrap();

    // let result: Game = game::create(&mut db, &item).expect("Creation error");

    // Ok(HttpResponse::Created().json(result))
}

#[put("/{id}")]
async fn update(
    db: Data<Database>,
    item_id: Path<Uuid>,
    Json(item): Json<GameChangeset>,
    req: HttpRequest,
) -> HttpResponse {
    println!("update endpoint");
    let mut db = db.pool.get().unwrap();

    // log this api call
    apicalllog::create(&mut db, &req);

    let result = game::update(&mut db, item_id.into_inner(), &item);

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[delete("/{id}")]
async fn destroy(
    db: Data<Database>,
    item_id: Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    println!("destroy endpoint");
    let mut db = db.pool.get().unwrap();

    // log this api call
    apicalllog::create(&mut db, &req);

    let result = game::delete(&mut db, item_id.into_inner());

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
