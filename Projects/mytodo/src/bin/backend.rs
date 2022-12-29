#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

use mytodo::db::models::Task;
use mytodo::db::{establish_connection, query_task};
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Task>,
}

#[get("/tasks")]
fn tasks_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    let conn = establish_connection();
    for task in query_task(&conn) {
        response.data.push(task);
    }

    Json(response)
}

fn main() {
    rocket::ignite().mount("/", routes![tasks_get]).launch();
}
