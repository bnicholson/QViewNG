use backend::{database, models::{team::{NewTeam, Team, TeamBuilder}, user::UserBuilder}};
use uuid::Uuid;

use crate::fixtures::users::create_and_insert_user;

pub fn get_team_payload(db: &mut database::Connection, did: Uuid) -> NewTeam {
    TeamBuilder::new_default(did)
        .set_name("Better Team than Last Year")
        .set_coachid(create_and_insert_user(db, "Tiffany", "somethingcool@").id)
        .build()
        .unwrap()
}

pub fn seed_team(db: &mut database::Connection, did: Uuid) -> Team {
    TeamBuilder::new_default(did)
        .set_name("Team 1")
        .set_coachid(create_and_insert_user(db, "Tiffany", "somethingcool@").id)
        .build_and_insert(db)
        .unwrap()
}

pub fn seed_teams(
    db: &mut database::Connection, 
    did: Uuid
) -> Vec<Team> {
    vec![
        TeamBuilder::new_default(did)
            .set_name("Team 1")
            .set_coachid(create_and_insert_user(db, "Tiffany", "somethingcool@").id)
            .build_and_insert(db)
            .unwrap(),
        TeamBuilder::new_default(did)
            .set_coachid(create_and_insert_user(db, "Seth", "ThisKINDofpWd54@").id)
            .set_name("Come Get Some")
            .set_quizzer_one_id(create_and_insert_user(db, "Trishell", "Something78)").id)
            .set_quizzer_two_id(create_and_insert_user(db, "David", "wutwaziDUing8").id)
            .build_and_insert(db)
            .unwrap(),
        TeamBuilder::new_default(did)
            .set_coachid(create_and_insert_user(db, "Kimberly", "ThisKINDofpWd54@").id)
            .set_name("Luke Found a Frog")
            .set_quizzer_one_id(create_and_insert_user(db, "Tyler", "Something78)").id)
            .set_quizzer_two_id(create_and_insert_user(db, "Taylor", "wutwaziDUing8").id)
            .set_quizzer_three_id(create_and_insert_user(db, "Tiffany", "34techCompanies43").id)
            .set_quizzer_four_id(create_and_insert_user(db, "Sam", "pyramidsInTheExpanse").id)
            .set_quizzer_five_id(create_and_insert_user(db, "John", "gundersoncapitoL").id)
            .set_quizzer_six_id(create_and_insert_user(db, "Lucas", "merrygoHarris90").id)
            .build_and_insert(db)
            .unwrap()
    ]
}

pub fn seed_teams_with_names(
    db: &mut database::Connection, 
    did: Uuid, 
    team_name_1: &str,
    team_name_2: &str,
    team_name_3: &str,
) -> (Team,Team,Team) {
    (
        TeamBuilder::new_default(did)
            .set_name(team_name_1)
            .set_coachid(
                UserBuilder::new_default("Kevin")
                    .set_hash_password("not_kevins_pwd")
                    .build_and_insert(db)
                    .unwrap()
                    .id
                )
            .build_and_insert(db)
            .unwrap(),
        TeamBuilder::new_default(did)
            .set_coachid(create_and_insert_user(db, "Seth", "ThisKINDofpWd54@").id)
            .set_name(team_name_2)
            .set_quizzer_one_id(create_and_insert_user(db, "Trishell", "Something78)").id)
            .set_quizzer_two_id(create_and_insert_user(db, "David", "wutwaziDUing8").id)
            .build_and_insert(db)
            .unwrap(),
        TeamBuilder::new_default(did)
            .set_coachid(create_and_insert_user(db, "Kimberly", "ThisKINDofpWd54@").id)
            .set_name(team_name_3)
            .set_quizzer_one_id(create_and_insert_user(db, "Tyler", "Something78)").id)
            .set_quizzer_two_id(create_and_insert_user(db, "Taylor", "wutwaziDUing8").id)
            .set_quizzer_three_id(create_and_insert_user(db, "Tiffany", "34techCompanies43").id)
            .set_quizzer_four_id(create_and_insert_user(db, "Sam", "pyramidsInTheExpanse").id)
            .set_quizzer_five_id(create_and_insert_user(db, "John", "gundersoncapitoL").id)
            .set_quizzer_six_id(create_and_insert_user(db, "Lucas", "merrygoHarris90").id)
            .build_and_insert(db)
            .unwrap()
    )
}
