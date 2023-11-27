use std::error::Error;
use sqlx::Connection;
use sqlx::Row;
// db_connection.rs
use sqlx::{Pool, Postgres};

pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = Pool::connect("postgres://postgres:tJGDccmOS0nhAchXSVGS@learning-db.c859oi58mdy2.ap-south-1.rds.amazonaws.com/postgres").await?;
    Ok(pool)
}
pub async fn fetch_data(pool: &Pool<Postgres>) -> Result<i64, sqlx::Error> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks")
        .fetch_one(pool)
        .await?;

    Ok(count)
}

pub async fn fetch_task_names(pool: &Pool<Postgres>) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String,)>("SELECT name FROM tasks")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.0)
        .collect();

    Ok(rows)
}

