use backend::{database, models::{game::Game, game_statsgroup::{GameStatsGroup, GameStatsGroupBuilder, NewGameStatsGroup}, gameevent::{GameEventBuilder, GameEventCode}, statsgroup::{NewStatsGroup, StatsGroup, StatsGroupBuilder}, tournament::TournamentBuilder, user::UserBuilder}};

use crate::fixtures::games::{seed_1_game_with_minimum_required_dependencies, seed_2_games_1_round_with_minimum_required_dependencies};

// StatsGroups now require a tournament_id (FK). Create a throwaway owner + tournament
// and return its id for fixtures that don't otherwise have a tournament handy.
fn seed_tournament_id(db: &mut database::Connection) -> uuid::Uuid {
    let owner = UserBuilder::new_default("StatsGroup Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap();
    TournamentBuilder::new_default("StatsGroup Tournament")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap()
        .tid
}

pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewStatsGroup {
    let tournament_id = seed_tournament_id(db);
    StatsGroupBuilder::new_default("Test StatsGroup 2217", tournament_id)
        .set_description(Some("StatsGroup for integration test create.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (StatsGroup, StatsGroup) {
    let tournament_id = seed_tournament_id(db);
    (
        StatsGroupBuilder::new_default("Test StatsGroup 1", tournament_id)
            .set_description(Some("This is StatsGroup 1's description.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        StatsGroupBuilder::new_default("Test StatsGroup 2", tournament_id)
            .set_description(Some("This is StatsGroup 2's description.".to_string()))
            .build_and_insert(db)
            .unwrap()
    )
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> StatsGroup {
    let tournament_id = seed_tournament_id(db);
    StatsGroupBuilder::new_default("Test StatsGroup 1", tournament_id)
        .set_description(Some("StatsGroup 1 testing update.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> StatsGroup {
    let tournament_id = seed_tournament_id(db);
    StatsGroupBuilder::new_default("Test StatsGroup 1", tournament_id)
        .set_description(Some("StatsGroup 1 testing delete.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_add_game_to_statsgroup_works_integration_test(db: &mut database::Connection) -> (StatsGroup, Game, NewGameStatsGroup) {
    let (game, tour, _, _, _, _, _, _, _, _) = seed_1_game_with_minimum_required_dependencies(db);
    let statsgroup = StatsGroupBuilder::new_default("Test StatsGroup for adding games", tour.tid)
        .set_description(Some("StatsGroup for testing adding games.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let new_game_statsgroup = GameStatsGroupBuilder::new(game.gid, statsgroup.sgid)
        .build()
        .unwrap();
    (statsgroup, game, new_game_statsgroup)
}

pub fn arrange_remove_game_from_statsgroup_works_integration_test(db: &mut database::Connection) -> (StatsGroup, Game, GameStatsGroup) {
    let (game, tour, _, _, _, _, _, _, _, _) = seed_1_game_with_minimum_required_dependencies(db);
    let statsgroup = StatsGroupBuilder::new_default("Test StatsGroup for removing games", tour.tid)
        .set_description(Some("StatsGroup for testing removing games.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let game_statsgroup = GameStatsGroupBuilder::new(game.gid, statsgroup.sgid)
        .build_and_insert(db)
        .unwrap();
    (statsgroup, game, game_statsgroup)
}

pub fn arrange_get_all_games_of_statsgroup_works_integration_test(db: &mut database::Connection) -> (StatsGroup, Game, Game) {
    let (game_1, game_2, tour, _, _, _, _) =
        seed_2_games_1_round_with_minimum_required_dependencies(db);
    let statsgroup = StatsGroupBuilder::new_default("Test StatsGroup for getting all games", tour.tid)
        .set_description(Some("StatsGroup for testing getting all games.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let _game_statsgroup_1 = GameStatsGroupBuilder::new(game_1.gid, statsgroup.sgid)
        .build_and_insert(db)
        .unwrap();
    let _game_statsgroup_2 = GameStatsGroupBuilder::new(game_2.gid, statsgroup.sgid)
        .build_and_insert(db)
        .unwrap();
    (statsgroup, game_1, game_2)
}

// ── Team-stats endpoint fixture ───────────────────────────────────────────────
// Seeds one example game modeled on the QuizMachine export (ExportQuizzes4.csv):
// an initialization block (RM/QT, then each team's TN, five QN, captain SC and
// co-captain SS) followed by correct tossups. "Red Team" answers 4 tossups and
// "Blue Team" answers 2, so final scores are Red 80, Blue 40 (20 pts per tossup).
// The game is linked to a division-scoped statsgroup, which is returned.
pub fn arrange_team_stats_works_integration_test(db: &mut database::Connection) -> StatsGroup {
    let (game, tour, division, _round, _room, _team_1, _team_2, _coach_1, _coach_2, _quizmaster) =
        seed_1_game_with_minimum_required_dependencies(db);

    let statsgroup = StatsGroupBuilder::new_default("Team Stats StatsGroup", tour.tid)
        .set_division_id(Some(division.did))
        .build_and_insert(db)
        .unwrap();
    GameStatsGroupBuilder::new(game.gid, statsgroup.sgid)
        .build_and_insert(db)
        .unwrap();

    seed_example_game_events(db, game.gid);

    statsgroup
}

fn seed_example_game_events(db: &mut database::Connection, gid: uuid::Uuid) {
    fn ev(db: &mut database::Connection, gid: uuid::Uuid, q: i32, e: i32, name: &str, team: i32, quizzer: i32, code: GameEventCode) {
        GameEventBuilder::new_default(gid)
            .set_question(Some(q))
            .set_eventnum(Some(e))
            .set_name(Some(name.to_string()))
            .set_team(Some(team))
            .set_quizzer(Some(quizzer))
            .set_event(Some(code))
            .build_and_insert(db)
            .unwrap();
    }

    // ── Question 1: initialization block ──
    ev(db, gid, 1, 0, "Tournament", 0, 0, GameEventCode::RM);
    ev(db, gid, 1, 1, "N", 0, 0, GameEventCode::QT);
    // Red team (team 0)
    ev(db, gid, 1, 2, "Red Team", 0, 0, GameEventCode::TN);
    ev(db, gid, 1, 3, "Red #1", 0, 0, GameEventCode::QN);
    ev(db, gid, 1, 4, "Red #2", 0, 1, GameEventCode::QN);
    ev(db, gid, 1, 5, "Red #3", 0, 2, GameEventCode::QN);
    ev(db, gid, 1, 6, "Red #4", 0, 3, GameEventCode::QN);
    ev(db, gid, 1, 7, "Red #5", 0, 4, GameEventCode::QN);
    ev(db, gid, 1, 8, "Red #1", 0, 0, GameEventCode::SC);
    ev(db, gid, 1, 9, "Red #2", 0, 1, GameEventCode::SS);
    // Blue team (team 1)
    ev(db, gid, 1, 10, "Blue Team", 1, 1, GameEventCode::TN);
    ev(db, gid, 1, 11, "Blue #1", 1, 0, GameEventCode::QN);
    ev(db, gid, 1, 12, "Blue #2", 1, 1, GameEventCode::QN);
    ev(db, gid, 1, 13, "Blue #3", 1, 2, GameEventCode::QN);
    ev(db, gid, 1, 14, "Blue #4", 1, 3, GameEventCode::QN);
    ev(db, gid, 1, 15, "Blue #5", 1, 4, GameEventCode::QN);
    ev(db, gid, 1, 16, "Blue #1", 1, 0, GameEventCode::SC);
    ev(db, gid, 1, 17, "Blue #2", 1, 1, GameEventCode::SS);
    // First tossup happens within question 1 (as in the export).
    ev(db, gid, 1, 18, "Red #1", 0, 0, GameEventCode::TC);

    // ── Remaining tossups ── Red answers 3 more, Blue answers 2.
    ev(db, gid, 2, 0, "Red #1", 0, 0, GameEventCode::TC);
    ev(db, gid, 3, 0, "Red #1", 0, 0, GameEventCode::TC);
    ev(db, gid, 4, 0, "Red #1", 0, 0, GameEventCode::TC);
    ev(db, gid, 5, 0, "Blue #1", 1, 0, GameEventCode::TC);
    ev(db, gid, 6, 0, "Blue #1", 1, 0, GameEventCode::TC);
}
