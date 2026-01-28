use backend::{database, models::{division::DivisionBuilder, game::{Game, GameBuilder, NewGame}, room::RoomBuilder, round::RoundBuilder, team::TeamBuilder, tournament::TournamentBuilder, user::UserBuilder}};
use diesel::prelude::*;
use uuid::Uuid;
use backend::schema::games;
use crate::fixtures;

pub fn seed_game_payload_dependencies(db: &mut database::Connection, tname: &str) -> (Uuid,Uuid,Uuid,Uuid,Uuid,Uuid,Uuid,Uuid) {
    let tournament = fixtures::tournaments::seed_tournament(db, tname);
    let division = fixtures::divisions::seed_division(db, tournament.tid);
    let room = fixtures::rooms::seed_room(db, tournament.tid);
    let round = fixtures::rounds::seed_round(db, division.did);
    let teams = fixtures::teams::seed_teams_with_names(db, division.did, "Jeffs Team", "Sams Team", "Scotts Team");
    let quizmaster_user = fixtures::users::seed_user(db); 
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
    GameBuilder::new_default(room_id, round_id)
        .set_tournamentid(Some(tid))
        .set_divisionid(Some(did))
        .set_leftteamid(left_team_id)
        .set_centerteamid(center_team_id)
        .set_rightteamid(right_team_id)
        .set_quizmasterid(qm_id)
        .build()
        .unwrap()
}

pub fn create_and_insert_game(db: &mut database::Connection, new_game: NewGame) -> Game {
    diesel::insert_into(games::table)
        .values(new_game)
        .returning(Game::as_returning())
        .get_result::<Game>(db)
        .expect("Failed to create game")
}

pub fn seed_game(db: &mut database::Connection) -> Game {
    let deps = seed_game_payload_dependencies(db, "Tour 1");
    let payload = get_game_payload(deps.0,deps.1,deps.2,deps.3,deps.4,Some(deps.5),deps.6,deps.7);
    create_and_insert_game(db, payload)
}

pub fn seed_games(db: &mut database::Connection) -> Vec<Game> {
    let deps_1 = seed_game_payload_dependencies(db, "Tour 1");
    let payload_1 = get_game_payload(deps_1.0,deps_1.1,deps_1.2,deps_1.3,deps_1.4,Some(deps_1.5),deps_1.6,deps_1.7);
    let game_1 = create_and_insert_game(db, payload_1);

    let deps_2 = seed_game_payload_dependencies(db, "Tour 2");
    let payload_2 = get_game_payload(deps_2.0,deps_2.1,deps_2.2,deps_2.3,deps_2.4,Some(deps_2.5),deps_2.6,deps_2.7);
    let game_2 = create_and_insert_game(db, payload_2);

    vec![game_1, game_2]
}

pub fn duplicate_team_in_game_case_one_payload(db: &mut database::Connection) -> NewGame {
    let deps_1 = seed_game_payload_dependencies(db, "Tour 1");
    get_game_payload(deps_1.0,deps_1.1,deps_1.2,deps_1.3,deps_1.4,None,deps_1.4,deps_1.7)
}

pub fn duplicate_team_in_game_case_two_payload(db: &mut database::Connection) -> NewGame {
    let deps_1 = seed_game_payload_dependencies(db, "Tour 2");
    get_game_payload(deps_1.0,deps_1.1,deps_1.2,deps_1.3,deps_1.4,Some(deps_1.4),deps_1.6,deps_1.7)
}

pub fn duplicate_team_in_game_case_three_payload(db: &mut database::Connection) -> NewGame {
    let deps_1 = seed_game_payload_dependencies(db, "Tour 3");
    get_game_payload(deps_1.0,deps_1.1,deps_1.2,deps_1.3,deps_1.4,Some(deps_1.6),deps_1.6,deps_1.7)
}

pub fn seed_get_games_of_round(db: &mut database::Connection) -> (Game, Game) {  // return Game because it contains gid and roundid (and roomid)

    // Required: multiple rounds (2 minimum) with multiple games (2 minimum) in each round

    let qm_1 = UserBuilder::new_default("Grace")
        .set_hash_password("gfjhfkhfkjhkgggh")
        .build_and_insert(db)
        .unwrap();

    let qm_2 = UserBuilder::new_default("Tyler")
        .set_hash_password("gfjhfkhfkj9kgggh")
        .build_and_insert(db)
        .unwrap();

    let coach_1 = UserBuilder::new_default("James")
        .set_hash_password("gfj5fkhfkjhkgggh")
        .build_and_insert(db)
        .unwrap();

    let coach_2 = UserBuilder::new_default("Charlie")
        .set_hash_password("gfjhfkhfkjhkg0gh")
        .build_and_insert(db)
        .unwrap();

    let coach_3 = UserBuilder::new_default("Janet")
        .set_hash_password("gfjhfkhfkjhxxkgggh")
        .build_and_insert(db)
        .unwrap();

    let coach_4 = UserBuilder::new_default("Pam")
        .set_hash_password("gfjhfkgggh")
        .build_and_insert(db)
        .unwrap();


    let tour_1 = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();


    let room_1 = RoomBuilder::new_default("Room 1", tour_1.tid)
        .build_and_insert(db)
        .unwrap();

    let room_2 = RoomBuilder::new_default("Room 2", tour_1.tid)
        .build_and_insert(db)
        .unwrap();


    let div_1 = DivisionBuilder::new_default("Div 1", tour_1.tid)
        .build_and_insert(db)
        .unwrap();


    let team_1 = TeamBuilder::new_default(div_1.did)
        .set_name("Team 1")
        .set_coachid(coach_1.id)
        .build_and_insert(db)
        .unwrap();

    let team_2 = TeamBuilder::new_default(div_1.did)
        .set_name("Team 2")
        .set_coachid(coach_2.id)
        .build_and_insert(db)
        .unwrap();

    let team_3 = TeamBuilder::new_default(div_1.did)
        .set_name("Team 3")
        .set_coachid(coach_3.id)
        .build_and_insert(db)
        .unwrap();

    let team_4 = TeamBuilder::new_default(div_1.did)
        .set_name("Team 4")
        .set_coachid(coach_4.id)
        .build_and_insert(db)
        .unwrap();


    let round_1 = RoundBuilder::new_default(div_1.did)
        .build_and_insert(db)
        .unwrap();

    let round_2 = RoundBuilder::new_default(div_1.did)
        .build_and_insert(db)
        .unwrap();


    let game_1 = GameBuilder::new_default(room_1.roomid, round_1.roundid)
        .set_leftteamid(team_1.teamid)
        .set_rightteamid(team_2.teamid)
        .set_quizmasterid(qm_1.id)
        .build_and_insert(db)
        .unwrap();

    let game_2 = GameBuilder::new_default(room_2.roomid, round_1.roundid)
        .set_leftteamid(team_3.teamid)
        .set_rightteamid(team_4.teamid)
        .set_quizmasterid(qm_2.id)
        .build_and_insert(db)
        .unwrap();

    let game_3 = GameBuilder::new_default(room_1.roomid, round_2.roundid)
        .set_tournamentid(Some(tour_1.tid))
        .set_leftteamid(team_3.teamid)
        .set_rightteamid(team_2.teamid)
        .set_quizmasterid(qm_1.id)
        .build_and_insert(db)
        .unwrap();

    let game_4 = GameBuilder::new_default(room_2.roomid, round_2.roundid)
        .set_tournamentid(Some(tour_1.tid))
        .set_leftteamid(team_1.teamid)
        .set_rightteamid(team_4.teamid)
        .set_quizmasterid(qm_2.id)
        .build_and_insert(db)
        .unwrap();

    (game_3, game_4)  // returning all Games of Round 2
}

pub fn seed_get_games_of_division(db: &mut database::Connection) -> (Uuid, Game, Game) {  // return Game because it contains gid and roundid (and roomid)

    // Required: multiple divisions (2 minimum) with multiple games (2 minimum) in each division

    let qm_1 = UserBuilder::new_default("Grace")
        .set_hash_password("gfjhfkhfkjhkgggh")
        .build_and_insert(db)
        .unwrap();

    let qm_2 = UserBuilder::new_default("Tyler")
        .set_hash_password("gfjhfkhfkj9kgggh")
        .build_and_insert(db)
        .unwrap();

    let qm_3 = UserBuilder::new_default("Jasmine")
        .set_hash_password("fun")
        .build_and_insert(db)
        .unwrap();

    let qm_4 = UserBuilder::new_default("Stephanie")
        .set_hash_password("777787787")
        .build_and_insert(db)
        .unwrap();

    let coach_1 = UserBuilder::new_default("James")
        .set_hash_password("gfj5fkhfkjhkgggh")
        .build_and_insert(db)
        .unwrap();

    let coach_2 = UserBuilder::new_default("Charlie")
        .set_hash_password("gfjhfkhfkjhkg0gh")
        .build_and_insert(db)
        .unwrap();

    let coach_3 = UserBuilder::new_default("Janet")
        .set_hash_password("gfjhfkhfkjhxxkgggh")
        .build_and_insert(db)
        .unwrap();

    let coach_4 = UserBuilder::new_default("Pam")
        .set_hash_password("gfjhfkgggh")
        .build_and_insert(db)
        .unwrap();


    let tour_1 = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();


    let room_1 = RoomBuilder::new_default("Room 1", tour_1.tid)
        .build_and_insert(db)
        .unwrap();

    let room_2 = RoomBuilder::new_default("Room 2", tour_1.tid)
        .build_and_insert(db)
        .unwrap();


    let div_1 = DivisionBuilder::new_default("Div 1", tour_1.tid)
        .build_and_insert(db)
        .unwrap();

    let div_2 = DivisionBuilder::new_default("Div 2", tour_1.tid)
        .build_and_insert(db)
        .unwrap();


    let team_1 = TeamBuilder::new_default(div_1.did)
        .set_name("Team 1")
        .set_coachid(coach_1.id)
        .build_and_insert(db)
        .unwrap();

    let team_2 = TeamBuilder::new_default(div_1.did)
        .set_name("Team 2")
        .set_coachid(coach_2.id)
        .build_and_insert(db)
        .unwrap();

    let team_3 = TeamBuilder::new_default(div_1.did)
        .set_name("Team 3")
        .set_coachid(coach_3.id)
        .build_and_insert(db)
        .unwrap();

    let team_4 = TeamBuilder::new_default(div_1.did)
        .set_name("Team 4")
        .set_coachid(coach_4.id)
        .build_and_insert(db)
        .unwrap();

    let team_5 = TeamBuilder::new_default(div_2.did)
        .set_name("Team 5")
        .set_coachid(coach_1.id)
        .build_and_insert(db)
        .unwrap();

    let team_6 = TeamBuilder::new_default(div_2.did)
        .set_name("Team 6")
        .set_coachid(coach_2.id)
        .build_and_insert(db)
        .unwrap();

    let team_7 = TeamBuilder::new_default(div_2.did)
        .set_name("Team 7")
        .set_coachid(coach_3.id)
        .build_and_insert(db)
        .unwrap();

    let team_8 = TeamBuilder::new_default(div_2.did)
        .set_name("Team 8")
        .set_coachid(coach_4.id)
        .build_and_insert(db)
        .unwrap();


    let round_1_of_div_1 = RoundBuilder::new_default(div_1.did)
        .build_and_insert(db)
        .unwrap();

    let round_1_of_div_2 = RoundBuilder::new_default(div_2.did)
        .build_and_insert(db)
        .unwrap();


    let game_1 = GameBuilder::new_default(room_1.roomid, round_1_of_div_1.roundid)
        .set_leftteamid(team_1.teamid)
        .set_rightteamid(team_2.teamid)
        .set_quizmasterid(qm_1.id)
        .build_and_insert(db)
        .unwrap();

    let game_2 = GameBuilder::new_default(room_2.roomid, round_1_of_div_1.roundid)
        .set_leftteamid(team_3.teamid)
        .set_rightteamid(team_4.teamid)
        .set_quizmasterid(qm_2.id)
        .build_and_insert(db)
        .unwrap();

    let game_3 = GameBuilder::new_default(room_1.roomid, round_1_of_div_2.roundid)
        .set_tournamentid(Some(tour_1.tid))
        .set_leftteamid(team_5.teamid)
        .set_rightteamid(team_6.teamid)
        .set_quizmasterid(qm_1.id)
        .build_and_insert(db)
        .unwrap();

    let game_4 = GameBuilder::new_default(room_2.roomid, round_1_of_div_2.roundid)
        .set_tournamentid(Some(tour_1.tid))
        .set_leftteamid(team_7.teamid)
        .set_rightteamid(team_8.teamid)
        .set_quizmasterid(qm_2.id)
        .build_and_insert(db)
        .unwrap();

    (div_2.did, game_3, game_4)  // returning all Games of Division 2
}

pub fn seed_get_games_of_tournament(db: &mut database::Connection) -> (Uuid, Game, Game) {  // return Game because it contains gid and roundid (and roomid)

    // Required: multiple tournaments (2 minimum) with multiple games (2 minimum) in each tournament

    let qm_1 = UserBuilder::new_default("Grace")
        .set_hash_password("gfjhfkhfkjhkgggh")
        .build_and_insert(db)
        .unwrap();

    let qm_2 = UserBuilder::new_default("Tyler")
        .set_hash_password("gfjhfkhfkj9kgggh")
        .build_and_insert(db)
        .unwrap();

    let qm_3 = UserBuilder::new_default("Jasmine")
        .set_hash_password("fun")
        .build_and_insert(db)
        .unwrap();

    let qm_4 = UserBuilder::new_default("Stephanie")
        .set_hash_password("777787787")
        .build_and_insert(db)
        .unwrap();

    let coach_1 = UserBuilder::new_default("James")
        .set_hash_password("gfj5fkhfkjhkgggh")
        .build_and_insert(db)
        .unwrap();

    let coach_2 = UserBuilder::new_default("Charlie")
        .set_hash_password("gfjhfkhfkjhkg0gh")
        .build_and_insert(db)
        .unwrap();

    let coach_3 = UserBuilder::new_default("Janet")
        .set_hash_password("gfjhfkhfkjhxxkgggh")
        .build_and_insert(db)
        .unwrap();

    let coach_4 = UserBuilder::new_default("Pam")
        .set_hash_password("gfjhfkgggh")
        .build_and_insert(db)
        .unwrap();


    let tour_1 = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();

    let tour_2 = TournamentBuilder::new_default("Tour 2")
        .build_and_insert(db)
        .unwrap();


    let room_1_of_tour_1 = RoomBuilder::new_default("Room 1", tour_1.tid)
        .build_and_insert(db)
        .unwrap();

    let room_2_of_tour_1 = RoomBuilder::new_default("Room 2", tour_1.tid)
        .build_and_insert(db)
        .unwrap();

    let room_1_of_tour_2 = RoomBuilder::new_default("Room 1", tour_2.tid)
        .build_and_insert(db)
        .unwrap();

    let room_2_of_tour_2 = RoomBuilder::new_default("Room 2", tour_2.tid)
        .build_and_insert(db)
        .unwrap();


    let div_1_of_tour_1 = DivisionBuilder::new_default("Div 1", tour_1.tid)
        .build_and_insert(db)
        .unwrap();

    let div_1_of_tour_2 = DivisionBuilder::new_default("Div 2", tour_2.tid)
        .build_and_insert(db)
        .unwrap();


    let team_1 = TeamBuilder::new_default(div_1_of_tour_1.did)
        .set_name("Team 1")
        .set_coachid(coach_1.id)
        .build_and_insert(db)
        .unwrap();

    let team_2 = TeamBuilder::new_default(div_1_of_tour_1.did)
        .set_name("Team 2")
        .set_coachid(coach_2.id)
        .build_and_insert(db)
        .unwrap();

    let team_3 = TeamBuilder::new_default(div_1_of_tour_1.did)
        .set_name("Team 3")
        .set_coachid(coach_3.id)
        .build_and_insert(db)
        .unwrap();

    let team_4 = TeamBuilder::new_default(div_1_of_tour_1.did)
        .set_name("Team 4")
        .set_coachid(coach_4.id)
        .build_and_insert(db)
        .unwrap();

    let team_5 = TeamBuilder::new_default(div_1_of_tour_2.did)
        .set_name("Team 5")
        .set_coachid(coach_1.id)
        .build_and_insert(db)
        .unwrap();

    let team_6 = TeamBuilder::new_default(div_1_of_tour_2.did)
        .set_name("Team 6")
        .set_coachid(coach_2.id)
        .build_and_insert(db)
        .unwrap();

    let team_7 = TeamBuilder::new_default(div_1_of_tour_2.did)
        .set_name("Team 7")
        .set_coachid(coach_3.id)
        .build_and_insert(db)
        .unwrap();

    let team_8 = TeamBuilder::new_default(div_1_of_tour_2.did)
        .set_name("Team 8")
        .set_coachid(coach_4.id)
        .build_and_insert(db)
        .unwrap();


    let round_1_of_tour_1 = RoundBuilder::new_default(div_1_of_tour_1.did)
        .build_and_insert(db)
        .unwrap();

    let round_1_of_tour_2 = RoundBuilder::new_default(div_1_of_tour_2.did)
        .build_and_insert(db)
        .unwrap();


    let game_1 = GameBuilder::new_default(room_1_of_tour_1.roomid, round_1_of_tour_1.roundid)
        .set_leftteamid(team_1.teamid)
        .set_rightteamid(team_2.teamid)
        .set_quizmasterid(qm_1.id)
        .build_and_insert(db)
        .unwrap();

    let game_2 = GameBuilder::new_default(room_2_of_tour_1.roomid, round_1_of_tour_1.roundid)
        .set_leftteamid(team_3.teamid)
        .set_rightteamid(team_4.teamid)
        .set_quizmasterid(qm_2.id)
        .build_and_insert(db)
        .unwrap();

    let game_3 = GameBuilder::new_default(room_1_of_tour_2.roomid, round_1_of_tour_2.roundid)
        .set_tournamentid(Some(tour_1.tid))
        .set_leftteamid(team_5.teamid)
        .set_rightteamid(team_6.teamid)
        .set_quizmasterid(qm_1.id)
        .build_and_insert(db)
        .unwrap();

    let game_4 = GameBuilder::new_default(room_2_of_tour_2.roomid, round_1_of_tour_2.roundid)
        .set_tournamentid(Some(tour_1.tid))
        .set_leftteamid(team_7.teamid)
        .set_rightteamid(team_8.teamid)
        .set_quizmasterid(qm_2.id)
        .build_and_insert(db)
        .unwrap();

    (tour_2.tid, game_3, game_4)  // returning all Games of Tournament 2
}
