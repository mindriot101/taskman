#[derive(Queryable)]
pub struct Task {
    pub id: u32,
    pub description: String,
}
