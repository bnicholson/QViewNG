use backend::models::game::{Game, NewGame};
use diesel::prelude::*;
use uuid::Uuid;
use backend::schema::games;

use crate::fixtures;

pub fn new_game_one(
        tid: Uuid, 
        did: Uuid, 
        room_id: Uuid, 
        round_id: Uuid, 
        left_team_id: Uuid, 
        center_team_id: Option<Uuid>, 
        right_team_id: Uuid, 
        qm_id: Uuid
    ) -> NewGame {
    NewGame {
        org: "Nazarene".to_string(),
        tournamentid: Some(tid),
        divisionid: Some(did),
        roomid: room_id,
        roundid: round_id,
        clientkey: "".to_string(),
        ignore: false,
        ruleset: "Tournament".to_string(),
        leftteamid: left_team_id,
        centerteamid: center_team_id,
        rightteamid: right_team_id,
        quizmasterid: qm_id,
        contentjudgeid: None
    }
}

// pub fn new_game_two(did: Uuid) -> NewGame {
//     NewGame {
//         did: did,
//         scheduled_start_time: Utc.with_ymd_and_hms(2045, 5, 23, 00, 00, 0).unwrap()
//     }
// }

// pub fn new_game_three(did: Uuid) -> NewGame {
//     NewGame {
//         did: did,
//         scheduled_start_time: Utc.with_ymd_and_hms(2065, 5, 23, 00, 00, 0).unwrap()
//     }
// }

// pub fn new_game(did: Uuid, sched_start_time: DateTime<Utc>) -> NewGame {
//     NewGame {
//         did: did,
//         scheduled_start_time: sched_start_time
//     }
// }

pub fn seed_game_payload_dependencies(conn: &mut PgConnection, tname: &str) -> (Uuid,Uuid,Uuid,Uuid,Uuid,Uuid,Uuid,Uuid) {
    let tournament = fixtures::tournaments::seed_tournament(conn, tname);
    let division = fixtures::divisions::seed_division(conn, tournament.tid);
    let room = fixtures::rooms::seed_room(conn, tournament.tid);
    let round = fixtures::rounds::seed_round(conn, division.did);
    let teams = fixtures::teams::seed_teams_with_names(conn, division.did, "Jeffs Team", "Sams Team", "Scotts Team");
    let quizmaster_user = fixtures::users::seed_user(conn); 
    (tournament.tid, division.did, room.roomid, round.roundid, teams.0.teamid, teams.1.teamid, teams.2.teamid, quizmaster_user.id)
}

pub fn get_game_payload(
    tid: Uuid, 
    did: Uuid, 
    room_id: Uuid, 
    round_id: Uuid, 
    left_team_id: Uuid, 
    center_team_id: Option<Uuid>, 
    right_team_id: Uuid, 
    qm_id: Uuid
) -> NewGame {
    new_game_one(tid, did, room_id, round_id, left_team_id, center_team_id, right_team_id, qm_id)
}

fn create_and_insert_game(conn: &mut PgConnection, new_game: NewGame) -> Game {
    diesel::insert_into(games::table)
        .values(new_game)
        .returning(Game::as_returning())
        .get_result::<Game>(conn)
        .expect("Failed to create game")
}

pub fn seed_game(conn: &mut PgConnection) -> Game {
    let deps = seed_game_payload_dependencies(conn, "Tour 1");
    let payload = get_game_payload(deps.0,deps.1,deps.2,deps.3,deps.4,Some(deps.5),deps.6,deps.7);
    create_and_insert_game(conn, payload)
}

pub fn seed_games(conn: &mut PgConnection) -> Vec<Game> {
    let deps_1 = seed_game_payload_dependencies(conn, "Tour 1");
    let payload_1 = get_game_payload(deps_1.0,deps_1.1,deps_1.2,deps_1.3,deps_1.4,Some(deps_1.5),deps_1.6,deps_1.7);
    let game_1 = create_and_insert_game(conn, payload_1);
    
    let deps_2 = seed_game_payload_dependencies(conn, "Tour 2");
    let payload_2 = get_game_payload(deps_2.0,deps_2.1,deps_2.2,deps_2.3,deps_2.4,Some(deps_2.5),deps_2.6,deps_2.7);
    let game_2 = create_and_insert_game(conn, payload_2);

    vec![game_1, game_2]
}

// pub fn seed_games_with_sched_start_times(
//     conn: &mut PgConnection, 
//     did: Uuid, 
//     start_time_1: DateTime<Utc>,
//     start_time_2: DateTime<Utc>,
//     start_time_3: DateTime<Utc>,
// ) -> Vec<Game> {
//     let new_game_1 = new_game(did, start_time_1);
//     let new_game_2 = new_game(did, start_time_2);
//     let new_game_3 = new_game(did, start_time_3);

//     vec![
//         create_and_insert_game(conn, new_game_1),
//         create_and_insert_game(conn, new_game_2),
//         create_and_insert_game(conn, new_game_3),
//     ]
// }

pub fn duplicate_team_in_game_case_one_payload(conn: &mut PgConnection) -> NewGame {
    let deps_1 = seed_game_payload_dependencies(conn, "Tour 1");
    get_game_payload(deps_1.0,deps_1.1,deps_1.2,deps_1.3,deps_1.4,None,deps_1.4,deps_1.7)
}

pub fn duplicate_team_in_game_case_two_payload(conn: &mut PgConnection) -> NewGame {
    let deps_1 = seed_game_payload_dependencies(conn, "Tour 1");
    get_game_payload(deps_1.0,deps_1.1,deps_1.2,deps_1.3,deps_1.4,Some(deps_1.4),deps_1.6,deps_1.7)
}

pub fn duplicate_team_in_game_case_three_payload(conn: &mut PgConnection) -> NewGame {
    let deps_1 = seed_game_payload_dependencies(conn, "Tour 1");
    get_game_payload(deps_1.0,deps_1.1,deps_1.2,deps_1.3,deps_1.4,Some(deps_1.6),deps_1.6,deps_1.7)
}
