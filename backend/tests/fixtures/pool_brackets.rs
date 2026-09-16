use backend::database;
use backend::models::division::DivisionBuilder;
use backend::models::roundgroup::RoundGroupBuilder;
use backend::models::game::GameBuilder;
use backend::models::pool_bracket::PoolBracketBuilder;
use backend::models::room::RoomBuilder;
use backend::models::round::RoundBuilder;
use backend::models::team::TeamBuilder;
use backend::models::team_teamgroup::TeamTeamgroupBuilder;
use backend::models::teamgroup::TeamGroupBuilder;
use backend::models::tournament::TournamentBuilder;
use backend::models::user::UserBuilder;
use uuid::Uuid;

/// Seeds one pool bracket ("pool" type) with a 1-to-1 teamgroup holding three associated teams, and
/// two games whose `poolbracket_id` is that bracket. Returns the pool bracket id. Used to exercise
/// the pool-bracket team-rows (via teamgroup) and game-rows (via poolbracket_id) endpoints.
pub fn seed_pool_bracket_profile(db: &mut database::Connection) -> Uuid {
    let owner = UserBuilder::new_default("PB Owner")
        .set_hash_password("OwnerPwd123!")
        .build_and_insert(db)
        .unwrap();
    let qm = UserBuilder::new_default("PB QM")
        .set_hash_password("QmPwd123!")
        .build_and_insert(db)
        .unwrap();
    let coach = UserBuilder::new_default("PB Coach")
        .set_hash_password("CoachPwd123!")
        .build_and_insert(db)
        .unwrap();

    let tournament = TournamentBuilder::new_default("PB Tour")
        .set_owner_id(owner.id)
        .build_and_insert(db)
        .unwrap();
    let division = DivisionBuilder::new_default("PB Div", tournament.tid)
        .build_and_insert(db)
        .unwrap();
    let roundgroup = RoundGroupBuilder::new(division.did)
        .set_name("Pool Play")
        .set_creator_userid(owner.id)
        .build_and_insert(db)
        .unwrap();
    let bracket = PoolBracketBuilder::new(roundgroup.did)
        .set_name("Pool A")
        .set_type("pool")
        .set_creator_userid(owner.id)
        .build_and_insert(db)
        .unwrap();

    // 1-to-1 teamgroup for the bracket, with three teams associated via team_teamgroups.
    let group = TeamGroupBuilder::new(bracket.pool_bracket_id)
        .set_creator_userid(owner.id)
        .build_and_insert(db)
        .unwrap();

    let mut teams = Vec::new();
    for name in ["Team 1", "Team 2", "Team 3"] {
        let team = TeamBuilder::new_default(division.did)
            .set_name(name)
            .set_coachid(coach.id)
            .build_and_insert(db)
            .unwrap();
        TeamTeamgroupBuilder::new(team.teamid, group.team_group_id)
            .set_creator_userid(owner.id)
            .build_and_insert(db)
            .unwrap();
        teams.push(team);
    }

    // Two games bound to this pool bracket, played in parallel rooms of the same round. They must
    // differ on a column of the games (org, roomid, roundid, clientkey) unique key, so give each its
    // own room rather than colliding on the same (room, round, empty clientkey).
    let room_1 = RoomBuilder::new_default("Room 1", tournament.tid)
        .build_and_insert(db)
        .unwrap();
    let room_2 = RoomBuilder::new_default("Room 2", tournament.tid)
        .build_and_insert(db)
        .unwrap();
    let round = RoundBuilder::new_default(roundgroup.roundgroup_id)
        .set_name("1")
        .build_and_insert(db)
        .unwrap();

    GameBuilder::new_default(room_1.roomid, round.roundid)
        .set_leftteamid(teams[0].teamid)
        .set_rightteamid(teams[1].teamid)
        .set_quizmasterid(qm.id)
        .set_poolbracket_id(bracket.pool_bracket_id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_2.roomid, round.roundid)
        .set_leftteamid(teams[0].teamid)
        .set_rightteamid(teams[2].teamid)
        .set_quizmasterid(qm.id)
        .set_poolbracket_id(bracket.pool_bracket_id)
        .build_and_insert(db)
        .unwrap();

    bracket.pool_bracket_id
}
