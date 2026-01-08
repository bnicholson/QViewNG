use diesel::r2d2::{self, ConnectionManager, PooledConnection};

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
        let database_url =
            std::env::var(&db_url_env_var_name).expect(&format!("{} environment variable expected.", &db_url_env_var_name));
        let database_pool = Pool::builder()
            .connection_timeout(std::time::Duration::from_secs(5))
            .build(ConnectionManager::<DbCon>::new(database_url))
            .unwrap();

        Database {
            pool: database_pool,
        }
    }

    /// get a [`Connection`] to a database
    pub fn get_connection(&self) -> Connection {
        self.pool.get().unwrap()
    }
}