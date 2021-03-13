use crate::config::database_url;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, PoolError};
use diesel::Connection;
use r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> MysqlConnection {
    let database_url = database_url();
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn create_pool() -> DbPool {
    init_pool(database_url().as_str()).expect("Failed to create pool")
}
