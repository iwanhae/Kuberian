use std::env;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub fn new_connection_pool() -> Pool<SqliteConnectionManager> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| String::from("./kuberian.db"));
    let manager = SqliteConnectionManager::file(database_url);
    let pool = Pool::new(manager).expect("fail to generate db connection pool");

    pool
}
