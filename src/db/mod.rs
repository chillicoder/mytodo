use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn mark_task(connection: &SqliteConnection, task_id: i32, _status: bool) -> models::Task {
    let mut old_task = schema::task::table
        .filter(schema::task::id.eq(task_id))
        .first::<models::Task>(connection)
        .expect("error loading task");

    let updated_rows = diesel::update(&old_task).set(schema::task::done.eq(true))
        .execute(connection)
        .expect("error updating task");

    if updated_rows > 0 {
        old_task.done = true;
    }

    println!("updated rows: {}", updated_rows);

    return old_task;
}

pub fn create_task(connection: &SqliteConnection, title: &str) {
    let task = models::NewTask { title };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("error inserting new task");
}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("error loading tasks")
}

