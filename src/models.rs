use chrono::naive::NaiveDateTime;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub due_date: Option<NaiveDateTime>,
    pub completed: bool,
}
