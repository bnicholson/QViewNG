use backend::database::Database;
use backend::schema::{divisions, tournaments, tournaments_admins, users, rooms, rounds, teams};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

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
