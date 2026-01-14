use diesel::r2d2::{self, ConnectionManager, PoolError, PooledConnection};

type DbCon = diesel::PgConnection;
// type DbCon = diesel::SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<DbCon>>;
pub type Connection = PooledConnection<ConnectionManager<DbCon>>;

#[derive(Clone)]
/// wrapper function for a database pool
pub struct Database {
    pub pool: Pool,
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
        }
    }

    /// get a [`Connection`] to a database
    pub fn get_connection(&self) -> Result<Connection, PoolError> {
        self.pool.get()
    }
}