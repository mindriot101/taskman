use super::schema::tasks;
use priority::Priority;
use diesel::{self, result, RunQueryDsl, sqlite::SqliteConnection};

#[derive(Debug, Queryable, Identifiable)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub priority: Option<Priority>,
}

#[derive(Debug, Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub description: &'a str,
    pub priority: Option<Priority>,
}

impl<'a> NewTask<'a> {
    pub fn new(description: &'a str, priority: Option<Priority>) -> Self {
        NewTask {
            description,
            priority,
        }
    }

    pub fn create(&self, connection: &SqliteConnection) -> Result<usize, result::Error> {
        diesel::insert_into(tasks::table)
            .values(self)
            .execute(connection)
    }
}
