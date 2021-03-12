#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;

pub mod config;
pub mod models;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = config::database_url();
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
