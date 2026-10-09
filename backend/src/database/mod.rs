pub mod seed_data;
pub mod clean_db;

use std::any::Any;
use std::sync::Arc;
use diesel::r2d2::{self, ConnectionManager, PoolError, PooledConnection};
use diesel_migrations::{EmbeddedMigrations, embed_migrations};

type DbCon = diesel::PgConnection;
// type DbCon = diesel::SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<DbCon>>;
pub type Connection = PooledConnection<ConnectionManager<DbCon>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Clone)]
/// wrapper function for a database pool
pub struct Database {
    pub pool: Pool,
    /// Anything that must live exactly as long as this database handle and all its clones — the
    /// integration tests park a guard here that drops their per-test database when the last
    /// clone goes away. Declared after `pool` so the pool is released first.
    _keepalive: Option<Arc<dyn Any + Send + Sync>>,
}

impl Database {
    /// create a new [`Database`]
    pub fn new(db_url_env_var_name: &str) -> Database {
        Self::new_with_pool_size(&db_url_env_var_name, false)
    }

    pub fn new_single_threaded(db_url_env_var_name: &str) -> Database {
        Self::new_with_pool_size(&db_url_env_var_name, true)
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
            _keepalive: None
        }
    }

    /// A pool for `database_url` itself (rather than an env var naming it) that opens connections
    /// lazily: one up front, more only as they're needed, up to `max_size`. `new` instead blocks
    /// until all of its connections are open, which suits the long-lived server but makes every
    /// short-lived integration-test database pay for ten connections it rarely uses.
    pub fn from_url(database_url: &str, max_size: u32) -> Database {
        let database_pool = Pool::builder()
            .max_size(max_size)
            .min_idle(Some(1))
            .connection_timeout(std::time::Duration::from_secs(5))
            .build(ConnectionManager::<DbCon>::new(database_url))
            .unwrap();

        Database {
            pool: database_pool,
            _keepalive: None
        }
    }

    /// Ties `keepalive` to this handle and every clone of it: it's dropped only once the last one is.
    pub fn with_keepalive(mut self, keepalive: Arc<dyn Any + Send + Sync>) -> Database {
        self._keepalive = Some(keepalive);
        self
    }

    /// get a [`Connection`] to a database
    pub fn get_connection(&self) -> Result<Connection, PoolError> {
        self.pool.get()
    }
}