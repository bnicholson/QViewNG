use backend::{database::Database, schema::tournaments};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub const TEST_DB_URL: &str = "TEST_DATABASE_URL";

pub fn establish_test_connection() -> PgConnection {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    
    let mut conn = PgConnection::establish(&database_url)
        .expect("Failed to connect to test database");
    
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    
    conn
}

// pub fn clean_database(conn: &mut PgConnection) {
//     diesel::sql_query("TRUNCATE TABLE your_tables CASCADE")
//         .execute(conn)
//         .expect("Failed to clean database");
// }

pub fn clean_database() {
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    // *includes all cascading deletes
    diesel::delete(tournaments::table)
        .execute(&mut conn)
        .expect("Failed to clean tournaments");
}

// pub async fn create_test_transaction(pool: &PgPool) -> Transaction<'_, Postgres> {
//     pool.begin().await.expect("Failed to begin transaction")
// }

// pub async fn setup_test_pool() -> PgPool {
//     let database_url = std::env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set for tests");
    
//     PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&database_url)
//         .await
//         .expect("Failed to create pool")
// }
