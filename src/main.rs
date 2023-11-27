#[macro_use]
extern crate rocket;
// use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Value, json};
use rocket::State;
use sqlx::{Pool, Postgres, Row};  // Add this line to import Pool
// use sqlx::types::Json;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Task {
    name: String,
    pending: bool,
}
mod db_connection;
use db_connection::create_pool;
use db_connection::fetch_data;
use db_connection::fetch_task_names;
use db_connection::fetch_completed_tasks_count;
use db_connection::insert_task;

struct DbConn(Pool<Postgres>);  // Now Pool is recognized here

#[rocket::get("/")]
async fn index(state: &State<DbConn>) -> Value {
    log::info!("Received request to '/' endpoint");
    // Fetch the number of pending tasks
    let pending_tasks_count = match fetch_data(&state.0).await {
        Ok(count) => count.to_string(),
        Err(_) => "Error fetching data".to_string(),
    };

    // Fetch the number of completed tasks
    let completed_tasks_count = match fetch_completed_tasks_count(&state.0).await {
        Ok(count) => count.to_string(),
        Err(_) => "Error fetching data".to_string(),
    };

    // Fetch the task names and pending status
    let tasks = match fetch_task_names(&state.0).await {
        Ok(task_list) => {
            let mut tasks_map = serde_json::Map::new();
            for (name, pending) in task_list {
                tasks_map.insert(name, serde_json::Value::Bool(pending));
            }
            serde_json::Value::Object(tasks_map)
        },
        Err(_) => serde_json::Value::String("Error fetching tasks".to_string()),
    };

    // Construct the JSON response
    log::info!("Processed request to '/' endpoint");
    log::info!("Proceeding to send json");
    json!({
        "Number of Pending Tasks": pending_tasks_count,
        "Number of Completed Tasks": completed_tasks_count,
        "Tasks": tasks
    })
}


#[rocket::post("/", format = "json", data = "<task>")]
async fn add_task(task: Json<Task>, state: &State<DbConn>) -> Value {
    match insert_task(&state.0, &task.into_inner()).await {
        Ok(_) => json!({"status": "success"}),
        Err(e) => {
            log::error!("Failed to insert task: {}", e);
            json!({"status": "error"})
        },
    }
}





#[rocket::launch]
async fn rocket() -> _ {
    env_logger::init();  // Initialize the logger

    let db_pool = create_pool().await.expect("database pool failed to initialize");

    rocket::build()
        .manage(DbConn(db_pool))
        .mount("/", rocket::routes![index, add_task])
}

