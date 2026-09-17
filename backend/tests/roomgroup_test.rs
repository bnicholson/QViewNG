mod common;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::database::Database;
use backend::models::roomgroup::{self, RoomGroup, RoomGroupBuilder};
use backend::models::tournament::TournamentBuilder;
use backend::models::user::UserBuilder;
use backend::routes::configure_routes;
use backend::services::common::{EntityResponse, PagedResponse};
use serde_json::json;
use crate::common::{TEST_DB_URL, clean_database, make_token};

/// POST /api/roomgroups: a user with `room:create` may create one under a tournament (type defaults
/// to "building"); missing permission, unauthenticated, and a bad tournament are rejected.
#[actix_web::test]
async fn create_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let user = UserBuilder::new_default("RG Creator").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let tournament = TournamentBuilder::new_default("RG Tour").set_owner_id(user.id).build_and_insert(&mut conn).unwrap();

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    // ── Success: user with room:create (type omitted → defaults to "building") ──
    let token = make_token(user.id, vec!["tournament_manager".to_string()], vec!["room:create".to_string()]);
    let req = test::TestRequest::post()
        .uri("/api/roomgroups")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({ "tournamentid": tournament.tid, "name": "Main Hall", "notes": "The big one" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: EntityResponse<RoomGroup> = test::read_body_json(resp).await;
    let rg = body.data.unwrap();
    assert_eq!(rg.tournamentid, tournament.tid);
    assert_eq!(rg.name, "Main Hall");
    assert_eq!(rg.type_, "building");
    assert_eq!(rg.notes, "The big one");
    assert_eq!(rg.creator_userid, user.id);
    assert!(!rg.del_fl);

    // ── Fail: authenticated but lacks room:create ──
    let no_perm = make_token(user.id, vec!["member".to_string()], vec!["room:read".to_string()]);
    let req = test::TestRequest::post()
        .uri("/api/roomgroups")
        .insert_header(("Authorization", format!("Bearer {}", no_perm)))
        .set_json(json!({ "tournamentid": tournament.tid, "name": "Nope" }))
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: unauthenticated ──
    let req = test::TestRequest::post()
        .uri("/api/roomgroups")
        .set_json(json!({ "tournamentid": tournament.tid, "name": "Nope" }))
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), StatusCode::UNAUTHORIZED);

    // ── Fail: tournament does not exist ──
    let req = test::TestRequest::post()
        .uri("/api/roomgroups")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({ "tournamentid": uuid::Uuid::new_v4(), "name": "Orphan" }))
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

/// GET /api/roomgroups/{id} and GET /api/roomgroups (index) return roomgroups.
#[actix_web::test]
async fn read_and_index_work() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let user = UserBuilder::new_default("RG Reader").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let tournament = TournamentBuilder::new_default("RG Tour").set_owner_id(user.id).build_and_insert(&mut conn).unwrap();
    let rg = RoomGroupBuilder::new(tournament.tid).set_name("Hall A").set_creator_userid(user.id).build_and_insert(&mut conn).unwrap();

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    let req = test::TestRequest::get().uri(&format!("/api/roomgroups/{}", rg.roomgroupid)).to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let got: RoomGroup = test::read_body_json(resp).await;
    assert_eq!(got.roomgroupid, rg.roomgroupid);
    assert_eq!(got.tournamentid, tournament.tid);
    assert_eq!(got.name, "Hall A");
    assert_eq!(got.type_, "building");

    let req = test::TestRequest::get().uri("/api/roomgroups?page=0&page_size=10").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let paged: PagedResponse<RoomGroup> = test::read_body_json(resp).await;
    assert_eq!(paged.count, 1);
    assert_eq!(paged.items[0].name, "Hall A");
}

/// PUT /api/roomgroups/{id}: a user with `room:update` may rename it; others are rejected.
#[actix_web::test]
async fn update_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let user = UserBuilder::new_default("RG Editor").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let tournament = TournamentBuilder::new_default("RG Tour").set_owner_id(user.id).build_and_insert(&mut conn).unwrap();
    let rg = RoomGroupBuilder::new(tournament.tid).set_name("Old Name").set_creator_userid(user.id).build_and_insert(&mut conn).unwrap();

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    // ── Fail: lacks room:update ──
    let no_perm = make_token(user.id, vec!["member".to_string()], vec!["room:read".to_string()]);
    let req = test::TestRequest::put()
        .uri(&format!("/api/roomgroups/{}", rg.roomgroupid))
        .insert_header(("Authorization", format!("Bearer {}", no_perm)))
        .set_json(json!({ "name": "Hacked" }))
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), StatusCode::UNAUTHORIZED);

    // ── Success: room:update ──
    let token = make_token(user.id, vec!["tournament_manager".to_string()], vec!["room:update".to_string()]);
    let req = test::TestRequest::put()
        .uri(&format!("/api/roomgroups/{}", rg.roomgroupid))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({ "name": "New Name", "notes": "updated" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let reread = roomgroup::read(&mut conn, rg.roomgroupid).unwrap();
    assert_eq!(reread.name, "New Name");
    assert_eq!(reread.notes, "updated");
    assert_eq!(reread.last_modified_userid, user.id);
}

/// DELETE /api/roomgroups/{id}: a user with `room:delete` soft-deletes it (hidden from reads).
#[actix_web::test]
async fn delete_soft_deletes() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let user = UserBuilder::new_default("RG Deleter").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let tournament = TournamentBuilder::new_default("RG Tour").set_owner_id(user.id).build_and_insert(&mut conn).unwrap();
    let rg = RoomGroupBuilder::new(tournament.tid).set_name("Doomed").set_creator_userid(user.id).build_and_insert(&mut conn).unwrap();

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    let token = make_token(user.id, vec!["tournament_manager".to_string()], vec!["room:delete".to_string()]);
    let req = test::TestRequest::delete()
        .uri(&format!("/api/roomgroups/{}", rg.roomgroupid))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    assert!(roomgroup::read(&mut conn, rg.roomgroupid).is_err(), "soft-deleted roomgroup is hidden from read");
    assert!(roomgroup::read_including_deleted(&mut conn, rg.roomgroupid).unwrap().del_fl, "row remains with del_fl = true");
}

/// GET /api/tournaments/{tid}/roomgroups returns the roomgroups belonging to that tournament, and
/// only those (roomgroups are tournament-scoped now).
#[actix_web::test]
async fn tournament_roomgroups_lookup_works() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    let owner = UserBuilder::new_default("RG Tour Owner").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let tournament = TournamentBuilder::new_default("RG Tour").set_owner_id(owner.id).build_and_insert(&mut conn).unwrap();
    let other = TournamentBuilder::new_default("Other Tour").set_owner_id(owner.id).build_and_insert(&mut conn).unwrap();

    RoomGroupBuilder::new(tournament.tid).set_name("Building A").set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();
    RoomGroupBuilder::new(tournament.tid).set_name("Building B").set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();
    // A roomgroup under a different tournament must be excluded.
    RoomGroupBuilder::new(other.tid).set_name("Other Building").set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/tournaments/{}/roomgroups", tournament.tid))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let items: Vec<RoomGroup> = test::read_body_json(resp).await;
    assert_eq!(items.len(), 2, "both of this tournament's roomgroups, ordered by name");
    let names: Vec<&str> = items.iter().map(|rg| rg.name.as_str()).collect();
    assert_eq!(names, vec!["Building A", "Building B"]);
    assert!(items.iter().all(|rg| rg.tournamentid == tournament.tid));
}
