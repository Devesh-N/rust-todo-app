use std::error::Error;
use sqlx::Connection;
use sqlx::Row;
// db_connection.rs
use crate::Task;
use rocket::serde::{json::Json, Deserialize, Serialize};

use sqlx::{Pool, Postgres};


pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = Pool::connect("postgres://postgres:postgres@todo-app-database.c69shgyjpvvb.us-east-1.rds.amazonaws.com/postgres").await?;
    Ok(pool)
}

pub async fn fetch_data(pool: &Pool<Postgres>) -> Result<i64, sqlx::Error> {
    log::info!("Fetching count of pending tasks");
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE pending = TRUE")
        .fetch_one(pool)
        .await?;
    log::info!("Fetched count: {}", count);

    Ok(count)
}

pub async fn update_task_by_name(pool: &Pool<Postgres>, task_name: &str, task: &Task) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE tasks SET pending = $1 WHERE name = $2")
        .bind(task.pending)
        .bind(task_name)
        .execute(pool)
        .await?;
    Ok(())
}


pub async fn delete_task_by_name(pool: &Pool<Postgres>, task_name: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM tasks WHERE name = $1")
        .bind(task_name)
        .execute(pool)
        .await?;
    Ok(())
}



pub async fn insert_task(pool: &Pool<Postgres>, task: &Task) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO tasks (name, pending) VALUES ($1, $2)")
        .bind(&task.name)
        .bind(task.pending)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn fetch_task_names(pool: &Pool<Postgres>) -> Result<Vec<(String, bool)>, sqlx::Error> {
    log::info!("Fetching task names and statuses");
    let tasks = sqlx::query_as::<_, (String, bool)>("SELECT name, pending FROM tasks")
        .fetch_all(pool)
        .await?;
    log::info!("Fetched {} tasks", tasks.len());

    Ok(tasks)
}

pub async fn fetch_completed_tasks_count(pool: &Pool<Postgres>) -> Result<i64, sqlx::Error> {
    log::info!("Fetching count of completed tasks");
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE pending = FALSE")
        .fetch_one(pool)
        .await?;
    log::info!("Fetched count: {}", count);

    Ok(count)
}