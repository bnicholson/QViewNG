use backend::{database, models::{division::DivisionBuilder, game::{Game, GameBuilder}, game_statsgroup::{GameStatsGroup, GameStatsGroupBuilder, NewGameStatsGroup}, room::RoomBuilder, round::RoundBuilder, statsgroup::{NewStatsGroup, StatsGroup, StatsGroupBuilder}, team::TeamBuilder, tournament::TournamentBuilder, user::UserBuilder}};

use crate::fixtures::games::seed_1_game_with_minimum_required_dependencies;

pub fn arrange_create_works_integration_test() -> NewStatsGroup {
    StatsGroupBuilder::new_default("Test StatsGroup 2217")
        .set_description(Some("StatsGroup for integration test create.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (StatsGroup, StatsGroup) {
    (
        StatsGroupBuilder::new_default("Test StatsGroup 1")
            .set_description(Some("This is StatsGroup 1's description.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        StatsGroupBuilder::new_default("Test StatsGroup 2")
            .set_description(Some("This is StatsGroup 2's description.".to_string()))
            .build_and_insert(db)
            .unwrap()
    )
}

pub fn arrange_get_statsgroup_by_id_integration_test(db: &mut database::Connection) -> StatsGroup {
    StatsGroupBuilder::new_default("Test StatsGroup 1")
        .set_description(Some("This is StatsGroup 1's description.".to_string()))
        .build_and_insert(db)
        .unwrap();
    StatsGroupBuilder::new_default("Test StatsGroup 2")
        .set_description(Some("This is StatsGroup 2's description.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> StatsGroup {
    StatsGroupBuilder::new_default("Test StatsGroup 1")
        .set_description(Some("StatsGroup 1 testing update.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> StatsGroup {
    StatsGroupBuilder::new_default("Test StatsGroup 1")
        .set_description(Some("StatsGroup 1 testing delete.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_add_game_to_statsgroup_works_integration_test(db: &mut database::Connection) -> (StatsGroup, Game, NewGameStatsGroup) {
    let (game, _, _, _, _, _, _, _, _, _) = seed_1_game_with_minimum_required_dependencies(db);
    let statsgroup = StatsGroupBuilder::new_default("Test StatsGroup for adding games")
        .set_description(Some("StatsGroup for testing adding games.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let new_game_statsgroup = GameStatsGroupBuilder::new(game.gid, statsgroup.sgid)
        .build()
        .unwrap();
    (statsgroup, game, new_game_statsgroup)
}
