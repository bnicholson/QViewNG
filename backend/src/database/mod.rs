use std::time::{Instant,Duration};
use diesel::r2d2::{self, ConnectionManager, PoolError, PooledConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use pgtemp::PgTempDB;

type DbCon = diesel::PgConnection;
// type DbCon = diesel::SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<DbCon>>;
pub type Connection = PooledConnection<ConnectionManager<DbCon>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Clone)]
/// wrapper function for a database pool
pub struct Database {
    pub pool: Pool,
    _temp_db: Option<std::sync::Arc<PgTempDB>>,
}

impl Database {
    /// create a new [`Database`]
    pub fn new(db_url_env_var_name: &str) -> Database {
        Self::new_with_pool_size(&db_url_env_var_name, false)
    }

    pub fn new_single_threaded(db_url_env_var_name: &str) -> Database {
        Self::new_with_pool_size(&db_url_env_var_name, true)
    }

    pub async fn new_prepared_in_memory_db() -> Database {
        
        let pool_size = 10;

        let temp_db = PgTempDB::async_new().await;

        tokio::time::sleep(Duration::from_millis(500)).await;

        let url = temp_db.connection_uri();

        let manager = ConnectionManager::<DbCon>::new(url);

        let database_pool = Pool::builder()
            .max_size(pool_size)
            .connection_timeout(std::time::Duration::from_secs(5))
            .build(manager)
            .unwrap();

        let start = Instant::now();
        let max_wait = Duration::from_secs(5);

        let mut conn = loop {
            match database_pool.get() {
                Ok(c) => break c,
                Err(e) => {
                    if start.elapsed() > max_wait {
                        panic!("Timed out waiting for temp Postgres to be ready: {e}");
                    }
                    eprintln!("Waiting for DB ({}s elapsed): {}", start.elapsed().as_secs(), e);
                    tokio::time::sleep(Duration::from_millis(200)).await;
                    continue;
                }
            }
        };

        conn.run_pending_migrations(MIGRATIONS)
            .expect("Migrations failed");

        Database {
            pool: database_pool,
            _temp_db: Some(std::sync::Arc::new(temp_db)),  // keep the temp DB alive
        }
    }

    pub fn new_with_pool_size(db_url_env_var_name: &str, single_threaded: bool) -> Database {

        let database_url =
            std::env::var(&db_url_env_var_name).expect(&format!("{} environment variable expected.", &db_url_env_var_name));
            
        let pool_size = if single_threaded { 1 } else { 10 };

        let database_pool = Pool::builder()
            .max_size(pool_size)
            .connection_timeout(std::time::Duration::from_secs(5))
            .build(ConnectionManager::<DbCon>::new(database_url))
            .unwrap();

        Database {
            pool: database_pool,
            _temp_db: None
        }
    }

    /// get a [`Connection`] to a database
    pub fn get_connection(&self) -> Result<Connection, PoolError> {
        self.pool.get()
    }
}