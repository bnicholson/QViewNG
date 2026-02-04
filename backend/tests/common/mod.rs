use backend::database::Database;
use backend::schema::{
    divisions, 
    games, 
    rooms, 
    rounds, 
    statsgroups, 
    teams, 
    tournamentgroups, 
    tournamentgroups_tournaments, 
    tournaments, 
    tournaments_admins, 
    users, 
    rosters,
    rosters_quizzers,
    rosters_coaches,
    equipmentsets,
    equipment
};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
// use pgtemp::PgTempDB;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub const TEST_DB_URL: &str = "TEST_DATABASE_URL";

pub const PAGE_NUM: i64 = 0;
pub const PAGE_SIZE: i64 = 10;

pub fn establish_test_connection() -> PgConnection {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    
    let mut conn = PgConnection::establish(&database_url)
        .expect("Failed to connect to test database");
    
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    
    conn
}

pub fn clean_database() {
    establish_test_connection();  // mostly for running pending migrations
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    diesel::delete(equipment::table)
        .execute(&mut conn)
        .expect("Failed to clean equipment");

    diesel::delete(equipmentsets::table)
        .execute(&mut conn)
        .expect("Failed to clean equipmentsets");

    diesel::delete(rosters_coaches::table)
        .execute(&mut conn)
        .expect("Failed to clean rosters_coaches");

    diesel::delete(rosters_quizzers::table)
        .execute(&mut conn)
        .expect("Failed to clean rosters_quizzers");

    diesel::delete(rosters::table)
        .execute(&mut conn)
        .expect("Failed to clean rosters");

    diesel::delete(statsgroups::table)
        .execute(&mut conn)
        .expect("Failed to clean statsgroups");

    diesel::delete(tournamentgroups_tournaments::table)
        .execute(&mut conn)
        .expect("Failed to clean tournamentgroups_tournaments");

    diesel::delete(tournamentgroups::table)
        .execute(&mut conn)
        .expect("Failed to clean tournamentgroups");

    diesel::delete(games::table)
        .execute(&mut conn)
        .expect("Failed to clean games");

    diesel::delete(teams::table)
        .execute(&mut conn)
        .expect("Failed to clean teams");

    diesel::delete(rounds::table)
        .execute(&mut conn)
        .expect("Failed to clean rounds");

    diesel::delete(rooms::table)
        .execute(&mut conn)
        .expect("Failed to clean rooms");

    diesel::delete(tournaments_admins::table)
        .execute(&mut conn)
        .expect("Failed to clean admins of tournaments");

    diesel::delete(users::table)
        .execute(&mut conn)
        .expect("Failed to clean users");

    diesel::delete(divisions::table)
        .execute(&mut conn)
        .expect("Failed to clean divisions");

    diesel::delete(tournaments::table)
        .execute(&mut conn)
        .expect("Failed to clean tournaments");
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
