#[macro_use]
extern crate rocket;

use rocket::serde::json::{Value, json};
use rocket::State;
use sqlx::{Pool, Postgres, Row};  // Add this line to import Pool

mod db_connection;
use db_connection::create_pool;
use db_connection::fetch_data;
use db_connection::fetch_task_names;

struct DbConn(Pool<Postgres>);  // Now Pool is recognized here

#[rocket::get("/")]
async fn index(state: &State<DbConn>) -> Value {
    // Fetch the number of tasks
    let item_num = match fetch_data(&state.0).await {
        Ok(count) => count.to_string(),
        Err(_) => "Error fetching data".to_string(),
    };

    // Fetch the task names
    let tasks = match fetch_task_names(&state.0).await {
        Ok(names) => names.join(", "), // Join the task names with a comma
        Err(_) => "Error fetching tasks".to_string(),
    };

    // Construct the JSON response
    json!({
        "Number of tasks": item_num,
        "Current Tasks": tasks
    })
}

#[rocket::get("/data")]
async fn get_data(state: &State<DbConn>) -> Value {
    match fetch_data(&state.0).await {
        Ok(data) => json!({"data": data.to_string()}),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            json!({"error": format!("Failed to fetch data: {:?}", e)})
        },
    }
}


#[rocket::launch]
async fn rocket() -> _ {
    let db_pool = create_pool().await.expect("database pool failed to initialize");

    rocket::build()
        .manage(DbConn(db_pool))
        .mount("/", rocket::routes![index, get_data])
}
