use backend::{
    database, 
    models::{
        roster::{NewRoster, Roster, RosterBuilder}, 
        roster_coach::{NewRosterCoach, RosterCoachBuilder}, 
        user::{User, UserBuilder}
    }
};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewRoster {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    RosterBuilder::new("Test Roster 2317", coach.id)
        .set_description(Some("Roster for integration test create.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (Roster, Roster) {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    (
        RosterBuilder::new("Test Roster 1", coach.id)
            .set_description(Some("This is Roster 1's description.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        RosterBuilder::new("Test Roster 2", coach.id)
            .set_description(Some("This is Roster 2's description.".to_string()))
            .build_and_insert(db)
            .unwrap()
    )
}

pub fn arrange_get_roster_by_id_integration_test(db: &mut database::Connection) -> Roster {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    RosterBuilder::new("Test Roster 1", coach.id)
        .set_description(Some("This is Roster 1's description.".to_string()))
        .build_and_insert(db)
        .unwrap();
    RosterBuilder::new("Test Roster 2", coach.id)
        .set_description(Some("This is Roster 2's description.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> Roster {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    RosterBuilder::new("Test Roster 1", coach.id)
        .set_description(Some("Roster 1 testing update.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> Roster {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    RosterBuilder::new("Test Roster 1", coach.id)
        .set_description(Some("Roster 1 testing delete.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_add_quizzer_to_roster_works_integration_test(db: &mut database::Connection) -> (Roster, User) {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let quizzer = UserBuilder::new_default("Quizzer 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let roster = RosterBuilder::new("Test Roster for adding quizzers", coach.id)
        .set_description(Some("Roster for testing adding quizzers.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (roster, quizzer)
}

pub fn arrange_add_rostercoach_to_roster_works_integration_test(db: &mut database::Connection) -> NewRosterCoach {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let roster = RosterBuilder::new("Roster 1", coach.id)
        .set_description(Some("Roster for testing adding rostercoaches.".to_string()))
        .build_and_insert(db)
        .unwrap();
    RosterCoachBuilder::new_default(coach.id, roster.rosterid)
        .build()
        .unwrap()
}

// pub fn arrange_add_game_to_statsgroup_works_integration_test(db: &mut database::Connection) -> (Roster, Game, NewGameRoster) {
//     let (game, _, _, _, _, _, _, _, _, _) = seed_1_game_with_minimum_required_dependencies(db);
//     let statsgroup = RosterBuilder::new_default("Test Roster for adding games")
//         .set_description(Some("Roster for testing adding games.".to_string()))
//         .build_and_insert(db)
//         .unwrap();
//     let new_game_statsgroup = GameRosterBuilder::new(game.gid, statsgroup.sgid)
//         .build()
//         .unwrap();
//     (statsgroup, game, new_game_statsgroup)
// }

// pub fn arrange_remove_game_from_statsgroup_works_integration_test(db: &mut database::Connection) -> (Roster, Game, GameRoster) {
//     let (game, _, _, _, _, _, _, _, _, _) = seed_1_game_with_minimum_required_dependencies(db);
//     let statsgroup = RosterBuilder::new_default("Test Roster for removing games")
//         .set_description(Some("Roster for testing removing games.".to_string()))
//         .build_and_insert(db)
//         .unwrap();
//     let game_statsgroup = GameRosterBuilder::new(game.gid, statsgroup.sgid)
//         .build_and_insert(db)
//         .unwrap();
//     (statsgroup, game, game_statsgroup)
// }

// pub fn arrange_get_all_games_of_statsgroup_works_integration_test(db: &mut database::Connection) -> (Roster, Game, Game) {
//     let (game_1, game_2, _, _, _, _, _) = 
//         seed_2_games_1_round_with_minimum_required_dependencies(db);
//     let statsgroup = RosterBuilder::new_default("Test Roster for getting all games")
//         .set_description(Some("Roster for testing getting all games.".to_string()))
//         .build_and_insert(db)
//         .unwrap();
//     let _game_statsgroup_1 = GameRosterBuilder::new(game_1.gid, statsgroup.sgid)
//         .build_and_insert(db)
//         .unwrap();
//     let _game_statsgroup_2 = GameRosterBuilder::new(game_2.gid, statsgroup.sgid)
//         .build_and_insert(db)
//         .unwrap();
//     (statsgroup, game_1, game_2)
// }
