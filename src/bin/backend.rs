#![feature(proc_macro_hygiene, decl_macro)]

use mytodo::db::models::Task;
use mytodo::db::query_task;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use serde::Serialize;

#[macro_use]
extern crate rocket;
extern crate serde;

#[macro_use]
extern crate rocket_contrib;

#[database("sqlite_db")]
struct MyDatabase(diesel::SqliteConnection);

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Task>,
}

#[get("/tasks")]
fn tasks_get(conn: MyDatabase) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    for task in query_task(&*conn) {
        response.data.push(task);
    }

    Json(response)
}

fn main() {
    rocket::ignite()
        .attach(MyDatabase::fairing())
        .mount("/", routes![tasks_get])
        .launch();
}
