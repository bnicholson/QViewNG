use backend::{database, models::{division::{Division, DivisionBuilder}, team::{NewTeam, Team, TeamBuilder}, tournament::{Tournament, TournamentBuilder}, tournament_admin::TournamentAdminBuilder, user::{User, UserBuilder}}};
use uuid::Uuid;

use crate::fixtures::users::create_and_insert_user;

/// Returns `(tournament, division, owner, admin_user, unrelated_user)` for testing
/// team create ABAC: owner and admin should be allowed (conditions 2 & 3),
/// and any user with `team:create` permission should be allowed (condition 1).
pub fn arrange_team_create_works_integration_test(
    db: &mut database::Connection,
) -> (Tournament, Division, User, User, User) {
    let owner = UserBuilder::new_default("Tour Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap();

    let tournament = TournamentBuilder::new_default("Test Tour")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap();

    let division = DivisionBuilder::new_default("Test Div", tournament.tid)
        .build_and_insert(db)
        .unwrap();

    let admin_user = UserBuilder::new_default("Tour Admin")
        .set_hash_password("AdminPwd123!")
        .build_and_insert(db)
        .unwrap();
    TournamentAdminBuilder::new_default(tournament.tid, admin_user.id)
        .build_and_insert(db)
        .unwrap();

    let unrelated_user = UserBuilder::new_default("Unrelated User")
        .set_hash_password("UnrelPwd123!")
        .build_and_insert(db)
        .unwrap();

    (tournament, division, owner, admin_user, unrelated_user)
}

/// Returns `(tournament, division, team, owner, admin_user, unrelated_user)` for testing
/// team update ABAC: owner and admin should be allowed (conditions 2 & 3),
/// and any user with `team:update` permission should be allowed (condition 1).
pub fn arrange_team_update_works_integration_test(
    db: &mut database::Connection,
) -> (Tournament, Division, Team, User, User, User) {
    let owner = UserBuilder::new_default("Tour Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap();

    let tournament = TournamentBuilder::new_default("Test Tour")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap();

    let division = DivisionBuilder::new_default("Test Div", tournament.tid)
        .build_and_insert(db)
        .unwrap();

    let team = TeamBuilder::new_default(division.did)
        .set_name("Team to Update")
        .set_coachid(create_and_insert_user(db, "InitialCoach", "CoachPwd123!").id)
        .build_and_insert(db)
        .unwrap();

    let admin_user = UserBuilder::new_default("Tour Admin")
        .set_hash_password("AdminPwd123!")
        .build_and_insert(db)
        .unwrap();
    TournamentAdminBuilder::new_default(tournament.tid, admin_user.id)
        .build_and_insert(db)
        .unwrap();

    let unrelated_user = UserBuilder::new_default("Unrelated User")
        .set_hash_password("UnrelPwd123!")
        .build_and_insert(db)
        .unwrap();

    (tournament, division, team, owner, admin_user, unrelated_user)
}

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
