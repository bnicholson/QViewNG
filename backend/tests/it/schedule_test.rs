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
fn counts(conn: &mut Connection, did: Uuid) -> (usize, usize, usize, usize) {
    (
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

    // 4 teams, 1 pool, 1 pool group → a single round-robin: 3 rounds × 2 games = 6 games, 2 rooms needed.
    let (_tournament, division, owner_id, room_a, room_b) = arrange(&mut conn, 4);

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;
    let token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);

    let body = json!({
        "did": division.did,
        "pool_group_names": ["Group 1"],
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
    assert_eq!(created["pool_groups_created"], 1);
    assert_eq!(created["pools_created"], 1);
    assert_eq!(created["team_placements_created"], 4);
    assert_eq!(created["rounds_created"], 3);
    assert_eq!(created["games_created"], 6);

    // Everything landed in the database.
    let (groups, pools, rounds, games) = counts(&mut conn, division.did);
    assert_eq!((groups, pools, rounds, games), (1, 1, 3, 6));
}

#[actix_web::test]
async fn create_schedule_multiple_pool_groups_works() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // Two pool groups, each with its own "Pool A". Pool names are qualified by pool group so they
    // don't collide on the division-wide unique pool-name rule; round names run 1..6 across both.
    let (_tournament, division, owner_id, room_a, room_b) = arrange(&mut conn, 4);

    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;
    let token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);

    let body = json!({
        "did": division.did,
        "pool_group_names": ["Group 1", "Group 2"],
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

    // Two pool groups worth: 2 groups, 2 pools, 8 placements, 6 rounds, 12 games.
    let (groups, pools, rounds, games) = counts(&mut conn, division.did);
    assert_eq!((groups, pools, rounds, games), (2, 2, 6, 12));
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
        "pool_group_names": ["Group 1"],
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
    // pool group/pool/placements/round that were inserted before the failing game.
    let (groups, pools, rounds, games) = counts(&mut conn, division.did);
    assert_eq!((groups, pools, rounds, games), (0, 0, 0, 0));
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
        "pool_group_names": ["Group 1"],
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

    let (groups, pools, rounds, games) = counts(&mut conn, division.did);
    assert_eq!((groups, pools, rounds, games), (0, 0, 0, 0));
}

/// POSTs a schedule for `did` with the given pool group names (1 pool each) and asserts it was created.
async fn create_schedule<S, B>(app: &S, owner_id: Uuid, did: Uuid, pool_group_names: &[&str], room_a: Uuid, room_b: Uuid)
where
    S: actix_web::dev::Service<actix_http::Request, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error>,
    B: actix_web::body::MessageBody,
{
    let token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["division:create".to_string()]);
    let req = test::TestRequest::post()
        .uri("/api/schedule")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&json!({
            "did": did, "pool_group_names": pool_group_names, "num_pools": 1,
            "round_duration_minutes": 30, "start_time": null, "room_slots": [room_a, room_b],
        }))
        .to_request();
    assert_eq!(test::call_service(app, req).await.status(), StatusCode::CREATED);
}

/// DELETEs each round via `/api/rounds/{id}` as the tournament owner (with round:delete).
async fn delete_rounds<S, B>(app: &S, owner_id: Uuid, round_ids: &[Uuid])
where
    S: actix_web::dev::Service<actix_http::Request, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error>,
    B: actix_web::body::MessageBody,
{
    let token = make_token(owner_id, vec!["tournament_manager".to_string()], vec!["round:delete".to_string()]);
    for round_id in round_ids {
        let req = test::TestRequest::delete()
            .uri(&format!("/api/rounds/{}", round_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();
        assert_eq!(test::call_service(app, req).await.status(), StatusCode::OK);
    }
}

#[actix_web::test]
async fn deleting_rounds_deletes_their_games() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // A real schedule via the endpoint: 4 teams, 1 pool group of 1 pool → 1 group, 1 pool, 3 rounds, 6 games.
    let (_tournament, division, owner_id, room_a, room_b) = arrange(&mut conn, 4);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;
    create_schedule(&app, owner_id, division.did, &["Group 1"], room_a, room_b).await;
    assert_eq!(counts(&mut conn, division.did), (1, 1, 3, 6));

    // Delete two of the three rounds.
    let rounds = backend::models::round::read_all_rounds_of_division(&mut conn, division.did, &PAGE).unwrap();
    let deleted: Vec<Uuid> = rounds[..2].iter().map(|r| r.roundid).collect();
    let deleted_games: Vec<Uuid> = deleted.iter()
        .flat_map(|rid| backend::models::game::read_all_games_of_round(&mut conn, *rid, &PAGE).unwrap())
        .map(|g| g.gid)
        .collect();
    assert_eq!(deleted_games.len(), 4, "two rounds of two games each");
    delete_rounds(&app, owner_id, &deleted).await;

    // The deleted rounds and every one of their games are gone ...
    for rid in &deleted {
        assert!(backend::models::round::read(&mut conn, *rid).is_err(), "round {} is deleted", rid);
    }
    for gid in &deleted_games {
        assert!(backend::models::game::read(&mut conn, *gid).is_err(), "game {} is deleted", gid);
    }
    // ... while the remaining round keeps its two games, and the pool group + pool are untouched.
    let kept = rounds[2].roundid;
    assert_eq!(backend::models::game::read_all_games_of_round(&mut conn, kept, &PAGE).unwrap().len(), 2);
    assert_eq!(counts(&mut conn, division.did), (1, 1, 1, 2));
}

#[actix_web::test]
async fn deleting_all_rounds_of_a_pool_group_leaves_the_pool_group_its_pools_and_placements() {
    let db = test_db();
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // Two pool groups, each 1 pool of 4 teams → 2 groups, 2 pools, 6 rounds, 12 games total.
    // (With one pool per group, every pool holds all four teams whether or not teams are shuffled.)
    let (_tournament, division, owner_id, room_a, room_b) = arrange(&mut conn, 4);
    let app = test::init_service(App::new().app_data(web::Data::new(db)).configure(configure_routes)).await;
    create_schedule(&app, owner_id, division.did, &["Group 1", "Group 2"], room_a, room_b).await;
    assert_eq!(counts(&mut conn, division.did), (2, 2, 6, 12));

    let groups = backend::models::poolbracketgroup::read_all_of_division(&mut conn, division.did).unwrap();
    let group_1 = groups.iter().find(|g| g.name == "Group 1").expect("Group 1 exists").clone();
    let group_1_pools = backend::models::pool_bracket::read_all_of_poolbracketgroup(&mut conn, group_1.poolbracketgroupid).unwrap();
    assert_eq!(group_1_pools.len(), 1);
    let pool = group_1_pools[0].clone();
    let team_group = backend::models::teamgroup::read_of_pool_bracket(&mut conn, pool.pool_bracket_id).unwrap();
    let (placed_before, placed_count_before) =
        backend::models::team::read_team_rows_of_pool_bracket(&mut conn, pool.pool_bracket_id, &PAGE).unwrap();
    assert_eq!(placed_count_before, 4);

    // Group 1's rounds are exactly the rounds whose games are played in Group 1's pool.
    let all_games = backend::models::game::read_all_games_of_division(&mut conn, division.did, &PAGE).unwrap();
    let mut group_1_rounds: Vec<Uuid> = all_games.iter()
        .filter(|g| g.poolbracket_id == pool.pool_bracket_id)
        .map(|g| g.roundid)
        .collect();
    group_1_rounds.sort();
    group_1_rounds.dedup();
    assert_eq!(group_1_rounds.len(), 3);

    // Delete every one of Group 1's rounds, leaving Group 1 with no games at all.
    delete_rounds(&app, owner_id, &group_1_rounds).await;
    let remaining_games = backend::models::game::read_all_games_of_division(&mut conn, division.did, &PAGE).unwrap();
    assert!(remaining_games.iter().all(|g| g.poolbracket_id != pool.pool_bracket_id), "Group 1 has no games left");
    assert_eq!(remaining_games.len(), 6, "Group 2's six games are untouched");

    // The pool group itself is NOT deleted ...
    let groups_after = backend::models::poolbracketgroup::read_all_of_division(&mut conn, division.did).unwrap();
    assert_eq!(groups_after.len(), 2);
    let group_1_after = backend::models::poolbracketgroup::read(&mut conn, group_1.poolbracketgroupid)
        .expect("Group 1 still exists");
    assert!(!group_1_after.del_fl);

    // ... nor its pool ...
    assert_eq!(backend::models::pool_bracket::read_all_of_division(&mut conn, division.did).unwrap().len(), 2);
    let pools_after = backend::models::pool_bracket::read_all_of_poolbracketgroup(&mut conn, group_1.poolbracketgroupid).unwrap();
    assert_eq!(pools_after.len(), 1);
    let pool_after = backend::models::pool_bracket::read(&mut conn, pool.pool_bracket_id).expect("pool still exists");
    assert!(!pool_after.del_fl);
    assert_eq!(pool_after.team_group_id, Some(team_group.team_group_id));

    // ... nor the pool's team placements.
    assert!(backend::models::teamgroup::read(&mut conn, team_group.team_group_id).is_ok(), "pool's team group still exists");
    assert_eq!(backend::models::team_teamgroup::read_all_of_team_group(&mut conn, team_group.team_group_id).unwrap().len(), 4);
    let (placed_after, placed_count_after) =
        backend::models::team::read_team_rows_of_pool_bracket(&mut conn, pool.pool_bracket_id, &PAGE).unwrap();
    assert_eq!(placed_count_after, 4);
    let mut before_ids: Vec<Uuid> = placed_before.iter().map(|t| t.teamid).collect();
    let mut after_ids: Vec<Uuid> = placed_after.iter().map(|t| t.teamid).collect();
    before_ids.sort();
    after_ids.sort();
    assert_eq!(before_ids, after_ids, "the same four teams are still placed in the pool");

    // Only Group 1's rounds went away: Group 2's three rounds and six games remain.
    assert_eq!(counts(&mut conn, division.did), (2, 2, 3, 6));
}
