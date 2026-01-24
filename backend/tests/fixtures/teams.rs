use backend::models::team::{Team,NewTeam};
use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use uuid::Uuid;
use backend::schema::teams;

use crate::fixtures::users::{create_and_insert_user, new_user_one};

pub fn new_team_one(conn: &mut PgConnection, did: Uuid) -> NewTeam {
    NewTeam {
        did: did,
        coachid: create_and_insert_user(conn, new_user_one("Fanny", "ThisKINDofpWd54@")).id,
        name: "Better Team than Last Year".to_string(),
        quizzer_one_id: Some(create_and_insert_user(conn, new_user_one("Grace", "Something78)")).id),
        quizzer_two_id: Some(create_and_insert_user(conn, new_user_one("Jesse", "wutwaziDUing8")).id),
        quizzer_three_id: Some(create_and_insert_user(conn, new_user_one("Garret", "34techCompanies43")).id),
        quizzer_four_id: Some(create_and_insert_user(conn, new_user_one("Julie", "pyramidsInTheExpanse")).id),
        quizzer_five_id: None,
        quizzer_six_id: None
    }
}

pub fn new_team_two(conn: &mut PgConnection, did: Uuid) -> NewTeam {
    NewTeam {
        did: did,
        coachid: create_and_insert_user(conn, new_user_one("Seth", "ThisKINDofpWd54@")).id,
        name: "Come Get Some".to_string(),
        quizzer_one_id: Some(create_and_insert_user(conn, new_user_one("Trishell", "Something78)")).id),
        quizzer_two_id: Some(create_and_insert_user(conn, new_user_one("David", "wutwaziDUing8")).id),
        quizzer_three_id: None,
        quizzer_four_id: None,
        quizzer_five_id: None,
        quizzer_six_id: None
    }
}

pub fn new_team_three(conn: &mut PgConnection, did: Uuid) -> NewTeam {
    NewTeam {
        did: did,
        coachid: create_and_insert_user(conn, new_user_one("Kimberly", "ThisKINDofpWd54@")).id,
        name: "Luke Found a Frog".to_string(),
        quizzer_one_id: Some(create_and_insert_user(conn, new_user_one("Tyler", "Something78)")).id),
        quizzer_two_id: Some(create_and_insert_user(conn, new_user_one("Taylor", "wutwaziDUing8")).id),
        quizzer_three_id: Some(create_and_insert_user(conn, new_user_one("Tiffany", "34techCompanies43")).id),
        quizzer_four_id: Some(create_and_insert_user(conn, new_user_one("Sam", "pyramidsInTheExpanse")).id),
        quizzer_five_id: Some(create_and_insert_user(conn, new_user_one("John", "pyramidsInTheExpanse")).id),
        quizzer_six_id: Some(create_and_insert_user(conn, new_user_one("Lucas", "pyramidsInTheExpanse")).id)
    }
}

// pub fn new_team(did: Uuid, sched_start_time: DateTime<Utc>) -> NewTeam {
//     NewTeam {
//         did: did,
//         scheduled_start_time: sched_start_time
//     }
// }

pub fn get_team_payload(conn: &mut PgConnection, did: Uuid) -> NewTeam {
    new_team_one(conn, did)
}

fn create_and_insert_team(conn: &mut PgConnection, new_team: NewTeam) -> Team {
    diesel::insert_into(teams::table)
        .values(new_team)
        .returning(Team::as_returning())
        .get_result::<Team>(conn)
        .expect("Failed to create team")
}

// pub fn seed_team(conn: &mut PgConnection, did: Uuid) -> Team {
//     let new_team = new_team_one(did);
//     create_and_insert_team(conn, new_team)
// }

pub fn seed_teams(
    conn: &mut PgConnection, 
    did: Uuid
) -> Vec<Team> {
    let new_team_1 = new_team_one(conn, did);
    let new_team_2 = new_team_two(conn, did);
    let new_team_3 = new_team_three(conn, did);

    vec![
        create_and_insert_team(conn, new_team_1),
        create_and_insert_team(conn, new_team_2),
        create_and_insert_team(conn, new_team_3),
    ]
}

// pub fn seed_teams_with_sched_start_times(
//     conn: &mut PgConnection, 
//     did: Uuid, 
//     start_time_1: DateTime<Utc>,
//     start_time_2: DateTime<Utc>,
//     start_time_3: DateTime<Utc>,
// ) -> Vec<Team> {
//     let new_team_1 = new_team(did, start_time_1);
//     let new_team_2 = new_team(did, start_time_2);
//     let new_team_3 = new_team(did, start_time_3);

//     vec![
//         create_and_insert_team(conn, new_team_1),
//         create_and_insert_team(conn, new_team_2),
//         create_and_insert_team(conn, new_team_3),
//     ]
// }
