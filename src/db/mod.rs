pub mod schema;
pub mod models;

use errors::Result;
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;

pub(crate) fn establish_connection() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    SqliteConnection::establish(&database_url).map_err(From::from)
}
