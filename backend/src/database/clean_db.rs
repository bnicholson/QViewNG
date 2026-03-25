use crate::database;
use crate::schema::{
    activation_tokens, apicalllog, computers, divisions, equipment, equipmentregistrations, equipmentsets, extensioncords, gameevents, games, interfaceboxes, jumppads, microphonerecorders, password_reset_tokens, permissions, projectors, roles, roles_permissions, rooms, rosters, rosters_coaches, rosters_quizzers, rounds, statsgroups, teams, tournamentgroups, tournamentgroups_tournaments, tournaments, tournaments_admins, user_sessions, users, users_roles
};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};  // , MigrationHarness};

// use pgtemp::PgTempDB;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub const TEST_DB_URL: &str = "TEST_DATABASE_URL";

pub const PAGE_NUM: i64 = 0;
pub const PAGE_SIZE: i64 = 10;

// pub fn establish_test_connection() -> PgConnection {
//     let database_url = std::env::var("TEST_DATABASE_URL")
//         .expect("TEST_DATABASE_URL must be set");
    
//     let mut conn = PgConnection::establish(&database_url)
//         .expect("Failed to connect to test database");
    
//     conn.run_pending_migrations(MIGRATIONS)
//         .expect("Failed to run migrations");
    
//     conn
// }

pub fn clean_database(conn: &mut database::Connection) {
    // establish_test_connection();  // mostly for running pending migrations
    // let db = Database::new(TEST_DB_URL);
    // let mut conn = db.get_connection().expect("Failed to get connection.");

    diesel::delete(roles_permissions::table)
        .execute(conn)
        .expect("Failed to clean roles_permissions");

    diesel::delete(users_roles::table)
        .execute(conn)
        .expect("Failed to clean users_roles");

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

    diesel::delete(rooms::table)
        .execute(conn)
        .expect("Failed to clean rooms");

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
    
    diesel::delete(divisions::table)
        .execute(conn)
        .expect("Failed to clean divisions");

    diesel::delete(tournaments::table)
        .execute(conn)
        .expect("Failed to clean tournaments");
    
    diesel::delete(users::table)
        .execute(conn)
        .expect("Failed to clean users");
}

// pub fn prepare_database() -> {

//     let db = PgTempDB::async_new().await;

//     let url = db.connection_uri();
    
//     let mut conn = PgConnection::establish(&url)
//         .expect("Failed to connect to test database");
    
//     conn.run_pending_migrations(MIGRATIONS)
//         .expect("Failed to run migrations");

//     db
// }
