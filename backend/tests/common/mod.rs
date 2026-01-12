use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn establish_test_connection() -> PgConnection {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    
    let mut conn = PgConnection::establish(&database_url)
        .expect("Failed to connect to test database");
    
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    
    conn
}

pub fn clean_database(conn: &mut PgConnection) {
    diesel::sql_query("TRUNCATE TABLE your_tables CASCADE")
        .execute(conn)
        .expect("Failed to clean database");
}

// type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// pub fn create_test_pool() -> DbPool {
//     let database_url = std::env::var("TEST_DATABASE_URL")
//         .expect("TEST_DATABASE_URL must be set");
    
//     let manager = ConnectionManager::<PgConnection>::new(database_url);
//     r2d2::Pool::builder()
//         .max_size(1)
//         .build(manager)
//         .expect("Failed to create pool")
// }
