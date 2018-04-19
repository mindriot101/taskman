use opts::Opts;
use db::establish_connection;
use errors::Result;
use diesel::sqlite::SqliteConnection;

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
        match self.opts {
            ref opts @ Opts::Add { .. } => {
                println!("{:?}", opts);
            }
        }

        Ok(())
    }
}
