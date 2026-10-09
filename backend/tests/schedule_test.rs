
mod common;

use actix_http::StatusCode;
use actix_web::{App, test, web};
use backend::database::{Database, Connection};
use backend::models::common::PaginationParams;
use backend::models::user::UserBuilder;
use backend::models::tournament::{Tournament, TournamentBuilder};
use backend::models::division::{Division, DivisionBuilder};
use backend::models::team::TeamBuilder;
use backend::models::room::RoomBuilder;
use backend::routes::configure_routes;
use serde_json::json;
use uuid::Uuid;
use crate::common::{make_token, test_db};

const PAGE: PaginationParams = PaginationParams { page: 0, page_size: 500 };

/// A division with `team_count` teams and two rooms (each with a quizmaster), plus the tournament
/// owner. Returns (tournament, division, owner_id, room_a, room_b).
fn arrange(conn: &mut Connection, team_count: usize) -> (Tournament, Division, Uuid, Uuid, Uuid) {
    let owner = UserBuilder::new_default("Sched Owner").set_hash_password("OwnerPwd123!").build_and_insert(conn).unwrap();
    let tournament = TournamentBuilder::new_default("Sched Tour").set_owner_id(owner.id).build_and_insert(conn).unwrap();
    let division = DivisionBuilder::new_default("Sched Div", tournament.tid).build_and_insert(conn).unwrap();

    let coach = UserBuilder::new_default("Sched Coach").set_hash_password("CoachPwd123!").build_and_insert(conn).unwrap();
    for i in 0..team_count {
        TeamBuilder::new_default(division.did)
            .set_name(&format!("Team {}", i + 1))
            .set_coachid(coach.id)
            .build_and_insert(conn)
            .unwrap();
    }

    let qm = UserBuilder::new_default("Sched QM").set_hash_password("QmPwd12345!").build_and_insert(conn).unwrap();
    let room_a = RoomBuilder::new_default("Room A", tournament.tid).set_quizmaster_id(Some(qm.id)).build_and_insert(conn).unwrap();
    let room_b = RoomBuilder::new_default("Room B", tournament.tid).set_quizmaster_id(Some(qm.id)).build_and_insert(conn).unwrap();

    (tournament, division, owner.id, room_a.roomid, room_b.roomid)
}

/// Counts of everything the schedule creates for a division, for before/after assertions.
fn counts(conn: &mut Connection, did: Uuid) -> (usize, usize, usize, usize, usize) {
    (
        backend::models::roundgroup::read_all_of_division(conn, did).unwrap().len(),
        backend::models::poolbracketgroup::read_all_of_division(conn, did).unwrap().len(),
        backend::models::pool_bracket::read_all_of_division(conn, did).unwrap().len(),
        backend::models::round::read_all_rounds_of_division(conn, did, &PAGE).unwrap().len(),
        backend::models::game::read_all_games_of_division(conn, did, &PAGE).unwrap().len(),
    )
}

#[actix_web::test]
async fn create_schedule_works() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // 4 teams, 1 pool, 1 session → a single round-robin: 3 rounds × 2 games = 6 games, 2 rooms needed.
    let (_tournament, division, owner_id, room_a, room_b) = arrange(&mut conn, 4);

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;
    let token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);

    let body = json!({
        "did": division.did,
        "session_names": ["Session 1"],
        "num_pools": 1,
        "round_duration_minutes": 30,
        "start_time": null,
        "room_slots": [room_a, room_b],
    });

    let req = test::TestRequest::post()
        .uri("/api/schedule")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let created: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(created["sessions_created"], 1);
    assert_eq!(created["pool_groups_created"], 1);
    assert_eq!(created["pools_created"], 1);
    assert_eq!(created["team_placements_created"], 4);
    assert_eq!(created["rounds_created"], 3);
    assert_eq!(created["games_created"], 6);

    // Everything landed in the database.
    let (sessions, groups, pools, rounds, games) = counts(&mut conn, division.did);
    assert_eq!((sessions, groups, pools, rounds, games), (1, 1, 1, 3, 6));
}

#[actix_web::test]
async fn create_schedule_multiple_sessions_works() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // Two sessions, each its own pool group + "Pool A". Pool names are qualified by session so they
    // don't collide on the division-wide unique pool-name rule.
    let (_tournament, division, owner_id, room_a, room_b) = arrange(&mut conn, 4);

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;
    let token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);

    let body = json!({
        "did": division.did,
        "session_names": ["Session 1", "Session 2"],
        "num_pools": 1,
        "round_duration_minutes": 30,
        "start_time": null,
        "room_slots": [room_a, room_b],
    });

    let req = test::TestRequest::post()
        .uri("/api/schedule")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Two sessions worth: 2 groups, 2 pools, 8 placements, 6 rounds, 12 games.
    let (sessions, groups, pools, rounds, games) = counts(&mut conn, division.did);
    assert_eq!((sessions, groups, pools, rounds, games), (2, 2, 2, 6, 12));
}

#[actix_web::test]
async fn create_schedule_rolls_back_on_failure() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let (_tournament, division, owner_id, room_a, _room_b) = arrange(&mut conn, 4);

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;
    let token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);

    // The same room in both slots forces two concurrent games in one round to share a room, which
    // violates the games (room, round, clientkey) unique key partway through the transaction.
    let body = json!({
        "did": division.did,
        "session_names": ["Session 1"],
        "num_pools": 1,
        "round_duration_minutes": 30,
        "start_time": null,
        "room_slots": [room_a, room_a],
    });

    let req = test::TestRequest::post()
        .uri("/api/schedule")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // Because the work runs in one transaction, the failed run created NOTHING — not even the
    // session/pool group/pool/placements/round that were inserted before the failing game.
    let (sessions, groups, pools, rounds, games) = counts(&mut conn, division.did);
    assert_eq!((sessions, groups, pools, rounds, games), (0, 0, 0, 0, 0));
}

#[actix_web::test]
async fn create_schedule_requires_two_teams() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // Only one team → nothing can be scheduled, and nothing is created.
    let (_tournament, division, owner_id, room_a, room_b) = arrange(&mut conn, 1);

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;
    let token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);

    let body = json!({
        "did": division.did,
        "session_names": ["Session 1"],
        "num_pools": 1,
        "round_duration_minutes": 30,
        "start_time": null,
        "room_slots": [room_a, room_b],
    });

    let req = test::TestRequest::post()
        .uri("/api/schedule")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let (sessions, groups, pools, rounds, games) = counts(&mut conn, division.did);
    assert_eq!((sessions, groups, pools, rounds, games), (0, 0, 0, 0, 0));
}

#[actix_web::test]
async fn deleting_round_cascades_games_and_deleting_session_cascades_empty_pool_group() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // Build a real schedule via the endpoint: 4 teams, 1 pool, 1 session → 1 group, 1 pool, 3 rounds, 6 games.
    let (_tournament, division, owner_id, room_a, room_b) = arrange(&mut conn, 4);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    let create_token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);
    let create_req = test::TestRequest::post()
        .uri("/api/schedule")
        .insert_header(("Authorization", format!("Bearer {}", create_token)))
        .set_json(&json!({
            "did": division.did, "session_names": ["Session 1"], "num_pools": 1,
            "round_duration_minutes": 30, "start_time": null, "room_slots": [room_a, room_b],
        }))
        .to_request();
    assert_eq!(test::call_service(&app, create_req).await.status(), StatusCode::CREATED);
    assert_eq!(counts(&mut conn, division.did), (1, 1, 1, 3, 6));

    // Deleting one round deletes that round's games (2), but the pool group still has games elsewhere.
    let rounds = backend::models::round::read_all_rounds_of_division(&mut conn, division.did, &PAGE).unwrap();
    let round_token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["round:delete".to_string()]);
    let del_round = test::TestRequest::delete()
        .uri(&format!("/api/rounds/{}", rounds[0].roundid))
        .insert_header(("Authorization", format!("Bearer {}", round_token)))
        .to_request();
    assert_eq!(test::call_service(&app, del_round).await.status(), StatusCode::OK);
    // 2 rounds / 4 games remain; the group and pool survive because games remain.
    assert_eq!(counts(&mut conn, division.did), (1, 1, 1, 2, 4));

    // Deleting the session deletes its remaining rounds + games, which empties the pool group — so the
    // group and its pool are removed too.
    let session_token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["roundgroup:delete".to_string()]);
    let del_session = test::TestRequest::delete()
        .uri(&format!("/api/roundgroups/{}", rounds[0].roundgroup_id))
        .insert_header(("Authorization", format!("Bearer {}", session_token)))
        .to_request();
    assert_eq!(test::call_service(&app, del_session).await.status(), StatusCode::OK);
    assert_eq!(counts(&mut conn, division.did), (0, 0, 0, 0, 0));
}

#[actix_web::test]
async fn deleting_one_session_leaves_other_sessions_intact() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // Two sessions, each 1 pool of 4 teams → 2 groups, 2 pools, 6 rounds, 12 games total.
    let (_tournament, division, owner_id, room_a, room_b) = arrange(&mut conn, 4);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;

    let create_token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);
    let create_req = test::TestRequest::post()
        .uri("/api/schedule")
        .insert_header(("Authorization", format!("Bearer {}", create_token)))
        .set_json(&json!({
            "did": division.did, "session_names": ["Session 1", "Session 2"], "num_pools": 1,
            "round_duration_minutes": 30, "start_time": null, "room_slots": [room_a, room_b],
        }))
        .to_request();
    assert_eq!(test::call_service(&app, create_req).await.status(), StatusCode::CREATED);
    assert_eq!(counts(&mut conn, division.did), (2, 2, 2, 6, 12));

    // Delete only "Session 1".
    let session_1 = backend::models::roundgroup::read_all_of_division(&mut conn, division.did).unwrap()
        .into_iter().find(|rg| rg.name == "Session 1").expect("Session 1 exists");
    let session_token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["roundgroup:delete".to_string()]);
    let del = test::TestRequest::delete()
        .uri(&format!("/api/roundgroups/{}", session_1.roundgroup_id))
        .insert_header(("Authorization", format!("Bearer {}", session_token)))
        .to_request();
    assert_eq!(test::call_service(&app, del).await.status(), StatusCode::OK);

    // Only Session 1's session/group/pool/rounds/games are gone; Session 2 is untouched.
    assert_eq!(counts(&mut conn, division.did), (1, 1, 1, 3, 6));
}
