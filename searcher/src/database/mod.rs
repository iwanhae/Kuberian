pub mod models;
pub mod schema;

use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| String::from("./kuberian.db"));
    let manger = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manger)
        .expect("Could not build connection pool")
}
