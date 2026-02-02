use backend::{database, models::roster::{NewRoster, Roster, RosterBuilder}};


pub fn arrange_create_works_integration_test() -> NewRoster {
    RosterBuilder::new("Test Roster 2317")
        .set_description(Some("Roster for integration test create.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (Roster, Roster) {
    (
        RosterBuilder::new_default("Test Roster 1")
            .set_description(Some("This is Roster 1's description.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        RosterBuilder::new_default("Test Roster 2")
            .set_description(Some("This is Roster 2's description.".to_string()))
            .build_and_insert(db)
            .unwrap()
    )
}

// pub fn arrange_get_statsgroup_by_id_integration_test(db: &mut database::Connection) -> Roster {
//     RosterBuilder::new_default("Test Roster 1")
//         .set_description(Some("This is Roster 1's description.".to_string()))
//         .build_and_insert(db)
//         .unwrap();
//     RosterBuilder::new_default("Test Roster 2")
//         .set_description(Some("This is Roster 2's description.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }

// pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> Roster {
//     RosterBuilder::new_default("Test Roster 1")
//         .set_description(Some("Roster 1 testing update.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }

// pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> Roster {
//     RosterBuilder::new_default("Test Roster 1")
//         .set_description(Some("Roster 1 testing delete.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }

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
