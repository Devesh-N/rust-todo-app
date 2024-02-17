#[macro_use]
extern crate rocket;
use reqwest;
use rocket::serde::json::{Value, json};
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};
use rocket::State;
use sqlx::{Pool, Postgres, Row};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Request};
use rocket::request::{self, FromRequest, Outcome};
use rocket::http::Status;

mod db_connection;
use db_connection::create_pool;
use db_connection::fetch_data;
use db_connection::fetch_task_names;
use db_connection::fetch_completed_tasks_count;
use db_connection::insert_task;
use db_connection::delete_task_by_name;
use db_connection::update_task_by_name;

#[derive(Deserialize, Serialize)]
pub struct Task {
    name: String,
    pending: bool,
}
struct DbConn(Pool<Postgres>);
struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

async fn validate_api_key(api_key: &str) -> Result<bool, reqwest::Error> {
    if api_key.is_empty() {
        return Ok(false);
    }

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:9090/validate-api-key")
        .json(&json!({"APIKey": api_key}))
        .send()
        .await?;

    if res.status() != reqwest::StatusCode::OK {
        return Ok(false);
    }

    let json_response: Value = res.json().await?;
    Ok(json_response["isValid"].as_bool().unwrap_or(false))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        async fn is_valid(key: &str) -> bool {
            match validate_api_key(key).await {
                Ok(valid) => valid,
                Err(_) => false,
            }
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key).await => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}



#[rocket::get("/")]
async fn index(_api_key: ApiKey<'_>, state: &State<DbConn>) -> Value {
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

#[rocket::put("/task/<name>", format = "json", data = "<task>")]
async fn update_task(name: String, task: Json<Task>, state: &State<DbConn>) -> Value {
    match update_task_by_name(&state.0, &name, &task.into_inner()).await {
        Ok(_) => json!({"status": "success"}),
        Err(e) => {
            log::error!("Failed to update task: {}", e);
            json!({"status": "error"})
        },
    }
}


#[rocket::delete("/task/<name>")]
async fn delete_task(name: String, state: &State<DbConn>) -> Value {
    match delete_task_by_name(&state.0, &name).await {
        Ok(_) => json!({"status": "success"}),
        Err(e) => {
            log::error!("Failed to delete task: {}", e);
            json!({"status": "error"})
        }
    }
}



#[rocket::launch]
async fn rocket() -> _ {
    env_logger::init();  // Initialize the logger
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::All, // You might want to be more restrictive
        // ... other CORS options ...
        ..Default::default()
    };
    let db_pool = create_pool().await.expect("database pool failed to initialize");

    rocket::build()
        .manage(DbConn(db_pool))
        .mount("/", rocket::routes![index, add_task, delete_task, update_task])
}

