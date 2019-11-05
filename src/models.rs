use chrono::naive::NaiveDateTime;
use super::schema::tasks;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub due_date: Option<NaiveDateTime>,
    pub completed: bool,
}

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewTask<'a> {
    pub title: Option<&'a str>,
    pub due_date: Option<NaiveDateTime>,
}
