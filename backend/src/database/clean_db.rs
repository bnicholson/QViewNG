use crate::database;
use crate::schema::{
    activation_tokens, apicalllog, computers, create_tournament_applicants, divisions, equipment, equipmentregistrations, equipmentsets, extensioncords, gameeventlogs, gameevents, games, interfaceboxes, jumppads, microphonerecorders, password_reset_tokens, permissions, pool_brackets, poolbracketgroups, projectors, roles, roles_permissions, rooms, roomgroups, rosters, rosters_coaches, rosters_quizzers, rounds, statsgroups, team_teamgroups, teamgroups, teams, tournamentgroups, tournamentgroups_tournaments, tournaments, tournaments_admins, user_sessions, users, users_roles
};
use chrono::Utc;
use diesel::prelude::*;

/// Empties every table, children first so foreign keys never block a delete. The integration tests
/// run this once on their template database (tests/it/common/mod.rs) and by `reset_test_db` for the
/// few tests that start over mid-test.
pub fn clean_database(conn: &mut database::Connection) {

    let start_time_for_db_purge = Utc::now();
    println!("Starting DB Purge");

    diesel::delete(team_teamgroups::table)
        .execute(conn)
        .expect("Failed to clean team_teamgroups");

    diesel::delete(roles_permissions::table)
        .execute(conn)
        .expect("Failed to clean roles_permissions");

    diesel::delete(users_roles::table)
        .execute(conn)
        .expect("Failed to clean users_roles");

    diesel::delete(gameeventlogs::table)
        .execute(conn)
        .expect("Failed to clean gameeventlogs");

    diesel::delete(gameevents::table)
        .execute(conn)
        .expect("Failed to clean gameevents");

    diesel::delete(permissions::table)
        .execute(conn)
        .expect("Failed to clean permissions");

    diesel::delete(roles::table)
        .execute(conn)
        .expect("Failed to clean roles");

    diesel::delete(apicalllog::table)
        .execute(conn)
        .expect("Failed to clean apicalllog");

    diesel::delete(equipmentregistrations::table)
        .execute(conn)
        .expect("Failed to clean equipmentregistrations");

    diesel::delete(equipment::table)
        .execute(conn)
        .expect("Failed to clean equipment");

    diesel::delete(extensioncords::table)
        .execute(conn)
        .expect("Failed to clean extensioncords");

    diesel::delete(projectors::table)
        .execute(conn)
        .expect("Failed to clean projectors");

    diesel::delete(microphonerecorders::table)
        .execute(conn)
        .expect("Failed to clean microphonerecorders");

    diesel::delete(interfaceboxes::table)
        .execute(conn)
        .expect("Failed to clean interfaceboxes");

    diesel::delete(jumppads::table)
        .execute(conn)
        .expect("Failed to clean jumppads");

    diesel::delete(computers::table)
        .execute(conn)
        .expect("Failed to clean computers");

    diesel::delete(equipmentsets::table)
        .execute(conn)
        .expect("Failed to clean equipmentsets");

    diesel::delete(rosters_coaches::table)
        .execute(conn)
        .expect("Failed to clean rosters_coaches");

    diesel::delete(rosters_quizzers::table)
        .execute(conn)
        .expect("Failed to clean rosters_quizzers");

    diesel::delete(rosters::table)
        .execute(conn)
        .expect("Failed to clean rosters");

    diesel::delete(statsgroups::table)
        .execute(conn)
        .expect("Failed to clean statsgroups");

    diesel::delete(tournamentgroups_tournaments::table)
        .execute(conn)
        .expect("Failed to clean tournamentgroups_tournaments");

    diesel::delete(tournamentgroups::table)
        .execute(conn)
        .expect("Failed to clean tournamentgroups");
    
    diesel::delete(games::table)
        .execute(conn)
        .expect("Failed to clean games");

    diesel::delete(teams::table)
        .execute(conn)
        .expect("Failed to clean teams");

    diesel::delete(rounds::table)
        .execute(conn)
        .expect("Failed to clean rounds");

    diesel::delete(tournaments_admins::table)
        .execute(conn)
        .expect("Failed to clean admins of tournaments");

    diesel::delete(user_sessions::table)
        .execute(conn)
        .expect("Failed to clean user sessions");

    diesel::delete(password_reset_tokens::table)
        .execute(conn)
        .expect("Failed to clean password reset tokens");

    diesel::delete(activation_tokens::table)
        .execute(conn)
        .expect("Failed to clean activation tokens");

    // pool_brackets now holds the FK to teamgroups (reversed 1:1), so brackets must go first.
    diesel::delete(pool_brackets::table)
        .execute(conn)
        .expect("Failed to clean pool_brackets");

    // poolbracketgroups is referenced by pool_brackets (cleaned above) and references divisions
    // (cleaned below), so it goes after pool_brackets and before divisions.
    diesel::delete(poolbracketgroups::table)
        .execute(conn)
        .expect("Failed to clean poolbracketgroups");

    diesel::delete(teamgroups::table)
        .execute(conn)
        .expect("Failed to clean teamgroups");

    diesel::delete(rooms::table)
        .execute(conn)
        .expect("Failed to clean rooms");

    // roomgroups is referenced by rooms (cleaned above) and references tournaments (cleaned below),
    // so it goes after rooms and before tournaments.
    diesel::delete(roomgroups::table)
        .execute(conn)
        .expect("Failed to clean roomgroups");

    diesel::delete(divisions::table)
        .execute(conn)
        .expect("Failed to clean divisions");

    diesel::delete(tournaments::table)
        .execute(conn)
        .expect("Failed to clean tournaments");

    diesel::delete(create_tournament_applicants::table)
        .execute(conn)
        .expect("Failed to clean create_tournament_applicants");

    diesel::delete(users::table)
        .execute(conn)
        .expect("Failed to clean users");

    let end_time_for_db_purge = Utc::now();
    let duration_for_db_purge = end_time_for_db_purge.naive_utc() - start_time_for_db_purge.naive_utc();
    println!("Done. DB Purging Time Duration: {}\n", duration_for_db_purge);
}
