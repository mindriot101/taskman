use diesel;
use opts::Opts;
use db::establish_connection;
use errors::Result;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use db::models::NewTask;

pub struct TaskMan {
    connection: SqliteConnection,
    opts: Opts,
}

impl TaskMan {
    pub fn from_opts(opts: Opts) -> Result<Self> {
        let connection = establish_connection()?;
        Ok(TaskMan { connection, opts })
    }

    pub fn run(&mut self) -> Result<()> {
        // TODO: remove this clone
        let opts = self.opts.clone();
        match opts {
            Opts::Add {
                ref description, ..
            } => {
                self.add_task(description)?;
            }
        }

        Ok(())
    }

    fn add_task(&mut self, description: &str) -> Result<()> {
        use db::schema::tasks;

        let new_task = NewTask {
            description: description,
        };

        diesel::insert_into(tasks::table)
            .values(&new_task)
            .execute(&self.connection)?;
        Ok(())
    }
}
