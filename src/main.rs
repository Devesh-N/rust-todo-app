#[macro_use]
extern crate rocket;

use rocket::serde::json::{Value, json};
use rocket::State;
use sqlx::{Pool, Postgres, Row};  // Add this line to import Pool

mod db_connection;
use db_connection::create_pool;
use db_connection::fetch_data;

struct DbConn(Pool<Postgres>);  // Now Pool is recognized here

#[rocket::get("/")]
async fn index(state: &State<DbConn>) -> Value {
    // Attempt to fetch data from the database
    let item_num = match fetch_data(&state.0).await {
        Ok(data) => data.to_string(), // Convert the data to a string if successful
        Err(_) => "Error fetching data".to_string(), // Handle error case
    };

    let tasks = "nothing"; // This is a static string, as per your original code

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
