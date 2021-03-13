use crate::config::database_url;
use diesel::r2d2::{ConnectionManager, PoolError};
use diesel::{select, Connection, MysqlConnection, RunQueryDsl, sql_types};
use r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

no_arg_sql_function!(last_insert_id, sql_types::Binary);

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

pub fn get_last_insert_id(connection: &MysqlConnection) -> sql_types::Binary {
    select(last_insert_id).get_result::<sql_types::Binary>(connection).unwrap()
}
