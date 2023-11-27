use std::error::Error;
use sqlx::Connection;
use sqlx::Row;
// db_connection.rs
use sqlx::{Pool, Postgres};

pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = Pool::connect("postgres://postgres:tJGDccmOS0nhAchXSVGS@learning-db.c859oi58mdy2.ap-south-1.rds.amazonaws.com/postgres").await?;
    Ok(pool)
}
pub async fn fetch_data(pool: &Pool<Postgres>) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as("SELECT id FROM tasks")
        .fetch_one(pool)
        .await?;

    Ok(row.0)
}

