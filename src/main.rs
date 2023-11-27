#[macro_use]
extern crate rocket;
use rocket::serde::json::{Value, json};
use std::error::Error;
use rocket::futures::future::Select;
use sqlx::Connection;
use sqlx::Row;
#[rocket::tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let url = "postgres://postgres:tJGDccmOS0nhAchXSVGS@learning-db.c859oi58mdy2.ap-south-1.rds.amazonaws.com/postgres";
    let mut conn = sqlx::postgres::PgConnection::connect(url).await?;
    let res = sqlx::query("SELECT * FROM tasks")
        .fetch_one(&mut conn)
        .await?;
    let stringgot: String = res.get("sum");
    println!("{}", stringgot);
    Ok(())
}


#[rocket::get("/")]
fn index() -> Value {


    let item_num = 0;
    let tasks = "nothing";


    let json_response = json!({
        "Number of tasks": item_num,
        "Current Tasks": tasks
    });
    json_response
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![index])
}
// fn test() {
//     db();
// }
