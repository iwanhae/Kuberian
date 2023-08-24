use std::env;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags;

pub fn new_connection_pool() -> Pool<SqliteConnectionManager> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| String::from("./kuberian.db"));
    let manager =
        SqliteConnectionManager::file(database_url).with_flags(OpenFlags::SQLITE_OPEN_READ_ONLY);
    Pool::new(manager).expect("fail to generate db connection pool")
}
