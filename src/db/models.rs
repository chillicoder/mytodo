use super::schema::task;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
}

#[derive(Queryable, Identifiable, Serialize)]
#[table_name = "task"]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

