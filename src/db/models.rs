use super::schema::tasks;

#[derive(Queryable)]
pub struct Task {
    pub id: u32,
    pub description: String,
}

#[derive(Debug, Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub description: &'a str,
}
