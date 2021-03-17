use super::config::database_url;
use diesel::r2d2::{ConnectionManager, PoolError};
use diesel::{Connection, PgConnection};
use r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    let database_url = database_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn create_pool() -> DbPool {
    init_pool(database_url().as_str()).expect("Failed to create pool")
}
