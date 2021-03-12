extern crate dotenv;

use crate::config::database_url;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

pub fn establish_connection() -> MysqlConnection {
    let database_url = database_url();
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn create_pool() -> DbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url());

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
