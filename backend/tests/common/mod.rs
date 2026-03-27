use backend::database::Database;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;
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

/// Mint a signed JWT using the default dev secret, with the given roles and
/// permissions embedded in the claims. The token is valid for 1 hour.
/// Use this to supply an `Authorization: Bearer <token>` header in tests that
/// hit auth-gated endpoints.
pub fn make_token(user_id: Uuid, roles: Vec<String>, permissions: Vec<String>) -> String {
    let exp = (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize;
    let claims = serde_json::json!({
        "sub": user_id.to_string(),
        "exp": exp,
        "roles": roles,
        "permissions": permissions,
    });
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"qview_dev_secret_changeme"),
    ).expect("Failed to encode test JWT")
}

pub fn clean_database() {
    establish_test_connection();  // mostly for running pending migrations
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    backend::database::clean_db::clean_database(&mut conn);
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
