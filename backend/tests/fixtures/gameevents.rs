use backend::{database, models::{game::Game, gameevent::{GameEvent, GameEventBuilder, NewGameEvent}}};
use crate::fixtures::games::{seed_1_game_with_minimum_required_dependencies, seed_2_games_1_round_with_minimum_required_dependencies};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewGameEvent {
    let (game, _, _, _, _, _, _, _, _, _) = seed_1_game_with_minimum_required_dependencies(db);
    GameEventBuilder::new_default(game.gid)
        .set_question(Some(1))
        .set_eventnum(Some(1))
        .set_name(Some("Tori".to_string()))
        .set_team(Some(0))
        .set_quizzer(Some(2))
        .set_event(Some("TC".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (GameEvent, GameEvent) {
    let (game, _, _, _, _, _, _, _, _, _) = seed_1_game_with_minimum_required_dependencies(db);
    let gameevent_1 = GameEventBuilder::new_default(game.gid)
        .set_question(Some(1))
        .set_eventnum(Some(1))
        .set_name(Some("Tori".to_string()))
        .set_team(Some(0))
        .set_quizzer(Some(2))
        .set_event(Some("TC".to_string()))
        .build_and_insert(db)
        .unwrap();
    let gameevent_2 = GameEventBuilder::new_default(game.gid)
        .set_question(Some(2))
        .set_eventnum(Some(1))
        .set_name(Some("Kevin".to_string()))
        .set_team(Some(1))
        .set_quizzer(Some(2))
        .set_event(Some("TC".to_string()))
        .build_and_insert(db)
        .unwrap();
    (gameevent_1, gameevent_2)
}

pub fn arrange_get_gameevent_by_id_works_integration_test(db: &mut database::Connection) -> GameEvent {
    let (game, _, _, _, _, _, _, _, _, _) = seed_1_game_with_minimum_required_dependencies(db);
    GameEventBuilder::new_default(game.gid)
        .set_question(Some(1))
        .set_eventnum(Some(1))
        .set_name(Some("Tori".to_string()))
        .set_team(Some(0))
        .set_quizzer(Some(2))
        .set_event(Some("TC".to_string()))
        .build_and_insert(db)
        .unwrap();
    GameEventBuilder::new_default(game.gid)
        .set_question(Some(2))
        .set_eventnum(Some(1))
        .set_name(Some("Kevin".to_string()))
        .set_team(Some(1))
        .set_quizzer(Some(2))
        .set_event(Some("TC".to_string()))
        .build_and_insert(db)
        .unwrap()
}
