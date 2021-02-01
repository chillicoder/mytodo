#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;

#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::Json;

use rocket_cors::{ AllowedHeaders, AllowedOrigins, Error};

use backend::db::query_task;
use mytodo::JsonApiResponse;
use mytodo::Task;
use rocket_contrib::databases::diesel;
use serde::Serialize;

#[database("sqlite_db")]
struct MyDatabase(diesel::SqliteConnection);

#[get("/tasks")]
fn tasks_get(conn: MyDatabase) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    for db_task in query_task(&conn) {
        let api_task = mytodo::Task {
            id: db_task.id,
            title: db_task.title,
        };
        response.data.push(api_task);
    }

    Json(response)
}

fn main() -> Result<(),Error> {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors()?;

    rocket::ignite()
        .attach(MyDatabase::fairing())
        .mount("/", routes![tasks_get])
        .attach(cors)
        .launch();

    Ok(())
}
