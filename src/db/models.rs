use super::schema::tasks;
use priority::Priority;

#[derive(Queryable, Identifiable)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub priority: Option<Priority>,
}

#[derive(Debug, Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub description: &'a str,
    pub priority: Option<Priority>,
}
