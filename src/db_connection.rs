use std::error::Error;
use sqlx::Connection;
use sqlx::Row;
// db_connection.rs
use sqlx::{Pool, Postgres};

pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = Pool::connect("postgres://postgres:tJGDccmOS0nhAchXSVGS@learning-db.c859oi58mdy2.ap-south-1.rds.amazonaws.com/postgres").await?;
    Ok(pool)
}
// In db_connection.rs
pub async fn fetch_data(pool: &Pool<Postgres>) -> Result<i64, sqlx::Error> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE pending = TRUE")
        .fetch_one(pool)
        .await?;

    Ok(count)
}


// In db_connection.rs
pub async fn fetch_task_names(pool: &Pool<Postgres>) -> Result<Vec<(String, bool)>, sqlx::Error> {
    let tasks = sqlx::query_as::<_, (String, bool)>("SELECT name, pending FROM tasks")
        .fetch_all(pool)
        .await?;

    Ok(tasks)
}

// In db_connection.rs
pub async fn fetch_completed_tasks_count(pool: &Pool<Postgres>) -> Result<i64, sqlx::Error> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE pending = FALSE")
        .fetch_one(pool)
        .await?;

    Ok(count)
}


