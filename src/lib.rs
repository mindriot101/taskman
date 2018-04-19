extern crate diesel;
extern crate dotenv;
extern crate failure;
#[macro_use]
extern crate structopt;

pub mod opts;
mod db;
pub mod errors;

use errors::Result;
use diesel::sqlite::SqliteConnection;

pub struct TaskMan {
    connection: SqliteConnection,
}

impl TaskMan {
    pub fn from_opts(opts: opts::Opts) -> Result<Self> {
        let connection = db::establish_connection()?;
        Ok(TaskMan { connection })
    }

    pub fn run(&mut self) {}
}
