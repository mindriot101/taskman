use opts::Opts;
use db::establish_connection;
use priority::Priority;
use errors::Result;
use diesel::sqlite::SqliteConnection;
use db::models::{NewTask, Task};
use std::io::{self, Write};
use diesel::prelude::*;

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
                description,
                priority,
                ..
            } => {
                self.add_task(description, priority)?;
            }

            Opts::List {} => {
                self.list_tasks()?;
            }
        }

        Ok(())
    }

    fn add_task(&mut self, description: String, priority: Option<Priority>) -> Result<()> {
        NewTask::new(&description, priority).create(&self.connection)?;
        Ok(())
    }

    fn list_tasks(&mut self) -> Result<()> {
        use db::schema::tasks::dsl::*;
        let results = tasks.load::<Task>(&self.connection)?;

        let stdout = io::stdout();
        let mut handle = stdout.lock();
        Presenter::new(&results).write(&mut handle)?;

        Ok(())
    }
}

struct Presenter<'a> {
    tasks: &'a [Task],
}

impl<'a> Presenter<'a> {
    fn new(tasks: &'a [Task]) -> Self {
        Presenter { tasks }
    }

    fn write(&self, handle: &mut io::StdoutLock) -> Result<()> {
        self.write_header(handle)?;
        Ok(())
    }

    fn write_header(&self, handle: &mut io::StdoutLock) -> Result<()> {
        writeln!(handle, "{:6} {}", "Number", "Description");
        writeln!(handle, "====== ===========");

        for (i, task) in self.tasks.iter().enumerate() {
            writeln!(handle, "{:6} {}", i + 1, task.description);
        }

        Ok(())
    }
}
